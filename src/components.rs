use crate::prelude::*;
use std::collections::HashSet;

#[derive(Clone, PartialEq)] 
pub struct MovableRender {
    pub color: ColorPair,
    pub left_move_glyph_vec: Vec<FontCharType>,
    pub right_move_glyph_vec: Vec<FontCharType>,
    pub idle_glyph_vec: Vec<FontCharType>,
    pub attack_glyph_vec: Vec<FontCharType>
}

pub type GlyphGrid = Vec<Vec<FontCharType>>;

#[derive(Clone, PartialEq)]
pub struct ImmovableRender3x3 {
    pub color: ColorPair,
    pub glyph_grid: Vec<GlyphGrid>
}

#[derive(Clone, PartialEq)]
pub struct MissileRender {
    pub color: ColorPair,
    pub glyph_vec: Vec<FontCharType>
}

#[derive(Clone, Copy, PartialEq)]
pub enum Mode {
    LeftMove,
    RightMove,
    Attack,
    Idle
}

#[derive(Clone, Copy, PartialEq)]
pub struct MovableSprite {
    pub mode: Mode
}

#[derive(Clone, Copy, PartialEq)]
pub struct Player;

#[derive(Clone, Copy, PartialEq)]
pub struct ImmovableEnemy;

#[derive(Clone, PartialEq)]
pub struct FieldOfView {
    pub visible_tiles: HashSet<Point>,
    pub radius: i32, 
    pub is_dirty: bool
}

impl FieldOfView {
    pub fn new(radius: i32) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius, 
            is_dirty: true
        }
    }

    pub fn clone_dirty(&self) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius: self.radius,
            is_dirty: true
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct Ranged {
    pub range: i32
}

#[derive(Copy, Clone, PartialEq)]
pub struct InflictsDamage {
    pub damage: i32
}

#[derive(Clone, Copy, PartialEq)]
pub struct Homing;

#[derive(Clone, Copy, PartialEq)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Point
}