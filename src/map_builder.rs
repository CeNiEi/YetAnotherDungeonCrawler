use crate::prelude::*;

pub struct MapBuilder {
    pub map: Map,
    pub immovable_enemies: Vec<Point>,
    pub movable_enemies: Vec<Point>,
    pub player_start: Point
}

impl MapBuilder {
    pub fn new() -> Self {
        let map = Map::new();
        let mut immovable_enemies = vec![];
        let mut movable_enemies = vec![];
        map.tiles.iter().enumerate().for_each(|(idx, ch)| {
            if *ch == 'O' {
                immovable_enemies.push(map_pos(idx))
            } else if *ch == 'M' {
                movable_enemies.push(map_pos(idx))
            }
        });

        Self {
            map, 
            immovable_enemies, 
            movable_enemies,
            player_start: Point::new(54, 5)
        }
    }
}
