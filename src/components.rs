use macroquad::math::Vec2;
use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DrawableComponent {
    pub texture_id: Uuid,
    pub position: Vec2,
    pub rotation: f32
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VelocityComponent {
    pub velocity: Vec2
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlayerComponent;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AsteroidComponent;