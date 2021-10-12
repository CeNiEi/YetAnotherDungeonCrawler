use crate::prelude::*;

//---RENDER COMPONENTS---//

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
pub struct RangedRender {
    pub color: ColorPair,
}


//---MODE COMPONENTS---//
#[derive(Clone, Copy, PartialEq)]
pub enum MovableSpriteMode {
    LeftMove,
    RightMove,
    Attack,
    Idle
}

#[derive(Clone, Copy, PartialEq)]
pub struct MovableSprite {
    pub mode: MovableSpriteMode
}

pub enum RangedSpriteMode {
    North, 
    West, 
    South, 
    East
}


pub struct RangedSprite {
    pub mode: RangedSpriteMode
}

//--ENTITY COMPONENTS--//
#[derive(Clone, Copy, PartialEq)]
pub struct Player;

#[derive(Clone, Copy, PartialEq)]
pub struct ImmovableEnemy;

#[derive(Copy, Clone, PartialEq)]
pub struct Ranged; 

#[derive(Copy, Clone, PartialEq)]
pub struct InflictsDamage {
    pub damage: i32
}

#[derive(Copy, Clone, PartialEq)]
pub struct Homing;


//---MESSAGES OF INTENT---//
#[derive(Clone, Copy, PartialEq)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Point
}

#[derive(Clone, Copy, PartialEq)]
pub struct WantsToAttack {
    pub attacker: Entity, 
    pub victim: Entity
}

#[derive(Clone, Copy, PartialEq)]
pub struct WantsToChangeMovableSpriteMode {
    pub entity: Entity, 
    pub mode: MovableSpriteMode
}

