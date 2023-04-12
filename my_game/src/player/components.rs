
use bevy::prelude::*;



#[derive(Component,Clone)]
pub struct Player {
    pub name: String,
    //x y z position
    pub x : f32,
    pub y : f32,
    pub z : f32,
}