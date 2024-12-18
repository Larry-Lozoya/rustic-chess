use bevy::prelude::*;
use crate::components::*;

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

