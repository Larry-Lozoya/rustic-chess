use bevy::prelude::*;
use setup::setup;

mod setup;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}