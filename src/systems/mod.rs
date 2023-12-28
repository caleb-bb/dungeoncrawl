//This adds the player_input module without making it visible to the rest of the crate.
mod player_input;

use crate::prelude::*;

// player_input_system/0 is magicked into existence by the #[system] procedural macro in player_input.rs
pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .build()
}
