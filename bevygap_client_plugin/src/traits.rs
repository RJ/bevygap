use crate::prelude::*;
use bevy::ecs::world::Command;
use bevy::prelude::*;

struct BevygapConnectCommand;

impl Command for BevygapConnectCommand {
    fn apply(self, world: &mut World) {
        let mut s = world.resource_mut::<NextState<BevygapClientState>>();
        s.set(BevygapClientState::Request);
    }
}

pub trait BevygapConnectExt {
    fn bevygap_connect_client(&mut self);
}

impl<'w, 's> BevygapConnectExt for Commands<'w, 's> {
    fn bevygap_connect_client(&mut self) {
        self.add(BevygapConnectCommand);
    }
}
