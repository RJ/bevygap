This is the bevy plugin that runs on the game server instance on edgegap, part of  [bevygap](https://github.com/RJ/bevygap)

Read the [bevygap book](https://rj.github.io/bevygap/) to learn more.


On startup it fetches its context from the edgegap API.

has fns to verify connecting players, and delete their session once they disconnect.

talks to NATS



cargo run -p bevygap_server_plugin --example nats