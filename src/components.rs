use macroquad::math::Vec2;
use macroquad::prelude::Texture2D;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub position: Vec2
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Velocity {
    pub velocity: Vec2
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub struct Texture {
    pub texture_id: i32
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rotation {
    pub  rotation: f32
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;