// use async_nats::jetstream;
// use async_nats::jetstream::stream::StorageType;
// use crate::arbitrium_env::ArbitriumEnv;
// use crate::http_client::*;
use bevy::prelude::*;
use bevy::utils::HashMap;
// use bevy::tasks::block_on;
use bevygap_shared::*;
// use futures::StreamExt;
// use std::str::from_utf8;
// use std::time::Duration;

use bevy_tokio_tasks::{TokioTasksPlugin, TokioTasksRuntime};
use lightyear::connection::netcode::ClientId;
use lightyear::server::events::{ConnectEvent, DisconnectEvent};

use crate::arbitrium_env::ArbitriumEnv;
use crate::edgegap_context_plugin::ArbitriumContext;

/// Plugin for gameservers that run on edgegap.
#[derive(Default)]
pub struct BevygapServerPlugin {
    /// if true, use mock envs instead of reading Arbitrium ones.
    pub mock_env: bool,
    /// Certificate digest, for self-signed certs
    pub cert_digest: String,
}

impl BevygapServerPlugin {
    pub fn self_signed_digest(cert_digest: String) -> Self {
        Self {
            cert_digest,
            ..Default::default()
        }
    }
}

#[derive(Resource)]
struct CertDigest(String);

#[derive(Event)]
pub struct BevygapReady;

impl Plugin for BevygapServerPlugin {
    fn build(&self, app: &mut App) {
        info!("BevygapServerPlugin::build");
        let arb_env = if self.mock_env {
            info!("Reading MOCK Arbitrium ENVs");
            ArbitriumEnv::from_example()
        } else {
            info!("Reading Arbitrium ENVs");
            ArbitriumEnv::from_env().expect("Failed to read Arbitrium ENVs")
        };
        app.insert_resource(arb_env);
        app.insert_resource(CertDigest(self.cert_digest.clone()));
        app.add_plugins(TokioTasksPlugin::default());
        // app.add_plugins(EdgegapContextPlugin);
        app.add_systems(Startup, setup_nats);
        app.add_systems(
            Update,
            (
                crate::edgegap_context_plugin::fetch_context.run_if(resource_added::<BevygapNats>),
                context_added.run_if(resource_added::<ArbitriumContext>),
            ),
        );

        app.observe(handle_lightyear_client_connect);
        app.observe(handle_lightyear_client_disconnect);
    }
}

// switch to observers for ConnectEvent and DisconnectEvent!

fn handle_lightyear_client_disconnect(
    trigger: Trigger<DisconnectEvent>,
    nats_sender: ResMut<NatsSender>,
) {
    let client_id = trigger.event().client_id;
    info!("Lightyear disconnect event for client_id {}", client_id);
    nats_sender.client_disconnected(client_id.to_bits());
}

fn handle_lightyear_client_connect(
    trigger: Trigger<ConnectEvent>,
    nats_sender: ResMut<NatsSender>,
) {
    let client_id = trigger.event().client_id;
    info!("Lightyear connect event for client_id {}", client_id);
    nats_sender.client_connected(client_id.to_bits());
}

fn context_added(
    context: Res<ArbitriumContext>,
    nats_sender: ResMut<NatsSender>,
    mut commands: Commands,
    digest: Res<CertDigest>,
) {
    info!("CONTEXT added: {context:?}");
    info!("CONTEXT fqdn: {}", context.fqdn());
    nats_sender.cert_digest(context.public_ip(), digest.0.clone());
    nats_sender.arbitrium_context(context.clone());
    commands.trigger(BevygapReady);
}

#[derive(Debug, Event)]
enum NatsEvent {
    ClientConnected(ClientId),
    ClientDisconnected(ClientId),
    ArbitriumContext(ArbitriumContext),
    CertDigest(String, String),
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

    fn arbitrium_context(&self, context: ArbitriumContext) {
        self.0
            .send(NatsEvent::ArbitriumContext(context))
            .expect("Unable to send NatsEvent for arbitrium_context")
    }

    fn cert_digest(&self, ip: String, digest: String) {
        self.0
            .send(NatsEvent::CertDigest(ip, digest))
            .expect("Unable to send NatsEvent for cert_digest")
    }
}

fn setup_nats(runtime: ResMut<TokioTasksRuntime>, mut commands: Commands) {
    info!("Setting up NATS");

    let (nats_event_sender, mut nats_event_receiver) =
        tokio::sync::mpsc::unbounded_channel::<NatsEvent>();
    commands.insert_resource(NatsSender(nats_event_sender));

    // this probably isn't ideal, but i need panics from tokio tasks to
    // kill the process.
    let default_panic = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        default_panic(info);
        std::process::exit(1);
    }));

    runtime.spawn_background_task(|mut ctx| async move {
        let bgnats = match BevygapNats::new_and_connect("bevygap_server_plugin").await {
            Ok(nats) => nats,
            Err(e) => {
                error!("Failed to setup NATS: {}", e);
                panic!("Failed to setup NATS");
            }
        };
        info!("NATS connected");

        let kv_c2s = bgnats.kv_c2s().clone();
        let kv_sessions = bgnats.kv_active_connections().clone();
        let kv_cert_digests = bgnats.kv_cert_digests().clone();
        let client = bgnats.client().clone();

        ctx.run_on_main_thread(move |ctx| {
            ctx.world.insert_resource(bgnats);
        })
        .await;

        let mut client_id_to_session_id = HashMap::new();

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
                            let session_id_key = String::from_utf8(session_id.into())
                                .expect("Failed to convert session_id to string");
                            info!("Client ID {client_id} associated with session id: {session_id_key}",);
                            client_id_to_session_id.insert(client_id, session_id_key.clone());
                            kv_sessions
                                .put(session_id_key, client_id.to_string().into())
                                .await
                                .expect("Failed to put client_id in KV");
                            // delete the mappings.
                            // this signifies the session
                            // let _ = kv_c2s.delete(client_id.to_string()).await;
                        }
                    }
                }
                NatsEvent::ClientDisconnected(client_id) => {
                    info!("Client disconnected: {}, writing to nats kv", client_id);
                    if let Some(session_id) = client_id_to_session_id.get(&client_id) {
                        kv_sessions
                            .delete(session_id)
                            .await
                            .expect("Failed to del client_id in KV");
                    } else {
                        error!("Client disconnected but not found in client_id_to_session_id");
                    }
                }
                NatsEvent::ArbitriumContext(context) => {
                    info!("ArbitriumContext added: {context:?}");
                    let arb_context_bytes = context.to_bytes();
                    // TODO nats key should be on the subject?
                    client
                        .publish("gameserver.contexts", arb_context_bytes.into())
                        .await
                        .expect("Failed to write context to NATS");
                }
                NatsEvent::CertDigest(ip, digest) => {
                    // TODO need cleanup when a deployment terminates, remove keys
                    info!("CertDigest added: {ip} -> {digest}");
                    let key = ip;
                    kv_cert_digests
                        .put(key, digest.into())
                        .await
                        .expect("Failed to put digest in KV");
                }
            }
            client.flush().await.expect("Failed to flush NATS");
        }
    });
}
