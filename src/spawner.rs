use crate::prelude::*;

// This takes a mutable reference to "world". Notice that this lets you update a World without borrowing that World itself.
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
