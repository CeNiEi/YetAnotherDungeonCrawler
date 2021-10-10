use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(ImmovableEnemy)]
pub fn player_input(
    ecs: &mut SubWorld,
    #[resource] map: &Map,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] camera: &mut Camera,
    #[resource] turn_state: &mut TurnState,
    commands: &mut CommandBuffer,
) {
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::zero(),
        };

        let mut players = <(Entity, &Point)>::query().filter(component::<Player>());

        let (player_entity, destination) = players
            .iter(ecs)
            .find_map(|(entity, pos)| Some((*entity, *pos + delta)))
            .unwrap();

        let mut enemies = <(Entity, &Point)>::query().filter(component::<ImmovableEnemy>());
        if delta.x != 0 || delta.y != 0 {
            let mut hit_something = false;
            enemies
                .iter(ecs)
                .filter(|(_, pos)| **pos == destination)
                .for_each(|(entity, _)| {
                    hit_something = true;
                    commands.push((
                        (),
                        WantsToAttack {
                            attacker: player_entity,
                            victim: *entity,
                        },
                    ));
                    commands.push((
                        (),
                        WantsToChangeMode {
                            entity: player_entity,
                            mode: Mode::Attack,
                        },
                    ));
                });

            if !hit_something {
                commands.push((
                    (),
                    WantsToMove {
                        entity: player_entity,
                        destination,
                    },
                ));

                if delta == Point::new(-1, 0) {
                    commands.push((
                        (),
                        WantsToChangeMode {
                            entity: player_entity,
                            mode: Mode::LeftMove,
                        },
                    ));
                } else if delta == Point::new(1, 0) {
                    commands.push((
                        (),
                        WantsToChangeMode {
                            entity: player_entity,
                            mode: Mode::RightMove,
                        },
                    ));
                } else {
                    commands.push((
                        (),
                        WantsToChangeMode {
                            entity: player_entity,
                            mode: Mode::Idle,
                        },
                    ));
                }
            }

            *turn_state = TurnState::PlayerTurn;
        }
    }
}
