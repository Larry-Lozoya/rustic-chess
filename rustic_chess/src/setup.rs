use bevy::ecs::component;
use bevy::prelude::*;
use bevy::sprite::*;
use crate::components::*;

use bevy::color::*;

pub const BLACK: Srgba = Srgba::rgb(0.,0.,0.);
pub const WHITE: Srgba = Srgba::rgb(255., 255., 255.);

pub const SQUARE_SIZE: f32 = 64.;

pub const LEFT: f32 = -SQUARE_SIZE * 4. - SQUARE_SIZE / 2.;
pub const BOTTOM: f32 = -SQUARE_SIZE * 4. - SQUARE_SIZE / 2.;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

}

pub fn chess_board(mut commands: Commands){
    let black_squares = Sprite{
        color: BLACK.into(),
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
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("individual/pawn-black-16x16.png"),
            transform: Transform::from_xyz(0., 0., 2.),
            ..default()
        },
        TextureAtlas {
            index: 1,
            layout: texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
                UVec2::new(34, 24),
                3,
                1,
                None,
                None,
            )),
        },
        Pieces,
    ));
}




