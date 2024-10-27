use bevy::prelude::*;
use bevygap_matchmaker_bemw::server::BevygapMatchmakerServerPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, bevy::log::LogPlugin::default()));
    app.add_plugins(BevygapMatchmakerServerPlugin);
    app.run();
}
