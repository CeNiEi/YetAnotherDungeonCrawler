mod player_input;
mod map_render;
mod entity_render;

use crate::prelude::*;

pub fn build_scheduler() -> Schedule {
    Schedule::builder()
    .add_system(player_input::player_input_system())
    .add_system(map_render::map_render_system())
    .add_system(entity_render::movable_entity_render_system())
    .add_system(entity_render::immovable_entity_render_3x3_system())
    .build()
}