use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]

pub fn end_turn(ecs: &SubWorld, #[resource] turn_state: &mut TurnState) {
    // grab anything with a Health component and filter out the ones that also
    // have a Player component. There is, of course, only one of these.
    let mut player_hp = <&Health>::query().filter(component::<Player>());
    let current_state = turn_state.clone();
    let mut new_state = match turn_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        _ => current_state,
    };

    player_hp.iter(ecs).for_each(|hp| {
        if hp.current < 1 {
            new_state = TurnState::GameOver;
        }
    });
    *turn_state = new_state;
}
