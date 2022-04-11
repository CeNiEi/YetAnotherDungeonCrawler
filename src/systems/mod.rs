mod combat;
mod end_turn;
mod entity_render;
mod fov;
mod homing;
mod hud;
mod map_render;
mod mode_change;
mod movement;
mod player_input;
mod tooltips;

use crate::prelude::*;

pub fn build_input_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .add_system(fov::fov_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::movable_entity_render_system())
        .add_system(entity_render::immovable_entity_render_3x3_system())
        .add_system(entity_render::single_missile_entity_render_system())
        .add_system(entity_render::splash_missile_entity_render_system())
        .add_system(entity_render::item_render_system())
        .flush()
        .add_system(hud::hud_system())
        .add_system(tooltips::tooltips_system())
        .build()
}

pub fn build_player_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(movement::movement_system())
        .flush()
        .add_system(combat::combat_system())
        .flush()
        .add_system(fov::fov_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::movable_entity_render_system())
        .add_system(entity_render::immovable_entity_render_3x3_system())
        .add_system(entity_render::single_missile_entity_render_system())
        .add_system(entity_render::splash_missile_entity_render_system())
        .add_system(entity_render::item_render_system())
        .flush()
        .add_system(hud::hud_system())
        .add_system(tooltips::tooltips_system())
        .flush()
        .add_system(end_turn::end_turn_system())
        .build()
}

pub fn build_monster_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(movement::movement_system())
        .flush()
        .add_system(combat::combat_system())
        .add_system(combat::auto_reduce_health_system())
        .flush()
        .add_system(homing::homing_system())
        .flush()
        .add_system(fov::fov_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::movable_entity_render_system())
        .add_system(entity_render::immovable_entity_render_3x3_system())
        .add_system(entity_render::single_missile_entity_render_system())
        .add_system(entity_render::splash_missile_entity_render_system())
        .add_system(entity_render::item_render_system())
        .flush()
        .add_system(mode_change::change_ranged_sprite_mode_system())
        .add_system(mode_change::change_movable_sprite_mode_system())
        .add_system(hud::hud_system())
        .add_system(tooltips::tooltips_system())
        .flush()
        .add_system(end_turn::end_turn_system())
        .build()
}
