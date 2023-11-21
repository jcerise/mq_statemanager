use std::borrow::Borrow;
mod input;
mod components;
mod systems;

use std::collections::HashMap;
use legion::{IntoQuery, Read, Schedule, World};
use macroquad::prelude::*;
use crate::components::{Player, Position, Rotation, Texture, Velocity};
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

struct GameStateManager {
    current_state: GameState,
    previous_state: GameState,
    active_controls: Box<dyn ControlSet>,
    pub world: World,
}

impl GameStateManager {
    fn update_state(&mut self, new_state: GameState){
        self.previous_state = self.current_state.clone();
        self.current_state = new_state;
        self.active_controls = self.current_state.value()
    }

    fn revert_state(&mut self){
        self.current_state = self.previous_state.clone()
    }
}

struct RenderData {
    position: Vec2,
    rotation: f32,
    texture: i32,
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

    let mut texture_assets = HashMap::new();

    // Load our player texture
    let ship_texture: Texture2D = load_texture("resources/ship.png").await.unwrap();

    texture_assets.insert(1, ship_texture);

    // Create our legion world
    let mut world = World::default();

    // Load our player entity into the world
    let ship_position = Vec2::new(screen_width() / 2., screen_height() / 2.);
    world.push(
        (Player, Texture{texture_id: 1}, Position{position: ship_position}, Velocity{velocity: Vec2::new(0.0, 0.0)}, Rotation{rotation: 0.0})
    );

    // Init our game manager to the main menu state, the previous state will also reflect this
    let mut game_manager = GameStateManager{
        current_state: GameState::MainMenu,
        previous_state: GameState::MainMenu,
        active_controls: GameState::MainMenu.value(),
        world,
    };

    let mut input_manager = InputManager{};

    loop {
        clear_background(BLACK);

        // Grab any input that is present for this frame, and map it to a valid action, if any
        let current_action = input_manager.map_input();

        // Handle the action for the current controlset. If an invalid action is provided, ignore
        if let Some(new_state) = game_manager.active_controls.execute_action(current_action, &mut game_manager.world) {
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
                let mut render_data = Vec::new();
                let mut query = <(&Position, &Rotation, &Texture)>::query();
                for (position, rotation, texture) in query.iter(&game_manager.world) {
                    render_data.push(RenderData {
                        position: position.position.clone(),
                        rotation: rotation.rotation.clone(),
                        texture: texture.texture_id.clone()
                    });
                }

                for data in render_data {
                    let draw_params = DrawTextureParams{
                        rotation: data.rotation,
                        ..Default::default()
                    };

                    draw_texture_ex(&texture_assets.get(1_i32.borrow()).unwrap(), data.position.x, data.position.y, WHITE, draw_params);
                }
            }
            GameState::Pause => {}
            GameState::GameOver => {}
        }

        next_frame().await;
    }
}
