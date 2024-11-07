use serde::{Deserialize, Serialize};
use std::fmt;

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

impl fmt::Display for SessionRequestFeedback {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SessionRequestFeedback::Acknowledged => write!(f, "Sending request"),
            SessionRequestFeedback::SessionRequestAccepted(id) => {
                write!(f, "Request accepted: {}", id)
            }
            SessionRequestFeedback::ProgressReport(msg) => write!(f, "In-progress: {msg}"),
            SessionRequestFeedback::SessionReady {
                token: _,
                ip,
                port,
                cert_digest: _,
            } => write!(f, "Session Ready! {ip}:{port}"),
            SessionRequestFeedback::Error(code, msg) => write!(f, "Error {code}: {msg}"),
        }
    }
}

/// Send up the websocket to the matchmaker when a client wants to play.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestSession {
    /// name of game to play
    pub game: String,
    /// version of game to play
    pub version: String,
    /// client ip address override
    pub client_ip: Option<String>,
}

impl RequestSession {
    pub fn game_name_and_version(&self) -> Result<(String, String), String> {
        let name_pattern = regex::Regex::new(r"^[a-zA-Z0-9\s_-]+$").unwrap();
        let ver_pattern = regex::Regex::new(r"^[a-zA-Z0-9\s_-]+$").unwrap();

        if !name_pattern.is_match(&self.game) {
            return Err("Game name invalid".to_string());
        }

        if !ver_pattern.is_match(&self.version) {
            return Err("Game version invalid".to_string());
        }

        if self.game.len() > 30 {
            return Err("Game name too long (max 30 chars)".to_string());
        }

        if self.version.len() > 30 {
            return Err("Game version too long (max 30 chars)".to_string());
        }

        Ok((self.game.clone(), self.version.clone()))
    }
}
