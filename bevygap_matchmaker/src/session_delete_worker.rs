use crate::MatchmakerState;
use async_nats::jetstream::{self};
use edgegap::apis::sessions_api::*;
use futures::StreamExt;
use log::*;

// need an erlang/OTP like supervision tree!
pub async fn delete_session_worker_supervisor(
    state: &MatchmakerState,
) -> Result<(), async_nats::Error> {
    loop {
        let state = state.clone();
        let handle = tokio::spawn(async move {
            let res = delete_session_worker(&state).await;
            if let Err(e) = res {
                error!("delete_session_worker error: {e:?}");
            }
        });
        futures::future::join_all([handle]).await;
        warn!("delete_session_worker exited, restarting after timeout");
        tokio::time::sleep(std::time::Duration::from_secs(30)).await;
    }
    // Ok(())
}

async fn delete_session_worker(state: &MatchmakerState) -> Result<(), async_nats::Error> {
    let stream = state.nats.delete_session_stream();
    let consumer = stream
        .create_consumer(jetstream::consumer::pull::Config {
            durable_name: Some("api-deleter-1".to_string()),
            description: Some("Calls edgegap session delete api".to_string()),
            ack_policy: jetstream::consumer::AckPolicy::Explicit,
            ..Default::default()
        })
        .await?;

    loop {
        let mut messages = consumer.fetch().max_messages(100).messages().await?;
        while let Some(Ok(message)) = messages.next().await {
            let session_id = String::from_utf8(message.payload.to_vec())?;
            match session_delete(state.configuration(), session_id.as_str()).await {
                Ok(session_delete_response) => {
                    info!("session_delete ok: {:?}", session_delete_response);
                    message.ack().await?;
                }
                Err(edgegap::apis::Error::ResponseError(resp_content)) => {
                    match resp_content.status.as_u16() {
                        404 => {
                            // session already deleted or never existed.
                            warn!(
                                "session_delete 404: {session_id} - already deleted or not found?"
                            );
                            message.ack().await?;
                        }
                        410 => {
                            // "instance already terminated"
                            warn!("session_delete 410 'instance already terminated': {session_id}");
                            message.ack().await?;
                        }
                        code => {
                            error!("session_delete error status = {code} for {session_id} {resp_content:?}");
                        }
                    }
                }
                Err(e) => {
                    // TODO What to do about junk data on queue that can never be deleted?
                    error!("unhandled session_delete error {session_id}: {e:?}");
                }
            }
        }
        tokio::time::sleep(std::time::Duration::from_millis(5000)).await;
    }

    // Ok(())
}
