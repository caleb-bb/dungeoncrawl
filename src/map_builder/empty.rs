use super::MapArchitect;
use crate::prelude::*;

pub struct EmptyArchitect {}

impl MapArchitect for EmptyArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
        };
        mb.fill(TileType::Floor);
        mb.player_start = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        mb.amulet_start = mb.find_most_distant();
        // the blank space in the 0..x for loop indicates that we just want to
        // do something x times. We don't need a variable.
        for _ in 0..50 {
            mb.monster_spawns.push(Point::new(
                rng.range(1, SCREEN_WIDTH),
                rng.range(1, SCREEN_WIDTH),
            ))
        }
        mb
    }
}

// reference implementation for map builder
// a reference implementation is a boilerplate implementation of a given trait
// that validates that that trait works
// First, we define a struct. Then we implement the trait in question (in this
// case, MapArchitect) for the reference implementation.
