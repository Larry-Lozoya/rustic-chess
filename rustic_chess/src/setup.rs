use bevy::ecs::component;
use bevy::prelude::*;
use bevy::scene::ron::de;
use bevy::sprite::*;
use palettes::css::GREEN;
use crate::components::*;
use bevy::color::*;

pub const WHITE: Srgba = Srgba::rgb(200., 200., 200.);

pub const SQUARE_SIZE: f32 = 64.;

pub const LEFT: f32 = - SQUARE_SIZE * 4. - SQUARE_SIZE / 2.;
pub const BOTTOM: f32 = - SQUARE_SIZE * 4. - SQUARE_SIZE / 2.;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

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
    Pawn,
    ));

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
    Pawn,
    ));

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








