use async_nats::client::PublishErrorKind;
use axum::extract::{Request, State};
use axum::http::HeaderMap;
use axum::{
    extract::ws::CloseFrame,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    extract::ConnectInfo,
    extract::Query,
    response::IntoResponse,
};
use bevygap_shared::protocol::RequestSession;
use log::*;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio_stream::StreamExt as _;

use crate::AppState;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub(crate) struct QsParams {
    #[serde(default, deserialize_with = "crate::empty_string_as_none")]
    client_ip: Option<String>,
}

// Chunked transfer to give streaming results
// maybe the service immediately yields a key so you can sub to a stream like
// session_requests.1234567890
// and then results are streamed back on that?

/// The handler for the HTTP request (this gets called when the HTTP request lands at the start
/// of websocket negotiation). After this completes, the actual switching from HTTP to
/// websocket protocol will occur.
/// This is the last point where we can extract TCP/IP metadata such as IP address of the client
/// as well as things from HTTP headers such as user-agent of the browser etc.
pub(crate) async fn handler_websocket(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Query(params): Query<QsParams>,
    State(state): State<Arc<AppState>>,
    ws: WebSocketUpgrade,
    req: Request,
) -> impl IntoResponse {
    let client_ip = get_client_ip(&params, &addr, req.headers(), &state);

    info!("ws responder for ip {client_ip}");
    // finalize the upgrade process by returning upgrade callback.
    // we can customize the callback by sending additional info such as address.
    ws.on_upgrade(move |socket| handle_socket(socket, client_ip, state))
}

async fn read_initial_request_message(
    socket: &mut WebSocket,
    timeout: Duration,
) -> Result<RequestSession, String> {
    if let Ok(Some(msg)) = tokio::time::timeout(timeout, socket.recv()).await {
        if let Ok(msg) = msg {
            info!("< {msg:?}");
            if let Message::Text(text) = msg {
                match serde_json::from_str::<RequestSession>(&text) {
                    Ok(request) => Ok(request),
                    Err(e) => Err(format!(
                        "Failed to parse request message as RequestSession: {}",
                        e
                    )),
                }
            } else {
                Err(format!("Expected text message, got {msg:?}"))
            }
        } else {
            Err(format!("Failed to receive request message: {msg:?}"))
        }
    } else {
        Err(format!(
            "Timeout waiting for websocket request message: {timeout:?}"
        ))
    }
}

/// Actual websocket statemachine (one will be spawned per connection)
///
async fn handle_socket(mut socket: WebSocket, client_ip: String, state: Arc<AppState>) {
    // all errors are strings that we send back to the client.
    match handle_socket_inner(&mut socket, client_ip, state).await {
        Ok(()) => {
            let _ = socket
                .send(Message::Close(Some(CloseFrame {
                    code: axum::extract::ws::close_code::NORMAL,
                    reason: std::borrow::Cow::from("Goodbye"),
                })))
                .await;
        }
        Err(s) => {
            warn!("{s}");
            let _ = socket.send(Message::Text(format!("ERR {s}"))).await;
            // let _ = socket
            //     .send(Message::Close(Some(CloseFrame {
            //         code: axum::extract::ws::close_code::ERROR,
            //         reason: std::borrow::Cow::from("Goodbye"),
            //     })))
            //     .await;
        }
    }
    info!("websocket connection closed");
}

/// We await the request message, which includes the game name and version,
/// send a request to NATS, subscribe to replies, and stream back to the client.
async fn handle_socket_inner(
    socket: &mut WebSocket,
    client_ip: String,
    state: Arc<AppState>,
) -> Result<(), String> {
    // Await the request message the client should send once the websocket is connected.
    let request_session = read_initial_request_message(socket, Duration::from_secs(10)).await?;

    let (game_name, game_ver) = request_session.game_name_and_version()?;

    let subject = format!("matchmaker.request.{game_name}.{game_ver}");

    // this should be safe because of our regex check on name and version..
    let payload = format!(
        "{{\"client_ip\":\"{client_ip}\", \"game\":\"{game_name}\", \"version\":\"{game_ver}\"}}"
    );

    info!("Sending request to {subject} with payload {payload}");

    let client = state.bgnats.client().clone();
    let reply_inbox = client.new_inbox();
    let mut response_subscriber = client.subscribe(reply_inbox.to_owned()).await.unwrap();
    // TODO this publish needs to "opt in to no_responder messages" somehow, per
    // https://docs.nats.io/reference/reference-protocols/nats-protocol
    client
        .publish_with_reply(subject, reply_inbox, payload.into())
        .await
        .map_err(|e| match e.kind() {
            PublishErrorKind::Send => "Failed to send mm request".to_string(),
            PublishErrorKind::MaxPayloadExceeded => "Request too large".to_string(),
        })?;

    // now we wait for response messages on this nats inbox, and send back to ws client.
    // receiving an empty message from nats means the end of stream.

    while let Some(msg) = response_subscriber.next().await {
        if msg.payload.is_empty() {
            info!("got empty response, breaking");
            break;
        }
        let chunk = String::from_utf8(msg.payload.to_vec()).unwrap();
        info!("> {chunk}");
        if socket.send(Message::Text(chunk)).await.is_err() {
            return Err("Can't send chunk to ws client".to_string());
        }
    }
    Ok(())
}

/// Logic to decide what to use as the clients IP address for the purposes of Edgegap sessions.
///
/// In order of preference:
/// ?client_ip=XXX querystring param (for dev)
/// X-Forwarded-For header (for proxy setups)
/// the source IP of the http client
///
/// Additionally if they above yields a localhost address, we replace it with
/// settings.fake_ip, which is also useful for dev.
fn get_client_ip(
    params: &QsParams,
    addr: &SocketAddr,
    headers: &HeaderMap,
    state: &AppState,
) -> String {
    // client_ip is the one sent to Edgegap, to decide which server to assign the player to.
    // We use one provided in the qs, otherwise the connecting IP of the http client.
    let mut client_ip = params.client_ip.clone().unwrap_or(addr.ip().to_string());

    // Check for X-Forwarded-For header, since this is probably running behind a proxy
    if let Some(forwarded_for) = headers.get("X-Forwarded-For") {
        if let Ok(forwarded_ip) = forwarded_for.to_str() {
            // Use the first IP in the X-Forwarded-For header
            if let Some(first_ip) = forwarded_ip.split(',').next() {
                client_ip = first_ip.trim().to_string();
                info!("Using X-Forwarded-For IP: {}", client_ip);
            }
        }
    }
    // TODO this default IP should be configurable too, for easier dev
    if client_ip == "127.0.0.1" || client_ip == "::1" {
        // localhost tends to spawn deployments in random places..
        client_ip = state.settings.fake_ip.to_string();
        warn!("Using fake IP, request came from localhost: {client_ip}");
    }
    client_ip
}
