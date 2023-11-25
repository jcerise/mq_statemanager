use legion::{IntoQuery, Read, World, Write};
use macroquad::input::{is_key_down, KeyCode};
use macroquad::math::Vec2;
use crate::{GameState};
use crate::components::{DrawableComponent, PlayerComponent, VelocityComponent};

pub trait InputManaged {
    fn map_input(&mut self) -> Vec<Action>;
}

pub(crate) trait ControlSet {
    fn execute_action(&mut self, actions: Vec<Action>, world: &mut World) -> Option<GameState>;
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

        let mut key_code = KeyCode::Unknown;
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

        let mut actions: Vec<Action> = Vec::new();
        for key_code in keys.iter(){
            match key_code {
                KeyCode::Enter => actions.push(Action::Confirm),
                KeyCode::Escape => actions.push(Action::Revert),
                KeyCode::Right => actions.push(Action::RotateShipRight),
                KeyCode::Left => actions.push(Action::RotateShipLeft),
                KeyCode::Up => actions.push(Action::ThrustShip),
                _ => actions.push(Action::NoOp),
            }
        }
        actions
    }
}

pub struct MainMenuControls;

impl ControlSet for MainMenuControls {
    fn execute_action(&mut self, actions: Vec<Action>, world: &mut World) -> Option<GameState> {
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

pub struct GamePlayControls;

impl ControlSet for GamePlayControls {
    fn execute_action(&mut self, actions: Vec<Action>, world: &mut World) -> Option<GameState>{
        for action in actions.iter(){
            match action {
                Action::Revert => {
                    return Some(GameState::MainMenu)
                },
                Action::RotateShipRight => {
                    let mut query = <(Write<DrawableComponent>, Read<PlayerComponent>)>::query();
                    for (drawable, _) in query.iter_mut(world) {
                        drawable.rotation += 0.1;
                    }
                     return None
                },
                Action::RotateShipLeft => {
                    let mut query = <(Write<DrawableComponent>, Read<PlayerComponent>)>::query();
                    for (drawable, _) in query.iter_mut(world) {
                        drawable.rotation -= 0.1;
                    }
                    return None
                },
                Action::ThrustShip => {
                    let mut query = <(Write<VelocityComponent>, Read<DrawableComponent>, Read<PlayerComponent>)>::query();
                    for (velocity, drawable, _) in query.iter_mut(world) {
                        let acceleration = Vec2::from_angle(drawable.rotation) * 0.1;
                        velocity.velocity += acceleration;
                    }
                    return None
                }
                _ =>  return None
            }
        }
        None
    }
}