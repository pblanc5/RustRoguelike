use tcod::{
    console::{Root, Offscreen, blit}, Console, BackgroundFlag,
    input::{Key},
    map::{FovAlgorithm, Map as FovMap}};

use crate::{
    map::{MAP_WIDTH, MAP_HEIGHT, Tile, TileType},
    game_object::GameObject, COLOR_DARK_WALL, COLOR_DARK_GROUND
 };

pub const SCREEN_WIDTH: i32 = 80;
pub const SCREEN_HEIGHT: i32 = 50;

pub const LIMIT_FPS: i32 = 20;

// FOV Constants
const FOV_ALGO: FovAlgorithm = FovAlgorithm::Basic; // default FOV algorithm
const FOV_LIGHT_WALLS: bool = true; // light walls or not
const TORCH_RADIUS: i32 = 10;

pub struct Display {
    root: Root,
    console: Offscreen,
    pub fov: FovMap,
    // pub game: &'a mut Game<'a>
}

impl Display {
    pub fn new(root: Root, console: Offscreen, fov: FovMap) -> Self {
        Display {
            root,
            console,
            fov,
            //game,
        }
    }

    pub fn render_game_object(&mut self, object: &GameObject) {
        self.console.set_default_foreground(object.color);
        self.console.put_char(object.x, object.y, object.char, BackgroundFlag::None);
    }

    pub fn render_game_tile(&mut self, tile: &Tile, x: i32, y: i32) {
        match tile.tile_type {
            TileType::Wall => self
                .console
                .set_char_background(x, y, COLOR_DARK_WALL, BackgroundFlag::Set),
            TileType::Empty => self
                .console
                .set_char_background(x, y, COLOR_DARK_GROUND, BackgroundFlag::Set) 
        }
    }

    pub fn update_fov(&mut self, x: i32, y: i32) {
        self.fov.compute_fov(
            x, 
            y, 
            TORCH_RADIUS, 
            FOV_LIGHT_WALLS, 
            FOV_ALGO)
    }

    pub fn blit(&mut self) {
        blit(&mut self.console,
            (0,0),
            (MAP_WIDTH, MAP_HEIGHT),
            &mut self.root,
            (0, 0), 
            1.0, 
            1.0);
    }

    pub fn get_keypress(&mut self) -> Key {
        self.root.wait_for_keypress(true)
    }

    pub fn toggle_fullscreen(&mut self) {
        let is_fullscreen = self.root.is_fullscreen();
        self.root.set_fullscreen(!is_fullscreen);
    }

    pub fn clear_console(&mut self) {
        self.console.clear();
    }

    pub fn flush(&mut self) {
        self.root.flush();
    }

    pub fn closed(&self) -> bool {
        self.root.window_closed()
    }
}