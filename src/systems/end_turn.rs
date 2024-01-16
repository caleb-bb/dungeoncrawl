use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Point)]

pub fn end_turn(ecs: &SubWorld, #[resource] turn_state: &mut TurnState, #[resource] map: &Map) {
    // grab anything with a Health component and filter out the ones that also
    // have a Player component. There is, of course, only one of these.
    let mut player_hp = <(&Health, &Point)>::query().filter(component::<Player>());
    let mut amulet = <&Point>::query().filter(component::<AmuletOfYala>());
    let current_state = turn_state.clone();
    let amulet_default = Point::new(-1, -1);
    let amulet_pos = amulet.iter(ecs).nth(0).unwrap_or(&amulet_default);
    // on a None option, unwrap_or will return whatever we pass to it as an arg
    // note that unwrap_or takes a borrowed reference to a value as a param, so
    // we can't just direction pass in Point::new(a, b)
    let mut new_state = match turn_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        _ => current_state,
    };

    player_hp.iter(ecs).for_each(|(hp, pos)| {
        if hp.current < 1 {
            new_state = TurnState::GameOver;
        }
        if pos == amulet_pos {
            new_state = TurnState::Victory;
        }
        let idx = map.point2d_to_index(*pos);
        if map.tiles[idx] == TileType::Exit {
            new_state = TurnState::NextLevel;
        }
    });
    *turn_state = new_state;
}
