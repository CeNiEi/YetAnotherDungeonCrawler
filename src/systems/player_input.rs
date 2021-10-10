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
    #[resource] turn_state: &mut TurnState,
    commands: &mut CommandBuffer
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
                <(Entity, &mut Point, &mut MovableSprite)>::query().filter(component::<Player>());
            players.iter_mut(ecs).for_each(|(entity, pos, movable_sprite)| {
                if delta == Point::new(-1, 0) {
                    movable_sprite.mode = Mode::LeftMove;
                } else if delta == Point::new(1, 0) {
                    movable_sprite.mode = Mode::RightMove
                } else {
                    movable_sprite.mode = Mode::Idle;
                }

                let destination = *pos + delta;
                commands.push(((), WantsToMove{entity: *entity, destination}));
                *turn_state = TurnState::PlayerTurn;
            });
        }
    }
}
