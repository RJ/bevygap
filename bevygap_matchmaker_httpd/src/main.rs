use axum::extract::{Request, State};
use axum::http::{header, HeaderValue, Method};
use axum::routing::post;
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
use clap::Parser;
use log::*;
use serde::{de, Deserialize, Deserializer};
use std::net::SocketAddr;
use std::sync::Arc;
use std::{fmt, str::FromStr};
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::*, util::*};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Settings {
    /// Domain to allow CORS access
    /// (ie, host serving your index.html)
    #[arg(long, default_value = "http://localhost:8000")]
    cors: String,

    /// The ip:port to bind the http listener to
    #[arg(long, default_value = "0.0.0.0:3000")]
    bind: String,

    /// A fake IP to use instead of the client IP, if the request comes from localhost.
    ///
    /// This is useful for local development – use your normal IP so that deployments you
    /// trigger are geographically near by.
    ///
    /// The default fake IP is near London, United Kindom.
    #[arg(long, default_value = "81.128.157.100")]
    fake_ip: String,
}

impl Settings {
    pub fn allowed_origin(&self) -> String {
        self.cors.trim().to_string()
    }
}

struct AppState {
    bgnats: BevygapNats,
    settings: Settings,
}

#[tokio::main]
async fn main() {
    setup_logging();
    let settings = Settings::parse();

    let bgnats = BevygapNats::new_and_connect("bevygap_matchmaker_httpd")
        .await
        .unwrap();
    let app_state = Arc::new(AppState {
        bgnats,
        settings: settings.clone(),
    });

    info!(
        "bevygap_matchmaker_httpd CORS allowed origin: {:?}",
        settings.allowed_origin()
    );
    // to support multiple cors origins, we'd need to have a list, and check if the request is from
    // one of them, if so, return it as the cors header.
    let wannaplay_cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(
            settings
                .allowed_origin()
                .parse::<HeaderValue>()
                .expect("failed parsing cors domain"),
        );

    // build our application with a route
    let app = Router::new()
        .route("/", get(index_handler))
        // this probably warrants a formtoken like system or something too..
        .route("/matchmaker/wannaplay", post(wannaplay_handler))
        .layer(wannaplay_cors)
        .with_state(app_state);

    // run it
    let listener = tokio::net::TcpListener::bind(settings.bind.as_str())
        .await
        .unwrap();
    info!(
        "bevygap_matchmaker_httpd listening on {}",
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
    req: Request,
) -> Result<impl IntoResponse, AppError> {
    // client_ip is the one sent to Edgegap, to decide which server to assign the player to.
    // We use one provided in the qs, otherwise the connecting IP of the http client.
    let mut client_ip = params.client_ip.unwrap_or(addr.ip().to_string());
    // Check for X-Forwarded-For header, since this is probably running behind a proxy
    if let Some(forwarded_for) = req.headers().get("X-Forwarded-For") {
        if let Ok(forwarded_ip) = forwarded_for.to_str() {
            // Use the first IP in the X-Forwarded-For header
            if let Some(first_ip) = forwarded_ip.split(',').next() {
                client_ip = first_ip.trim().to_string();
                info!("Using X-Forwarded-For IP: {}", client_ip);
            }
        }
    }

    // TODO this default IP should be configurable too, for easier dev
    if client_ip == "127.0.0.1" || client_ip == "::1" {
        // localhost tends to spawn deployments in random places..
        client_ip = state.settings.fake_ip.to_string();
        warn!("Using fake IP, request came from localhost: {client_ip}");
    }

    info!("wannaplay_handler req for ip {client_ip}");
    let payload = format!("{{\"client_ip\":\"{client_ip}\"}}");
    let resp = state
        .bgnats
        .client()
        // .request_with_headers(subject, headers, payload)
        .request("session.gensession", payload.into())
        .await?;

    info!("Got matchmaker response: {:?}", resp);
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
