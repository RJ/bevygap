use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_tokio_tasks::{TokioTasksPlugin, TokioTasksRuntime};
use bevygap_shared::nats::*;
use lightyear::connection::netcode::ClientId;
use lightyear::connection::server::{ConnectionRequestHandler, DeniedReason};
use lightyear::prelude::server::*;
use lightyear::server::events::{ConnectEvent, DisconnectEvent};
use std::sync::Arc;

use crate::arbitrium_env::ArbitriumEnv;
use crate::edgegap_context::{self, ArbitriumContext};

/// Plugin for gameservers that run on edgegap.
/// TODO We need to know if the cert is self signed or not - if so, we can extract the cert digest
/// and tell the browser to use it.
/// If not, and it's a trusted cert, do nothing.
pub struct BevygapServerPlugin;

#[derive(Resource)]
struct CertDigest(String);

#[derive(Event)]
pub struct NatsConnected;

#[derive(Event)]
pub struct BevygapReady;

impl Plugin for BevygapServerPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<TokioTasksPlugin>() {
            app.add_plugins(TokioTasksPlugin::default());
        }
        // Load the Edgegap ENVs
        info!("Reading Arbitrium ENVs");
        let arb_env = ArbitriumEnv::from_env().expect("Failed to read Arbitrium ENVs");
        app.insert_resource(arb_env);

        // When using a self-signed cert for your NATS server, the server needs the root CA .pem file
        // in order to verify the server's certificate. Since this file is around 2kB, and Edgegap
        // limits you to 255 bytes in ENV vars, we set this from a command line arg instead.
        //
        // If present, we write the contents to an ENV var, which is later read by setup_nats().
        // In future, we hope to just set this ENV var directly in the Edgegap Dashboard.
        inject_ca_root_env_var_from_cmdline_arg();

        app.add_systems(Startup, (extract_cert_digest, setup_nats).chain());

        app.observe(edgegap_context::fetch_context_on_nats_connected);
        app.observe(send_context_to_nats);
        app.observe(setup_connection_request_handler);

        app.observe(handle_lightyear_client_connect);
        app.observe(handle_lightyear_client_disconnect);
    }
}

#[allow(unreachable_patterns)]
fn extract_cert_digest(
    server_config: Res<lightyear::server::config::ServerConfig>,
    mut commands: Commands,
) {
    let net_config = &server_config.net[0];
    let digest = match &net_config {
        NetConfig::Netcode { io, .. } => match &io.transport {
            ServerTransport::WebTransportServer { certificate, .. } => Some(
                certificate.certificate_chain().as_slice()[0]
                    .hash()
                    .to_string(),
            ),
            _ => None,
        },
        _ => None,
    };
    let Some(digest) = digest else {
        panic!(
            "Unable to extract cert digest. Is there a webtransport server transport configured?"
        );
    };
    info!("Extracted cert digest: {}", digest);
    commands.insert_resource(CertDigest(digest));
}

/// If --ca_contents XXXXXX present on command line, set NATS_CA_CONTENTS to XXXXXX
fn inject_ca_root_env_var_from_cmdline_arg() {
    use std::env;
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        return;
    }
    let mut found_flag = false;
    for arg in args {
        if found_flag {
            let ca_root = arg.clone();
            info!(
                "Found --ca_contents, setting NATS_CA_CONTENTS to [{} bytes]",
                ca_root.len()
            );
            env::set_var("NATS_CA_CONTENTS", ca_root);
            return;
        }
        if arg == "--ca_contents" {
            found_flag = true;
            continue;
        }
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

/// We create a BevygapConnectionRequestHandler and store it in a resource.
/// This is handed to lightyear, and used to accept or deny incoming client connections.
fn setup_connection_request_handler(
    _trigger: Trigger<NatsConnected>,
    bgnats: Res<BevygapNats>,
    mut commands: Commands,
    mut server_config: ResMut<lightyear::server::config::ServerConfig>,
) {
    // we store this in a resource, because we'll need to push new data into it
    let crh = BevygapConnectionRequestHandler::new(bgnats.clone());
    let arc_crh = Arc::new(crh);
    commands.insert_resource(CRH(arc_crh.clone()));
    for net in server_config.net.iter_mut() {
        net.set_connection_request_handler(arc_crh.clone());
    }
}

/// Context loaded, nats connected: time to send our metadata to NATS,
/// then trigger the ready event.
fn send_context_to_nats(
    _trigger: Trigger<edgegap_context::ContextLoaded>,
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

/// Exists purely to allow us to trigger an event via command queue
/// see setup_nats() below.
struct DeferredTriggerCommand<T>(T);

impl<T: Event> bevy::ecs::world::Command for DeferredTriggerCommand<T> {
    fn apply(self, world: &mut World) {
        world.trigger(self.0);
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
            // main thread work is executed by TokioTasks plugin by removing the TokioTasksRuntime resource,
            // doing the work, then reinserting the resource.
            // if we use a trigger here, the observer will fire instantly, and run while the TokioTasksRuntime is not present in the world.
            // unfortunately, our observer requests the TokioTasksRuntime resource, and panics if it's not present.
            //
            // so we have to defer the trigger, by using a Command queue.
            // instead of:
            // ctx.world.trigger(NatsConnected);
            // we do:
            ctx.world.commands().push(DeferredTriggerCommand(NatsConnected));
            // so the actual triggering happens after the TokioTasksRuntime resource is reinserted into the world.
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

// /// Reasons for denying a connection request
// #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
// pub enum DeniedReason {
//     ServerFull,
//     Banned,
//     InternalError,
//     AlreadyConnected,
//     TokenAlreadyUsed,
//     InvalidToken,
//     Custom(String),
// }

// / Trait for handling connection requests from clients.
// pub trait ConnectionRequestHandler: Debug + Send + Sync {
//     /// Handle a connection request from a client.
//     /// Returns None if the connection is accepted,
//     /// Returns Some(reason) if the connection is denied.
//     fn handle_request(&self, client_id: ClientId) -> Option<DeniedReason>;
// }

#[derive(Resource)]
pub struct CRH(Arc<BevygapConnectionRequestHandler>);

/// Only accept connections where the ClientId is in NATS associated with a session.
#[derive(Clone, Debug)]
pub struct BevygapConnectionRequestHandler {
    bgnats: BevygapNats,
}

// this is set as a arc dyn trait object.
// perhaps we should push valid ClientIDs into this struct as they arrive in nats?
// or lookup as needed? we can keep an arc ref to this in the server plugin, and hand a clone
// of it for registering with lightyear.
//
// Reasons we might actually want to reply with:
// Full  - no, mm should prevent this? maybe we do want this, in case of lobbies and races when users pick a server.
// banned - no, mm prevents
// internal error - maybe
// already connected - if client id is already connected?
// token already used - same as already used client id?
// invalidtoken - yep, if not registered in nats
// custom(string) - hmm.
//
// we don't want this to block, so i think we need to push data into it so it's always ready.
impl BevygapConnectionRequestHandler {
    pub fn new(bgnats: BevygapNats) -> Self {
        Self { bgnats }
    }
}

impl ConnectionRequestHandler for BevygapConnectionRequestHandler {
    fn handle_request(
        &self,
        client_id: lightyear::connection::id::ClientId,
    ) -> Option<DeniedReason> {
        info!("BevygapConnectionRequestHandler({client_id})");
        // TODO: check this ClientID is in nats, and there isn't already a player connected
        // TODO: check server isn't full
        None

        // pub enum DeniedReason {
        //     ServerFull,
        //     Banned,
        //     InternalError,
        //     AlreadyConnected,
        //     TokenAlreadyUsed,
        //     InvalidToken,
        //     Custom(String),
        // }
    }
}
