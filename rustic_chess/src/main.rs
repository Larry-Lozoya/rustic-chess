use std::thread::panicking;

use bevy::{prelude::*, transform::commands, winit::WinitSettings,color::palettes::basic::*};
use bevy_mod_picking::*;
use components::{Pawn, Peices};
use setup::{chess_board, setup, setupPieces};
use plugins::*;
use systems::button_system;

mod setup;
mod plugins;
mod components;
mod systems;
    /*
        Link for mouse events:
        https://docs.rs/winit_input_helper/0.15.3/src/winit_input_helper/winit_input_helper.rs.html#312-324
        https://docs.rs/bevy_input/0.14.2/bevy_input/struct.ButtonInput.html

        --MOUSE EVENTS?--
        https://github.com/aevyrie/bevy_mod_picking/blob/main/README.md
        https://docs.rs/bevy_mod_picking/latest/bevy_mod_picking/
        https://caballerocoll.com/blog/bevy-chess-tutorial/
        https://docs.rs/mouse_position/latest/mouse_position/mouse_position/struct.Position.html


        //https://bevyengine.org/examples-webgpu/camera/2d-top-down-camera/
        //https://caballerocoll.com/blog/bevy-chess-tutorial/
    */

fn main() {
    App::new()
        .add_plugins(SetupPlugin)
        .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, setup)
        .add_systems(Startup, chess_board)
        .add_systems(Startup, setupPieces)
        .add_systems(Update, button_system)
        .run();
}
