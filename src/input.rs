use legion::{IntoQuery, Read, World, Write};
use macroquad::input::{is_key_down, KeyCode};
use macroquad::math::Vec2;
use crate::{GameState};
use crate::components::{DrawableComponent, PlayerComponent, VelocityComponent};

pub trait InputManaged {
    fn map_input(&mut self) -> Action;
}

pub(crate) trait ControlSet {
    fn execute_action(&mut self, action: Action, world: &mut World) -> Option<GameState>;
}

pub enum Action {
    Confirm,
    Revert,
    RotateShipRight,
    RotateShipLeft,
    ThrustShip,
    NoOp,
}

pub struct InputManager;

impl InputManaged for InputManager {
    fn map_input(&mut self) -> Action {
        let mut key_code = KeyCode::Unknown;
        if is_key_down(KeyCode::Enter) {
            key_code = KeyCode::Enter
        }
        if is_key_down(KeyCode::Escape) {
            key_code = KeyCode::Escape
        }
        if is_key_down(KeyCode::Left) {
            key_code = KeyCode::Left
        }
        if is_key_down(KeyCode::Right) {
            key_code = KeyCode::Right
        }
        if is_key_down(KeyCode::Up) {
            key_code = KeyCode::Up
        }
        match key_code {
            KeyCode::Enter => Action::Confirm,
            KeyCode::Escape => Action::Revert,
            KeyCode::Right => Action::RotateShipRight,
            KeyCode::Left => Action::RotateShipLeft,
            KeyCode::Up => Action::ThrustShip,
            _ => Action::NoOp,
        }
    }
}

pub struct MainMenuControls;

impl ControlSet for MainMenuControls {
    fn execute_action(&mut self, action: Action, world: &mut World) -> Option<GameState> {
        match action {
            Action::Confirm => {
                Some(GameState::GamePlay)
            }
            _ => None
        }
    }
}

pub struct GamePlayControls;

impl ControlSet for GamePlayControls {
    fn execute_action(&mut self, action: Action, world: &mut World) -> Option<GameState>{
        match action {
            Action::Revert => {
                Some(GameState::MainMenu)
            },
            Action::RotateShipRight => {
                let mut query = <(Write<DrawableComponent>, Read<PlayerComponent>)>::query();
                for (drawable, _) in query.iter_mut(world) {
                    drawable.rotation += 0.1;
                }
                None
            },
            Action::RotateShipLeft => {
                let mut query = <(Write<DrawableComponent>, Read<PlayerComponent>)>::query();
                for (drawable, _) in query.iter_mut(world) {
                    drawable.rotation -= 0.1;
                }
                None
            },
            Action::ThrustShip => {
                let mut query = <(Write<VelocityComponent>, Read<DrawableComponent>, Read<PlayerComponent>)>::query();
                for (velocity, drawable, _) in query.iter_mut(world) {
                    let acceleration = Vec2::from_angle(drawable.rotation) * 0.1;
                    velocity.velocity += acceleration;
                }
                None
            }
            _ => None
        }
    }
}