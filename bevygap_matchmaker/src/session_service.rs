use crate::MatchmakerState;
use async_nats::service::ServiceExt;
use base64::prelude::*;
use edgegap_async::{apis::sessions_api::*, apis::Error as EdgegapError, models::SessionModel};
use futures::StreamExt;
use lightyear::prelude::ConnectToken;
use log::*;
use serde::{de, Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Deserialize, Debug)]
struct SessionRequest {
    /// the ip of the client that wants a session
    client_ip: String,
    /// the rest of the request, with no fixed schema.
    #[allow(dead_code)]
    obj: serde_json::Map<String, serde_json::Value>,
}

#[derive(Serialize)]
struct SessionResponse {
    connect_token: String,
    gameserver_ip: String,
    gameserver_port: u16,
    cert_digest: String,
}

impl std::fmt::Display for SessionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SessionResponse {{ connect_token: [{} bytes] }}",
            self.connect_token.len()
        )
    }
}

/// Decode a session request, which is a JSON object with a client_ip field.
fn decode_request(raw: &[u8]) -> Result<SessionRequest, serde_json::Error> {
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

pub async fn session_request_supervisor(state: &MatchmakerState) -> Result<(), async_nats::Error> {
    let handles = (0..5).map(|_| session_request_handler(state));

    futures::future::join_all(handles).await;

    Ok(())
}

async fn session_request_handler(state: &MatchmakerState) -> Result<(), async_nats::Error> {
    let client = state.nats_client();
    info!("Listening for session requests on 'session_requests'");

    // Uses the `service_builder` extension function to add a service definition to
    // the NATS client. As soon as `start` is called, the service is now visible
    // and available for interrogation and discovery.
    let service = client
        .service_builder()
        .description("Generate sessions for clients who want to play")
        .start("gensession", "0.0.1")
        .await?;

    let g = service.group_with_queue_group("session", "session_queue");

    let mut gensession = g.endpoint("gensession").await?;

    // Spawns a background loop that iterates over the stream of incoming requests. Note
    // that in order for service stats to update properly, you have to use the `respond`
    // function rather than manually publishing on a reply-to subject.
    let state = state.clone();
    tokio::spawn(async move {
        while let Some(request) = gensession.next().await {
            // The input to this endpoint is a JSON array of integers and the function
            // returns a string with the min value
            match decode_request(&request.message.payload) {
                Ok(session_request) => match session_responder(&state, &session_request).await {
                    Ok(response) => {
                        request
                            .respond(Ok(serde_json::to_string(&response).unwrap().into()))
                            .await
                            .unwrap();
                    }

                    Err(edgegap_async::apis::Error::ResponseError(e)) => {
                        error!("edgegap api error: {:?}", e);
                        let (err_code, err_msg) = match e.entity {
                            Some(SessionPostError::Status400(ee)) => (400, ee.message),
                            Some(SessionPostError::Status401(ee)) => (401, ee.message),
                            Some(SessionPostError::Status409(ee)) => (409, ee.message),
                            _ => (999, "unknown error".to_string()),
                        };
                        error!("error in session_responder: {err_code}={err_msg}");
                        request
                            .respond(Err(async_nats::service::error::Error {
                                status: err_msg,
                                code: err_code,
                            }))
                            .await
                            .unwrap();
                    }
                    Err(e) => {
                        error!("Unhandled error in session_responder: {:?}", e);
                        request
                            .respond(Err(async_nats::service::error::Error {
                                status: format!("error generating session: {}", e),
                                code: 500, // internal server error
                            }))
                            .await
                            .unwrap();
                    }
                },
                Err(e) => {
                    warn!("Error decoding session request: {}", e);
                    // TODO: not sure how to properly respond with an error here.
                    request
                        .respond(Err(async_nats::service::error::Error {
                            status: "error decoding session request!".to_string(),
                            code: 400, // bad request
                        }))
                        .await
                        .unwrap();
                }
            }
        }
    })
    .await
    .unwrap();

    Ok(())
}

// if app version disabled, get_session returns:
// Nats-Service-Error-Code: 0
//Nats-Service-Error: error generating session: error in response: status code 400 Bad Request

/// Generate the session on edgegap, the connect token, and reply via nats:
///
/// * Call edgegap's session creation API
/// * Create a connect token
/// * Store the ClientId --> SessionId in NATS KV.
/// * Reply with the connect token.
async fn session_responder(
    state: &MatchmakerState,
    session_request: &SessionRequest,
) -> Result<SessionResponse, EdgegapError<SessionPostError>> {
    // let client = state.nats_client();

    info!("Generating session for {session_request:?}");

    // When asking edgegap for a session, we want the following info for the api call:
    // * app_name
    // * app_version ?
    // * client ip
    // * deployment_request_id

    let mut session_model = SessionModel::new(state.settings.app_name.clone());
    session_model.ip_list = Some(vec![session_request.client_ip.to_string()]);
    session_model
        .webhook_url
        .clone_from(&state.settings.session_webhook_url);
    // create session via edgegap api:
    let post_session = session_post(state.configuration(), session_model).await?;

    info!("{post_session:?}");

    /*
       session creation sometimes is status=ready on the first request, if the was a suitable deployment
       but sometimes is Waiting.. for a couple of secs until deployment finishes.

       seems to usually be fast enough that we can just block here even if deploying..

       webhook sends to "webhook.session" any session callback like this:
       {"session_id": "950dd2eaff09-S", "status": "Ready", "ready": true, "kind": "Seat",
        "user_count": 1, "linked": true, "webhook_url": "https://example.com/hook/session",
         "deployment_request_id": "57f84a8e1298"}

        so perhaps we should have clients watch a requesting_session.SESSION_ID queue,
        and write updates to that (from polling or a webhook).

    */

    let mut session_get;
    let mut tries = 0;
    let mut first_seen_session_id = false;
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    loop {
        tries += 1;
        info!("GET SESSION... ({tries})");
        session_get = get_session(state.configuration(), post_session.session_id.as_str())
            .await
            .map_err(|e| {
                EdgegapError::Io(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("get session error: {}", e),
                ))
            })?;

        info!("{session_get:?}");

        // Avoid session leakage!
        // the first time we get a response with a session_id, we store it in unclaimed_sessions,
        // so we can automatically delete it if it goes unused.
        if !first_seen_session_id {
            first_seen_session_id = true;
            let session_id_str = session_get.session_id.clone();
            info!("Writing session_id to unclaimed_sessions KV: {session_id_str}");
            let val = session_id_str.clone().into();
            state
                .nats
                .kv_unclaimed_sessions()
                .put(session_id_str, val)
                .await
                .expect("Failed to put session_id in unclaimed_sessions KV");
        }

        if session_get.ready {
            break;
        }

        if tries > 50 {
            return Err(EdgegapError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                "session not ready timeout on tries",
            )));
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    }

    // info!("{session_get:?}");

    // We must wait until the session is ready / linked before telling the client to connect.
    // You can ask for a webhook, but for now we just poll until it's ready.

    let deployment = session_get.deployment.expect("deployment not found");

    let Some(ports) = deployment.ports else {
        return Err(edgegap_async::apis::Error::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            "No ports found in deployment!",
        )));
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
    info!("client_id = {client_id}");

    let public_ip_str = deployment.public_ip.as_str();

    let ip = public_ip_str
        .parse::<std::net::IpAddr>()
        .expect("Failed parsing server ip");

    // TODO once the session is ready, the cert digest should have been reported, but
    // there is definitely a race here so we should block on it for a second or so?
    let cert_digest = state
        .nats
        .kv_cert_digests()
        .get(public_ip_str)
        .await
        .expect("Failed to get cert digest from KV");
    let cert_digest = String::from_utf8(cert_digest.unwrap().into())
        .expect("Failed to convert cert digest to string");

    info!("Got cert digest {cert_digest} for {public_ip_str}");

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

    // user-level code using lightyear doesn't even see the connect token, so we do the
    // lookup based on clientid.
    let client_id_str = client_id.to_string();
    state
        .nats
        .kv_c2s()
        .put(
            client_id_str.as_str(),
            session_get.session_id.clone().into(),
        )
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
        .put(session_get.session_id.as_str(), client_id_str.into())
        .await
        .map_err(|e| {
            EdgegapError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to put token KV entry: {}", e),
            ))
        })?;

    info!(
        "Stored token for session {} in NATS KV",
        session_get.session_id
    );

    let resp = SessionResponse {
        connect_token: token_base64,
        gameserver_ip: deployment.public_ip,
        gameserver_port: port as u16,
        cert_digest,
    };

    Ok(resp)
}

// prevent session leaks:
/*
    When a client connects to the game server, the server puts the session_id to KV active_sessions.

    If we write session_id to unclaimed_sessions on issue, then delete when it appears in active_sessions,
    we can poll the unclaimed_sessions for keys older than a timeout, for deletion?


*/
