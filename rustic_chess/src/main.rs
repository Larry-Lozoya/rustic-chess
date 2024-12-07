use bevy::prelude::*;
use setup::{chess_board, setup, setupPieces};
use plugins::*;

mod setup;
mod plugins;
mod components;
    /*
        Link for mouse events:
        https://docs.rs/winit_input_helper/0.15.3/src/winit_input_helper/winit_input_helper.rs.html#312-324
        https://docs.rs/bevy_input/0.14.2/bevy_input/struct.ButtonInput.html
    */

fn main() {
    App::new()
        .add_plugins(SetupPlugin)
        .add_systems(Startup,setup)
        .add_systems(Startup, chess_board)
        .add_systems(Startup, setupPieces)
        .add_systems(Update, print_mouse.run_if(resource_changed::<ButtonInput<MouseButton>>),)
        .run();
}

fn print_mouse(mouse: Res<ButtonInput<MouseButton>>){
    println!("Mouse: {:?}", mouse.get_pressed().collect::<Vec<_>>());
}





//https://bevyengine.org/examples-webgpu/camera/2d-top-down-camera/
//https://caballerocoll.com/blog/bevy-chess-tutorial/