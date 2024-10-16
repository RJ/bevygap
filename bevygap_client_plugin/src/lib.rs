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

#[derive(Debug, Clone, Deserialize, Default)]
struct SessionResponse {
    connect_token: String,
    gameserver_ip: String,
    gameserver_port: u16,
    cert_digest: String,
}

#[derive(States, Debug, Clone, Default, Eq, PartialEq, Hash)]
pub enum BevygapClientState {
    #[default]
    Dormant,
    /// Entering this state triggers a "want to play" request to the matchmaker
    Request,
    /// The request has been sent, awaiting a response
    AwaitingResponse,
    /// Got a good response from the matchmaker, ready to connect to the gameserver
    ReadyToConnect,
    /// We triggered a connection attempt.
    Finished,
    /// The request failed
    Error(u16, String),
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
        app.register_request_type::<SessionResponse>();
        app.init_resource::<BevygapClientConfig>();
        app.init_state::<BevygapClientState>();

        app.add_systems(OnEnter(BevygapClientState::Request), request_token);

        app.add_systems(
            Update,
            handle_matchmaker_response.run_if(in_state(BevygapClientState::AwaitingResponse)),
        );

        app.add_systems(OnEnter(BevygapClientState::ReadyToConnect), connect_client);
    }
}

fn request_token(
    mut ev_request: EventWriter<TypedRequest<SessionResponse>>,
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
            .with_type::<SessionResponse>(),
    );
    next_state.set(BevygapClientState::AwaitingResponse);
}

#[allow(unused_variables)]
fn handle_matchmaker_response(
    mut ev_response: ResMut<Events<TypedResponse<SessionResponse>>>,
    mut ev_response_error: ResMut<Events<TypedResponseError<SessionResponse>>>,
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

        let cert_digest = response.cert_digest.replace(':', "");
        info!("Using cert digest {cert_digest}");

        let tok_bytes = BASE64_STANDARD.decode(&response.connect_token).unwrap();
        assert_eq!(
            tok_bytes.len(),
            2048,
            "ConnectTokens should be 2048 bytes exactly"
        );
        let connect_token = ConnectToken::try_from_bytes(tok_bytes.as_slice()).unwrap();

        // TODO be defensive here
        let server_addr: SocketAddr =
            format!("{}:{}", response.gameserver_ip, response.gameserver_port)
                .parse()
                .expect("invalid gameserver addr/port from matchmaker response?");

        info!("Got matchmaker response, game server: {server_addr:?}");

        if let NetConfig::Netcode { auth, io, .. } = &mut client_config.net {
            info!("Setting Netcode connect token and server addr");
            *auth = Authentication::Token(connect_token);
            // inject gameserver address and port into lightyear client transport
            // (preserves existing client_addr if it was already set)
            let client_addr = match &mut io.transport {
                client::ClientTransport::WebTransportClient { client_addr, .. } => client_addr,
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

fn connect_client(mut commands: Commands, mut next_state: ResMut<NextState<BevygapClientState>>) {
    info!("Connecting to server...");
    commands.connect_client();
    next_state.set(BevygapClientState::Finished);
}
