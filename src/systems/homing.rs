use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Homing)]
pub fn homing(#[resource] map: &Map, ecs: &SubWorld, commands: &mut CommandBuffer) {
    let mut player = <(&Point, &Player)>::query(); 
    let player_pos = player.iter(ecs).nth(0).unwrap().0;
    let player_idx = map_idx(player_pos.x, player_pos.y);

    let search_target = vec![player_idx];
    let dijkstra_map = DijkstraMap::new(
        SCREEN_WIDTH,
        SCREEN_HEIGHT, 
        &search_target,
        map, 
        1024.0
    );

    let mut missiles = <(Entity, &Point, &Ranged)>::query().filter(component::<Homing>());
    missiles.iter(ecs).for_each(|(_, pos, ranged)| {
        let idx = map_idx(pos.x, pos.y);
        if let Some(destination) = DijkstraMap::find_lowest_exit(&dijkstra_map, idx, map) {
            let distance = DistanceAlg::Pythagoras.distance2d(*pos, *player_pos);
            let destination = if distance > 1.2 {
                map.index_to_point2d(destination)
            } else {
                *player_pos
            };
        }
    })


}