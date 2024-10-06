// use async_nats::jetstream;
// use async_nats::jetstream::stream::StorageType;
// use crate::arbitrium_env::ArbitriumEnv;
// use crate::http_client::*;
use bevy::prelude::*;
// use bevy::tasks::block_on;
use bevygap_shared::*;
// use futures::StreamExt;
// use std::str::from_utf8;
// use std::time::Duration;

use bevy_tokio_tasks::{TokioTasksPlugin, TokioTasksRuntime};
use lightyear::connection::netcode::ClientId;
use lightyear::server::events::{ConnectEvent, DisconnectEvent};

use crate::arbitrium_env::ArbitriumEnv;
use crate::edgegap_context_plugin::{ArbitriumContext, EdgegapContextPlugin};

/// Plugin for gameservers that run on edgegap.
#[derive(Default)]
pub struct BevygapGameserverPlugin {
    /// if true, use mock envs instead of reading Arbitrium ones.
    pub mock_env: bool
}

impl Plugin for BevygapGameserverPlugin {
    fn build(&self, app: &mut App) {
        let arb_env = if self.mock_env {
            info!("Reading MOCK Arbitrium ENVs");
            ArbitriumEnv::from_example()
        } else {
            info!("Reading Arbitrium ENVs");
            ArbitriumEnv::from_env().expect("Failed to read Arbitrium ENVs")
        };
        app.insert_resource(arb_env);

        app.add_plugins(TokioTasksPlugin::default());
        app.add_plugins(EdgegapContextPlugin);

        app.add_systems(
            Update,
            (context_added, setup_nats)
                .chain()
                .run_if(resource_added::<ArbitriumContext>),
        );

        app.add_systems(
            Update,
            (
                handle_lightyear_connect_events,
                handle_lightyear_disconnect_events,
            ),
        );
    }
}

fn handle_lightyear_disconnect_events(
    mut events: EventReader<DisconnectEvent>,
    nats_sender: ResMut<NatsSender>,
) {
    for ev in events.read() {
        let client_id = ev.client_id;
        info!("Lightyear disconnect event for client_id {}", client_id);
        nats_sender.client_disconnected(client_id.to_bits());
    }
}

fn handle_lightyear_connect_events(
    mut events: EventReader<ConnectEvent>,
    nats_sender: ResMut<NatsSender>,
) {
    for ev in events.read() {
        let client_id = ev.client_id;
        info!("Lightyear connect event for client_id {}", client_id);
        nats_sender.client_connected(client_id.to_bits());
    }
}

fn context_added(context: Res<ArbitriumContext>) {
    info!("CONTEXT added: {context:?}");
    info!("CONTEXT fqdn: {}", context.fqdn());
    // info!("CONTEXT request_id: {}", context.request_id());
    // info!("CONTEXT location: {}", context.location());
    // info!("CONTEXT sockets: {}", context.sockets());
    // info!("CONTEXT public_ip: {}", context.public_ip());
}

#[derive(Debug, Event)]
enum NatsEvent {
    ClientConnected(ClientId),
    ClientDisconnected(ClientId),
}

#[derive(Resource)]
struct NatsSender(tokio::sync::mpsc::UnboundedSender<NatsEvent>);

// these sends should never fail, it's an unbounded channel and if the
// receiving task can't recv, it panics anyway.
impl NatsSender {
    fn client_connected(&self, client_id: u64) {
        self.0
            .send(NatsEvent::ClientConnected(client_id))
            .expect("Unable to send NatsEvent for client_connected")
    }

    fn client_disconnected(&self, client_id: u64) {
        self.0
            .send(NatsEvent::ClientDisconnected(client_id))
            .expect("Unable to send NatsEvent for client_disconnected")
    }
}

fn setup_nats(
    runtime: ResMut<TokioTasksRuntime>,
    arb_context: Res<ArbitriumContext>,
    mut commands: Commands,
) {
    let arb_context_bytes = arb_context.to_bytes();

    let (nats_event_sender, mut nats_event_receiver) =
        tokio::sync::mpsc::unbounded_channel::<NatsEvent>();
    commands.insert_resource(NatsSender(nats_event_sender));

    let fqdn = arb_context.fqdn();
    let nats_key = fqdn.replace('.', "_");

    runtime.spawn_background_task(|mut ctx| async move {
        let bgnats = match BevygapNats::new_and_connect(nats_key.as_str()).await {
            Ok(nats) => nats,
            Err(e) => {
                error!("Failed to setup NATS: {}", e);
                panic!("Failed to setup NATS");
            }
        };
        // let server_kv = nats.server_kv.clone();
        // let server_key = nats.server_key.clone();
        
        let kv_c2s = bgnats.kv_c2s().clone();
        let kv_sessions = bgnats.kv_sessions().clone();

        // Write our context to nats to announce our presence.
        bgnats.client()
            .publish("gameserver.contexts", arb_context_bytes.into())
            .await
            .expect("Failed to write context to NATS");

        bgnats.client().flush().await.expect("Failed to flush NATS");

        ctx.run_on_main_thread(move |ctx| {
            ctx.world.insert_resource(bgnats);
        })
        .await;



        // Loop over nats_event_receiver and log received NatsEvents
        info!("Starting NatsEvent loop");
        loop {
            let Some(ev) = nats_event_receiver.recv().await else {
                // If we can't manage session stuff via nats, kill the server.
                panic!("NatsEvent channel closed, aborting.");
            };
            match ev {
                NatsEvent::ClientConnected(client_id) => {
                    info!("Client connected: {}, writing to nats kv", client_id);
                    // TODO lookup the session id here?
                    let session_id = kv_c2s
                        .get(client_id.to_string())
                        .await
                        .expect("Failed to get session_id from KV");
                    match session_id {
                        None => {
                            panic!("Client ID is not mapped to a session id! wtf.");
                        }
                        Some(session_id) => {
                            let key = format!("{nats_key}.{client_id}");
                            info!(
                                "Client ID {client_id} associated with session id: {session_id:?}, storing in {key}",
                            );
                            kv_sessions
                                .put(key, session_id)
                                .await
                                .expect("Failed to put client_id in KV");
                        }
                    }
                }
                NatsEvent::ClientDisconnected(client_id) => {
                    info!("Client disconnected: {}, writing to nats kv", client_id);
                    let key = format!("{nats_key}.{client_id}");
                    kv_sessions
                        .delete(key)
                        .await
                        .expect("Failed to del client_id in KV");
                }
            }
        }
    });
}
