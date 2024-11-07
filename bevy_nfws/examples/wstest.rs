use bevy::prelude::*;
use bevy_nfws::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(NfwsPlugin);

    app.add_systems(Startup, connect);
    app.add_systems(Update, poll);

    app.run();
}

fn connect(mut commands: Commands) {
    let url = "ws://127.0.0.1:3000/matchmaker/ws".to_string();
    info!("Connecting to {url}");
    let mut ch = NfwsHandle::new(url);
    ch.send_text(
        "{\"client_ip\":\"127.0.0.1\", \"game\":\"bevygap-spaceships\", \"version\":\"1\"}"
            .to_string(),
    );
    commands.spawn(ch);
}

fn poll(mut q: Query<(Entity, &mut NfwsHandle)>, mut commands: Commands) {
    for (entity, mut wschan) in q.iter_mut() {
        let ev = wschan.next_event();
        match ev {
            NfwsPollResult::Closed => {
                info!("EV None = closed, despawning");
                commands.entity(entity).despawn();
                continue;
            }
            NfwsPollResult::Empty => continue,
            NfwsPollResult::Event(rec) => info!("EV: {rec:?}"),
        }
    }
}
