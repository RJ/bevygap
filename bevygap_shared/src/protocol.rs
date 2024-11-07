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
