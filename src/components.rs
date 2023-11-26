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
pub struct TimedExistenceComponent {
    pub created_at: f64,
    pub max_lifetime: f64
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlayerComponent {
    pub fire_rate: f64,
    pub last_bullet_fired: f64
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AsteroidComponent;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BulletComponent;