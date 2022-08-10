use std::{collections::HashMap, env};

use ndarray::{arr2, s, Array2, Array3};

const DEFAULT_SHAPE: (usize, usize) = (200, 200);

type TileId = usize;
type Image = Array2<u32>;
type WaveFunction = Array3<bool>;
type Entropy = u8;
type EntropyField = Array2<Entropy>;
type Config = HashMap<String, bool>;

struct TileSet {
    tileset: Vec<Tile>,
}
impl TileSet {
    fn new() -> TileSet {
        TileSet {
            tileset: Vec::new(),
        }
    }

    fn new_tile(self: &mut TileSet, image: Image) -> () {
        let tile: Tile = Tile::new(self.len(), image);
        self.tileset.push(tile)
    }

    fn iter(self: &TileSet) -> std::slice::Iter<Tile> {
        self.tileset.iter()
    }

    fn get(self: &TileSet, id: TileId) -> Option<&Tile> {
        if id < self.len() {
            Some(&self.tileset[id])
        } else {
            None
        }
    }

    fn len(self: &TileSet) -> usize {
        self.tileset.len()
    }

    fn find(self: &TileSet, tile: Image) -> Option<TileId> {
        for t in &self.tileset {
            if t.tile == tile {
                return Some(t.id);
            }
        }
        None
    }

    fn add_up(self: &mut TileSet, id1: TileId, id2: TileId) -> () {
        self.tileset[id1].u.push(id2);
        self.tileset[id2].d.push(id1);
    }
    fn add_down(self: &mut TileSet, id1: TileId, id2: TileId) -> () {
        self.add_up(id2, id1)
    }
    fn add_left(self: &mut TileSet, id1: TileId, id2: TileId) -> () {
        self.tileset[id1].l.push(id2);
        self.tileset[id2].r.push(id1);
    }
    fn add_right(self: &mut TileSet, id1: TileId, id2: TileId) -> () {
        self.add_left(id2, id1)
    }
}

#[derive(Clone)]
struct Tile {
    id: TileId,
    tile: Image,
    l: Vec<TileId>,
    r: Vec<TileId>,
    u: Vec<TileId>,
    d: Vec<TileId>,
}
impl Tile {
    fn left(self: &Tile, id: TileId) -> bool {
        self.l.contains(&id)
    }

    fn right(self: &Tile, id: TileId) -> bool {
        self.r.contains(&id)
    }

    fn up(self: &Tile, id: TileId) -> bool {
        self.u.contains(&id)
    }

    fn down(self: &Tile, id: TileId) -> bool {
        self.d.contains(&id)
    }

    fn new(id: TileId, tile: Image) -> Tile {
        Tile {
            id,
            tile,
            l: Vec::new(),
            r: Vec::new(),
            u: Vec::new(),
            d: Vec::new(),
        }
    }
}

fn main() {
    let config: Config = parse_args();
    let &debug = config.get(&"debug".to_string()).unwrap_or(&false);
    let &animated = config.get(&"animated".to_string()).unwrap_or(&false);

    let tileset: TileSet = get_tiles(get_input());

    let mut res: Option<Image> = None;
    while res.is_none() {
        if animated || debug {
            res = wfc_from_tileset_animated(&tileset, None);
        } else {
            res = wfc_from_tileset(&tileset, None);
        }
    }
    let _out_image: Image = res.unwrap();

    // FIXME: show out_image
}

fn parse_args() -> Config {
    let args: Vec<String> = env::args().collect();

    let debug: bool = args.iter().any(|s| s.eq("--debug") || s.eq("-D"));
    let animated: bool = args.iter().any(|s| s.eq("--animated") || s.eq("-A"));

    let mut config: Config = HashMap::new();

    config.insert("debug".to_string(), debug);
    config.insert("animated".to_string(), animated);

    config
}

fn wfc_from_tileset_animated(tileset: &TileSet, shape: Option<(usize, usize)>) -> Option<Image> {
    println!("for now animation not supported"); // FIXME: implement animation
    wfc_from_tileset(tileset, shape)
}

fn wfc_from_tileset(tileset: &TileSet, shape: Option<(usize, usize)>) -> Option<Image> {
    let mut wave_function: WaveFunction =
        create_wave_function(tileset, shape.unwrap_or(DEFAULT_SHAPE));
    let mut entropy_field: EntropyField =
        create_entropy_field(&wave_function, shape.unwrap_or(DEFAULT_SHAPE));

    let mut done: bool = false;
    while !done {
        done = wfc_step(&mut wave_function, &mut entropy_field, tileset);
    }

    create_image(&wave_function, &tileset)
}

fn create_image(wave_function: &WaveFunction, tileset: &TileSet) -> Option<Image> {
    // TODO: create image
    None
}

fn wfc_step(
    wave_function: &mut WaveFunction,
    entropy_field: &mut EntropyField,
    tileset: &TileSet,
) -> bool {
    // TODO: find min entropy
    // TODO: choose random tile
    // TODO: collapse the wavefunction
    // TODO: check if done or not
    true
}

fn create_entropy_field(wave_function: &WaveFunction, shape: (usize, usize)) -> EntropyField {
    let mut entropy: EntropyField = Array2::zeros(shape);
    for x in 0..shape.0 {
        for y in 0..shape.1 {
            entropy[[x, y]] =
                wave_function
                    .slice(s![x, y, ..])
                    .iter()
                    .fold(0, |n, &b| if b { n + 1 } else { n })
        }
    }
    entropy
}

fn create_wave_function(tileset: &TileSet, shape: (usize, usize)) -> WaveFunction {
    let dim: (usize, usize, usize) = (shape.0, shape.1, tileset.len());
    let wave_function: WaveFunction = Array3::from_elem(dim, true);
    //FIXME: depending on rules not every tile can be everywhere
    wave_function
}

fn get_tiles(image: Image) -> TileSet {
    let shape: &[usize] = image.shape();
    assert_eq!(shape.len(), 2);
    assert!(shape[0] >= 4 && shape[1] >= 4);

    let mut tileset: TileSet = TileSet::new();

    for x in (0..shape[0]).step_by(2) {
        for y in (0..shape[1]).step_by(2) {
            let slice: Image = image.clone().slice_move(s![x..x + 2, y..y + 2]);
            if !tileset.iter().any(|t| t.tile == slice) {
                tileset.new_tile(slice);
            }
        }
    }

    fn valid_coords(x: i32, y: i32, shape: (i32, i32)) -> bool {
        0 <= x && x < shape.0 - 2 && 0 <= y && y < shape.1 - 2
    }

    for x in (0..shape[0]).step_by(2) {
        for y in (0..shape[1]).step_by(2) {
            let slice: Image = image.clone().slice_move(s![x..x + 2, y..y + 2]);
            let base_id: TileId = tileset.find(slice).unwrap();

            // check up
            if valid_coords(x as i32, y as i32 - 2, (shape[0] as i32, shape[1] as i32)) {
                let up_slice = image.clone().slice_move(s![x..x + 2, y - 2..y]);
                let up_id: TileId = tileset.find(up_slice).unwrap();
                tileset.add_up(base_id, up_id);
            }

            // check down
            if valid_coords(x as i32, y as i32 + 2, (shape[0] as i32, shape[1] as i32)) {
                let down_slice = image.clone().slice_move(s![x..x + 2, y + 2..y + 4]);
                let down_id: TileId = tileset.find(down_slice).unwrap();
                tileset.add_down(base_id, down_id);
            }

            // check left
            if valid_coords(x as i32 - 2, y as i32, (shape[0] as i32, shape[1] as i32)) {
                let left_slice = image.clone().slice_move(s![x - 2..x, y..y + 2]);
                let left_id: TileId = tileset.find(left_slice).unwrap();
                tileset.add_left(base_id, left_id);
            }

            // check right
            if valid_coords(x as i32 + 2, y as i32, (shape[0] as i32, shape[1] as i32)) {
                let right_slice = image.clone().slice_move(s![x + 2..x + 4, y..y + 2]);
                let right_id: TileId = tileset.find(right_slice).unwrap();
                tileset.add_right(base_id, right_id);
            }
        }
    }

    tileset
}

fn get_input() -> Image {
    // FIXME: use png as input

    const R: u32 = 0xFF0000FF;
    const L: u32 = 0x00FF00FF;
    const B: u32 = 0x0000FFFF;

    arr2(&[
        [B, B, B, B, R, R, B, B, B, B],
        [B, B, B, B, R, R, B, B, B, B],
        [B, B, B, B, R, R, B, B, B, B],
        [B, B, B, B, R, R, B, B, B, B],
        [R, R, R, R, L, L, R, R, R, R],
        [R, R, R, R, L, L, R, R, R, R],
        [B, B, B, B, R, R, B, B, B, B],
        [B, B, B, B, R, R, B, B, B, B],
        [B, B, B, B, R, R, B, B, B, B],
        [B, B, B, B, R, R, B, B, B, B],
    ])
}
