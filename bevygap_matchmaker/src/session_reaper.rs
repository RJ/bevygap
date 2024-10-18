/// Detects orphaned edgegap sessions and schedules them for deletion by the API
/// Actual API-delete call happens in the session_delete_worker.
use crate::MatchmakerState;
use ::time::OffsetDateTime;
use async_nats::jetstream::kv::Operation;
use futures::{StreamExt, TryStreamExt};
use log::*;
use tokio::time::{self, Duration};

pub(crate) async fn session_cleanup_supervisor(
    orig_state: &MatchmakerState,
) -> Result<(), async_nats::Error> {
    let state = orig_state.clone();
    let handle1 = tokio::spawn(async move {
        loop {
            let _ = session_cleanup_watcher(&state).await;
            error!("session_cleanup_watcher exited, restarting");
        }
    });
    let state = orig_state.clone();
    let handle2 = tokio::spawn(async move {
        loop {
            let _ = unclaimed_session_reaper(&state).await;
            error!("unclaimed_session_reaper exited, restarting");
        }
    });
    futures::future::join_all([handle1, handle2]).await;
    Ok(())
}

/// Get all the session keys in unclaimed sessions - if any are older than 30 seconds,
/// enqueue them for deletion.
/// Session ids must be removed from unclaimed_sessions once a gameserver connection happens.
async fn unclaimed_session_reaper(state: &MatchmakerState) -> Result<(), async_nats::Error> {
    // how often to check for orphaned sessions:
    let mut interval = time::interval(Duration::from_millis(5000));
    let kv = state.nats.kv_unclaimed_sessions();
    loop {
        interval.tick().await;
        let mut keys = kv.keys().await?.boxed();
        while let Some(key) = keys.try_next().await? {
            let Ok(Some(entry)) = kv.entry(&key).await else {
                continue;
            };
            let session_id = String::from_utf8(entry.value.to_vec())
                .expect("Failed to convert session_id to string");
            let age = OffsetDateTime::now_utc() - entry.created;
            info!("* Session {session_id} is {age} old");
            if age > Duration::from_secs(crate::MAX_SESSION_CREATION_SECONDS + 2) {
                warn!("Unclaimed session {session_id} is older than 30 seconds = {age}");
                // write to delete_sessions work queue and remove from unclaimed_sessions KV
                state
                    .nats
                    .enqueue_session_delete(session_id.clone())
                    .await?;
                kv.delete(&key).await?;
            }
        }
    }
    Ok(())
}

/// Deletes sessions once a gameserver removes the active_sessions KV entry.
///  this is the happy path, where there were no orphans..
async fn session_cleanup_watcher(state: &MatchmakerState) -> Result<(), async_nats::Error> {
    let kv = state.nats.kv_active_connections();
    let mut watcher = kv.watch(">").await?;
    while let Some(event) = watcher.next().await {
        info!("{event:?}");
        match event {
            Ok(event) => {
                let session_id = event.key;
                if event.operation == Operation::Delete {
                    info!("active_connection deleted, deleting session {session_id}",);
                    state
                        .nats
                        .enqueue_session_delete(session_id.clone())
                        .await?;
                }
                if event.operation == Operation::Put {
                    info!("New Session put {session_id}, deleting from unclaimed_sessions ");
                    // delete this session_id from unclaimed_sessions.
                    let _ = state
                        .nats
                        .kv_unclaimed_sessions()
                        .delete(session_id.as_str())
                        .await;
                }
            }
            Err(e) => {
                warn!("KV event error watching for session cleanup: {:?}", e);
            }
        }
    }
    Ok(())
}
