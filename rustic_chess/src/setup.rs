use bevy::ecs::{component, event};
use bevy::prelude::*;
use bevy::scene::ron::de;
use bevy_mod_picking::*;
use bevy_mod_picking::events::Pointer;
use bevy_mod_picking::prelude::On;
use bevy_mod_picking::events::Click;
use bevy::sprite::*;
use palettes::css::{BLACK, BLUE, GREEN};
use crate::components::*;
use bevy::color::*;
use bevy::input::mouse::MouseButtonInput;

pub const WHITE: Srgba = Srgba::rgb(200., 200., 200.);

pub const SQUARE_SIZE: f32 = 64.;

pub const LEFT: f32 = - SQUARE_SIZE * 4. - SQUARE_SIZE / 2.;
pub const BOTTOM: f32 = - SQUARE_SIZE * 4. - SQUARE_SIZE / 2.;


pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    

    commands.spawn(NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Center,
            width: Val::Px(400.0),
            height: Val::Px(100.0),
            ..default()
        },
        ..default()
    })
    .with_children(|parent| {
        // First button (Move Pawn)
        parent.spawn(ButtonBundle {
            style: Style {
                width: Val::Px(100.0),
                height: Val::Px(100.0),
                ..default()
            },
            background_color: BLUE.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Move Pawn",
                    TextStyle {
                        font_size: 20.0,
                        color: WHITE.into(),
                        font: default(),
                    },
                ),
                ..default()
            });
        });
    
        // Second button (Move NEW Pawn)
        parent.spawn(ButtonBundle {
            style: Style {
                width: Val::Px(100.0),
                height: Val::Px(100.0),
                ..default()
            },
            background_color: BLUE.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Move NEW Pawn",
                    TextStyle {
                        font_size: 20.0,
                        color: WHITE.into(),
                        font: default(),
                    },
                ),
                ..default()
            });
        });
    });
}

pub fn chess_board(mut commands: Commands){
    let black_squares = Sprite{
        color: GREEN.into(),
        custom_size: Some(Vec2::splat(SQUARE_SIZE)),
        ..Default::default()
    };
    let white_squares = Sprite{
        color: WHITE.into(),
        custom_size: Some(Vec2::splat(SQUARE_SIZE)),
        ..Default::default()
    };
    
    for i in 1..9 {
        for j in 1..9 {
            let sprite = if (i + j) % 2 == 0 {
                black_squares.clone()
            } else {
                white_squares.clone()
            };

            let x_t = LEFT + j as f32 * SQUARE_SIZE;
            let y_t = BOTTOM + i as f32 * SQUARE_SIZE;
            commands.spawn((
                SpriteBundle {
                    sprite,
                    transform: Transform::from_xyz(x_t, y_t, 0.),
                    ..Default::default()
                },
                Square,
            ));
        }
    }
}

pub fn setupPieces(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
){
    /*
    White Pawns
    */
    commands.spawn((SpriteBundle{
        texture: asset_server.load("pieces/individual/pawn-white-16x16.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(100., 100.)),
            ..default()
        },
        transform: Transform::from_xyz(-225., -145., 0.5),
        ..Default::default()
    },
    Pawn, Peices::Pawn("white".to_string(), 1.0)));

    commands.spawn((SpriteBundle{
        texture: asset_server.load("pieces/individual/pawn-white-16x16.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(100., 100.)),
            ..default()
        },
        transform: Transform::from_xyz(-160., -145., 0.5),
        ..Default::default()
    },
    Pawn,
    ));

    commands.spawn((SpriteBundle{
        texture: asset_server.load("pieces/individual/pawn-white-16x16.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(100., 100.)),
            ..default()
        },
        transform: Transform::from_xyz(-95., -145., 0.5),
        ..Default::default()
    },
    Pawn,
    ));

    commands.spawn((SpriteBundle{
        texture: asset_server.load("pieces/individual/pawn-white-16x16.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(100., 100.)),
            ..default()
        },
        transform: Transform::from_xyz(-30., -145., 0.5),
        ..Default::default()
    },
    Pawn,
    ));

    commands.spawn((SpriteBundle{
        texture: asset_server.load("pieces/individual/pawn-white-16x16.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(100., 100.)),
            ..default()
        },
        transform: Transform::from_xyz(35., -145., 0.5),
        ..Default::default()
    },
    Pawn,
    ));

    commands.spawn((SpriteBundle{
        texture: asset_server.load("pieces/individual/pawn-white-16x16.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(100., 100.)),
            ..default()
        },
        transform: Transform::from_xyz(95., -145., 0.5),
        ..Default::default()
    },
    Pawn,
    ));

    commands.spawn((SpriteBundle{
        texture: asset_server.load("pieces/individual/pawn-white-16x16.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(100., 100.)),
            ..default()
        },
        transform: Transform::from_xyz(160., -145., 0.5),
        ..Default::default()
    },
    Pawn,
    ));

    commands.spawn((SpriteBundle{
        texture: asset_server.load("pieces/individual/pawn-white-16x16.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(100., 100.)),
            ..default()
        },
        transform: Transform::from_xyz(225., -145., 0.5),
        ..Default::default()
    },
    Pawn,
    ));

    /*
    Black Pawns
    */
    commands.spawn((SpriteBundle{
        texture: asset_server.load("pieces/individual/pawn-black-16x16.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(100., 100.)),
            ..default()
        },
        transform: Transform::from_xyz(-225., 175., 0.5),
        ..Default::default()
    },
    Pawn, Peices::Pawn("black".to_string(), 2.0)));

    commands.spawn((SpriteBundle{
        texture: asset_server.load("pieces/individual/pawn-black-16x16.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(100., 100.)),
            ..default()
        },
        transform: Transform::from_xyz(-160., 175., 0.5),
        ..Default::default()
    },
    Pawn,
    ));

    commands.spawn((SpriteBundle{
        texture: asset_server.load("pieces/individual/pawn-black-16x16.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(100., 100.)),
            ..default()
        },
        transform: Transform::from_xyz(-95., 175., 0.5),
        ..Default::default()
    },
    Pawn,
    ));

    commands.spawn((SpriteBundle{
        texture: asset_server.load("pieces/individual/pawn-black-16x16.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(100., 100.)),
            ..default()
        },
        transform: Transform::from_xyz(-30., 175., 0.5),
        ..Default::default()
    },
    Pawn,
    ));

    commands.spawn((SpriteBundle{
        texture: asset_server.load("pieces/individual/pawn-black-16x16.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(100., 100.)),
            ..default()
        },
        transform: Transform::from_xyz(35., 175., 0.5),
        ..Default::default()
    },
    Pawn,
    ));

    commands.spawn((SpriteBundle{
        texture: asset_server.load("pieces/individual/pawn-black-16x16.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(100., 100.)),
            ..default()
        },
        transform: Transform::from_xyz(95., 175., 0.5),
        ..Default::default()
    },
    Pawn,
    ));

    commands.spawn((SpriteBundle{
        texture: asset_server.load("pieces/individual/pawn-black-16x16.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(100., 100.)),
            ..default()
        },
        transform: Transform::from_xyz(160., 175., 0.5),
        ..Default::default()
    },
    Pawn,
    ));

    commands.spawn((SpriteBundle{
        texture: asset_server.load("pieces/individual/pawn-black-16x16.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(100., 100.)),
            ..default()
        },
        transform: Transform::from_xyz(225., 175., 0.5),
        ..Default::default()
    },
    Pawn,
    ));

    /*
    White Rook
     */
    commands.spawn((SpriteBundle{
        texture: asset_server.load("pieces/individual/rook-white-16x16.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(100., 100.)),
            ..default()
        },
        transform: Transform::from_xyz(-225., -215., 0.5),
        ..Default::default()
    },
    Rook,
    ));

    commands.spawn((SpriteBundle{
        texture: asset_server.load("pieces/individual/rook-white-16x16.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(100., 100.)),
            ..default()
        },
        transform: Transform::from_xyz(225., -215., 0.5),
        ..Default::default()
    },
    Rook,
    ));

    /*
    Black Rook
     */

     commands.spawn((SpriteBundle{
        texture: asset_server.load("pieces/individual/rook-black-16x16.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(100., 100.)),
            ..default()
        },
        transform: Transform::from_xyz(-225., 235., 0.5),
        ..Default::default()
    },
    Rook,
    ));

    commands.spawn((SpriteBundle{
        texture: asset_server.load("pieces/individual/rook-black-16x16.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(100., 100.)),
            ..default()
        },
        transform: Transform::from_xyz(225., 235., 0.5),
        ..Default::default()
    },
    Rook,
    ));

    /*
    White King
    */

    commands.spawn((SpriteBundle{
        texture: asset_server.load("pieces/individual/king-white-16x16.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(80., 80.)),
            ..default()
        },
        transform: Transform::from_xyz(35., -225., 0.5),
        ..Default::default()
    },
    King,
    ));

    /*
    Black King
    */

    commands.spawn((SpriteBundle{
        texture: asset_server.load("pieces/individual/king-black-16x16.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(80., 80.)),
            ..default()
        },
        transform: Transform::from_xyz(35., 225., 0.5),
        ..Default::default()
    },
    King,
    ));

    /*
    white Knights
    */
    commands.spawn((SpriteBundle{
        texture: asset_server.load("pieces/individual/knight-white-16x16.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(100., 100.)),
            ..default()
        },
        transform: Transform::from_xyz(-160., -215., 0.5),
        ..Default::default()
    },
    Knight,
    ));
    commands.spawn((SpriteBundle{
        texture: asset_server.load("pieces/individual/knight-white-16x16.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(100., 100.)),
            ..default()
        },
        transform: Transform::from_xyz(160., -215., 0.5),
        ..Default::default()
    },
    Knight,
    ));
    /*
    black knights
     */
    commands.spawn((SpriteBundle{
        texture: asset_server.load("pieces/individual/knight-black-16x16.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(100., 100.)),
            ..default()
        },
        transform: Transform::from_xyz(-160., 235., 0.5),
        ..Default::default()
    },
    Knight,
    ));
    commands.spawn((SpriteBundle{
        texture: asset_server.load("pieces/individual/knight-black-16x16.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(100., 100.)),
            ..default()
        },
        transform: Transform::from_xyz(160., 235., 0.5),
        ..Default::default()
    },
    Knight,
    ));
    /*
    white bishops
    */
    commands.spawn((SpriteBundle{
        texture: asset_server.load("pieces/individual/bishop-white-16x16.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(90., 90.)),
            ..default()
        },
        transform: Transform::from_xyz(-95., -220., 0.5),
        ..Default::default()
    },
    Bishop,
    ));
    commands.spawn((SpriteBundle{
        texture: asset_server.load("pieces/individual/bishop-white-16x16.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(90., 90.)),
            ..default()
        },
        transform: Transform::from_xyz(95., -220., 0.5),
        ..Default::default()
    },
    Bishop,
    ));
    /*
    Black bishops
     */
    commands.spawn((SpriteBundle{
        texture: asset_server.load("pieces/individual/bishop-black-16x16.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(90., 90.)),
            ..default()
        },
        transform: Transform::from_xyz(-95., 230., 0.5),
        ..Default::default()
    },
    Bishop,
    ));
    commands.spawn((SpriteBundle{
        texture: asset_server.load("pieces/individual/bishop-black-16x16.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(90., 90.)),
            ..default()
        },
        transform: Transform::from_xyz(95., 230., 0.5),
        ..Default::default()
    },
    Bishop,
    ));

    /*
    White Queen
     */
    commands.spawn((SpriteBundle{
        texture: asset_server.load("pieces/individual/queen-white-16x16.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(80., 80.)),
            ..default()
        },
        transform: Transform::from_xyz(-30., -220., 0.5),
        ..Default::default()
    },
    Queen,
    ));
    /*
    Black Queen
     */
    commands.spawn((SpriteBundle{
        texture: asset_server.load("pieces/individual/queen-black-16x16.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(80., 80.)),
            ..default()
        },
        transform: Transform::from_xyz(-30., 225., 0.5),
        ..Default::default()
    },
    Queen,
    ));
}

pub fn mouse_button_location(mut mousebtn_evr: EventReader<CursorMoved>,){
    use bevy::input::ButtonState;

    for ev in mousebtn_evr.read(){
        println!("New cursor position: X: {}, Y: {}, in Window ID: {:?}", ev.position.x, ev.position.y, ev.window);
    }
}

pub fn mouse_button_events(
    mut mousebtn_evr: EventReader<MouseButtonInput>,
) {
    use bevy::input::ButtonState;

    for ev in mousebtn_evr.read() {
        match ev.state {
            ButtonState::Pressed => {
                println!("Mouse button press: {:?}", ev.button);
            }
            ButtonState::Released => {
                println!("Mouse button release: {:?}", ev.button);
            }
        }
    }
}

