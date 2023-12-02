use legion::{IntoQuery, Read, World, Write};
use macroquad::input::{is_key_down, KeyCode};
use macroquad::math::{Rect, Vec2};
use macroquad::time::get_time;
use crate::{GameState, TextureMap};
use crate::components::{BulletComponent, CollisionComponent, DrawableComponent, PlayerComponent, TimedExistenceComponent, VelocityComponent};

pub trait InputManaged {
    fn map_input(&mut self) -> Vec<Action>;
}

pub(crate) trait ControlSet {
    fn execute_action(&mut self, actions: Vec<Action>, world: &mut World, texture_map: &TextureMap) -> Option<GameState>;
}

pub enum Action {
    Confirm,
    Revert,
    RotateShipRight,
    RotateShipLeft,
    ThrustShip,
    FireBullet,
    NoOp,
}

pub struct InputManager;

impl InputManaged for InputManager {
    fn map_input(&mut self) -> Vec<Action> {
        // Keep track of multiple actions, as multiple keys can be pressed at the same time
        let mut keys: Vec<KeyCode> = Vec::new();

        if is_key_down(KeyCode::Enter) {
            keys.push(KeyCode::Enter)
        }
        if is_key_down(KeyCode::Escape) {
            keys.push(KeyCode::Escape)
        }
        if is_key_down(KeyCode::Left) {
            keys.push(KeyCode::Left)
        }
        if is_key_down(KeyCode::Right) {
            keys.push(KeyCode::Right)
        }
        if is_key_down(KeyCode::Up) {
            keys.push(KeyCode::Up)
        }
        if is_key_down(KeyCode::Space) {
            keys.push(KeyCode::Space)
        }

        let mut actions: Vec<Action> = Vec::new();
        for key_code in keys.iter(){
            match key_code {
                KeyCode::Enter => actions.push(Action::Confirm),
                KeyCode::Escape => actions.push(Action::Revert),
                KeyCode::Right => actions.push(Action::RotateShipRight),
                KeyCode::Left => actions.push(Action::RotateShipLeft),
                KeyCode::Up => actions.push(Action::ThrustShip),
                KeyCode::Space => actions.push(Action::FireBullet),
                _ => actions.push(Action::NoOp),
            }
        }
        actions
    }
}

pub struct MainMenuControls;

impl ControlSet for MainMenuControls {
    fn execute_action(&mut self, actions: Vec<Action>, world: &mut World, texture_map: &TextureMap) -> Option<GameState> {
        for action in actions.iter() {
            match action {
                Action::Confirm => {
                    return Some(GameState::GamePlay)
                }
                _ =>  return None
            }
        }
        None
    }
}

pub struct GameOverControls;

impl ControlSet for GameOverControls {
    fn execute_action(&mut self, actions: Vec<Action>, world: &mut World, texture_map: &TextureMap) -> Option<GameState> {
        for action in actions.iter() {
            match action {
                Action::Revert => {
                    return Some(GameState::MainMenu)
                }
                _ =>  return None
            }
        }
        None
    }
}

pub struct GamePlayControls;

impl ControlSet for GamePlayControls {
    fn execute_action(&mut self, actions: Vec<Action>, world: &mut World, texture_map: &TextureMap) -> Option<GameState>{
        let mut return_state: Option<GameState> = None;
        for action in actions.iter(){
            match action {
                Action::Revert => {
                    return_state = Some(GameState::MainMenu);
                },
                Action::RotateShipRight => {
                    let mut query = <(Write<DrawableComponent>, Read<PlayerComponent>)>::query();
                    for (drawable, _) in query.iter_mut(world) {
                        drawable.rotation += 0.1;
                    }
                },
                Action::RotateShipLeft => {
                    let mut query = <(Write<DrawableComponent>, Read<PlayerComponent>)>::query();
                    for (drawable, _) in query.iter_mut(world) {
                        drawable.rotation -= 0.1;
                    }
                },
                Action::ThrustShip => {
                    let mut query = <(Write<VelocityComponent>, Read<DrawableComponent>, Read<PlayerComponent>)>::query();
                    for (velocity, drawable, _) in query.iter_mut(world) {
                        let acceleration = Vec2::from_angle(drawable.rotation) * 0.1;
                        velocity.velocity += acceleration;
                    }
                },
                Action::FireBullet => {
                    let frame_t = get_time();
                    let mut query = <(Read<DrawableComponent>, Read<PlayerComponent>)>::query();
                    let mut pending_entities = Vec::new();
                    for (drawable, player) in query.iter_mut(world) {
                        if frame_t - player.last_bullet_fired  > player.fire_rate {
                            if let Some(bullet_texture_id) = texture_map.mapping.get("bullet") {
                                pending_entities.push(
                                    (
                                        DrawableComponent{texture_id: *bullet_texture_id, position: drawable.position, rotation: 0.0},
                                        VelocityComponent{velocity: Vec2::from_angle(drawable.rotation) * 15.},
                                        TimedExistenceComponent{created_at: frame_t, max_lifetime: 1.0},
                                        BulletComponent{},
                                        CollisionComponent{rect: Rect::new(drawable.position[0], drawable.position[1], 16., 16.), collided: false}
                                    )
                                );
                            }
                        }
                    }
                    let mut bullet_fired = false;
                    for pending_entity in pending_entities.iter() {
                        bullet_fired = true;
                        // I don't like this...but if we try and create the entity on the world in the loop above, the borrow checker rightfully complains
                        // TODO: Find a better way to do this
                        world.push((pending_entity.0.clone(), pending_entity.1.clone(), pending_entity.2.clone(), pending_entity.3.clone(), pending_entity.4.clone()));
                    }
                    if bullet_fired {
                        let mut player_query = <Write<PlayerComponent>>::query();
                        for player in player_query.iter_mut(world) {
                            player.last_bullet_fired = frame_t;
                        }
                    }

                }
                _ =>  return_state = None
            }
        }
        return_state
    }
}