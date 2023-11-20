mod input;

use macroquad::prelude::*;
use crate::input::{ControlSet, GamePlayControls, InputManaged, InputManager, MainMenuControls};

#[derive(Clone)]
enum GameState {
    MainMenu,
    GamePlay,
    Pause,
    GameOver,
}

impl GameState {
    fn value(&self) -> Box<dyn ControlSet> {
        match *self {
            GameState::MainMenu => Box::new(MainMenuControls),
            GameState::GamePlay => Box::new(GamePlayControls),
            GameState::GameOver => Box::new(MainMenuControls),
            GameState::Pause => Box::new(MainMenuControls)

        }
    }
}

trait StateManaged {
    fn update_state(&mut self, new_state: GameState);
    fn revert_state(&mut self);
}

struct GameStateManager {
    current_state: GameState,
    previous_state: GameState,
    active_controls: Box<dyn ControlSet>,
}

impl StateManaged for GameStateManager {
    fn update_state(&mut self, new_state: GameState){
        self.previous_state = self.current_state.clone();
        self.current_state = new_state;
        self.active_controls = self.current_state.value()
    }

    fn revert_state(&mut self){
        self.current_state = self.previous_state.clone()
    }
}

fn conf() -> Conf {
    Conf {
        window_title: "MQ GameState".to_string(),
        window_width: 640,
        window_height: 480,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {

    // Init our game manager to the main menu state, the previous state will also reflect this
    let mut game_manager = GameStateManager{
        current_state: GameState::MainMenu,
        previous_state: GameState::MainMenu,
        active_controls: GameState::MainMenu.value(),
    };

    let mut input_manager = InputManager{};

    loop {
        clear_background(BLACK);

        // Grab any input that is present for this frame, and map it to a valid action, if any
        let key_press = get_last_key_pressed();
        let current_action = input_manager.map_input(key_press);

        // Handle the action for the current controlset. If an invalid action is provided, ignore
        if let Some(new_state) = game_manager.active_controls.execute_action(current_action) {
            game_manager.update_state(new_state);
        }

        match game_manager.current_state {
            GameState::MainMenu => {
                let title = "MQ State Management";
                let instructions = "Press <ENTER> to start";
                draw_text_ex(
                    title,
                    screen_width() / 2. - measure_text(title, None, 50, 1.0).width / 2.0,
                    screen_height() / 2.,
                    TextParams{
                        font_size: 50,
                        color: WHITE,
                        ..Default::default()
                    });
                draw_text_ex(
                    instructions,
                    screen_width() / 2. - measure_text(instructions, None, 30, 1.0).width / 2.0,
                    screen_height() / 2. + 50.,
                    TextParams{
                        font_size: 30,
                        color: WHITE,
                        ..Default::default()
                    });
            }
            GameState::GamePlay => {
                let instructions = "Press <ESCAPE> to go back";
                draw_text_ex(
                    instructions,
                    screen_width() / 2. - measure_text(instructions, None, 50, 1.0).width / 2.0,
                    screen_height() / 2.,
                    TextParams{
                        font_size: 50,
                        color: WHITE,
                        ..Default::default()
                    });
            }
            GameState::Pause => {}
            GameState::GameOver => {}
        }

        next_frame().await;
    }
}
