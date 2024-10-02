use reqwest::header::USER_AGENT;
use reqwest::Client;
use reqwest::Method;

pub async fn get_context(
    context_url: &str,
    context_token: &str,
) -> Result<serde_json::Value, Error> {
    let client = Client::new();
    let req = client
        .request(Method::GET, context_url)
        .header(USER_AGENT, "bevy_edgegap_gameserver")
        .header("authorization", context_token)
        .build()?;

    let resp = client.execute(req).await?;
    let status = resp.status();
    let content = resp.text().await?;

    if status.is_success() {
        let value: serde_json::Value = serde_json::from_str(&content)?;
        Ok(value)
    } else {
        Err(Error::ResponseError(ResponseContent { status, content }))
    }
}

use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ResponseContent {
    pub status: reqwest::StatusCode,
    pub content: String,
}

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::ResponseError(_) => return None,
        })
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}
