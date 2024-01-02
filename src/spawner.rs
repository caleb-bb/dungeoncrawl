use crate::prelude::*;

// This takes a mutable reference to "world". Notice that this lets you update a
// World without borrowing that World itself.
// This lets us mutate stuff in place without ownership changing hands
// However, recall that mutable references lock the original data to any other access
// in the scope where the mutable reference occurs
pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        Player,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
    ));
}

// ecs.push is pushing a vector of components (that is, an entity) to the ecs
// This one has the following components:
// Enemy (it is an enemy)
// Point (it has a location, which is "pos" here)
pub fn spawn_monster(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    ecs.push((
        Enemy,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: match rng.range(0, 4) {
                0 => to_cp437('E'),
                1 => to_cp437('O'),
                2 => to_cp437('o'),
                _ => to_cp437('g'),
            },
        },
        MovingRandomly {},
    ));
}
