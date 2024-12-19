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

//Setting the color whtie, Bevy also has built in colors like black, blue, red, and green.
pub const WHITE: Srgba = Srgba::rgb(200., 200., 200.);

//Creating a variable to hold the square size.
pub const SQUARE_SIZE: f32 = 64.;

//Setting the size of each square.
pub const LEFT: f32 = - SQUARE_SIZE * 4. - SQUARE_SIZE / 2.;
pub const BOTTOM: f32 = - SQUARE_SIZE * 4. - SQUARE_SIZE / 2.;


/*
    We use the NodeBundle to spawn a continer for the buttons.
    This NodeBundle takes in specific pramaters and we only set the ones we needed while setting the rest to default.
    Inside of this NodeBundle we spawn a ButtonBundle. This is something that was built in on Bevy.
    Within each of our buttons we have set our style elements and assing a name.
    The names repersent what the button will be doing.
    Each button is spawned with a specific name and a text value that tells the user what the button will do.
    This is the same for each button. 
*/
pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    

    commands.spawn(NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Center,
            width: Val::Px(600.0),
            height: Val::Px(100.0),
            ..default()
        },
        ..default()
    })
    .with_children(|parent| {
        parent.spawn((ButtonBundle {
            style: Style {
                width: Val::Px(100.0),
                height: Val::Px(100.0),
                ..default()
            },
            background_color: BLUE.into(),
            ..default()
        },
        Name::new("MoveFirstWhitePawn"),
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Move W P1",
                    TextStyle {
                        font_size: 20.0,
                        color: WHITE.into(),
                        font: default(),
                    },
                ),
                ..default()
            });
        });

        parent.spawn((ButtonBundle {
            style: Style {
                width: Val::Px(100.0),
                height: Val::Px(100.0),
                ..default()
            },
            background_color: BLUE.into(),
            ..default()
        },Name::new("MoveSecondWhitePawn"),

        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Move W P2",
                    TextStyle {
                        font_size: 20.0,
                        color: WHITE.into(),
                        font: default(),
                    },
                ),
                ..default()
            });
        });

        parent.spawn((ButtonBundle {
            style: Style {
                width: Val::Px(100.0),
                height: Val::Px(100.0),
                ..default()
            },
            background_color: BLUE.into(),
            ..default()
        },Name::new("MoveThirdWhitePawn"),

        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Move W P3",
                    TextStyle {
                        font_size: 20.0,
                        color: WHITE.into(),
                        font: default(),
                    },
                ),
                ..default()
            });
        });

        parent.spawn((ButtonBundle {
            style: Style {
                width: Val::Px(100.0),
                height: Val::Px(100.0),
                ..default()
            },
            background_color: BLUE.into(),
            ..default()
        },Name::new("MoveForthWhitePawn"),

        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Move W P4",
                    TextStyle {
                        font_size: 20.0,
                        color: WHITE.into(),
                        font: default(),
                    },
                ),
                ..default()
            });
        });
        parent.spawn((ButtonBundle {
            style: Style {
                width: Val::Px(100.0),
                height: Val::Px(100.0),
                ..default()
            },
            background_color: BLUE.into(),
            ..default()
        },Name::new("MoveFifthWhitePawn"),

        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Move W P5",
                    TextStyle {
                        font_size: 20.0,
                        color: WHITE.into(),
                        font: default(),
                    },
                ),
                ..default()
            });
        });
        parent.spawn((ButtonBundle {
            style: Style {
                width: Val::Px(100.0),
                height: Val::Px(100.0),
                ..default()
            },
            background_color: BLUE.into(),
            ..default()
        },Name::new("MoveSixWhitePawn"),

        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Move W P6",
                    TextStyle {
                        font_size: 20.0,
                        color: WHITE.into(),
                        font: default(),
                    },
                ),
                ..default()
            });
        });

        parent.spawn((ButtonBundle {
            style: Style {
                width: Val::Px(100.0),
                height: Val::Px(100.0),
                ..default()
            },
            background_color: BLUE.into(),
            ..default()
        },Name::new("MoveSevenWhitePawn"),

        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Move W P7",
                    TextStyle {
                        font_size: 20.0,
                        color: WHITE.into(),
                        font: default(),
                    },
                ),
                ..default()
            });
        });

        parent.spawn((ButtonBundle {
            style: Style {
                width: Val::Px(100.0),
                height: Val::Px(100.0),
                ..default()
            },
            background_color: BLUE.into(),
            ..default()
        },Name::new("MoveEightWhitePawn"),

        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Move W P8",
                    TextStyle {
                        font_size: 20.0,
                        color: WHITE.into(),
                        font: default(),
                    },
                ),
                ..default()
            });
        });

        parent.spawn((ButtonBundle {
            style: Style {
                width: Val::Px(100.0),
                height: Val::Px(100.0),
                ..default()
            },
            background_color: BLUE.into(),
            ..default()
        },Name::new("MoveFirstBlackPawn"),

        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Move B P1",
                    TextStyle {
                        font_size: 20.0,
                        color: WHITE.into(),
                        font: default(),
                    },
                ),
                ..default()
            });
        });

        parent.spawn((ButtonBundle {
            style: Style {
                width: Val::Px(100.0),
                height: Val::Px(100.0),
                ..default()
            },
            background_color: BLUE.into(),
            ..default()
        },Name::new("MoveSecondBlackPawn"),

        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Move B P2",
                    TextStyle {
                        font_size: 20.0,
                        color: WHITE.into(),
                        font: default(),
                    },
                ),
                ..default()
            });
        });

        parent.spawn((ButtonBundle {
            style: Style {
                width: Val::Px(100.0),
                height: Val::Px(100.0),
                ..default()
            },
            background_color: BLUE.into(),
            ..default()
        },Name::new("MoveThirdBlackPawn"),

        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Move B P3",
                    TextStyle {
                        font_size: 20.0,
                        color: WHITE.into(),
                        font: default(),
                    },
                ),
                ..default()
            });
        });

        parent.spawn((ButtonBundle {
            style: Style {
                width: Val::Px(100.0),
                height: Val::Px(100.0),
                ..default()
            },
            background_color: BLUE.into(),
            ..default()
        },Name::new("MoveForthBlackPawn"),

        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Move B P4",
                    TextStyle {
                        font_size: 20.0,
                        color: WHITE.into(),
                        font: default(),
                    },
                ),
                ..default()
            });
        });

        parent.spawn((ButtonBundle {
            style: Style {
                width: Val::Px(100.0),
                height: Val::Px(100.0),
                ..default()
            },
            background_color: BLUE.into(),
            ..default()
        },Name::new("MoveFifthBlackPawn"),

        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Move B P5",
                    TextStyle {
                        font_size: 20.0,
                        color: WHITE.into(),
                        font: default(),
                    },
                ),
                ..default()
            });
        });

        parent.spawn((ButtonBundle {
            style: Style {
                width: Val::Px(100.0),
                height: Val::Px(100.0),
                ..default()
            },
            background_color: BLUE.into(),
            ..default()
        },Name::new("MoveSixBlackPawn"),

        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Move B P6",
                    TextStyle {
                        font_size: 20.0,
                        color: WHITE.into(),
                        font: default(),
                    },
                ),
                ..default()
            });
        });

        parent.spawn((ButtonBundle {
            style: Style {
                width: Val::Px(100.0),
                height: Val::Px(100.0),
                ..default()
            },
            background_color: BLUE.into(),
            ..default()
        },Name::new("MoveSevenBlackPawn"),

        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Move B P7",
                    TextStyle {
                        font_size: 20.0,
                        color: WHITE.into(),
                        font: default(),
                    },
                ),
                ..default()
            });
        });

        parent.spawn((ButtonBundle {
            style: Style {
                width: Val::Px(100.0),
                height: Val::Px(100.0),
                ..default()
            },
            background_color: BLUE.into(),
            ..default()
        },Name::new("MoveEightBlackPawn"),

        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Move B P8",
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

/*
Here is where we are spawing in our chess board
*/

pub fn chess_board(mut commands: Commands){
    /*
    This is where we are making the sprite of our squares 
    where we are having a color for green squares and then we have 
    a color for our white squares we are giving each square a custom size using the Vec2::splat 
    where our SQUARE_SIZE is of size 64.0
    */
    let green_squares = Sprite{
        color: GREEN.into(),
        custom_size: Some(Vec2::splat(SQUARE_SIZE)),
        ..Default::default()
    };
    let white_squares = Sprite{
        color: WHITE.into(),
        custom_size: Some(Vec2::splat(SQUARE_SIZE)),
        ..Default::default()
    };
    
    /*
    This is where we are going to be spawning in our squares
    */

    // for i in 1 to 9 since we have an 8 by 8 grid
    for i in 1..9 {
        // println!("{} *******", i);
        for j in 1..9 {
            // println!("{}", j);
            // The reason for the double for loop is because we want to be going row by row
            // So the first for loop is our first row then "j" is our column
            // If i for example is row 1 and j is col 1 then if we do 1+1 = 2 and if we mod this we get 0 
            // Meaning this would be our white square if we get a remander from i+j then that means we have a green square
            // This is just so we get odd then even and untill the inner for loop is done and we move on the i = row 2
            let sprite = if (i + j) % 2 == 0 {
                green_squares.clone()
            } else {
                white_squares.clone()
            };
            
            // This is where we are spawning in our squares where we have an x and a y
            // The way we set this up though is that we go from bottom to top
            // x_t means that how far left we are where we start at the -64 * 4 - 64 / 2 
            // This ends up giving us -288 and the bottom is -224
            // After this we want to spawn in our actual square with the approriate color on that specific location
            // When the index increases we do the same thing and our values only shift to the left by the square size so bottom wont change until
            // you reach your next index where we will be higher than before.
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
        Each Piece is its own entity meaning all the pieces are seperate from each other.
        The commands.spawn takes in an asset that we specify with a file path leading it to the specific image we want to use.
        Within this spawn we pass the Pawn struct referencing the Pieces enum while passing in the data needed.
        The data needed for each pice is a color (white or black) and an ID. So the white pices are odd numbers and the black pieces are even numbers.
        Each piece has a png that is scaled to fit within the squares on the board. 
     */

    /*
    White Pawns
    */
    //PAWN 1
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

    
    //PAWN 2
    commands.spawn((SpriteBundle{
        texture: asset_server.load("pieces/individual/pawn-white-16x16.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(100., 100.)),
            ..default()
        },
        transform: Transform::from_xyz(-160., -145., 0.5),
        ..Default::default()
    },
    Pawn, Peices::Pawn("white".to_string(), 3.0)));

    //PAWN 3
    commands.spawn((SpriteBundle{
        texture: asset_server.load("pieces/individual/pawn-white-16x16.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(100., 100.)),
            ..default()
        },
        transform: Transform::from_xyz(-95., -145., 0.5),
        ..Default::default()
    },
    Pawn, Peices::Pawn("white".to_string(), 5.0)));


    //PAWN 4
    commands.spawn((SpriteBundle{
        texture: asset_server.load("pieces/individual/pawn-white-16x16.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(100., 100.)),
            ..default()
        },
        transform: Transform::from_xyz(-30., -145., 0.5),
        ..Default::default()
    },
    Pawn, Peices::Pawn("white".to_string(), 7.0)));

    //PAWN 5
    commands.spawn((SpriteBundle{
        texture: asset_server.load("pieces/individual/pawn-white-16x16.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(100., 100.)),
            ..default()
        },
        transform: Transform::from_xyz(35., -145., 0.5),
        ..Default::default()
    },
    Pawn, Peices::Pawn("white".to_string(), 9.0)));


    //PAWN 6
    commands.spawn((SpriteBundle{
        texture: asset_server.load("pieces/individual/pawn-white-16x16.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(100., 100.)),
            ..default()
        },
        transform: Transform::from_xyz(95., -145., 0.5),
        ..Default::default()
    },
    Pawn, Peices::Pawn("white".to_string(), 11.0)));

    //PAWN 7
    commands.spawn((SpriteBundle{
        texture: asset_server.load("pieces/individual/pawn-white-16x16.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(100., 100.)),
            ..default()
        },
        transform: Transform::from_xyz(160., -145., 0.5),
        ..Default::default()
    },
    Pawn, Peices::Pawn("white".to_string(), 13.0)));

    //PAWN 8
    commands.spawn((SpriteBundle{
        texture: asset_server.load("pieces/individual/pawn-white-16x16.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(100., 100.)),
            ..default()
        },
        transform: Transform::from_xyz(225., -145., 0.5),
        ..Default::default()
    },
    Pawn, Peices::Pawn("white".to_string(), 15.0)));

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
    Pawn, Peices::Pawn("black".to_string(), 4.0)));

    commands.spawn((SpriteBundle{
        texture: asset_server.load("pieces/individual/pawn-black-16x16.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(100., 100.)),
            ..default()
        },
        transform: Transform::from_xyz(-95., 175., 0.5),
        ..Default::default()
    },
    Pawn, Peices::Pawn("black".to_string(), 6.0)));

    commands.spawn((SpriteBundle{
        texture: asset_server.load("pieces/individual/pawn-black-16x16.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(100., 100.)),
            ..default()
        },
        transform: Transform::from_xyz(-30., 175., 0.5),
        ..Default::default()
    },
    Pawn, Peices::Pawn("black".to_string(), 8.0)));

    commands.spawn((SpriteBundle{
        texture: asset_server.load("pieces/individual/pawn-black-16x16.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(100., 100.)),
            ..default()
        },
        transform: Transform::from_xyz(35., 175., 0.5),
        ..Default::default()
    },
    Pawn, Peices::Pawn("black".to_string(), 10.0)));

    commands.spawn((SpriteBundle{
        texture: asset_server.load("pieces/individual/pawn-black-16x16.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(100., 100.)),
            ..default()
        },
        transform: Transform::from_xyz(95., 175., 0.5),
        ..Default::default()
    },
    Pawn, Peices::Pawn("black".to_string(), 12.0)));

    commands.spawn((SpriteBundle{
        texture: asset_server.load("pieces/individual/pawn-black-16x16.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(100., 100.)),
            ..default()
        },
        transform: Transform::from_xyz(160., 175., 0.5),
        ..Default::default()
    },
    Pawn, Peices::Pawn("black".to_string(), 14.0)));

    commands.spawn((SpriteBundle{
        texture: asset_server.load("pieces/individual/pawn-black-16x16.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(100., 100.)),
            ..default()
        },
        transform: Transform::from_xyz(225., 175., 0.5),
        ..Default::default()
    },
    Pawn, Peices::Pawn("black".to_string(), 16.0)));

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
