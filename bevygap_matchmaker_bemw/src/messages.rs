use bevy::prelude::*;
use bevy_eventwork::NetworkMessage;
use bevy_eventwork_mod_websockets::WebSocketProvider;
use serde::{Deserialize, Serialize};

/// Messages sent from (C)lient to (S)erver
/// I suppose this could include requests to join specific regions or teams in future.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum C2SMessage {
    WannaPlay {
        /// allowed to overwrite requesting client ip for debugging.
        client_ip: Option<String>,
    },
}

impl NetworkMessage for C2SMessage {
    const NAME: &'static str = "mm:c2s";
}

/// Messages sent (S)erver to (C)lient
#[derive(Serialize, Deserialize, Clone, Debug, Event)]
pub enum S2CMessage {
    /// The service has begun processing the request.
    Acknowledged,
    /// The edgegap session was created, we are now awaiting readyness
    SessionRequestAccepted(String),
    /// Session readyness update
    ProgressReport(String),
    /// The session is ready to connect to
    SessionReady(SessionReady),
    /// There was an error.
    Error(u16, String),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SessionReady {
    pub token: String,
    pub ip: String,
    pub port: u16,
    pub cert_digest: String,
}

impl NetworkMessage for S2CMessage {
    const NAME: &'static str = "mm:s2c";
}

#[allow(unused)]
pub(crate) fn client_register_network_messages(app: &mut App) {
    use bevy_eventwork::AppNetworkMessage;

    // The client registers messages that arrives from the server, so that
    // it is prepared to handle them. Otherwise, an error occurs.
    app.listen_for_message::<S2CMessage, WebSocketProvider>();
}

#[allow(unused)]
pub(crate) fn server_register_network_messages(app: &mut App) {
    use bevy_eventwork::AppNetworkMessage;

    // The server registers messages that arrives from a client, so that
    // it is prepared to handle them. Otherwise, an error occurs.
    app.listen_for_message::<C2SMessage, WebSocketProvider>();
}
