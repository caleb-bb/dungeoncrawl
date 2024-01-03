use crate::prelude::*;

mod collisions;
mod end_turn;
mod entity_render;
mod map_render;
mod movement;
mod player_input;
mod random_move;

// The call to flush() here does a few different things.
// First, it makes sure all changes up to this point are applied.
// Second, because those changes are applied, any deleted entities are gone before they
// are rendered. This is because the render system is applied AFTER the flush.
// Third, this guarantees that all systems up to this point have finished executing
// before the next one is run.
pub fn build_input_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .build()
}

pub fn build_player_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(movement::movement_system())
        .flush()
        .add_system(collisions::collisions_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(end_turn::end_turn_system())
        .build()
}

pub fn build_monster_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(random_move::random_move_system())
        .flush()
        .add_system(movement::movement_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(end_turn::end_turn_system())
        .build()
}
