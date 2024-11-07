use base64::prelude::*;
use bevy::prelude::*;
use bevy_nfws::prelude::*;
use bevygap_shared::protocol::*;
use lightyear::prelude::{client::*, *};
use std::net::SocketAddr;

pub mod prelude {
    pub use super::traits::*;
    pub use super::BevygapClientConfig;
    pub use super::BevygapClientPlugin;
    pub use super::BevygapClientState;
}
mod traits;

#[derive(States, Debug, Clone, Default, Eq, PartialEq, Hash)]
pub enum BevygapClientState {
    #[default]
    Dormant,
    /// Entering this state triggers a "want to play" request to the matchmaker
    Request,
    /// The request has been sent, awaiting a response
    AwaitingResponse(String),
    /// Got a good response from the matchmaker, ready to connect to the gameserver
    ReadyToConnect,
    /// We triggered a connection attempt.
    Finished,
    /// The request failed
    Error(u16, String),
}

impl BevygapClientState {
    // run condition alternative to in_state(Enum(_with_param_))
    // since in_state doesn't support enum variants with parameters
    fn pending_state() -> impl FnMut(Option<Res<State<BevygapClientState>>>) -> bool + Clone {
        move |current_state: Option<Res<State<BevygapClientState>>>| match current_state {
            Some(current_state) => {
                matches!(current_state.get(), BevygapClientState::AwaitingResponse(_))
            }
            _ => false,
        }
    }
}

/// Game-specific configuration.
#[derive(Resource, Debug, Clone)]
pub struct BevygapClientConfig {
    /// The websocket endpoint for the matchmaker, eg:
    /// ws://localhost:3000/matchmaker/ws
    pub matchmaker_url: String,
    /// If set, the client will pass this to the matchmaker, overriding the usual client IP detection.
    /// This is passed to Edgegap when making the Session.
    pub fake_client_ip: Option<String>,
    /// The certificate digest to use for the WebTransport connection.
    /// Essential on wasm with self-signed certs.
    pub certificate_digest: String,
    /// The name of the game, used in the matchmaker request.
    pub game_name: String,
    /// The version of the game, used in the matchmaker request.
    pub game_version: String,
}

impl Default for BevygapClientConfig {
    fn default() -> Self {
        Self {
            matchmaker_url: "ws://localhost:3000/matchmaker/ws".to_string(),
            fake_client_ip: None,
            certificate_digest: "".to_string(),
            game_name: "bevygap-spaceships".to_string(),
            game_version: "1".to_string(),
        }
    }
}

pub struct BevygapClientPlugin;

impl Plugin for BevygapClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(NfwsPlugin);
        app.init_resource::<BevygapClientConfig>();
        app.init_state::<BevygapClientState>();

        app.add_systems(OnEnter(BevygapClientState::Request), request_token);

        app.add_systems(
            Update,
            handle_matchmaker_response.run_if(BevygapClientState::pending_state()),
        );

        app.add_systems(OnEnter(BevygapClientState::ReadyToConnect), connect_client);
    }
}

fn request_token(
    mut next_state: ResMut<NextState<BevygapClientState>>,
    config: Res<BevygapClientConfig>,
    mut commands: Commands,
) {
    info!(
        "Initiating matchmaker websocket connection: {}",
        config.matchmaker_url
    );
    commands.spawn(NfwsHandle::new(config.matchmaker_url.clone()));

    next_state.set(BevygapClientState::AwaitingResponse(
        "Requesting ...".to_string(),
    ));
}

fn handle_matchmaker_response(
    mut q: Query<(Entity, &mut NfwsHandle)>,
    mut commands: Commands,
    mut lightyear_client_config: ResMut<ClientConfig>,
    mut next_state: ResMut<NextState<BevygapClientState>>,
    config: Res<BevygapClientConfig>,
) {
    for (entity, mut nfws) in q.iter_mut() {
        match nfws.next_event() {
            NfwsPollResult::Closed => {
                info!("EV None = closed, despawning");
                commands.entity(entity).despawn();
                continue;
            }
            NfwsPollResult::Empty => continue,
            NfwsPollResult::Event(rec) => {
                info!("EV: {rec:?}");
                match rec {
                    NfwsEvent::Connecting => {
                        info!("Matchmaker: {rec:?}");
                    }
                    NfwsEvent::Connected => {
                        info!("Matchmaker: {rec:?}");
                        let req = RequestSession {
                            client_ip: config.fake_client_ip.clone(),
                            game: config.game_name.clone(),
                            version: config.game_version.clone(),
                        };
                        let payload = serde_json::to_string(&req).unwrap();
                        info!("Sending payload: {payload}");
                        nfws.send_text(payload);
                    }
                    NfwsEvent::Error(nfws_err) => match nfws_err {
                        NfwsErr::Connecting => next_state.set(BevygapClientState::Error(
                            0,
                            "Can't connect to matchmaker".to_string(),
                        )),
                        NfwsErr::Receiving(msg) => next_state.set(BevygapClientState::Error(
                            0,
                            format!("Rcv error from matchmaker: {msg}"),
                        )),
                        NfwsErr::Sending(msg) => next_state.set(BevygapClientState::Error(
                            0,
                            format!("Send error to matchmaker: {msg}"),
                        )),
                    },
                    NfwsEvent::Closed(frame) => {
                        info!("Matchmaker connection closed: {frame:?}");
                    }
                    NfwsEvent::BinaryMessage(_vec) => {
                        warn!("Matchmaker: binary msg received, unhandled.");
                    }
                    NfwsEvent::TextMessage(msg) => {
                        let Ok(feedback) =
                            serde_json::from_slice::<SessionRequestFeedback>(msg.as_bytes())
                        else {
                            warn!("Unhandled msg type from matchmaker: {msg:?}");
                            warn!("Despawning client entity");
                            next_state.set(BevygapClientState::Error(
                                0,
                                "Unhandled response from matchmaker".to_string(),
                            ));
                            commands.entity(entity).despawn();
                            continue;
                        };
                        info!(">>> {feedback:?}");
                        match feedback {
                            SessionRequestFeedback::Acknowledged => {
                                next_state.set(BevygapClientState::AwaitingResponse(
                                    "Request acknowledged".to_string(),
                                ))
                            }
                            SessionRequestFeedback::SessionRequestAccepted(sess_id) => next_state
                                .set(BevygapClientState::AwaitingResponse(format!(
                                    "Session created: {sess_id}"
                                ))),
                            SessionRequestFeedback::ProgressReport(prog_msg) => {
                                next_state.set(BevygapClientState::AwaitingResponse(format!(
                                    "Progress: {prog_msg}"
                                )))
                            }
                            SessionRequestFeedback::Error(err_code, err_msg) => {
                                next_state.set(BevygapClientState::Error(err_code, err_msg))
                            }
                            SessionRequestFeedback::SessionReady {
                                token,
                                ip,
                                port,
                                cert_digest,
                            } => {
                                let cert_digest = cert_digest.replace(':', "");
                                info!("Using cert digest {cert_digest}");
                                let tok_bytes = BASE64_STANDARD.decode(&token).unwrap();
                                assert_eq!(
                                    tok_bytes.len(),
                                    2048,
                                    "ConnectTokens should be 2048 bytes exactly"
                                );
                                let connect_token =
                                    ConnectToken::try_from_bytes(tok_bytes.as_slice()).unwrap();

                                // TODO be defensive here
                                let server_addr: SocketAddr =
                                    format!("{ip}:{port}").parse().expect(
                                        "invalid gameserver addr/port from matchmaker response?",
                                    );

                                info!("Got matchmaker response, game server: {server_addr:?}");

                                if let NetConfig::Netcode { auth, io, .. } =
                                    &mut lightyear_client_config.net
                                {
                                    info!("Setting Netcode connect token and server addr");
                                    *auth = Authentication::Token(connect_token);
                                    // inject gameserver address and port into lightyear client transport
                                    // (preserves existing client_addr if it was already set)
                                    let client_addr = match &mut io.transport {
                                        client::ClientTransport::WebTransportClient {
                                            client_addr,
                                            ..
                                        } => client_addr,
                                        _ => panic!("Unsupported transport: {:?}", io.transport),
                                    };
                                    io.transport = client::ClientTransport::WebTransportClient {
                                        client_addr: *client_addr,
                                        server_addr,
                                        #[cfg(target_family = "wasm")]
                                        certificate_digest: cert_digest,
                                    };
                                } else {
                                    panic!("Unsupported netconfig, only supports Netcode for now.");
                                }
                                next_state.set(BevygapClientState::ReadyToConnect);
                            }
                        }
                    }
                }
            }
        }
    }
}

fn connect_client(mut commands: Commands, mut next_state: ResMut<NextState<BevygapClientState>>) {
    info!("Connecting to server...");
    commands.connect_client();
    next_state.set(BevygapClientState::Finished);
}
