mod input;
mod components;
mod systems;

extern crate rand;

use std::collections::HashMap;
use legion::{IntoQuery, Resources, Schedule, World};
use macroquad::prelude::*;
use macroquad::rand::ChooseRandom;
use rand::Rng;
use rand::rngs::ThreadRng;
use uuid::Uuid;
use crate::components::{AsteroidComponent, CollisionComponent, DrawableComponent, PlayerComponent, ScoreComponent, VelocityComponent};
use crate::input::{ControlSet, GameOverControls, GamePlayControls, InputManaged, InputManager, MainMenuControls};
use crate::systems::{apply_velocity_system, destroy_timed_entities_system, handle_bullet_collisions_system, handle_player_collision_system, rotate_asteroids_system};

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
            GameState::GameOver => Box::new(GameOverControls),
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

pub struct ScoreResource {
    score: i32
}

pub struct GameOverResource {
    game_over: bool
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
    let small_asteroid_texture: Texture2D = load_texture("resources/small_asteroid.png").await.unwrap();

    let ship_texture_id = Uuid::new_v4();
    let bullet_texture_id = Uuid::new_v4();
    let large_asteroid1_texture_id = Uuid::new_v4();
    let large_asteroid2_texture_id = Uuid::new_v4();
    let large_asteroid3_texture_id = Uuid::new_v4();
    let small_asteroid_texture_id = Uuid::new_v4();

    texture_map.mapping.insert("ship".to_string(), ship_texture_id);
    texture_map.mapping.insert("bullet".to_string(), bullet_texture_id);
    texture_map.mapping.insert("large_asteroid_1".to_string(), large_asteroid1_texture_id);
    texture_map.mapping.insert("large_asteroid_2".to_string(), large_asteroid2_texture_id);
    texture_map.mapping.insert("large_asteroid_3".to_string(), large_asteroid3_texture_id);
    texture_map.mapping.insert("small_asteroid".to_string(), small_asteroid_texture_id);

    texture_assets.insert(ship_texture_id, ship_texture);
    texture_assets.insert(bullet_texture_id, bullet_texture);
    texture_assets.insert(large_asteroid1_texture_id, large_asteroid_texture_1);
    texture_assets.insert(large_asteroid2_texture_id, large_asteroid_texture_2);
    texture_assets.insert(large_asteroid3_texture_id, large_asteroid_texture_3);
    texture_assets.insert(small_asteroid_texture_id, small_asteroid_texture);

    // Add the large asteroid textures to a vector, so we can randomly choose one each time we
    // instantiate a new large asteroid
    let mut large_asteroid_textures: Vec<Uuid> = Vec::new();
    large_asteroid_textures.push(large_asteroid1_texture_id);
    large_asteroid_textures.push(large_asteroid2_texture_id);
    large_asteroid_textures.push(large_asteroid3_texture_id);

    // Init our game manager to the main menu state, the previous state will also reflect this
    let (world, resources) = new_game(&mut rng, &texture_map, &large_asteroid_textures);
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
        .add_system(handle_bullet_collisions_system())
        .add_system(handle_player_collision_system())
        .build();

    let mut final_score = 0;

    loop {
        clear_background(BLACK);

        // Update the time resource on each tick
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
                let title = "MQ Asteroids";
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
                // If we got after a previous game ending, reset everything


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

                // Draw the players score to the top of the screen
                if let Some(score_resource) = game_manager.resources.get::<ScoreResource>() {
                    let score_text = format!("{}", score_resource.score.to_string());
                    draw_text_ex(
                        &score_text,
                        screen_width() / 2. - measure_text(&score_text, None, 50, 1.0).width / 2.0,
                        measure_text(&score_text, None, 50, 1.0).height + 5.0,
                        TextParams{
                            font_size: 50,
                            color: WHITE,
                            ..Default::default()
                        });
                }
            }
            GameState::Pause => {}
            GameState::GameOver => {
                let title = "GAME OVER";
                draw_text_ex(
                    title,
                    screen_width() / 2. - measure_text(title, None, 50, 1.0).width / 2.0,
                    screen_height() / 2.,
                    TextParams{
                        font_size: 50,
                        color: WHITE,
                        ..Default::default()
                    });
                let score_text = format!("Your score was: {}", final_score.to_string());
                draw_text_ex(
                    &score_text,
                    screen_width() / 2. - measure_text(&score_text, None, 30, 1.0).width / 2.0,
                    screen_height() / 2. + 50.,
                    TextParams{
                        font_size: 30,
                        color: WHITE,
                        ..Default::default()
                    });
            }
        }

        // Check if the player has lost (ship collided with an asteroid)
        let game_over = {
            if let Some(game_over_resource) = game_manager.resources.get::<GameOverResource>() {
                game_over_resource.game_over
            } else {
                false
            }
        };

        if game_over {
            game_manager.update_state(GameState::GameOver);
             //  Set the final score, and reset everything
            final_score = game_manager.resources.get::<ScoreResource>().unwrap().score;

            let (world, resources) = new_game(&mut rng, &texture_map, &large_asteroid_textures);
            game_manager.world = world;
            game_manager.resources = resources;
        }

        next_frame().await;
    }
}

fn new_game(rng: &mut ThreadRng, texture_map: &TextureMap, large_asteroid_textures: &Vec<Uuid>) -> (World, Resources) {
    // Create our legion world, and any shared resources our systems will need
    let mut world = World::default();

    let mut resources = Resources::default();
    resources.insert(ScreenDimensions{width: screen_width(), height: screen_height()});
    resources.insert(texture_map.clone());
    resources.insert(TimeResource{absolute_time: get_time()});
    resources.insert(ScoreResource{score: 0});
    resources.insert(GameOverResource{game_over: false});

    // Load our player entity into the world
    let ship_position = Vec2::new(screen_width() / 2., screen_height() / 2.);
    let ship_texture_id = texture_map.mapping.get("ship").unwrap();
    world.push(
        (
            PlayerComponent{last_bullet_fired: 0.0, fire_rate: 0.2},
            DrawableComponent{texture_id: *ship_texture_id, position: ship_position, rotation: 0.0},
            VelocityComponent{velocity: Vec2::new(0.0, 0.0)},
            CollisionComponent{rect: Rect::new(ship_position[0], ship_position[1], 16., 16.), collided: false},
        )
    );

    // Add eight large asteroids, and set them moving in random directions, at random velocity
    for _ in 0..12 {
        let rotation = rng.gen_range(-10.0..=10.0);
        let pos = Vec2::new(rng.gen_range(0.0..=screen_width()), rng.gen_range(0.0..=screen_height()));
        let tex_uuid = large_asteroid_textures.choose().unwrap();

        world.push(
            (
                AsteroidComponent{is_large: true},
                DrawableComponent{texture_id: *tex_uuid, position: pos, rotation: rotation},
                VelocityComponent{velocity: Vec2::from_angle(rotation) * rng.gen_range(0.1..=1.0)},
                CollisionComponent{rect: Rect::new(pos[0], pos[1], 16., 16.), collided: false},
                ScoreComponent{value: 5},
            )
        );
    }

    (world, resources)
}
