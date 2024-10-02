This is the bevy plugin that runs on the game server instance on edgegap.

On startup it fetches its context from edgegap API.

has fns to verify connecting players, and delete their session once they disconnect.

talks to NATS?

## NATS 

### KV buckets

* bucket: `contexts`
* key: FQDN
* value: JSON of the context from the edgegap api get_context call

* bucket: `sessions`
* key: `{session_id}`
* value: JSON of the session?

* bucket: `gameserver`
* key: `fqdn`

* bucket: `connect_tokens`
* key: `client_id`
* value: `SessionID`
* TTL: ~1 min? just enough time to connect. we don't get the connect token as part of lightyear api,
  but the token encodes the clientid, so we use that to lookup the session.

when issuing a ConnectToken, we write the session id to the connect_tokens bucket.
gameserver verifies 








Overview:

Mostly persistent worlds, ~10 players per server instance. not round based, can join/leave whenever.
So seat based, 1 session = 1 user. casual FFA, no teams. just want players to join a low latency server that has some players on.

My infra:

* gameserver instances on edgegap, which listen on http3/webtransport.
* my custom matchmaker instance (not edgegap), with normal http api
* webserver that serves up the index.html and wasm for the game client

User loads the webpage, grabs the wasm, makes a "connect-me" api request to the matchmaker via http.

Matchmaker takes that user ip, and uses edgegap to:

* create a session `POST /v1/session/{session_id}` (which might trigger an auto-deploy)
* get session, to know the public ip and port to give client: `GET /v1/session/{session_id}`
* generate a connect token, which we associate with the {session_id} in NATS kv.
* tell client to connect to public ip and port, using the connect token.

client connects to gameserver instance, and provides connect token.



When a client connects to the gameserver, server will lookup the session_id from the connect token.
Failing this means rejecting the client connection.
Then does a `GET session` to edgegap API and verifies that:

* the session is ready=true
* the IP the client is connecting from is in "session_ips" (ie, not a snooped session_id)
* the gameserver doesn't already have an active client associated with {session_id} (no double connections)
* the session deployment public_ip matches our public_ip (ie, client is connected to the correct gameserver)

When a client disconnets from the gameserver, the gameserver does a `DELETE /v1/session/{session_id}`

If a gameserver crashes, all sessions linked to the crashing instance will be cleaned up automatically for me.



Gameserver should be putting active session info into NATS kv, and also publishing connect/disconnect events to NATS
for future observability tools. maybe even publishing score events like getting kills, so we can compute
a global leaderboard and ELO rankings.





