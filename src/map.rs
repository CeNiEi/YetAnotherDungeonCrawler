use crate::prelude::*;
const MAP: &str = include_str!("../MAP_LVL_2.txt");

pub struct Map {
    pub tiles: Vec<char>,
    pub revealed_tiles: Vec<bool>
}

impl Map {
    pub fn new() -> Self {
        let tiles = MAP
                .chars()
                .filter(|a| *a != '\n' && *a != '\r')
                .collect::<Vec<char>>();
        let num_of_tiles = tiles.len();
        Self {
            tiles,
            revealed_tiles: vec![false; num_of_tiles]
        }
    }

    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point) && self.tiles[map_idx(point.x, point.y)] == ' '
    }

    fn valid_exit(&self, loc: Point, delta: Point) -> Option<usize> {
        let destination = loc + delta;
        if self.in_bounds(destination) {
            if self.can_enter_tile(destination) {
                let idx = self.point2d_to_index(destination);
                Some(idx)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(SCREEN_WIDTH, SCREEN_HEIGHT)
    }

    fn in_bounds(&self, point: Point) -> bool {
        self.in_bounds(point)
    }
}

impl BaseMap for Map {
    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let location = self.index_to_point2d(idx);

        if let Some(idx) = self.valid_exit(location, Point::new(-1, 0)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(1, 0)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, -1)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, 1)) {
            exits.push((idx, 1.0))
        }
        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::Pythagoras.distance2d(self.index_to_point2d(idx1), self.index_to_point2d(idx2))
    }

    fn is_opaque(&self, idx: usize) -> bool {
        !(self.tiles[idx] == ' ' || self.tiles[idx] == 'O' || self.tiles[idx] == 'M')
    }
}

pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

pub fn map_pos(idx: usize) -> Point {
    Point::new(idx as i32 % SCREEN_WIDTH, idx as i32 / SCREEN_WIDTH)
}

pub fn set_wall(ch: char, pos: Point) -> char {
    match ch {
        'X' => {
            if pos.x % 2 == 0 {
                to_char(68)
            } else {
                to_char(69)
            }
        }
        'x' => {
            if pos.x % 2 == 0 {
                to_char(84)
            } else {
                to_char(85)
            }
        }

        '\\' => to_char(133),
        '/' => to_char(135),

        'A' => to_char(101),
        'a' => to_char(117),
        'B' => to_char(103),
        'b' => to_char(119),

        'E' => to_char(104),
        'e' => to_char(120),
        'F' => to_char(105),
        'f' => to_char(121),

        'C' => to_char(165),
        'c' => to_char(149),
        'D' => to_char(151),
        'd' => to_char(167),

        'y' => to_char(150),

        _ => to_char(0),
    }
}

pub fn set_floor(ch: char, pos: Point) -> char {
    match ch {
        '_' | '|' | '-' => to_char(0),

        _ => match pos.x % 2 == 0 {
            true => {
                if pos.y % 2 == 0 {
                    to_char(100)
                } else {
                    to_char(116)
                }
            }
            false => {
                if pos.y % 2 == 0 {
                    to_char(116)
                } else {
                    to_char(100)
                }
            }
        },
    }
}
