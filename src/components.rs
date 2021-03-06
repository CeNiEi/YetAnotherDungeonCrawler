use crate::prelude::*;
use std::collections::HashSet;

//---RENDER COMPONENTS---//

#[derive(Clone, PartialEq)]
pub struct ItemRender {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

#[derive(Clone, PartialEq)]
pub struct MovableRender {
    pub color: ColorPair,
    pub monster: bool,
    pub left_move_glyph_vec: Vec<FontCharType>,
    pub right_move_glyph_vec: Vec<FontCharType>,
    pub idle_glyph_vec: Vec<FontCharType>,
    pub right_attack_glyph_vec: Vec<FontCharType>,
    pub left_attack_glyph_vec: Vec<FontCharType>,
}

pub type GlyphGrid = Vec<Vec<FontCharType>>;

#[derive(Clone, PartialEq)]
pub struct ImmovableRender3x3 {
    pub color: ColorPair,
    pub glyph_grid: Vec<GlyphGrid>,
}

#[derive(Clone, PartialEq)]
pub struct RangedRender {
    pub color: ColorPair,
    pub landed_glyph_vec: Vec<FontCharType>,
}

//---MODE COMPONENTS---//
#[derive(Clone, Copy, PartialEq)]
pub enum MovableSpriteMode {
    LeftMove,
    RightMove,
    LeftAttack,
    RightAttack,
    Idle,
}

#[derive(Clone, Copy, PartialEq)]
pub struct MovableSprite {
    pub mode: MovableSpriteMode,
}

#[derive(Clone, Copy, PartialEq)]
pub enum RangedSpriteMode {
    Moving,
    Landed,
}

#[derive(Clone, Copy, PartialEq)]
pub struct RangedSprite {
    pub mode: RangedSpriteMode,
}

//--ENTITY COMPONENTS--//

#[derive(Clone, Debug, PartialEq)]
pub struct FieldOfView {
    pub visible_tiles: HashSet<Point>,
    pub radius: i32,
    pub is_dirty: bool,
}

impl FieldOfView {
    pub fn new(radius: i32) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius,
            is_dirty: true,
        }
    }
    pub fn clone_dirty(&self) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius: self.radius,
            is_dirty: true,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Carried(pub Entity);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Healer;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Item;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Key;

#[derive(Clone, PartialEq)]
pub struct Name(pub String);

#[derive(Clone, Copy, PartialEq)]
pub struct Player;

#[derive(Clone, Copy, PartialEq)]
pub struct Enemy;

#[derive(Copy, Clone, PartialEq)]
pub struct Homing;

#[derive(Clone, Copy, PartialEq)]
pub struct AreaOfEffect {
    pub radius: i32,
}

/* Not Needed Now

#[derive(Copy, Clone, PartialEq)]
pub struct InflictsDamage {
    pub damage: i32,
}
*/

#[derive(Copy, Clone, PartialEq)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

//---MESSAGES OF INTENT---//
#[derive(Clone, Copy, PartialEq)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Point,
}

#[derive(Clone, Copy, PartialEq)]
pub struct WantsToAttack {
    pub attacker: Entity,
    pub victim: Entity,
}

#[derive(Clone, Copy, PartialEq)]
pub struct WantsToChangeMovableSpriteMode {
    pub entity: Entity,
    pub mode: MovableSpriteMode,
}

#[derive(Clone, Copy, PartialEq)]
pub struct WantsToChangeRangedSpriteMode {
    pub entity: Entity,
    pub mode: RangedSpriteMode,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ActivateItem {
    pub used_by: Entity,
    pub item: Entity,
}
