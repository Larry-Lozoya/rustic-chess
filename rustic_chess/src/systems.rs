use bevy::prelude::*;
use crate::components::*;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);

/*
    This is the system we designed to keep trck on which button has been pressed.
    We have an interaction_query of type Query which takes in an Interaction, Name, background color and border color.
    The name is sent in from the button being pressed. This would be the "MoveFirstWhitePawn" and the Interaction is sent in depending on what the user is doing.
    We have an if statment set up that checks which button is pressed and depending on that button it will move (transform) the pawn.
    If the pawn is white we move it a positive distance and if the pawn is black we move it a negative distance. 
    This is the same code for each pawn but each button takes a different name and ID. The IDs are also passed once the name has been passed through.
    This only applies if the user clicks on a button. If a user hovers or is not on a button then the button does not do anything.    
 */
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
                if name.as_str()== "MoveFirstWhitePawn" {
                    for (mut transform, peice) in &mut pawn_query {
                        if let Peices::Pawn(ref pawn_color, ref pawn_num) = peice {
                            if pawn_color == "white" && *pawn_num == 1.0{
                                transform.translation.y += 64.0;
                                println!("Moving pawn: {}", pawn_color);
                                break;
                            }
                        }
                    }
                }else if name.as_str() == "MoveSecondWhitePawn"{
                    for (mut transform, peice) in &mut pawn_query {
                        if let Peices::Pawn(ref pawn_color, ref pawn_num) = peice {
                            if pawn_color == "white" && *pawn_num == 3.0{
                                transform.translation.y += 64.0;
                                println!("Moving pawn: {}", pawn_color);
                                break;
                            }
                        }
                    }
                }else if name.as_str() == "MoveThirdWhitePawn"{
                    for (mut transform, peice) in &mut pawn_query {
                        if let Peices::Pawn(ref pawn_color, ref pawn_num) = peice {
                            if pawn_color == "white" && *pawn_num == 5.0{
                                transform.translation.y += 64.0;
                                println!("Moving pawn: {}", pawn_color);
                                break;
                            }
                        }
                    }
                }else if name.as_str() == "MoveForthWhitePawn"{
                    for (mut transform, peice) in &mut pawn_query {
                        if let Peices::Pawn(ref pawn_color, ref pawn_num) = peice {
                            if pawn_color == "white" && *pawn_num == 7.0{
                                transform.translation.y += 64.0;
                                println!("Moving pawn: {}", pawn_color);
                                break;
                            }
                        }
                    }
                }else if name.as_str() == "MoveFifthWhitePawn"{
                    for (mut transform, peice) in &mut pawn_query {
                        if let Peices::Pawn(ref pawn_color, ref pawn_num) = peice {
                            if pawn_color == "white" && *pawn_num == 9.0{
                                transform.translation.y += 64.0;
                                println!("Moving pawn: {}", pawn_color);
                                break;
                            }
                        }
                    }
                }else if name.as_str() == "MoveSixWhitePawn"{
                    for (mut transform, peice) in &mut pawn_query {
                        if let Peices::Pawn(ref pawn_color, ref pawn_num) = peice {
                            if pawn_color == "white" && *pawn_num == 11.0{
                                transform.translation.y += 64.0;
                                println!("Moving pawn: {}", pawn_color);
                                break;
                            }
                        }
                    }
                }
                else if name.as_str() == "MoveSevenWhitePawn"{
                    for (mut transform, peice) in &mut pawn_query {
                        if let Peices::Pawn(ref pawn_color, ref pawn_num) = peice {
                            if pawn_color == "white" && *pawn_num == 13.0{
                                transform.translation.y += 64.0;
                                println!("Moving pawn: {}", pawn_color);
                                break;
                            }
                        }
                    }
                }
                else if name.as_str() == "MoveEightWhitePawn"{
                    for (mut transform, peice) in &mut pawn_query {
                        if let Peices::Pawn(ref pawn_color, ref pawn_num) = peice {
                            if pawn_color == "white" && *pawn_num == 15.0{
                                transform.translation.y += 64.0;
                                println!("Moving pawn: {}", pawn_color);
                                break;
                            }
                        }
                    }
                }else if name.as_str() == "MoveFirstBlackPawn"{
                    for (mut transform, peice) in &mut pawn_query {
                        if let Peices::Pawn(ref pawn_color, ref pawn_num) = peice {
                            if pawn_color == "black" && *pawn_num == 2.0{
                                transform.translation.y -= 64.0;
                                println!("Moving pawn: {}", pawn_color);
                                break;
                            }
                        }
                    }
                }else if name.as_str() == "MoveSecondBlackPawn"{
                    for (mut transform, peice) in &mut pawn_query {
                        if let Peices::Pawn(ref pawn_color, ref pawn_num) = peice {
                            if pawn_color == "black" && *pawn_num == 4.0{
                                transform.translation.y -= 64.0;
                                println!("Moving pawn: {}", pawn_color);
                                break;
                            }
                        }
                    }
                }else if name.as_str() == "MoveThirdBlackPawn"{
                    for (mut transform, peice) in &mut pawn_query {
                        if let Peices::Pawn(ref pawn_color, ref pawn_num) = peice {
                            if pawn_color == "black" && *pawn_num == 6.0{
                                transform.translation.y -= 64.0;
                                println!("Moving pawn: {}", pawn_color);
                                break;
                            }
                        }
                    }
                }else if name.as_str() == "MoveForthBlackPawn"{
                    for (mut transform, peice) in &mut pawn_query {
                        if let Peices::Pawn(ref pawn_color, ref pawn_num) = peice {
                            if pawn_color == "black" && *pawn_num == 8.0{
                                transform.translation.y -= 64.0;
                                println!("Moving pawn: {}", pawn_color);
                                break;
                            }
                        }
                    }
                }else if name.as_str() == "MoveFifthBlackPawn"{
                    for (mut transform, peice) in &mut pawn_query {
                        if let Peices::Pawn(ref pawn_color, ref pawn_num) = peice {
                            if pawn_color == "black" && *pawn_num == 10.0{
                                transform.translation.y -= 64.0;
                                println!("Moving pawn: {}", pawn_color);
                                break;
                            }
                        }
                    }
                }
                else if name.as_str() == "MoveSixBlackPawn"{
                    for (mut transform, peice) in &mut pawn_query {
                        if let Peices::Pawn(ref pawn_color, ref pawn_num) = peice {
                            if pawn_color == "black" && *pawn_num == 12.0{
                                transform.translation.y -= 64.0;
                                println!("Moving pawn: {}", pawn_color);
                                break;
                            }
                        }
                    }
                }else if name.as_str() == "MoveSevenBlackPawn"{
                    for (mut transform, peice) in &mut pawn_query {
                        if let Peices::Pawn(ref pawn_color, ref pawn_num) = peice {
                            if pawn_color == "black" && *pawn_num == 14.0{
                                transform.translation.y -= 64.0;
                                println!("Moving pawn: {}", pawn_color);
                                break;
                            }
                        }
                    }
                }else if name.as_str() == "MoveEightBlackPawn"{
                    for (mut transform, peice) in &mut pawn_query {
                        if let Peices::Pawn(ref pawn_color, ref pawn_num) = peice {
                            if pawn_color == "black" && *pawn_num == 16.0{
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

