use std::thread::panicking;

use bevy::{prelude::*, transform::commands, winit::WinitSettings,color::palettes::basic::*};
use bevy_mod_picking::*;
use components::{Pawn, Peices};
use setup::{chess_board, setup, setupPieces,mouse_button_events, mouse_button_location};
use plugins::*;

mod setup;
mod plugins;
mod components;
    /*
        Link for mouse events:
        https://docs.rs/winit_input_helper/0.15.3/src/winit_input_helper/winit_input_helper.rs.html#312-324
        https://docs.rs/bevy_input/0.14.2/bevy_input/struct.ButtonInput.html

        --MOUSE EVENTS?--
        https://github.com/aevyrie/bevy_mod_picking/blob/main/README.md
        https://docs.rs/bevy_mod_picking/latest/bevy_mod_picking/
        https://caballerocoll.com/blog/bevy-chess-tutorial/
        https://docs.rs/mouse_position/latest/mouse_position/mouse_position/struct.Position.html
    */

fn main() {
    App::new()
        .add_plugins(SetupPlugin)
        //.add_plugins(DefaultPickingPlugins)
        // .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 20.2)))
        .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, setup)
        .add_systems(Startup,setup)
        .add_systems(Startup, chess_board)
        .add_systems(Startup, setupPieces)
        .add_systems(Update, button_system)
        //.add_systems(Update, mouse_button_location)
        //.add_systems(Update, mouse_button_events)
        //.add_systems(Update, print_mouse.run_if(resource_changed::<ButtonInput<MouseButton>>),)
        .run();
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

pub fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &Name,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut pawn_query: Query<(&mut Transform, &Peices), With<Pawn>>,
) {
    for (interaction, name, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if name.as_str()== "MoveWhitePawnButton" {
                    println!("DID we press the button");
                    for (mut transform, peice) in &mut pawn_query {
                        if let Peices::Pawn(ref pawn_color, ref pawn_num) = peice {
                            if pawn_color == "white" && *pawn_num == 1.0{
                                transform.translation.y += 64.0;
                                println!("Moving pawn: {}", pawn_color);
                                break;
                            }
                        }
                    }
                }else if name.as_str() == "MoveBlackPawnButton"{
                    println!("DID we press the button");
                    for (mut transform, peice) in &mut pawn_query {
                        if let Peices::Pawn(ref pawn_color, ref pawn_num) = peice {
                            if pawn_color == "black" && *pawn_num == 2.0{
                                transform.translation.y -= 64.0;
                                println!("Moving pawn: {}", pawn_color);
                                break;
                            }
                        }
                    }
                }
                *color = Color::WHITE.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                //**text = "Button".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}





//https://bevyengine.org/examples-webgpu/camera/2d-top-down-camera/
//https://caballerocoll.com/blog/bevy-chess-tutorial/