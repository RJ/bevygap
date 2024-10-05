# Bevygap: Running Bevy+Lightyear multiplayer servers on Edgegap

This is a (WIP) suite of services for running multiplayer game servers on Edgegap, where the games are
built with bevy using the Lightyear networking library.

The goal is to have an easy-to-deploy system either yourself with docker-compose, or in the cloud,
to use Edgegap to spin up gameservers on demand. 

## bevygap_matchmaker

Clients wishing to connect to the game make a request to our matchmaker service, which then:

* Creates an Edgegap session (which may trigger a new server deployment)
* Creates a new Lightyear client id and connect token, associated with the session
* Stores the token/session mapping in NATS KV
* Returns the connect token, and gameserver IP and port to the client.
  (the gameserver ip+port will be a machine controlled by Edgegap, running your game server's docker image)


## bevy_edgegap_gameserver

A bevy plugin for the gameserver, which loads its deployment context from the edgegap API on boot,
and connects to our NATS instance in order to lookup session information.

When a player connects, we lookup the Edgegap session ID in NATS KV that corresponds to the `client_id` from the `ConnectToken`.

When the player disconnects, it makes a request to Edgegap's API to delete the session. (TODO)


## bevygap_httpd (TODO)

An http endpoint to make "i want to play" requests to the matchmaker.

## nats

Nats is the shared state and messaging backend between our various components.

# Running

You need an edgegap account, with your gameserver's docker image built and pushed to a registry.
The gameserver must use the bevygap_edgegap plugin, with NATS configured correctly (TODO)

```bash
# This will start nats, the matchmaker, and the httpd
docker compose up
```

```bash
# Example matchmaking request
nats req session.gensession '{"client_ip": "81.128.172.55", "foo":123}'
```
<pre>
14:24:33 Sending request on "session.gensession"
14:24:34 Received with rtt 481.2565ms
{"connect_token":"TkV...AAAA=","gameserver_ip":"172.104.159.122","gameserver_port":32041,"link":"172.104.159.122:32041"}
</pre>

```bash
# Note that NATS KV now links the assigned session id to the client id from the issued token.
nats kv ls sessions_ly2eg
nats kv ls sessions_eg2ly

# get the Session ID from the lightyear client id:
nats kv get sessions_ly2eg "123456.."
```
