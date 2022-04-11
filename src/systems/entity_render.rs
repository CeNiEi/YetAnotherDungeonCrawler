use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(MovableRender)]
#[read_component(MovableSprite)]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn movable_entity_render(
    ecs: &mut SubWorld,
    #[resource] camera: &Camera,
    #[resource] frame_num: &usize,
) {
    let mut draw_batch = DrawBatch::new();
    let offset = Point::new(camera.left_x, camera.top_y);

    let mut fov = <&FieldOfView>::query().filter(component::<Player>());

    let player_fov = fov.iter(ecs).nth(0);
    if player_fov == None {
        return;
    }
    let player_fov = player_fov.unwrap();

    <(&Point, &MovableRender, &MovableSprite)>::query()
        .iter(ecs)
        .filter(|(pos, _, _)| player_fov.visible_tiles.contains(&pos))
        .for_each(|(pos, render, movable_sprite)| {
            if render.monster {
                draw_batch.target(2);
            } else {
                draw_batch.target(1);
            }
            match movable_sprite.mode {
                MovableSpriteMode::Idle => draw_batch.set(
                    *pos - offset,
                    render.color,
                    render.idle_glyph_vec[*frame_num / 4],
                ),
                MovableSpriteMode::LeftMove => draw_batch.set(
                    *pos - offset,
                    render.color,
                    render.left_move_glyph_vec[*frame_num / 4],
                ),
                MovableSpriteMode::RightMove => draw_batch.set(
                    *pos - offset,
                    render.color,
                    render.right_move_glyph_vec[*frame_num / 4],
                ),
                MovableSpriteMode::RightAttack => draw_batch.set(
                    *pos - offset,
                    render.color,
                    render.right_attack_glyph_vec[*frame_num / 4],
                ),
                MovableSpriteMode::LeftAttack => draw_batch.set(
                    *pos - offset,
                    render.color,
                    render.left_attack_glyph_vec[*frame_num / 4],
                ),
            };
        });

    draw_batch.submit(12000).expect("Batch Error");
}

#[system]
#[read_component(Point)]
#[read_component(ImmovableRender3x3)]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn immovable_entity_render_3x3(
    ecs: &SubWorld,
    #[resource] camera: &Camera,
    #[resource] frame_num: &usize,
) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    let offset = Point::new(camera.left_x, camera.top_y);

    let mut fov = <&FieldOfView>::query().filter(component::<Player>());

    let player_fov = fov.iter(ecs).nth(0);
    if player_fov == None {
        return;
    }
    let player_fov = player_fov.unwrap();

    <(&Point, &ImmovableRender3x3)>::query()
        .iter(ecs)
        .filter(|(pos, _)| player_fov.visible_tiles.contains(&pos))
        .for_each(|(pos, render3x3)| {
            for row in -1_i32..=1_i32 {
                for col in -1_i32..=1_i32 {
                    draw_batch.set(
                        *pos - offset + Point::new(col, row),
                        render3x3.color,
                        render3x3.glyph_grid[*frame_num / 8][(row + 1) as usize]
                            [(col + 1) as usize],
                    );
                }
            }
        });
    draw_batch.submit(13000).expect("batch error");
}

#[system]
#[read_component(Point)]
#[read_component(RangedRender)]
#[read_component(RangedSprite)]
#[read_component(Homing)]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn single_missile_entity_render(
    ecs: &SubWorld,
    #[resource] camera: &Camera,
    #[resource] frame_num: &usize,
) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    let offset = Point::new(camera.left_x, camera.top_y);

    let mut fov = <&FieldOfView>::query().filter(component::<Player>());

    let player_fov = fov.iter(ecs).nth(0);
    if player_fov == None {
        return;
    }
    let player_fov = player_fov.unwrap();

    <(&Point, &RangedRender, &RangedSprite)>::query()
        .filter(!component::<AreaOfEffect>())
        .iter(ecs)
        .filter(|(pos, _, _)| player_fov.visible_tiles.contains(&pos))
        .for_each(
            |(pos, ranged_render, ranged_sprite)| match ranged_sprite.mode {
                RangedSpriteMode::Moving => {
                    draw_batch.set(*pos - offset, ranged_render.color, to_cp437(to_char(225)));
                }
                RangedSpriteMode::Landed => {
                    draw_batch.set(
                        *pos - offset,
                        ranged_render.color,
                        ranged_render.landed_glyph_vec[*frame_num / 15],
                    );
                }
            },
        );

    draw_batch.submit(15000).expect("batch error");
}

#[system]
#[read_component(Point)]
#[read_component(RangedRender)]
#[read_component(RangedSprite)]
#[read_component(Homing)]
#[read_component(AreaOfEffect)]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn splash_missile_entity_render(
    ecs: &SubWorld,
    #[resource] camera: &Camera,
    #[resource] map: &Map,
    #[resource] frame_num: &usize,
) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    let offset = Point::new(camera.left_x, camera.top_y);

    let mut fov = <&FieldOfView>::query().filter(component::<Player>());

    let player_fov = fov.iter(ecs).nth(0);
    if player_fov == None {
        return;
    }
    let player_fov = player_fov.unwrap();

    <(&Point, &RangedRender, &RangedSprite, &AreaOfEffect)>::query()
        .iter(ecs)
        .filter(|(pos, _, _, _)| player_fov.visible_tiles.contains(&pos))
        .for_each(
            |(pos, ranged_render, ranged_sprite, aoe)| match ranged_sprite.mode {
                RangedSpriteMode::Moving => {
                    draw_batch.set(*pos - offset, ranged_render.color, to_cp437(to_char(225)));
                }
                RangedSpriteMode::Landed => {
                    for x in -aoe.radius..=aoe.radius {
                        for y in -aoe.radius..=aoe.radius {
                            let final_pos = *pos + Point::new(x, y);
                            if map.tiles[map_idx(final_pos.x, final_pos.y)] == ' ' {
                                draw_batch.set(
                                    final_pos - offset,
                                    ranged_render.color,
                                    ranged_render.landed_glyph_vec[*frame_num / 8],
                                );
                            }
                        }
                    }
                }
            },
        );

    draw_batch.submit(16000).expect("Batch error");
}

#[system]
#[read_component(Point)]
#[read_component(ItemRender)]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn item_render(ecs: &SubWorld, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);

    let mut fov = <&FieldOfView>::query().filter(component::<Player>());

    let player_fov = fov.iter(ecs).nth(0);
    if player_fov == None {
        return;
    }
    let player_fov = player_fov.unwrap();

    let offset = Point::new(camera.left_x, camera.top_y);

    <(&Point, &ItemRender)>::query()
        .iter(ecs)
        .filter(|(pos, _)| player_fov.visible_tiles.contains(&pos))
        .for_each(|(pos, item_render)| {
            draw_batch.set(*pos - offset, item_render.color, item_render.glyph);
        });

    draw_batch.submit(18000).expect("Batch error");
}
