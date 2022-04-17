use crate::prelude::*;

pub struct MapBuilder {
    pub map: Map,
    pub immovable_enemies: Vec<Point>,
    pub movable_enemies: Vec<Point>,
    pub potions: Vec<Point>,
    pub player_start: Point,
    pub key_pos: Point
}

impl MapBuilder {
    pub fn new() -> Self {
        let map = Map::new();
        let mut immovable_enemies = vec![];
        let mut movable_enemies = vec![];
        let mut potions = vec![];
        let mut player_start = Point::zero();
        let mut key_pos = Point::zero();
        map.tiles.iter().enumerate().for_each(|(idx, ch)| {
            if *ch == 'O' {
                immovable_enemies.push(map_pos(idx))
            } else if *ch == 'M' {
                movable_enemies.push(map_pos(idx))
            }
            else if *ch == 'P'{
                player_start = map_pos(idx)
            } else if *ch == 'K' {
                key_pos = map_pos(idx)
            } else if *ch == 'H' {
                potions.push(map_pos(idx))
            }
        });

        Self {
            map, 
            immovable_enemies, 
            movable_enemies,
            potions,
            player_start,
            key_pos
        }
    }
}
