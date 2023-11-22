use legion::{IntoQuery, system};
use crate::components::{AsteroidComponent, DrawableComponent, PlayerComponent, VelocityComponent};
use crate::ScreenDimensions;

#[system(for_each)]
pub fn apply_velocity(velocity: &mut VelocityComponent, drawable: &mut DrawableComponent, player: Option<&PlayerComponent>, #[resource] screen_dimensions: &ScreenDimensions) {
    drawable.position += velocity.velocity;

    // Wrap the screen
    if drawable.position.x > screen_dimensions.width {
        drawable.position.x = 0.0;
    }

    if drawable.position.x < 0.0 {
        drawable.position.x = screen_dimensions.width
    }

    if drawable.position.y > screen_dimensions.height {
        drawable.position.y = 0.0;
    }

    if drawable.position.y < 0.0 {
        drawable.position.y = screen_dimensions.height
    }

    if let Some(player) = player {
        // If this entity is the player, apply a "braking force" on its velocity, so it slows down
        // over time, and cannot accelerate infinitely
        velocity.velocity *= 0.98;
    }
}

#[system(for_each)]
pub fn rotate_asteroids(drawable: &mut DrawableComponent, asteroid: &AsteroidComponent) {
    if drawable.rotation >= 0.0 {
        drawable.rotation += 0.01
    } else {
        drawable.rotation += -0.01
    }
}

