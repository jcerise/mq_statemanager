use legion::{Entity, IntoQuery, Query, system};
use legion::systems::CommandBuffer;
use legion::world::SubWorld;
use macroquad::math::{Rect, Vec2};
use rand::Rng;
use crate::components::{AsteroidComponent, BulletComponent, CollisionComponent, DrawableComponent, PlayerComponent, ScoreComponent, TimedExistenceComponent, VelocityComponent};
use crate::{ScoreResource, ScreenDimensions, TextureMap, TimeResource};

#[system(for_each)]
pub fn apply_velocity(velocity: &mut VelocityComponent,
                      drawable: &mut DrawableComponent,
                      collide: Option<&mut CollisionComponent>,
                      player: Option<&PlayerComponent>,
                      bullet: Option<&BulletComponent>,
                      #[resource] screen_dimensions: &ScreenDimensions) {
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

        if let Some(_) = player {
            // If this entity is the player, apply a "braking force" on its velocity, so it slows down
            // over time, and cannot accelerate infinitely
            velocity.velocity *= 0.98;
        }
    }

    // If this entity has a collision component, update the rectangle that surrounds the entity to match the current
    // position of the entity - We know it has a position, because it has a drawable component
    if let Some(collision) = collide {
        collision.rect.move_to(drawable.position);
    }
}

#[system]
#[write_component(DrawableComponent)]
#[read_component(AsteroidComponent)]
pub fn rotate_asteroids(world: &mut SubWorld) {
    let mut query = <(&AsteroidComponent, &mut DrawableComponent)>::query();
    for (_, drawable) in query.iter_mut(world) {
        if drawable.rotation >= 0.0 {
            drawable.rotation += 0.01
        } else {
            drawable.rotation += -0.01
        }
    }
}

#[system]
pub fn destroy_timed_entities(objects: &mut Query<(Entity, &TimedExistenceComponent)>, cmd: &mut CommandBuffer, world: &mut SubWorld, #[resource] time_resource: &TimeResource) {
    for (entity, timed) in objects.iter(world) {
        let max_lifetime = timed.created_at + timed.max_lifetime;
        if max_lifetime < time_resource.absolute_time {
            cmd.remove(*entity);
        }
    }
}

#[system]
#[read_component(BulletComponent)]
#[read_component(AsteroidComponent)]
#[read_component(CollisionComponent)]
#[read_component(DrawableComponent)]
#[read_component(ScoreComponent)]
pub fn handle_bullet_collisions(cmd: &mut CommandBuffer, world: &mut SubWorld, #[resource] texture_map: &TextureMap, #[resource] score_resource: &mut ScoreResource) {
    // Iterate through every bullet, and then every asteroid, to see if there are any collisions
    // This is inefficient, but for such a small game, is just fine
    let (mut bullet_world, mut asteroid_world) = world.split::<(&BulletComponent, &CollisionComponent)>();
    let mut bullet_query = <(Entity, &BulletComponent, &CollisionComponent)>::query();
    for (bullet_entity, _, bullet_collision) in bullet_query.iter_mut(&mut bullet_world) {
        let mut asteroid_query = <(Entity, &CollisionComponent, &AsteroidComponent, &DrawableComponent, &ScoreComponent)>::query();
        for (asteroid_entity, asteroid_collision, asteroid, asteroid_drawable, score) in asteroid_query.iter_mut(&mut asteroid_world) {
            if bullet_collision.rect.overlaps(&asteroid_collision.rect) {
                // If this entity overlaps with the current entity, add both for removal
                cmd.remove(*bullet_entity);
                cmd.remove(*asteroid_entity);

                // Update the score based on the size of the asteroid
                score_resource.score += score.value;

                if asteroid.is_large {
                    // If this was a large asteroid, spawn a random number of smaller asteroids
                    let mut rng = rand::thread_rng();
                    for _ in 2..rng.gen_range(3..=10) {
                        let rotation = rng.gen_range(-10.0..=10.0);
                        let pos = asteroid_drawable.position;
                        if let Some(tex_uuid) = texture_map.mapping.get("small_asteroid") {
                            cmd.push(
                                (
                                    AsteroidComponent{is_large: false},
                                    DrawableComponent{texture_id: *tex_uuid, position: pos, rotation: rotation},
                                    VelocityComponent{velocity: Vec2::from_angle(rotation) * rng.gen_range(0.1..=1.0)},
                                    CollisionComponent{rect: Rect::new(pos[0], pos[1], 16., 16.), collided: false},
                                    ScoreComponent{value: 10},
                                )
                            );
                        }
                    }
                }
            }
        }
    }
}


