use crate::prelude::*;

#[system(for_each)]
#[read_component(Player)]
pub fn movement(
    entity: &Entity,
    want_move: &WantsToMove,
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    if map.can_enter_tile(want_move.destination) {
        commands.add_component(want_move.entity, want_move.destination);
        if ecs
            .entry_ref(want_move.entity)
            .unwrap()
            .get_component::<Player>()
            .is_ok()
        {
            camera.on_player_move(want_move.destination);
        }
    }
    commands.remove(*entity);
}

// #[system(for_each)] is some magic we get from Legion.
// It derives a query from the system parameters and runs the systems
// once for each matching entity. So in this case, we have Entity and WantsToMove
// as types for two of our parameters. These are the only parameters that are
// components, so they form the query to return the entities on which this
// system will be executed.
