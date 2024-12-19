use crate::MatchmakerState;
use async_nats::error::Error as NatsError;
use async_nats::{Client, Subject};
use base64::prelude::*;
use bevygap_shared::protocol::*;
use edgegap_async::{apis::sessions_api::*, apis::Error as EdgegapError, models::SessionModel};
use futures::StreamExt;
use lightyear::prelude::ConnectToken;
use log::*;
use serde::{de, Deserialize};
use std::net::{IpAddr, SocketAddr};
use std::time::Duration;
use tokio::time::Instant;

#[derive(Deserialize, Debug)]
pub struct SessionRequest {
    /// the ip of the client that wants a session
    pub client_ip: String,
    /// the rest of the request, with no fixed schema.
    #[allow(dead_code)]
    pub obj: serde_json::Map<String, serde_json::Value>,
}

impl SessionRequest {
    pub fn new(client_ip: String) -> Self {
        Self {
            client_ip,
            obj: serde_json::Map::new(),
        }
    }

    pub fn from_raw(raw: &[u8]) -> Result<SessionRequest, serde_json::Error> {
        let mut parsed: serde_json::Map<String, serde_json::Value> = serde_json::from_slice(raw)?;

        let client_ip = parsed
            .remove("client_ip")
            .ok_or_else(|| de::Error::custom("Missing client_ip field"))?
            .as_str()
            .ok_or_else(|| de::Error::custom("client_ip is not a string"))?
            .to_string();

        Ok(SessionRequest {
            client_ip,
            obj: parsed,
        })
    }
}

/// For sending progress updates back to the caller
struct ChunkResponder {
    client: Client,
    reply_to: Subject,
}
impl ChunkResponder {
    async fn send(
        &self,
        feedback: SessionRequestFeedback,
    ) -> Result<(), NatsError<async_nats::client::PublishErrorKind>> {
        info!("sending feedback: {feedback:?}");
        let payload = serde_json::to_string(&feedback).unwrap();
        self.client
            .publish(self.reply_to.clone(), payload.into())
            .await
    }
    async fn finish(&self) -> Result<(), NatsError<async_nats::client::PublishErrorKind>> {
        self.client.publish(self.reply_to.clone(), "".into()).await
    }
}

/// one of these spawned per request.
/// should send response parts back to reply_to.
async fn stream_request_processor(
    state: &MatchmakerState,
    session_request: SessionRequest,
    responder: &ChunkResponder,
) -> Result<(), MyError<SessionPostError>> {
    // Sender for feedback responses, client will recieve multiple before the Finished one.
    info!("Generating streaming session for {session_request:?}");
    responder.send(SessionRequestFeedback::Acknowledged).await?;

    let mut session_model = SessionModel::new(state.settings.app_name.clone());
    session_model.ip_list = Some(vec![session_request.client_ip.to_string()]);
    session_model
        .webhook_url
        .clone_from(&state.settings.session_webhook_url);
    // create session via edgegap api.
    // this gives us our session_id, but could be in a non-Ready state for a while.
    let post_session = session_post(state.configuration(), session_model).await?;

    // info!("{post_session:?}");

    responder
        .send(SessionRequestFeedback::SessionRequestAccepted(
            post_session.session_id.clone(),
        ))
        .await?;

    let mut session_get;
    let mut tries = 0;
    // let mut first_seen_session_id = false;
    let start_time = Instant::now();
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    loop {
        tries += 1;
        info!("GET SESSION... ({tries})");
        session_get = get_session(state.configuration(), post_session.session_id.as_str())
            .await
            .map_err(|e| {
                error!("get session error: {:?}", e);
                EdgegapError::Io(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("get session error: {}", e),
                ))
            })?;
        let feedback = SessionRequestFeedback::ProgressReport(format!(
            "{} ({})",
            session_get.status, session_get.elapsed
        ));
        responder.send(feedback).await?;

        // info!("{session_get:?}");

        // Avoid session leakage!
        // the first time we get a response with a session_id, we store it in unclaimed_sessions,
        // so we can automatically delete it if it goes unused.
        // if !first_seen_session_id {
        // first_seen_session_id = true;
        // this kv buckets has 30sec expiry i think, but that starts ticking
        // while we spend 20+ secs waiting on a session sometimes, so it's deleted
        // before we need it. So renew the timeout each iteration for now:
        let session_id_str = session_get.session_id.clone();
        // info!("Writing session_id to unclaimed_sessions KV: {session_id_str}");
        let val = session_id_str.clone().into();
        state
            .nats
            .kv_unclaimed_sessions()
            .put(session_id_str, val)
            .await
            .expect("Failed to put session_id in unclaimed_sessions KV");
        // }

        if session_get.ready {
            break;
        }

        let elapsed = Instant::now().duration_since(start_time);
        if elapsed > Duration::from_secs(crate::MAX_SESSION_CREATION_SECONDS) {
            //TODO schedule delete of session id!
            return Err(MyError::Bevygap(
                408,
                "session still not ready, timed out.".into(),
            ));
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    }

    // info!("{session_get:?}");

    // We must wait until the session is ready / linked before telling the client to connect.
    // You can ask for a webhook, but for now we just poll until it's ready.

    let Some(deployment) = session_get.deployment else {
        return Err(MyError::Bevygap(500, "No deployment found".into()));
    };

    let Some(ports) = deployment.ports else {
        return Err(MyError::Bevygap(500, "No ports found in deployment".into()));
    };

    if ports.len() > 1 {
        warn!("multiple ports found for deployment.. using first one");
    }

    // use first port.
    // to support multiple, we'd need to store the name of the port mapping definition that we
    // use in edgegap, to look it up here.
    let port = ports
        .iter()
        .next()
        .and_then(|(_, port_info)| port_info.external)
        .expect("Couldn't get port");

    //  assign a new client_id
    let client_id = rand::random();
    // info!("client_id = {client_id}");

    let public_ip_str = deployment.public_ip.as_str();

    let ip = public_ip_str
        .parse::<std::net::IpAddr>()
        .expect("Failed parsing server ip");

    // TODO once the session is ready, the cert digest should have been reported, but
    // there is definitely a race here so we should block on it for a second or so?
    let cert_digest = lookup_cert_digest(state, &ip).await?;

    let server_addresses = SocketAddr::new(ip, port as u16);

    info!(
        "🏠 BUILD ConnectToken: server_addresses = {server_addresses} proto id: {}, client_id: {client_id}, privkey: {:?}",
        state.settings.protocol_id(),
        state.lightyear_private_key()
    );
    let token = ConnectToken::build(
        server_addresses,
        state.settings.protocol_id(),
        client_id,
        state.lightyear_private_key(),
    )
    .generate()
    .expect("Failed to generate token");

    let token_bytes = token.try_into_bytes().expect("Failed to serialize token");
    let token_base64 = BASE64_STANDARD.encode(token_bytes);

    register_ids_in_nats(state, client_id.to_string(), session_get.session_id).await?;

    responder
        .send(SessionRequestFeedback::SessionReady {
            token: token_base64,
            ip: deployment.public_ip,
            port: port as u16,
            cert_digest,
        })
        .await?;
    // send an empty chunk to finish:
    responder.finish().await?;
    Ok(())
}

async fn register_ids_in_nats(
    state: &MatchmakerState,
    client_id: String,
    session_id: String,
) -> Result<(), MyError<SessionPostError>> {
    let session_id_val = session_id.clone().into();
    state
        .nats
        .kv_c2s()
        .put(client_id.as_str(), session_id_val)
        .await
        .map_err(|e| {
            EdgegapError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to put token KV entry: {}", e),
            ))
        })?;
    state
        .nats
        .kv_s2c()
        .put(session_id.as_str(), client_id.into())
        .await
        .map_err(|e| {
            EdgegapError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to put token KV entry: {}", e),
            ))
        })?;
    Ok(())
}

async fn lookup_cert_digest(
    state: &MatchmakerState,
    public_ip: &IpAddr,
) -> Result<String, MyError<SessionPostError>> {
    let ip_str = public_ip.to_string();
    match state.nats.kv_cert_digests().get(ip_str).await {
        Ok(Some(cert_digest)) => Ok(String::from_utf8(cert_digest.into()).unwrap()),
        Ok(None) => Err(MyError::Bevygap(500, "No cert digest found".into())),
        Err(e) => {
            error!("err getting digest for {public_ip}: {e:?}");
            Err(MyError::Bevygap(
                500,
                "Error'ed on lookup for cert_digest".into(),
            ))
        }
    }
}

/// Subscribes to "matchmaker.request" and processes the session request stream.
/// for each request, it verifies a reply_to is specified, then spawns a task
/// to do the session creation, sending messages back to the reply_to subject
/// to report status, progress, and completion.
///
/// Can't use the built-in nats Service feature, because we want to stream back
/// multiple responses to each query.
pub(crate) async fn streaming_session_request_handler(
    state: &MatchmakerState,
) -> Result<(), async_nats::Error> {
    let client = state.nats_client().clone();

    let subject = format!(
        "matchmaker.request.{}.{}",
        state.settings.app_name, state.settings.app_version
    );
    info!("Listening for session requests on '{subject}'");

    let mut sub = client.subscribe(subject).await?;

    while let Some(message) = sub.next().await {
        info!("Matchmaking request on {}", message.subject);
        let Some(reply_to) = message.reply else {
            error!("got message with no reply-to, discarding");
            continue;
        };
        let responder = ChunkResponder {
            client: state.nats_client().clone(),
            reply_to: reply_to.clone(),
        };

        // decode the json request object
        let request = match SessionRequest::from_raw(&message.payload) {
            Ok(request) => request,
            Err(e) => {
                let err_response = format!("ERROR decoding session request {e:?}");
                responder
                    .send(SessionRequestFeedback::Error(500, err_response))
                    .await?;
                responder.finish().await?;
                continue;
            }
        };

        // spawn the handler fn, and then send an empty message to close the response afterwards.
        // and await? stuff in the handler gets thrown here, which will either be nats
        // or more likely a response from the edgegap api.
        let state = state.clone();
        tokio::spawn(async move {
            match stream_request_processor(&state, request, &responder).await {
                Ok(()) => {}
                Err(MyError::Bevygap(err_code, err_msg)) => {
                    error!("error in stream_request_processor: {err_code}={err_msg}");
                    let _ = responder
                        .send(SessionRequestFeedback::Error(err_code, err_msg))
                        .await;
                }
                Err(MyError::Edgegap(edgegap_async::apis::Error::ResponseError(e))) => {
                    let (err_code, err_msg) = match e.entity {
                        Some(SessionPostError::Status400(ee)) => (400, ee.message),
                        Some(SessionPostError::Status401(ee)) => (401, ee.message),
                        Some(SessionPostError::Status409(ee)) => (409, ee.message),
                        _ => (503, "unknown error".to_string()),
                    };
                    error!("error in session_responder: {err_code}={err_msg}");
                    let _ = responder
                        .send(SessionRequestFeedback::Error(err_code, err_msg))
                        .await;
                }
                Err(MyError::Nats(e)) => {
                    error!("Nats error in stream_request_processor: {:?}", e);
                    let err_response = format!("NATS error: {e:?}");
                    let _ = responder
                        .send(SessionRequestFeedback::Error(500, err_response))
                        .await;
                }
                Err(e) => {
                    error!("Error in stream_request_processor: {:?}", e);
                    let err_response = format!("SessionPostError: {e:?}");
                    let _ = responder
                        .send(SessionRequestFeedback::Error(500, err_response))
                        .await;
                }
            }
            // close the response by sending an empty message
            let _ = responder.finish().await;
        });
    }

    warn!("session_request_handler exiting?");
    Ok(())
}

// don't want to modify the auto-generated edgegap client error type,
// so wrap it up here to also add our nats error type.
#[derive(Debug)]
pub(crate) enum MyError<T> {
    Edgegap(EdgegapError<T>), // Wrap the third-party error
    Nats(async_nats::Error),  // Add a variant for async_nats errors
    Bevygap(u16, String),
}
impl<T> From<EdgegapError<T>> for MyError<T> {
    fn from(err: EdgegapError<T>) -> Self {
        MyError::Edgegap(err)
    }
}
impl<T> From<NatsError<async_nats::client::PublishErrorKind>> for MyError<T> {
    fn from(err: NatsError<async_nats::client::PublishErrorKind>) -> Self {
        MyError::Nats(Box::new(err))
    }
}
impl<T> From<async_nats::Error> for MyError<T> {
    fn from(err: async_nats::Error) -> Self {
        MyError::Nats(err)
    }
}
