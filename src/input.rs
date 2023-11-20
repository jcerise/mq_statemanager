use macroquad::input::KeyCode;
use crate::{GameState, GameStateManager, StateManaged};

pub trait InputManaged {
    fn map_input(&mut self, key_code: Option<KeyCode>) -> Action;
}

pub(crate) trait ControlSet {
    fn execute_action(&mut self, action: Action) -> Option<GameState>;
}

pub enum Action {
    Confirm,
    Revert,
    NoOp,
}

pub struct InputManager;

impl InputManaged for InputManager {
    fn map_input(&mut self, key_code: Option<KeyCode>) -> Action {
        match key_code {
            Some(KeyCode::Enter) => Action::Confirm,
            Some(KeyCode::Escape) => Action::Revert,
            None => Action::NoOp,
            _ => Action::NoOp,
        }
    }
}

pub struct MainMenuControls;

impl ControlSet for MainMenuControls {
    fn execute_action(&mut self, action: Action) -> Option<GameState> {
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
    fn execute_action(&mut self, action: Action) -> Option<GameState>{
        match action {
            Action::Revert => {
                Some(GameState::MainMenu)
            }
            _ => None
        }
    }
}