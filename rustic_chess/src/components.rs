use bevy::prelude::*;
/*
Here is where we are creating our stucts for our entitys so for example the 
Square is used when we are spawning in our squares for our chess board which has the type Square
And the pieces are the same
*/
#[derive(Component, Debug)]
pub struct Square;

#[derive(Component, Debug)]
pub struct Pawn;

/*
The only thing different is that we have this enum for our Pieces with two types for our 
pawn where we have a String, and a Float
The reason here is so that when we spawn in a Peice for example a pawn we can then set the 
type of Peices::Pawn to that specific Pawn entity and put that this is either a 
white pawn or a black pawn then a float where each pawn has a unique number for each entity
*/
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



