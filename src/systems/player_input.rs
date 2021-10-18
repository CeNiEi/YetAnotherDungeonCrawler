use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(ImmovableEnemy)]
pub fn player_input(
    ecs: &mut SubWorld,
    #[resource] key: &Option<VirtualKeyCode>,
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

        if delta.x != 0 || delta.y != 0 {
            let mut players = <(Entity, &Point)>::query().filter(component::<Player>());

            let (player_entity, destination) = players
                .iter(ecs)
                .find_map(|(entity, pos)| Some((*entity, *pos + delta)))
                .unwrap();

            let mut enemies = <(Entity, &Point)>::query().filter(component::<ImmovableEnemy>());

            let mut missiles_present = false;
            for _ in <Entity>::query().filter(component::<Ranged>()).iter(ecs) {
                missiles_present = true;
            }

            let mut hit_something = false;
            enemies
                .iter(ecs)
                .filter(|(_, pos)| DistanceAlg::Pythagoras.distance2d(**pos, destination) < 5.0)
                .for_each(|(entity, pos)| {
                    if *pos == destination {
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
                            WantsToChangeMovableSpriteMode {
                                entity: player_entity,
                                mode: MovableSpriteMode::Attack,
                            },
                        ));
                    } else {
                        if !missiles_present {
                            spawn_homing_missile(commands, *pos);
                        }
                    }
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
                        WantsToChangeMovableSpriteMode {
                            entity: player_entity,
                            mode: MovableSpriteMode::LeftMove,
                        },
                    ));
                } else if delta == Point::new(1, 0) {
                    commands.push((
                        (),
                        WantsToChangeMovableSpriteMode {
                            entity: player_entity,
                            mode: MovableSpriteMode::RightMove,
                        },
                    ));
                } else {
                    commands.push((
                        (),
                        WantsToChangeMovableSpriteMode {
                            entity: player_entity,
                            mode: MovableSpriteMode::Idle,
                        },
                    ));
                }
            }

            *turn_state = TurnState::PlayerTurn;
        }
    }
}
