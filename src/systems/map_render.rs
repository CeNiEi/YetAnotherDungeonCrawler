use crate::prelude::*;

#[system]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn map_render(ecs: &SubWorld, #[resource] map: &Map, #[resource] camera: &Camera, #[resource] frame_num: &mut usize) {
    *frame_num = (*frame_num + 1) % 40;

    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    
    let player_fov = fov.iter(ecs).nth(0);
    if player_fov == None {
        return;
    }

    let player_fov = player_fov.unwrap();

    let mut draw_batch = DrawBatch::new();
    for y in camera.top_y..=camera.bottom_y {
        for x in camera.left_x..=camera.right_x {
            let pos = Point::new(x - camera.left_x, y - camera.top_y);
            let pt = Point::new(x, y);
            if map.in_bounds(pt) && player_fov.visible_tiles.contains(&pt) {
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
