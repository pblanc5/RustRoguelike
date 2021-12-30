use std::cmp;

use rand::Rng;

// Map Size
pub const MAP_WIDTH: i32 = 80;
pub const MAP_HEIGHT: i32 = 45;

//parameters for dungeon generator
const ROOM_MAX_SIZE: i32 = 10;
const ROOM_MIN_SIZE: i32 = 6;
const MAX_ROOMS: i32 = 30;


#[derive(Clone, Copy, Debug)]
pub enum TileType {
    Empty,
    Wall
}

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub tile_type: TileType,
    pub blocked: bool,
    pub block_sight: bool
}

impl Tile {
    pub fn new(tile_type: TileType) -> Self {
        match tile_type {
            TileType::Empty => Tile {tile_type: TileType::Empty, blocked: false, block_sight: false},
            TileType::Wall => Tile {tile_type: TileType::Wall, blocked: true, block_sight: true}
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32
}

impl Rect {
    pub fn new(x :i32, y: i32, w: i32, h: i32) -> Self {
        Rect {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h
        }
    }

    pub fn center(&self) -> (i32, i32) {
        let center_x = (self.x1 + self.x2) / 2;
        let center_y = (self.y1 + self.y2) / 2;
        (center_x, center_y)
    }

    pub fn intersects_with(&self, other: &Rect) -> bool {
        (self.x1 <= other.x2)
            && (self.x2 >= other.x1)
            && (self.y1 <= other.y2)
            && (self.y2 >= other.y1)
    }
}

pub type Map = Vec<Vec<Tile>>;

pub fn make_map() -> (Map, i32, i32) {
    let mut map = vec![vec![Tile::new(TileType::Wall); MAP_HEIGHT as usize]; MAP_WIDTH as usize];
    let mut center_x: i32 = MAP_WIDTH / 2;
    let mut center_y: i32 = MAP_HEIGHT / 2;
    
    let mut rooms: Vec<Rect> = vec![];

    for _ in 0..MAX_ROOMS {
        // random height and width
        let w = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
        let h = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);

        // random position
        let x = rand::thread_rng().gen_range(0, MAP_WIDTH - w);
        let y = rand::thread_rng().gen_range(0, MAP_HEIGHT - h);

        let new_room = Rect::new(x, y, w, h);

        let failed = rooms
            .iter()
            .any(|other_room| new_room.intersects_with(other_room));

        if !failed {
            create_room(new_room, &mut map);
            let (new_x, new_y) = new_room.center();
            if rooms.is_empty() {
                center_x = new_x;
                center_y = new_y;
            } else {
                let (prev_x, prev_y) = rooms[rooms.len() - 1].center();
                if rand::random() {
                    create_h_tunnel(prev_x, new_x, new_y, &mut map);
                    create_v_tunnel(prev_y, new_y, new_x, &mut map);
                } else {
                    create_v_tunnel(prev_y, new_y, new_x, &mut map);
                    create_h_tunnel(prev_x, new_x, new_y, &mut map);
                }
            }

            rooms.push(new_room);
        } 
    }

    (map, center_x, center_y)
}

pub fn create_room(room: Rect, map: &mut Map) {
    for x in (room.x1 + 1)..room.x2 {
        for y in (room.y1 + 1)..room.y2 {
            map[x as usize][y as usize] = Tile::new(TileType::Empty);
        }
    }
}

pub fn create_h_tunnel(x1: i32, x2: i32, y: i32, map: &mut Map) {
    for x in cmp::min(x1, x2)..(cmp::max(x1, x2) + 1) {
        map[x as usize][y as usize] = Tile::new(TileType::Empty);
    }
}

pub fn create_v_tunnel(y1: i32, y2: i32, x: i32, map: &mut Map) {
    for y in cmp::min(y1, y2)..(cmp::max(y1, y2) + 1) {
        map[x as usize][y as usize] = Tile::new(TileType::Empty);
    }
}