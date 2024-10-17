use axum::body::Body;
use axum::extract::{Path, Request, State};
use axum::http::{header, HeaderMap, HeaderValue};
use axum::{extract::ConnectInfo, extract::Query, response::IntoResponse};
use log::*;
use serde::Deserialize;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_stream::StreamExt as _;

use crate::AppState;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub(crate) struct QsParams {
    #[serde(default, deserialize_with = "crate::empty_string_as_none")]
    client_ip: Option<String>,
}

// Chunked transfer to give streaming results
// maybe the service immediately yields a key so you can sub to a stream like
// session_requests.1234567890
// and then results are streamed back on that?

pub(crate) async fn session_chunked_responder(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Query(params): Query<QsParams>,
    Path((game_name, game_ver)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    req: Request,
) -> impl IntoResponse {
    let client_ip = get_client_ip(&params, &addr, req.headers(), &state);

    info!("session_chunked_responder for ip {client_ip}");
    // should include app name/ver?
    let payload = format!("{{\"client_ip\":\"{client_ip}\"}}");

    let client = state.bgnats.client().clone();
    let reply_inbox = client.new_inbox();
    let mut response_subscriber = client.subscribe(reply_inbox.to_owned()).await.unwrap();
    // this publish needs to "opt in to no_responder messages" somehow, per
    // https://docs.nats.io/reference/reference-protocols/nats-protocol
    client
        .publish_with_reply(
            format!("matchmaker.request.{game_name}.{game_ver}"),
            reply_inbox,
            payload.into(),
        )
        .await
        .unwrap();
    // TODO subject shoud have app name/ver in it?

    // we'll sub to reply messages over nats and funnel to the stream sending back http chunks.
    // receiving an empty message from nats means the end of stream.
    let (tx, rx) = mpsc::channel::<String>(100);

    let _j = tokio::spawn(async move {
        while let Some(msg) = response_subscriber.next().await {
            if msg.payload.is_empty() {
                // info!("got empty response, breaking");
                break;
            }
            // info!("Got chunk, writing to channel");
            let Ok(_) = tx
                .send(String::from_utf8(msg.payload.to_vec()).unwrap())
                .await
            else {
                warn!("Can't write to channel, closed: {}", tx.is_closed());
                break;
            };
        }
        // info!("reading from response_subscriber done");
        // tx should be dropped here, and rx will close, ending the stream.
    });

    let stream = tokio_stream::wrappers::ReceiverStream::new(rx).map(Ok::<String, Infallible>);

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/json"),
    );
    headers.insert(
        header::TRANSFER_ENCODING,
        HeaderValue::from_static("chunked"),
    );

    (
        headers,
        Body::from_stream(stream), // Wrap the stream in an HTTP body for chunked transfer
    )
}

/// Logic to decide what to use as the clients IP address for the purposes of Edgegap sessions.
///
/// In order of preference:
/// ?client_ip=XXX querystring param (for dev)
/// X-Forwarded-For header (for proxy setups)
/// the source IP of the http client
///
/// Additionally if they above yields a localhost address, we replace it with
/// settings.fake_ip, which is also useful for dev.
fn get_client_ip(
    params: &QsParams,
    addr: &SocketAddr,
    headers: &HeaderMap,
    state: &AppState,
) -> String {
    // client_ip is the one sent to Edgegap, to decide which server to assign the player to.
    // We use one provided in the qs, otherwise the connecting IP of the http client.
    let mut client_ip = params.client_ip.clone().unwrap_or(addr.ip().to_string());

    // Check for X-Forwarded-For header, since this is probably running behind a proxy
    if let Some(forwarded_for) = headers.get("X-Forwarded-For") {
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
    client_ip
}
