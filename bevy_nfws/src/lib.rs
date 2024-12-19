use async_channel::{unbounded, Receiver, Sender, TryRecvError};
use bevy::prelude::*;
use bevy_async_task::AsyncTaskPool;
use futures_util::{select, FutureExt, SinkExt, StreamExt};
use tokio_tungstenite_wasm::CloseCode;

pub mod prelude {
    pub use super::{NfwsCmd, NfwsErr, NfwsEvent, NfwsHandle, NfwsPlugin, NfwsPollResult};
}

pub struct NfwsPlugin;

impl Plugin for NfwsPlugin {
    fn build(&self, app: &mut App) {
        app.observe(start_new_ws_tasks);
    }
}

fn start_new_ws_tasks(
    trigger: Trigger<OnAdd, NfwsHandle>,
    mut task_pool: AsyncTaskPool<()>,
    mut q: Query<&mut NfwsHandle>,
) {
    let mut wschan = q.get_mut(trigger.entity()).unwrap();
    let cmd_rx = wschan.cmd_rx.take().unwrap();
    let ev_tx = wschan.ev_tx.take().unwrap();
    let url = wschan.ws_url.clone();
    debug!("spawned ws task for {:?}", trigger.entity());
    task_pool.spawn(async move {
        let ev_tx2 = ev_tx.clone();
        let ret = connect_websocket(url, cmd_rx, ev_tx).await;
        debug!("connect_websocket returned: {:?}", ret);
        match ret {
            Ok(()) => {
                // a WsEvent::Closed should have been sent.
            }
            Err(err) => {
                let _ = ev_tx2.send(NfwsEvent::Error(err)).await;
            }
        };
    });
}

#[derive(Debug, Clone)]
pub enum NfwsErr {
    Connecting,
    Receiving(String),
    Sending(String),
}

#[derive(Debug, Clone)]
pub enum NfwsEvent {
    Connecting,
    Connected,
    TextMessage(String),
    BinaryMessage(Vec<u8>),
    Error(NfwsErr),
    Closed(Result<String, String>),
}

#[derive(Debug, Clone)]
pub enum NfwsCmd {
    SendTextMessage(String),
    SendBinaryMessage(Vec<u8>),
    Disconnect,
}

#[derive(Debug, Component)]
pub struct NfwsHandle {
    cmd_tx: Sender<NfwsCmd>,
    cmd_rx: Option<Receiver<NfwsCmd>>,
    ev_tx: Option<Sender<NfwsEvent>>,
    ev_rx: Receiver<NfwsEvent>,
    ws_url: String,
}

/// Result of calling NfwsHandle::next_event.
#[derive(Debug)]
pub enum NfwsPollResult {
    /// Channel closed, socket gone.
    Closed,
    /// No messages ready.
    Empty,
    /// Messages received, ask again until Empty or Closed.
    Event(NfwsEvent),
}

impl NfwsHandle {
    pub fn new(ws_url: String) -> Self {
        let (cmd_tx, cmd_rx) = unbounded::<NfwsCmd>();
        let (ev_tx, ev_rx) = unbounded::<NfwsEvent>();
        Self {
            cmd_tx,
            cmd_rx: Some(cmd_rx),
            ev_tx: Some(ev_tx),
            ev_rx,
            ws_url,
        }
    }
    /// Consumes a (potentially) received event from the websocket
    pub fn next_event(&mut self) -> NfwsPollResult {
        match self.ev_rx.try_recv() {
            Ok(ev) => NfwsPollResult::Event(ev),
            Err(TryRecvError::Empty) => NfwsPollResult::Empty,
            Err(TryRecvError::Closed) => NfwsPollResult::Closed,
        }
    }

    pub fn send_text(&mut self, msg: String) -> bool {
        self.cmd_tx.try_send(NfwsCmd::SendTextMessage(msg)).is_ok()
    }
}

async fn connect_websocket(
    url: String,
    cmd_rx: Receiver<NfwsCmd>,
    ev_tx: Sender<NfwsEvent>,
) -> Result<(), NfwsErr> {
    let _ = ev_tx.send(NfwsEvent::Connecting).await;
    let Ok(ws) = tokio_tungstenite_wasm::connect(url).await else {
        return Err(NfwsErr::Connecting);
    };
    let (mut ws_sender, mut ws_receiver) = ws.split();

    debug!("Connected to ws server.");
    let _ = ev_tx.send(NfwsEvent::Connected).await;

    loop {
        let mut ws_recv = ws_receiver.next().fuse();
        let mut cmd_recv = Box::pin(cmd_rx.recv()).fuse();
        select! {
            // Handle incoming messages from websocket
            msg = ws_recv => {
                match msg {
                    Some(Ok(msg)) => {
                        debug!("Received message from server: {:?}", msg);
                        match msg {
                            tokio_tungstenite_wasm::Message::Text(msg) => {
                                let _ = ev_tx.send(NfwsEvent::TextMessage(msg)).await;
                            }
                            tokio_tungstenite_wasm::Message::Binary(msg) => {
                                let _ = ev_tx.send(NfwsEvent::BinaryMessage(msg)).await;
                            }
                            tokio_tungstenite_wasm::Message::Close(close_frame) => {
                                debug!("ws recv channel closed");
                                let ev = match close_frame {
                                    None => NfwsEvent::Closed(Ok("".to_string())),
                                    Some(close_frame) => {
                                        if close_frame.code == CloseCode::Normal {
                                            NfwsEvent::Closed(Ok(close_frame.reason.to_string()))
                                        } else {
                                            NfwsEvent::Closed(Err(format!("{:?} - {:?}", close_frame.code, close_frame.reason)))
                                        }
                                    }
                                };
                                let _ = ev_tx.send(ev).await;
                                return Ok(());
                            }
                        }
                    }
                    Some(Err(e)) => {
                        return Err(NfwsErr::Receiving(format!("Error receiving message: {:?}", e)));
                    }
                    None => {
                        debug!("ws recv channel closed");
                        return Ok(());
                    }
                }
            }

            // Handle commands from our application code
            cmd = cmd_recv => {
                debug!("ws cmd: {:?}", cmd);
                match cmd {
                    Err(e) => {
                        info!("ws channel err {e:?}");
                        return Ok(());
                    }
                    Ok(NfwsCmd::SendTextMessage(msg)) => {
                        if let Err(e) = ws_sender.send(tokio_tungstenite_wasm::Message::Text(msg)).await {
                            return Err(NfwsErr::Sending(format!(
                                "Error sending message: {:?}",
                                e
                            )));
                        }
                    }
                    Ok(NfwsCmd::SendBinaryMessage(msg)) => {
                        if let Err(e) = ws_sender.send(tokio_tungstenite_wasm::Message::Binary(msg)).await {
                            return Err(NfwsErr::Sending(format!(
                                "Error sending message: {:?}",
                                e
                            )));
                        }
                    }
                    Ok(NfwsCmd::Disconnect) => {
                        debug!("Received disconnect command");
                        break;
                    }
                }
            }
        }
    }
    Ok(())
}
