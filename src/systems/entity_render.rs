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
                MovableSpriteMode::Attack => draw_batch.set(
                    *pos - offset,
                    render.color,
                    render.attack_glyph_vec[*frame_num / 4],
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
            for row in 0..3 {
                for col in 0..3 {
                    draw_batch.set(
                        *pos - offset + Point::new(col, row),
                        render3x3.color,
                        render3x3.glyph_grid[*frame_num / 8][row][col],
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
pub fn homing_missile_entity_render(
    ecs: &SubWorld,
    #[resource] camera: &Camera,
) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    let offset = Point::new(camera.left_x, camera.top_y);

    <(&Point, &RangedRender, &RangedSprite)>::query()
        .filter(component::<Ranged>())
        .iter(ecs)
        .for_each(
            |(pos, ranged_render, ranged_sprite)| match ranged_sprite.mode {
                RangedSpriteMode::Moving => {
                    draw_batch.set(*pos - offset, ranged_render.color, to_cp437(to_char(225)));
                }
                RangedSpriteMode::Landed => {
                    draw_batch.set(*pos - offset, ranged_render.color, to_cp437(to_char(227)));
                }
            },
        );

    draw_batch.submit(15000).expect("batch error");
}
