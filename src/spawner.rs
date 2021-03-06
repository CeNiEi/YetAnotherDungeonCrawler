use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        MovableSprite {
            mode: MovableSpriteMode::Idle,
        },
        Player,
        Health {
            current: 20,
            max: 20,
        },
        FieldOfView::new(8),
        pos,
        MovableRender {
            color: ColorPair::new(WHITE, BLACK),
            monster: false,
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
            left_attack_glyph_vec: {
                let mut glyph_vec = vec![0; 10];
                for (i, symbol) in glyph_vec.iter_mut().enumerate() {
                    *symbol = to_cp437(to_char((14 * 16 + 5 + i) as u8));
                }
                glyph_vec
            },
            right_attack_glyph_vec: {
                let mut glyph_vec = vec![0; 10];
                for (i, symbol) in glyph_vec.iter_mut().enumerate() {
                    *symbol = to_cp437(to_char((48 + i + 1) as u8));
                }
                glyph_vec
            },
        },
    ));
}

pub fn spawn_immovable_enemy(ecs: &mut World, pos: Point) {
    ecs.push((
        Enemy,
        pos,
        Health { current: 5, max: 5 },
        Name("BossMonster".to_string()),
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

pub fn spawn_movable_enemy(ecs: &mut World, pos: Point) {
    ecs.push((
        Enemy,
        MovableSprite {
            mode: MovableSpriteMode::Idle,
        },
        pos,
        FieldOfView::new(6),
        Name("Minion".to_string()),
        Homing,
        Health { current: 5, max: 5 },
        MovableRender {
            color: ColorPair::new(WHITE, BLACK),
            monster: true,
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
            left_attack_glyph_vec: {
                let mut glyph_vec = vec![0; 10];
                for (i, symbol) in glyph_vec.iter_mut().enumerate() {
                    *symbol = to_cp437(to_char((64 + i + 1) as u8));
                }
                glyph_vec
            },
            right_attack_glyph_vec: {
                let mut glyph_vec = vec![0; 10];
                for (i, symbol) in glyph_vec.iter_mut().enumerate() {
                    *symbol = to_cp437(to_char((48 + i + 1) as u8));
                }
                glyph_vec
            },
        },
    ));
}

pub fn spawn_homing_missile(commands: &mut CommandBuffer, pos: Point) {
    let mut rng = RandomNumberGenerator::new();
    match rng.range(0, 5) {
        0..=2 => {
            commands.push((
                pos,
                Homing,
                Health { current: 0, max: 0 },
                RangedRender {
                    color: ColorPair::new(WHITE, BLACK),
                    landed_glyph_vec: {
                        let mut glyph_vec = vec![0; 3];
                        for (i, symbol) in glyph_vec.iter_mut().enumerate() {
                            *symbol = to_cp437(to_char((14 * 16 + 2 + i) as u8));
                        }
                        glyph_vec
                    },
                },
                RangedSprite {
                    mode: RangedSpriteMode::Moving,
                },
            ));
        }
        _ => {
            commands.push((
                pos,
                Homing,
                AreaOfEffect { radius: 1 },
                Health { current: 5, max: 5 },
                RangedRender {
                    color: ColorPair::new(WHITE, BLACK),
                    landed_glyph_vec: {
                        let mut glyph_vec = vec![0; 6];
                        for (i, symbol) in glyph_vec.iter_mut().enumerate() {
                            *symbol = to_cp437(to_char((15 * 16 + i) as u8));
                        }
                        glyph_vec
                    },
                },
                RangedSprite {
                    mode: RangedSpriteMode::Moving,
                },
            ));
        }
    }
}

pub fn spawn_key(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        pos,
        Name("key".to_string()),
        Key,
        ItemRender {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437(to_char((9 * 16 + 11) as u8)),
        },
    ));
}

pub fn spawn_healing_potion(ecs: &mut World, pos: Point) {
    ecs.push((
        Healer,
        Item,
        pos,
        ItemRender {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437(to_char((3 * 16 + 15) as u8)),
        },
        Name("Healing Potion".to_string()),
    ));
}
