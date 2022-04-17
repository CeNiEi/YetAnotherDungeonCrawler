use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
#[read_component(Homing)]
#[read_component(AreaOfEffect)]
#[read_component(MovableSprite)]
#[read_component(Item)]
#[read_component(Carried)]
pub fn player_input(
    ecs: &mut SubWorld,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
    commands: &mut CommandBuffer,
) {
    if let Some(key) = key {
        let mut players = <(Entity, &Point, &MovableSprite)>::query().filter(component::<Player>());

        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            VirtualKeyCode::Key1 => use_item(0, ecs, commands),
            VirtualKeyCode::Key2 => use_item(1, ecs, commands),
            VirtualKeyCode::Key3 => use_item(2, ecs, commands),
            VirtualKeyCode::Key4 => use_item(3, ecs, commands),
            VirtualKeyCode::Key5 => use_item(4, ecs, commands),
            VirtualKeyCode::Key6 => use_item(5, ecs, commands),
            VirtualKeyCode::Key7 => use_item(6, ecs, commands),
            VirtualKeyCode::Key8 => use_item(7, ecs, commands),
            VirtualKeyCode::Key9 => use_item(8, ecs, commands),
            _ => Point::zero(),
        };

        if delta.x != 0 || delta.y != 0 {
            let (player_entity, destination, mode) = players
                .iter(ecs)
                .find_map(|(entity, pos, movable_sprite)| {
                    Some((*entity, *pos + delta, movable_sprite.mode))
                })
                .unwrap();

            let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());

            let mut items = <(Entity, &Item, &Point)>::query();
            items
                .iter(ecs)
                .filter(|(_entity, _item, &item_pos)| item_pos == destination)
                .for_each(|(entity, _item, _item_pos)| {
                    commands.remove_component::<Point>(*entity);
                    commands.add_component(*entity, Carried(player_entity));
                });

            let mut missiles_present = false;
            for _ in <Entity>::query()
                .filter(
                    !component::<Enemy>() & (component::<Homing>() | component::<AreaOfEffect>()),
                )
                .iter(ecs)
            {
                missiles_present = true;
            }

            let mut hit_something = false;
            enemies
                .iter(ecs)
                .filter(|(_, pos)| DistanceAlg::Pythagoras.distance2d(**pos, destination) < 5.0)
                .for_each(|(entity, pos)| {
                    if DistanceAlg::Pythagoras.distance2d(*pos, destination) < 1.2 {
                        hit_something = true;
                        commands.push((
                            (),
                            WantsToAttack {
                                attacker: player_entity,
                                victim: *entity,
                            },
                        ));
                        if mode == MovableSpriteMode::LeftMove
                            || mode == MovableSpriteMode::LeftAttack
                        {
                            commands.push((
                                (),
                                WantsToChangeMovableSpriteMode {
                                    entity: player_entity,
                                    mode: MovableSpriteMode::LeftAttack,
                                },
                            ));
                        } else {
                            commands.push((
                                (),
                                WantsToChangeMovableSpriteMode {
                                    entity: player_entity,
                                    mode: MovableSpriteMode::RightAttack,
                                },
                            ));
                        }
                    } else {
                        if !missiles_present {
                            if let Err(_) =
                                ecs.entry_ref(*entity).unwrap().get_component::<Homing>()
                            {
                                spawn_homing_missile(commands, *pos);
                            }
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

                if delta == Point::new(0, 1) || delta == Point::new(0, -1) {
                } else if delta == Point::new(-1, 0) {
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

fn use_item(n: usize, ecs: &mut SubWorld, commands: &mut CommandBuffer) -> Point {
    let player_entity = <(Entity, &Player)>::query()
        .iter(ecs)
        .find_map(|(entity, _player)| Some(*entity))
        .unwrap();
    let item_entity = <(Entity, &Item, &Carried)>::query()
        .iter(ecs)
        .filter(|(_, _, carried)| carried.0 == player_entity)
        .enumerate()
        .filter(|(item_count, (_, _, _))| *item_count == n)
        .find_map(|(_, (item_entity, _, _))| Some(*item_entity));
    if let Some(item_entity) = item_entity {
        commands.push((
            (),
            ActivateItem {
                used_by: player_entity,
                item: item_entity,
            },
        ));
    }
    Point::zero()
}
