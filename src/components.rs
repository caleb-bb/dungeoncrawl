pub use crate::prelude::*;

// Note how the components file is relatively lightweight. A component is just
// a struct with some fields and some derived traits.
// Because components are lightweight, we can fit them all in one file.

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,    // (1)
    pub glyph: FontCharType, // (2)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player; // (3)

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MovingRandomly;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Point,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

// In addition to being maps, structs can also be tuples
#[derive(Clone, PartialEq)]
pub struct Name(pub String);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToAttack {
    pub attacker: Entity,
    pub victim: Entity,
}
