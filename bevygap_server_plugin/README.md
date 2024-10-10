This is the bevy plugin that runs on the game server instance on edgegap.

On startup it fetches its context from edgegap API.

has fns to verify connecting players, and delete their session once they disconnect.

talks to NATS

TODO: handle_request hook to verify session per client id?

   if let Some(denied_reason) = self
            .cfg
            .connection_request_handler
            .handle_request(crate::prelude::ClientId::Netcode(token.client_id))
        {
