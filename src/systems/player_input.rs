use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Player)]
#[write_component(MovableSprite)]
pub fn player_input(
    ecs: &mut SubWorld,
    #[resource] map: &Map,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] camera: &mut Camera,
) {
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::zero(),
        };

        if delta.x != 0 || delta.y != 0 {
            let mut players =
                <(&mut Point, &mut MovableSprite)>::query().filter(component::<Player>());
            players.iter_mut(ecs).for_each(|(pos, movable_sprite)| {

                if delta == Point::new(-1, 0) {
                movable_sprite.mode = Mode::LeftMove;
                } else if delta == Point::new(1, 0) {
                    movable_sprite.mode = Mode::RightMove
                }

                let destination = *pos + delta;
                if map.can_enter_tile(destination) {
                    *pos = destination;
                    camera.on_player_move(destination)
                }
            });
        }
    }
}
