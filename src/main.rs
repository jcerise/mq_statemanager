mod input;
mod components;
mod systems;

extern crate rand;

use std::collections::HashMap;
use legion::{IntoQuery, Resources, Schedule, World};
use macroquad::prelude::*;
use macroquad::rand::ChooseRandom;
use rand::Rng;
use uuid::Uuid;
use crate::components::{AsteroidComponent, DrawableComponent, PlayerComponent, VelocityComponent};
use crate::input::{ControlSet, GamePlayControls, InputManaged, InputManager, MainMenuControls};
use crate::systems::{apply_velocity_system, destroy_timed_entities_system, rotate_asteroids_system};

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
    pub resources: Resources,
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
    texture: Uuid,
}

pub struct ScreenDimensions {
    width: f32,
    height: f32,
}

pub struct TimeResource {
    absolute_time: f64
}

#[derive(Clone)]
pub struct TextureMap {
    mapping: HashMap<String, Uuid>
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
    let mut texture_map: TextureMap  = TextureMap{mapping: HashMap::new()};
    let mut rng = rand::thread_rng();

    // Load our textures
    let ship_texture: Texture2D = load_texture("resources/ship.png").await.unwrap();
    let bullet_texture: Texture2D = load_texture("resources/bullet.png").await.unwrap();
    let large_asteroid_texture_1: Texture2D = load_texture("resources/asteroid_2.png").await.unwrap();
    let large_asteroid_texture_2: Texture2D = load_texture("resources/asteroid_3.png").await.unwrap();
    let large_asteroid_texture_3: Texture2D = load_texture("resources/asteroid_4.png").await.unwrap();

    let ship_texture_id = Uuid::new_v4();
    let bullet_texture_id = Uuid::new_v4();
    let large_asteroid1_texture_id = Uuid::new_v4();
    let large_asteroid2_texture_id = Uuid::new_v4();
    let large_asteroid3_texture_id = Uuid::new_v4();

    texture_map.mapping.insert("ship".to_string(), ship_texture_id);
    texture_map.mapping.insert("bullet".to_string(), bullet_texture_id);
    texture_map.mapping.insert("large_asteroid_1".to_string(), large_asteroid1_texture_id);
    texture_map.mapping.insert("large_asteroid_2".to_string(), large_asteroid2_texture_id);
    texture_map.mapping.insert("large_asteroid_3".to_string(), large_asteroid3_texture_id);

    texture_assets.insert(ship_texture_id, ship_texture);
    texture_assets.insert(bullet_texture_id, bullet_texture);
    texture_assets.insert(large_asteroid1_texture_id, large_asteroid_texture_1);
    texture_assets.insert(large_asteroid2_texture_id, large_asteroid_texture_2);
    texture_assets.insert(large_asteroid3_texture_id, large_asteroid_texture_3);

    // Add the large asteroid textures to a vector, so we can randomly choose one each time we
    // instantiate a new large asteroid
    let mut large_asteroid_textures: Vec<Uuid> = Vec::new();
    large_asteroid_textures.push(large_asteroid1_texture_id);
    large_asteroid_textures.push(large_asteroid2_texture_id);
    large_asteroid_textures.push(large_asteroid3_texture_id);

    // Create our legion world
    let mut world = World::default();
    let mut resources = Resources::default();
    resources.insert(ScreenDimensions{width: screen_width(), height: screen_height()});
    resources.insert(texture_map.clone());
    resources.insert(TimeResource{absolute_time: get_time()});

    // Load our player entity into the world
    let ship_position = Vec2::new(screen_width() / 2., screen_height() / 2.);
    world.push(
        (
            PlayerComponent{last_bullet_fired: 0.0, fire_rate: 0.2},
            DrawableComponent{texture_id: ship_texture_id, position: ship_position, rotation: 0.0},
            VelocityComponent{velocity: Vec2::new(0.0, 0.0)}
        )
    );

    // Add eight large asteroids, and set them moving in random directions, at random velocity
    for _ in 0..8 {
        let rotation = rng.gen_range(-10.0..=10.0);
        let pos = Vec2::new(rng.gen_range(0.0..=screen_width()), rng.gen_range(0.0..=screen_height()));
        let tex_uuid = large_asteroid_textures.choose().unwrap();

        world.push(
            (AsteroidComponent, DrawableComponent{texture_id: *tex_uuid, position: pos, rotation: rotation}, VelocityComponent{velocity: Vec2::from_angle(rotation) * rng.gen_range(0.1..=1.0)})
        );
    }

    // Init our game manager to the main menu state, the previous state will also reflect this
    let mut game_manager = GameStateManager{
        current_state: GameState::MainMenu,
        previous_state: GameState::MainMenu,
        active_controls: GameState::MainMenu.value(),
        world,
        resources,
    };

    let mut input_manager = InputManager{};

    let mut schedule = Schedule::builder()
        .add_system(apply_velocity_system())
        .add_system(rotate_asteroids_system())
        .add_system(destroy_timed_entities_system())
        .build();

    loop {
        clear_background(BLACK);

        game_manager.resources.remove::<TimeResource>();
        game_manager.resources.insert(TimeResource{absolute_time:get_time()});

        // Grab any input that is present for this frame, and map it to a valid action, if any
        let current_actions = input_manager.map_input();

        // Handle the action for the current controlset. If an invalid action is provided, ignore
        if let Some(new_state) = game_manager.active_controls.execute_action(current_actions, &mut game_manager.world, &texture_map) {
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
                // Execute all systems
                schedule.execute(&mut game_manager.world, &mut game_manager.resources);

                let mut render_data = Vec::new();
                let mut query = <&DrawableComponent>::query();
                for drawable in query.iter(&game_manager.world) {
                    render_data.push(RenderData {
                        position: drawable.position.clone(),
                        rotation: drawable.rotation.clone(),
                        texture: drawable.texture_id.clone()
                    });
                }

                for data in render_data {
                    let draw_params = DrawTextureParams{
                        rotation: data.rotation,
                        ..Default::default()
                    };

                    draw_texture_ex(&texture_assets.get(&data.texture).unwrap(), data.position.x, data.position.y, WHITE, draw_params);
                }
            }
            GameState::Pause => {}
            GameState::GameOver => {}
        }

        next_frame().await;
    }
}
