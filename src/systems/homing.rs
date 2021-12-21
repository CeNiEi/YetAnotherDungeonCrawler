use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Homing)]
#[read_component(Enemy)]
#[read_component(RangedSprite)]
pub fn homing(#[resource] map: &Map, ecs: &SubWorld, commands: &mut CommandBuffer) {
    let mut player = <(Entity, &Point)>::query().filter(component::<Player>());

    let (player_entity, player_pos) = player.iter(ecs).nth(0).unwrap();
    let player_idx = map_idx(player_pos.x, player_pos.y);

    let search_target = vec![player_idx];
    let dijkstra_map = DijkstraMap::new(SCREEN_WIDTH, SCREEN_HEIGHT, &search_target, map, 1024.0);

    let mut missiles = <(Entity, &Point, &RangedSprite)>::query().filter(component::<Homing>());
    missiles.iter(ecs).for_each(|(entity, pos, ranged_sprite)| {
        if ranged_sprite.mode == RangedSpriteMode::Moving {
            let idx = map_idx(pos.x, pos.y);
            if let Some(destination) = DijkstraMap::find_lowest_exit(&dijkstra_map, idx, map) {
                let distance = DistanceAlg::Pythagoras.distance2d(*pos, *player_pos);

                let destination = if distance > 1.2 {
                    map.index_to_point2d(destination)
                } else {
                    *player_pos
                };

                if destination == *player_pos {
                    commands.push((
                        (),
                        WantsToChangeRangedSpriteMode {
                            entity: *entity,
                            mode: RangedSpriteMode::Landed,
                        },
                    ));
                    commands.push((
                        (),
                        WantsToAttack {
                            attacker: *entity,
                            victim: *player_entity,
                        },
                    ));
                } else {
                    commands.push((
                        (),
                        WantsToMove {
                            entity: *entity,
                            destination,
                        },
                    ));
                }
            }
        } else {
            if DistanceAlg::Pythagoras.distance2d(*pos, *player_pos) < 1.5 {
                commands.push((
                    (),
                    WantsToAttack {
                        attacker: *entity,
                        victim: *player_entity,
                    },
                ));
            }
        }
    });

    let mut moving_enemies =
        <(Entity, &Point)>::query().filter(component::<Homing>() & component::<Enemy>());

    moving_enemies.iter(ecs).for_each(|(entity, pos)| {
        let idx = map_idx(pos.x, pos.y);
        if let Some(destination) = DijkstraMap::find_lowest_exit(&dijkstra_map, idx, map) {
            let distance = DistanceAlg::Pythagoras.distance2d(*pos, *player_pos);
            let destination = if distance > 1.2 {
                map.index_to_point2d(destination)
            } else {
                *player_pos
            };

            if destination == *player_pos {
                commands.push((
                    (), 
                    WantsToAttack {
                        attacker: *entity, 
                        victim: *player_entity
                    }
                ));
            } else {
                commands.push((
                    (), 
                    WantsToMove {
                        entity: *entity, 
                        destination
                    }
                ));
            }

        }
    });
    
}
