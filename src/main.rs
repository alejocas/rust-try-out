use tcod::colors::*;
use tcod::console::*;

const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
const COLOR_DARK_GROUND: Color = Color {
    r: 50,
    g: 50,
    b: 50
};

const MAP_WIDTH: i32 = 40;
const MAP_HEIGHT: i32 = 20;

// Actual size of the window
const SCREEN_WIDTH: i32 = 40;
const SCREEN_HEIGHT: i32 = 25;

const LIMIT_FPS: i32 = 20; // 20 frames per second as maximum

#[derive(Debug)]
struct RogueObject {
    x: i32,
    y: i32,
    char: char,
    color: Color
}

impl RogueObject {
    pub fn new(x: i32, y: i32, char: char, color: Color) -> Self {
        RogueObject { x, y, char, color }
    }

    pub fn move_by(&mut self, dx: i32, dy: i32, game: &Game) {
        if !game.map[(self.x + dx) as usize][(self.y + dy) as usize].blocked {
            self.x += dx;
            self.y += dy;
        }
    }

    pub fn draw(&self, con: &mut dyn Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }
}

struct Tcod {
    root: Root,
    con: Offscreen
}

// A tile of the map and its properties
#[derive(Clone, Copy, Debug)]
struct Tile {
    blocked: bool,
    block_sight: bool
}

impl Tile {
    pub fn empty() -> Self {
        Tile {
            blocked: false,
            block_sight: false
        }
    }

    pub fn wall() -> Self {
        Tile {
            blocked: true,
            block_sight: true
        }
    }
}

type Map = Vec<Vec<Tile>>;

struct Game {
    map: Map
}

fn handle_keys(tcod: &mut Tcod, game: &Game, player: &mut RogueObject) -> bool {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;

    let key = tcod.root.wait_for_keypress(true);
    match key {
        // Movement keys
        Key { code: Up, .. } => player.move_by(0, -1, game),
        Key { code: Down, .. } => player.move_by(0, 1, game),
        Key { code: Left, .. } => player.move_by(-1, 0, game),
        Key { code: Right, .. } => player.move_by(1, 0, game),
        // Game options keys
        Key {
            code: Enter,
            alt: true,
            ..
        } => {
            let fullscreen: bool = tcod.root.is_fullscreen();
            tcod.root.set_fullscreen(!fullscreen);
        },
        Key { code: Escape, .. } => return true,
        _ => {}
    }
    false
}

fn make_map() -> Map {
    let mut map = vec![vec![Tile::empty(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];
    map[2][2] = Tile::wall();
    map[3][2] = Tile::wall();
    map[4][2] = Tile::wall();
    map[5][2] = Tile::wall();
    map[6][2] = Tile::wall();
    map[7][2] = Tile::wall();
    map[8][2] = Tile::wall();
    map[9][2] = Tile::wall();
    map[10][2] = Tile::wall();
    map[11][2] = Tile::wall();
    map[12][2] = Tile::wall();

    map[2][3] = Tile::wall();
    map[10][3] = Tile::wall();
    map[12][3] = Tile::wall();

    map[2][4] = Tile::wall();
    map[4][4] = Tile::wall();
    map[6][4] = Tile::wall();
    map[8][4] = Tile::wall();
    map[9][4] = Tile::wall();
    map[10][4] = Tile::wall();
    map[12][4] = Tile::wall();

    map[2][5] = Tile::wall();
    map[4][5] = Tile::wall();
    map[6][5] = Tile::wall();
    map[8][5] = Tile::wall();
    map[12][5] = Tile::wall();

    map[2][6] = Tile::wall();
    map[4][6] = Tile::wall();
    map[6][6] = Tile::wall();
    map[7][6] = Tile::wall();
    map[8][6] = Tile::wall();
    map[10][6] = Tile::wall();
    map[11][6] = Tile::wall();
    map[12][6] = Tile::wall();

    map[4][7] = Tile::wall();
    map[8][7] = Tile::wall();

    map[2][8] = Tile::wall();
    map[4][8] = Tile::wall();
    map[5][8] = Tile::wall();
    map[6][8] = Tile::wall();
    map[8][8] = Tile::wall();
    map[9][8] = Tile::wall();
    map[10][8] = Tile::wall();
    map[12][8] = Tile::wall();

    map[2][9] = Tile::wall();
    map[4][9] = Tile::wall();
    map[6][9] = Tile::wall();
    map[10][9] = Tile::wall();
    map[12][9] = Tile::wall();

    map[2][10] = Tile::wall();
    map[4][10] = Tile::wall();
    map[6][10] = Tile::wall();
    map[7][10] = Tile::wall();
    map[8][10] = Tile::wall();
    map[10][10] = Tile::wall();
    map[12][10] = Tile::wall();

    map[2][11] = Tile::wall();
    map[8][11] = Tile::wall();
    map[12][11] = Tile::wall();

    map[2][12] = Tile::wall();
    map[3][12] = Tile::wall();
    map[4][12] = Tile::wall();
    map[5][12] = Tile::wall();
    map[6][12] = Tile::wall();
    map[7][12] = Tile::wall();
    map[8][12] = Tile::wall();
    map[9][12] = Tile::wall();
    map[10][12] = Tile::wall();
    map[11][12] = Tile::wall();
    map[12][12] = Tile::wall();
    map
}

fn render_all(tcod: &mut Tcod, game: &Game, objects: &[RogueObject]) {
    for object in objects {
        object.draw(&mut tcod.con);
    }
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let wall = game.map[x as usize][y as usize].block_sight;
            if wall {
                tcod.con.set_char_background(x, y, COLOR_DARK_WALL, BackgroundFlag::Set);
            } else {
                tcod.con.set_char_background(x, y, COLOR_DARK_GROUND, BackgroundFlag::Set);
            }
        }
    }
}

fn main() {
    tcod::system::set_fps(LIMIT_FPS);

    let root: Root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rust - Roguelike game for [UdeA] OS course")
        .init();

    let con = Offscreen::new(MAP_WIDTH, MAP_HEIGHT);

    let mut tcod: Tcod = Tcod { root, con };

    let player = RogueObject::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, '@', WHITE);
    let npc = RogueObject::new(SCREEN_WIDTH / 2 + 1, SCREEN_HEIGHT / 2 + 1, '@', YELLOW);
    let mut objects = [player, npc];
    let game = Game {
        map: make_map()
    };
    while !tcod.root.window_closed() {
        tcod.con.clear();
        render_all(&mut tcod, &game, &objects);
        blit(
            &tcod.con,
            (0, 0),
            (MAP_WIDTH, MAP_HEIGHT),
            &mut tcod.root,
            (0, 0),
            1.0,
            1.0
        );
        tcod.root.flush();
        let mut_player = &mut objects[0];
        let exit = handle_keys(&mut tcod, &game, mut_player);
        if exit {
            break;
        }
    }
}
