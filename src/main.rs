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
    let mut _wave_function: WaveFunction =
        create_wave_function(tileset, shape.unwrap_or(DEFAULT_SHAPE));
    let mut _entropy_field: EntropyField =
        create_entropy_field(&_wave_function, shape.unwrap_or(DEFAULT_SHAPE));

    todo!(); // TODO: actually implement algorithm
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
    //TODO: depending on rules not every tile can be everywhere
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
    // TODO: create neighbour rules
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
