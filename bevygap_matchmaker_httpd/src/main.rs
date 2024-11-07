use async_nats::client::RequestErrorKind;
use axum::extract::{Request, State};
use axum::http::{header, HeaderValue, Method};
use axum::routing::post;
use axum::{
    extract::ConnectInfo,
    extract::Query,
    http::StatusCode,
    response::Html,
    response::{IntoResponse, Response},
    routing::{any, get},
    Router,
};
use bevygap_shared::nats::*;
use clap::Parser;
use log::*;
use serde::{de, Deserialize, Deserializer};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use std::{fmt, str::FromStr};
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::*, util::*};

mod session_request_handler;
mod session_request_handler_ws;

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

pub(crate) struct AppState {
    pub(crate) bgnats: BevygapNats,
    pub(crate) settings: Settings,
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
    let cors_layer = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::CONNECT])
        .allow_origin(
            settings
                .allowed_origin()
                .parse::<HeaderValue>()
                .expect("failed parsing cors domain"),
        );

    /*
    {"client_ip": "81.128.157.123", "game": "bevygap-spaceships", "version": "1"}

    Paste above request to:

    docker run --rm -ti ghcr.io/vi/websocat:nightly ws://100.109.105.19:3000/matchmaker/ws
     */

    // build our application with a route
    let app = Router::new()
        .route("/", get(index_handler))
        // this probably warrants a formtoken like system or something too..
        .route("/matchmaker/wannaplay", post(wannaplay_handler))
        .route(
            "/matchmaker/request/:game/:version",
            post(session_request_handler::session_chunked_responder),
        )
        .route(
            "/matchmaker/ws",
            any(session_request_handler_ws::handler_websocket),
        )
        .layer(cors_layer)
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
) -> Response {
    //Result<impl IntoResponse, AppError> {
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

    // this timeout should far exceed the cutoff time in the matchmaker.
    // it is merely a last line of defense.
    let request = async_nats::client::Request::new()
        .timeout(Some(Duration::from_secs(60)))
        .payload(payload.into());

    match state
        .bgnats
        .client()
        .send_request("session.gensession", request)
        .await
    {
        // Don't really understand the reasoning here, but if you respond to a service
        // request with an Err, you still get an Ok here, and have to examine the headers
        // to figure out if it was actually an error?
        // see: https://github.com/nats-io/nats.rs/blob/main/async-nats/tests/service_tests.rs#L245
        Ok(resp) => {
            if let Some((code, msg)) = maybe_message_error(&resp) {
                error!("Got error matchmaker response: {:?}", msg);
                (
                    StatusCode::from_u16(code as u16).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
                    msg,
                )
                    .into_response()
            } else {
                info!("Got OK matchmaker response: {:?}", resp);
                ([(header::CONTENT_TYPE, "text/json")], resp.payload).into_response()
            }
        }
        Err(e) => {
            warn!("Got Err matchmaker response: {:?}", e);
            match e.kind() {
                RequestErrorKind::TimedOut => {
                    (StatusCode::REQUEST_TIMEOUT, "Request timeout").into_response()
                }
                RequestErrorKind::NoResponders => {
                    (StatusCode::SERVICE_UNAVAILABLE, "No service responders").into_response()
                }
                RequestErrorKind::Other => {
                    (StatusCode::INTERNAL_SERVER_ERROR, "Unhandled error").into_response()
                }
            }
        }
    }
}

fn maybe_message_error(message: &async_nats::Message) -> Option<(usize, String)> {
    let h = message.headers.clone()?;
    if let Some(code) = h.get(async_nats::service::NATS_SERVICE_ERROR_CODE) {
        let msg_str = h
            .get(async_nats::service::NATS_SERVICE_ERROR)
            .unwrap()
            .to_string();
        Some((code.as_str().parse::<usize>().unwrap(), msg_str))
    } else {
        None
    }
}

/// Serde deserialization decorator to map empty Strings to None,
pub(crate) fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
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
