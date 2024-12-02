use bevy::prelude::*;
use setup::setup;

mod setup;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}





//https://bevyengine.org/examples-webgpu/camera/2d-top-down-camera/
//https://caballerocoll.com/blog/bevy-chess-tutorial/