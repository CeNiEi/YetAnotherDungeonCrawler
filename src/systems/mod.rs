mod player_input;
mod map_render;
mod entity_render;
mod ranged;
mod end_turn;
mod movement;

use crate::prelude::*;

pub fn build_input_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::movable_entity_render_system())
        .add_system(entity_render::immovable_entity_render_3x3_system())
        .add_system(entity_render::homing_missile_entity_render_system())
        .build()
}

pub fn build_player_scheduler() -> Schedule {
    Schedule::builder() 
        .add_system(movement::movement_system())
        .add_system(ranged::ranged_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::movable_entity_render_system())
        .add_system(entity_render::immovable_entity_render_3x3_system())
        .add_system(entity_render::homing_missile_entity_render_system())
        .add_system(end_turn::end_turn_system())
        .build()
}

pub fn build_monster_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(movement::movement_system())
        .add_system(ranged::ranged_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::movable_entity_render_system())
        .add_system(entity_render::immovable_entity_render_3x3_system())
        .add_system(entity_render::homing_missile_entity_render_system())
        .add_system(end_turn::end_turn_system())
        .build()
}