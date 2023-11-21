// use legion::{IntoQuery, system};
// use macroquad::color::WHITE;
// use macroquad::prelude::{draw_texture_ex, DrawTextureParams};
// use crate::components::{Position, Rotation, Texture};
//
// #[system(for_each)]
// pub fn draw_entities(texture: &Texture, rotation: &Rotation, position: &Position) {
//     let draw_params = DrawTextureParams{
//         rotation: rotation.rotation,
//         ..Default::default()
//     };
//     draw_texture_ex(&texture.texture, position.position.x, position.position.y, WHITE, draw_params);
// }