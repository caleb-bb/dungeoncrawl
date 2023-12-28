// Components using the Legion ecs are generally structs, but sometimes enums
pub use crate::prelude::*;

// ColorPair is a helper class from backet-lib that stores a foreground and background color
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;
