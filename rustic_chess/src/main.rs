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
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<(&mut Transform,  &mut Pawn, &mut Peices)>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                println!("DID we press the button");
                for (mut transform, mut currentPawn, mut peicesNumber) in &mut text_query {
                    match  *peicesNumber{
                        Peices::Pawn(_, 1.0) => transform.translation.y += 64.0,
                        Peices::Pawn(_, _) => todo!()
                    
                    }
                    
                    transform.translation.y += 64.0;
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

fn print_mouse(mouse: Res<ButtonInput<MouseButton>>){
    println!("Mouse: {:?}", mouse.get_pressed().collect::<Vec<_>>());
}





//https://bevyengine.org/examples-webgpu/camera/2d-top-down-camera/
//https://caballerocoll.com/blog/bevy-chess-tutorial/