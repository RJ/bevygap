# Bevygap Client Plugin

Instead of connecting lightyear using `commands.connect_client()`, this plugin will
make a request to the matchmaker, then modify lightyear's config to set the supplied
game server socket address and connect token, then call `commands.connect_client()` for you.

## Usage

```rust
use bevy_bevygap_client_plugin::prelude::*;

// ...

app.add_plugins(BevygapClientPlugin);

// ...

// trigger the connection by setting state to `BevygapClientState::Request`
// or use this function on Commands:
commands.bevygap_connect_client();
```