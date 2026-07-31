use bevy::prelude::*;

mod components;
mod render;
mod resources;
mod simulation;

pub use components::Consumer;
pub use components::Producer;
pub use simulation::*;

use crate::render::EconomyDisplayPlugin;
use crate::resources::{setup_world, update};

pub fn run_app() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EconomyDisplayPlugin)
        .insert_resource(Time::<Fixed>::from_hz(1.0))
        .add_systems(Startup, (setup_world, setup_consumers, setup_producers))
        .add_systems(FixedUpdate, get_simulation_step())
        .run();
}
