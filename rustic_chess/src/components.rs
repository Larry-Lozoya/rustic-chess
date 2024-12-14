use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Square;

#[derive(Component, Debug)]
pub struct Pawn;

#[derive(Component, Debug)]
pub enum Peices {
    Pawn(String, f32)
}


#[derive(Component, Debug)]
pub struct Knight;


#[derive(Component, Debug)]
pub struct Bishop;


#[derive(Component, Debug)]
pub struct Rook;


#[derive(Component, Debug)]
pub struct King;


#[derive(Component, Debug)]
pub struct Queen;



