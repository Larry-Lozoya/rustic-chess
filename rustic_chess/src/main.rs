use bevy::prelude::*;
use setup::{chess_board, setup};
use plugins::*;

mod setup;
mod plugins;
mod components;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup,setup)
        .add_systems(Startup, chess_board)
        .run();
}





//https://bevyengine.org/examples-webgpu/camera/2d-top-down-camera/
//https://caballerocoll.com/blog/bevy-chess-tutorial/