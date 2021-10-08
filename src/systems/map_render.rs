use crate::prelude::*;

#[system]
pub fn map_render(#[resource] map: &Map, #[resource] camera: &Camera, #[resource] frame_num: &mut usize) {
    *frame_num = (*frame_num + 1) % 40;
    let mut draw_batch = DrawBatch::new();
    for y in camera.top_y..=camera.bottom_y {
        for x in camera.left_x..=camera.right_x {
            let pos = Point::new(x - camera.left_x, y - camera.top_y);
            if map.in_bounds(Point::new(x, y)) {
                let idx = map_idx(x, y);

                draw_batch.target(0);
                draw_batch.set(
                    pos,
                    ColorPair::new(WHITE, BLACK),
                    to_cp437(set_floor(map.tiles[idx], Point::new(x, y))),
                );

                draw_batch.target(1);
                draw_batch.set(
                    pos,
                    ColorPair::new(WHITE, BLACK),
                    to_cp437(set_wall(map.tiles[idx], Point::new(x, y))),
                );
            }
        }
    }
    draw_batch.submit(0).expect("Batch error");
}
