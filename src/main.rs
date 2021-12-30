use display::Display;
use display::SCREEN_HEIGHT;
use display::SCREEN_WIDTH;
use game::Game;
use game_object::GameObject;
use map::MAP_HEIGHT;
use map::MAP_WIDTH;
use map::make_map;
use tcod::colors::*;
use tcod::console::*;
use tcod::input::Key;
use tcod::input::KeyCode;
use tcod::map::Map as FovMap;

mod map;
mod display;
mod game;
mod game_object;

const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
const COLOR_DARK_GROUND: Color = Color {r: 50, g: 50, b: 150 };

fn render_map(display: &mut Display, game: &Game) {
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let tile = game.get_tile_ref(x, y);
            display.render_game_tile(tile, x, y);
        }
    }
}

fn render_objects(display: &mut Display, game: &mut Game) {
    for object in game.objects.iter_mut() {
        display.render_game_object(object);
    }
}

fn handle_keypress(display: &mut Display, game: &mut Game) -> bool {
    let key = display.get_keypress();
        match key {
            Key { code: KeyCode::Up, .. }    => game.move_player_by(0, -1),
            Key { code: KeyCode::Down, .. }  => game.move_player_by(0, 1),
            Key { code: KeyCode::Left, .. }  => game.move_player_by(-1, 0),
            Key { code: KeyCode::Right, .. } => game.move_player_by(1, 0),
            Key { code: KeyCode::Enter, alt: true, .. } => display.toggle_fullscreen(),
            Key { code: KeyCode::Escape, ..} => return true,
            _ => {}
        }
        false
}

fn init_fov(display: &mut Display, game: &Game) {
    for x in 0..MAP_WIDTH {
        for y in 0..MAP_HEIGHT {
            let tile = game.get_tile_ref(x, y);
            display.fov.set(x, y, tile.block_sight, tile.blocked);
        }
    }
}

fn main() {
    let (map, player_x, player_y) = make_map();
    let mut game_objects: Vec<GameObject> = Vec::new();
    let mut game = Game::new(map, &mut game_objects);

    let console = Offscreen::new(MAP_WIDTH, MAP_HEIGHT);
    let fov = FovMap::new(MAP_WIDTH,MAP_HEIGHT);
    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rust/libtcod tutorial")
        .init();

    let mut display = Display::new(root, console, fov);

    let player = GameObject::new(player_x, player_y, '@', WHITE);
    let npc = GameObject::new(player_x - 5, player_y, '@', YELLOW);

    game.register_object(player);
    game.register_object(npc);

    // Init Fov
    init_fov(&mut display, &game);

    let mut previous_player_position = (-1, -1);

    // Game Loop
    while !display.closed() {
        display.clear_console();
        render_objects(&mut display, &mut game);
        render_map(&mut display, &game);
        display.blit();  
        display.flush();
        previous_player_position = game.get_player_position();
        let exit = handle_keypress(&mut display, &mut game);
        if exit {
            break;
        }
    }
}


