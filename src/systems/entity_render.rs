use crate::prelude::*;

//TRY TO MERGE ALL THE RENDERABLE COMPONENETS INTO ONE

#[system]
#[read_component(Point)]
#[read_component(MovableRender)]
#[write_component(MovableSprite)]
pub fn movable_entity_render(
    ecs: &mut SubWorld,
    #[resource] camera: &Camera,
    #[resource] frame_num: &usize,
) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);
    let offset = Point::new(camera.left_x, camera.top_y);

    <(&Point, &MovableRender, &mut MovableSprite)>::query()
        .iter_mut(ecs)
        .for_each(|(pos, render, movable_sprite)| {
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
pub fn immovable_entity_render_3x3(
    ecs: &SubWorld,
    #[resource] camera: &Camera,
    #[resource] frame_num: &usize,
) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);
    let offset = Point::new(camera.left_x, camera.top_y);

    <(&Point, &ImmovableRender3x3)>::query()
        .iter(ecs)
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
#[read_component(Ranged)]
pub fn single_missile_entity_render(
    ecs: &SubWorld,
    #[resource] camera: &Camera,
    #[resource] frame_num: &usize,
) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    let offset = Point::new(camera.left_x, camera.top_y);

    <(&Point, &RangedRender, &RangedSprite)>::query()
        .filter(!component::<AreaOfEffect>())
        .iter(ecs)
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
#[read_component(Ranged)]
#[read_component(AreaOfEffect)]
pub fn splash_missile_entity_render(
    ecs: &SubWorld,
    #[resource] camera: &Camera,
    #[resource] map: &Map,
    #[resource] frame_num: &usize,
) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    let offset = Point::new(camera.left_x, camera.top_y);

    <(&Point, &RangedRender, &RangedSprite, &AreaOfEffect)>::query()
        .iter(ecs)
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
