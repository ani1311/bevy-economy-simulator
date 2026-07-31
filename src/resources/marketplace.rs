use bevy::prelude::*;

use crate::components::Day;

#[derive(Resource)]
pub struct WorldState {
    today: Day,
}

impl WorldState {
    pub fn new() -> WorldState {
        WorldState { today: 0 }
    }

    pub fn today(&self) -> Day {
        self.today
    }

    pub fn advance_day(&mut self) {
        self.today += 1
    }
}

pub fn setup_world(mut commands: Commands) {
    let world = WorldState::new();

    commands.insert_resource(world);
}

pub fn update(mut world: ResMut<WorldState>) {
    world.advance_day();
}
