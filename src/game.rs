use crate::{map::{Map, Tile}, game_object::GameObject};

pub struct Game<'a> {
    map: Map,
    pub objects: &'a mut Vec<GameObject>,
}

impl<'a> Game<'a> {
    pub fn new(map: Map, objects: &'a mut Vec<GameObject>) -> Self {
        Game {
            map,
            objects,
        }
    }

    pub fn get_mut_player_ref(&mut self) -> &mut GameObject {
        &mut self.objects[0]
    }

    pub fn get_player_position(&self) -> (i32, i32) {
        let player = &self.objects[0];
        (player.x, player.y)
    }

    pub fn get_tile_ref(&self, x: i32, y: i32) -> &Tile {
        &self.map[x as usize][y as usize]
    }

    pub fn move_player_by(&mut self, dx: i32, dy: i32) {
        if !self.map[(self.objects[0].x + dx) as usize][(self.objects[0].y + dy) as usize].blocked {
            self.objects[0].x += dx;
            self.objects[0].y += dy;
        }  
    }

    pub fn register_object(&mut self, object: GameObject) {
        self.objects.push(object);
    }
}