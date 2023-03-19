
use specs::{Component, VecStorage};

use crate::common::Color;

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum ZLayer {
    Ground, Creature
}

#[derive(Clone, Debug)]
pub struct Render {
    pub glyph: Option<char>,
    pub foreground_color: Color,
    pub background_color: Option<Color>,
    pub z_layer: ZLayer
}

impl Component for Render {
    type Storage = VecStorage<Self>;
}
