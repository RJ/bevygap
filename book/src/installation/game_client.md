# Game Client

From your checked out forked version of the `bevygap-spaceships` repo, run the game client like this by overriding the matchmaker URL to use the service you ran in the previous step:

```bash
MATCHMAKER_URL=http://localhost:3000/matchmaker/ws cargo run -p client
```

When you click connect:

* game makes websocket request to `bevygap_matchmaker_httpd`
* `bevygap_matchmaker_httpd` makes request to `bevygap_matchmaker` via NATS
* `bevygap_matchmaker` sends the client IP to edgegap, creating a session
* `bevygap_matchmaker` waits for edgegap to find a gameserver for this session. Auto-deploy may be starting one up for you.
* Once a gameserver is ready, `bevygap_matchmaker` will create a suitable Connect Token and respond with the token, server IP, and port.
* `bevygap_matchmaker_httpd` will relay this to the client
* Client will establish connection to the gameserver, running on edgegap.

If you got this far, find me on Discord for a high five 🙌