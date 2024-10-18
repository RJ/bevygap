use base64::prelude::*;
use bevy::prelude::*;
use bevy_http_client::prelude::*;
use lightyear::prelude::{client::*, *};
use serde::Deserialize;
use std::net::SocketAddr;

pub mod prelude {
    pub use super::traits::*;
    pub use super::BevygapClientConfig;
    pub use super::BevygapClientPlugin;
    pub use super::BevygapClientState;
}
mod traits;

#[derive(Serialize, Deserialize, Debug)]
pub enum SessionRequestFeedback {
    /// The service has begun processing the request.
    Acknowledged,
    /// The edgegap session was created, we are now awaiting readyness
    SessionRequestAccepted(String),
    /// Session readyness update
    ProgressReport(String),
    /// The session is ready to connect to
    SessionReady {
        token: String,
        ip: String,
        port: u16,
        cert_digest: String,
    },
    /// There was an error.
    Error(u16, String),
}

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

#[derive(Resource, Debug, Clone)]
pub struct BevygapClientConfig {
    pub wannaplay_url: String,
    pub fake_client_ip: Option<String>,
    pub certificate_digest: String,
}

impl Default for BevygapClientConfig {
    fn default() -> Self {
        Self {
            wannaplay_url: "http://127.0.0.1:3000/wannaplay".to_string(),
            fake_client_ip: None,
            certificate_digest: "".to_string(),
        }
    }
}

pub struct BevygapClientPlugin;

impl Plugin for BevygapClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HttpClientPlugin);
        app.register_request_type::<SessionRequestFeedback>();
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
    mut ev_request: EventWriter<TypedRequest<SessionRequestFeedback>>,
    mut next_state: ResMut<NextState<BevygapClientState>>,
    config: Res<BevygapClientConfig>,
) {
    // Fake ip bit only for testing, and will fail if there's already a qs in the URL.
    let matchmaker_url = if let Some(ref fake_client_ip) = config.fake_client_ip {
        format!("{}?client_ip={fake_client_ip}", config.wannaplay_url)
    } else {
        config.wannaplay_url.clone()
    };
    info!("Sending request to matchmaker: {matchmaker_url}");
    ev_request.send(
        HttpClient::new()
            .post(matchmaker_url)
            .with_type::<SessionRequestFeedback>()
            .with_streaming(),
    );
    next_state.set(BevygapClientState::AwaitingResponse(
        "Requesting ...".to_string(),
    ));
}

#[allow(unused_variables)]
fn handle_matchmaker_response(
    mut ev_response: ResMut<Events<TypedResponsePart<SessionRequestFeedback>>>,
    mut ev_response_error: ResMut<Events<TypedResponseError<SessionRequestFeedback>>>,
    mut client_config: ResMut<ClientConfig>,
    mut next_state: ResMut<NextState<BevygapClientState>>,
    config: Res<BevygapClientConfig>,
) {
    for response_error in ev_response_error.drain() {
        error!("Matchmaker request error: {:?}", response_error);
        if let Some(r) = response_error.response {
            let msg = format!("{}\n{}", r.status_text, String::from_utf8_lossy(&r.bytes));
            next_state.set(BevygapClientState::Error(r.status, msg));
        } else {
            next_state.set(BevygapClientState::Error(
                0,
                "Can't talk to matchmaker".to_string(),
            ));
        }
    }

    for response in ev_response.drain() {
        let response = response.into_inner();
        // info!("GOT PART: {response:?}");
        match response {
            SessionRequestFeedback::Acknowledged => next_state.set(
                BevygapClientState::AwaitingResponse("Request acknowledged".to_string()),
            ),
            SessionRequestFeedback::SessionRequestAccepted(sess_id) => next_state.set(
                BevygapClientState::AwaitingResponse(format!("Session created: {sess_id}")),
            ),
            SessionRequestFeedback::ProgressReport(prog_msg) => next_state.set(
                BevygapClientState::AwaitingResponse(format!("Progress: {prog_msg}")),
            ),
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
                let connect_token = ConnectToken::try_from_bytes(tok_bytes.as_slice()).unwrap();

                // TODO be defensive here
                let server_addr: SocketAddr = format!("{ip}:{port}")
                    .parse()
                    .expect("invalid gameserver addr/port from matchmaker response?");

                info!("Got matchmaker response, game server: {server_addr:?}");

                if let NetConfig::Netcode { auth, io, .. } = &mut client_config.net {
                    info!("Setting Netcode connect token and server addr");
                    *auth = Authentication::Token(connect_token);
                    // inject gameserver address and port into lightyear client transport
                    // (preserves existing client_addr if it was already set)
                    let client_addr = match &mut io.transport {
                        client::ClientTransport::WebTransportClient { client_addr, .. } => {
                            client_addr
                        }
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

fn connect_client(mut commands: Commands, mut next_state: ResMut<NextState<BevygapClientState>>) {
    info!("Connecting to server...");
    commands.connect_client();
    next_state.set(BevygapClientState::Finished);
}
