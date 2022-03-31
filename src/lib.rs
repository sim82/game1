use bevy::prelude::*;

pub mod ai;
pub mod brainy;
pub mod crab_move;
pub mod pointer;
pub mod walk;

pub mod sprites {
    use bevy_aseprite::aseprite;
    aseprite!(pub Ferris, "assets/ferris2.0.aseprite");
    aseprite!(pub Pointer, "assets/pointer.aseprite");
    aseprite!(pub Pew, "assets/pew.aseprite");
    
}

#[derive(Component)]
pub struct TargetFlag;
