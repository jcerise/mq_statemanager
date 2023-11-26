use legion::{Entity, IntoQuery, system};
use legion::systems::CommandBuffer;
use crate::components::{AsteroidComponent, BulletComponent, DrawableComponent, PlayerComponent, TimedExistenceComponent, VelocityComponent};
use crate::{ScreenDimensions, TimeResource};

#[system(for_each)]
pub fn apply_velocity(velocity: &mut VelocityComponent, drawable: &mut DrawableComponent, player: Option<&PlayerComponent>, bullet: Option<&BulletComponent>, #[resource] screen_dimensions: &ScreenDimensions) {
    drawable.position += velocity.velocity;

    // Wrap the screen, except when we're working with a bullet, they should not wrap
    if bullet.is_none() {
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
}

#[system(for_each)]
pub fn rotate_asteroids(drawable: &mut DrawableComponent, asteroid: &AsteroidComponent) {
    if drawable.rotation >= 0.0 {
        drawable.rotation += 0.01
    } else {
        drawable.rotation += -0.01
    }
}

#[system(for_each)]
pub fn destroy_timed_entities(timed: &TimedExistenceComponent, entity: &Entity, cmd: &mut CommandBuffer, #[resource] time_resource: &TimeResource) {
    let max_lifetime = timed.created_at + timed.max_lifetime;
    if max_lifetime < time_resource.absolute_time {
        cmd.remove(*entity);
    }
}

