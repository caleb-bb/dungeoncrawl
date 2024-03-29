use crate::prelude::*;
mod template;
use template::Templates;

// This takes a mutable reference to "world". Notice that this lets you update a
// World without borrowing that World itself.
// This lets us mutate stuff in place without ownership changing hands
// However, recall that mutable references lock the original data to any other access
// in the scope where the mutable reference occurs
pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        Player { map_level: 0 },
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
        Health {
            current: 20,
            max: 20,
        },
        FieldOfView::new(8),
        Damage(1),
    ));
}

// ecs.push is pushing a vector of components (that is, an entity) to the ecs
// This one has the following components:
// Enemy (it is an enemy)
// Point (it has a location, which is "pos" here)
pub fn spawn_amulet_of_yala(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        AmuletOfYala,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('|'),
        },
        Name("Amulet of Yala".to_string()),
    ));
}

pub fn spawn_level(
    ecs: &mut World,
    rng: &mut RandomNumberGenerator,
    level: usize,
    spawn_points: &[Point],
) {
    let template = Templates::load();
    template.spawn_entities(ecs, rng, level, spawn_points);
}
