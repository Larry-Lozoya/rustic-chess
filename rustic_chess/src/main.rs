use bevy::prelude::*;
use setup::setup;
use plugins::*;

mod setup;
mod plugins;

fn main() {
    App::new()
        .add_plugins(SetupPlugin)
        .add_systems(Startup, setup)
        .run();
}





//https://bevyengine.org/examples-webgpu/camera/2d-top-down-camera/
//https://caballerocoll.com/blog/bevy-chess-tutorial/