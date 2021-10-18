use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        MovableSprite {
            mode: MovableSpriteMode::Idle
        },
        Player,
        pos,
        MovableRender {
            color: ColorPair::new(WHITE, BLACK),
            left_move_glyph_vec: {
                let mut glyph_vec = vec![0; 10];
                for (i, symbol) in glyph_vec.iter_mut().enumerate() {
                    *symbol = to_cp437(to_char((32 + i + 1) as u8));
                }
                glyph_vec
            },
            right_move_glyph_vec: {
                let mut glyph_vec = vec![0; 10];
                for (i, symbol) in glyph_vec.iter_mut().enumerate() {
                    *symbol = to_cp437(to_char((16 + i + 1) as u8));
                }
                glyph_vec
            },
            idle_glyph_vec: {
                let mut glyph_vec = vec![0; 10];
                for (i, symbol) in glyph_vec.iter_mut().enumerate() {
                    *symbol = to_cp437(to_char((i + 1) as u8));
                }
                glyph_vec
            },
            attack_glyph_vec: {
                let mut glyph_vec = vec![0; 10];
                for (i, symbol) in glyph_vec.iter_mut().enumerate() {
                    *symbol = to_cp437(to_char((i + 1) as u8));
                }
                glyph_vec
            },
        },
    ));
}

pub fn spawn_immovable_enemy(ecs: &mut World, pos: Point) {
    ecs.push((
        ImmovableEnemy,
        pos,
        ImmovableRender3x3 {
            color: ColorPair::new(WHITE, BLACK),
            glyph_grid: {
                let temp_grid = vec![vec![0_u16; 3]; 3];
                let mut glyph_grid_vec = vec![temp_grid; 5];
                for (i, grid) in glyph_grid_vec.iter_mut().enumerate() {
                    for row in 0..3 {
                        for col in 0..3 {
                            grid[row][col] =
                                to_cp437(to_char((11 + row as u8) * 16 + (i * 3 + col) as u8))
                        }
                    }
                }
                glyph_grid_vec
            },
        },
    ));
}

pub fn spawn_homing_missile(commands: &mut CommandBuffer, pos: Point) {
    commands.push((
        pos, 
        Ranged,
        InflictsDamage {
            damage: 8
        },
        RangedRender {
            color: ColorPair::new(WHITE, BLACK)    
        },
        RangedSprite {
            mode: RangedSpriteMode::Moving
        }
    ));
}
