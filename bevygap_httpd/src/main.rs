use axum::extract::State;
use axum::http::header;
use axum::{
    extract::ConnectInfo,
    extract::Query,
    http::StatusCode,
    response::Html,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use bevygap_shared::*;
use log::*;
use serde::{de, Deserialize, Deserializer};
use std::net::SocketAddr;
use std::sync::Arc;
use std::{fmt, str::FromStr};
use tracing_subscriber::{layer::*, util::*};

struct AppState {
    bgnats: BevygapNats,
}

#[tokio::main]
async fn main() {
    setup_logging();

    let bgnats = BevygapNats::new_and_connect("bevygap_httpd").await.unwrap();
    let app_state = Arc::new(AppState { bgnats });

    // build our application with a route
    let app = Router::new()
        .route("/", get(index_handler))
        .route("/wannaplay", get(wannaplay_handler))
        .with_state(app_state);

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    info!(
        "bevygap_httpdlistening on {}",
        listener.local_addr().unwrap()
    );

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

async fn index_handler() -> Html<&'static str> {
    Html("<h1>Bevygap Matchmaker Webservice.</h1><p>Nothing to see here, move along.</p>")
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct WannaplayParams {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    client_ip: Option<String>,
}

async fn wannaplay_handler(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Query(params): Query<WannaplayParams>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    let client_ip = params.client_ip.unwrap_or(addr.ip().to_string());
    info!("wannaplay_handler req for ip {client_ip}");
    let payload = format!("{{\"client_ip\":\"{client_ip}\"}}");
    let resp = state
        .bgnats
        .client()
        .request("session.gensession", payload.into())
        .await?;

    info!("Got mm response: {:?}", resp);
    // resp.payload
    let reply = ([(header::CONTENT_TYPE, "text/json")], resp.payload);
    Ok(reply)
}

/// Serde deserialization decorator to map empty Strings to None,
fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: fmt::Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
    }
}

fn setup_logging() {
    // Set environment for logging configuration
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    // Start logging to console
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::Layer::default().compact())
        .init();
}

// Make our own error that wraps `anyhow::Error`.
struct AppError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
