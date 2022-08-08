use std::{
    collections::{btree_map::Values, hash_map, HashMap},
    env,
};

use ndarray::{arr2, s, Array, Array2, Array3};

type TileID = usize;
type Image = Array2<u32>;
type Wave = Array3<bool>;
type Entropy = usize;

const DEFAULT_SHAPE: (usize, usize) = (200, 200);

struct TileSet {
    tiles: HashMap<TileID, Tile>,
}

impl TileSet {
    fn new() -> TileSet {
        TileSet {
            tiles: HashMap::new(),
        }
    }

    fn insert(self: &mut TileSet, id: TileID, tile: Tile) -> Option<Tile> {
        self.tiles.insert(id, tile)
    }

    fn values(self: &TileSet) -> hash_map::Values<'_, TileID, Tile> {
        self.tiles.values()
    }

    fn get(self: &TileSet, id: &TileID) -> Option<&Tile> {
        self.tiles.get(id)
    }

    fn len(self: &TileSet) -> usize {
        self.tiles.len()
    }
}

#[derive(Hash)]
struct Tile {
    id: TileID,
    tile: Array2<u32>,
    l: Vec<TileID>,
    r: Vec<TileID>,
    u: Vec<TileID>,
    d: Vec<TileID>,
}

impl Tile {
    fn left(self: &Tile, id: TileID) -> bool {
        self.l.contains(&id)
    }

    fn right(self: &Tile, id: TileID) -> bool {
        self.r.contains(&id)
    }

    fn up(self: &Tile, id: TileID) -> bool {
        self.u.contains(&id)
    }

    fn down(self: &Tile, id: TileID) -> bool {
        self.d.contains(&id)
    }

    fn new(id: TileID, tile: Array2<u32>) -> Tile {
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
    let (debug, animated): (bool, bool) = parse_args();

    let tileset: TileSet = get_tiles(get_input());

    let mut res: Option<Image> = None;
    while res.is_none() {
        if animated || debug {
            res = wfc_from_tileset_animated(&tileset, None);
        } else {
            res = wfc_from_tileset(&tileset, None);
        }
    }
    let out_image: Image = res.unwrap();

    // FIXME: show out_image
}

fn parse_args() -> (bool, bool) {
    let args: Vec<String> = env::args().collect();

    let debug: bool = args.iter().any(|s| s.eq("--debug") || s.eq("-D"));
    let animated: bool = args.iter().any(|s| s.eq("--animated") || s.eq("-A"));

    (debug, animated)
}

fn wfc_from_tileset_animated(tileset: &TileSet, shape: Option<(usize, usize)>) -> Option<Image> {
    println!("for now animation not supported"); // FIXME: implement animation
    wfc_from_tileset(tileset, shape)
}

fn wfc_from_tileset(tileset: &TileSet, shape: Option<(usize, usize)>) -> Option<Image> {
    let mut wave: Wave = create_wave(tileset, shape.unwrap_or(DEFAULT_SHAPE));
    let mut entropy: Array2<Entropy> = create_entropy(tileset, shape.unwrap_or(DEFAULT_SHAPE));

    todo!(); // TODO: actually implement algorithm
}

fn create_entropy(tileset: &TileSet, shape: (usize, usize)) -> Array2<Entropy> {
    Array2::from_elem(shape, tileset.len())
}

fn create_wave(tileset: &TileSet, shape: (usize, usize)) -> Wave {
    let dim: (usize, usize, usize) = (shape.0, shape.1, tileset.len());
    Array3::from_elem(dim, true)
}

fn get_tiles(image: Image) -> TileSet {
    let shape: &[usize] = image.shape();
    assert_eq!(shape.len(), 2);
    assert!(shape[0] >= 4 && shape[1] >= 4);

    let mut tileset: TileSet = TileSet::new();

    let mut id: TileID = 0;
    for x in (0..shape[0]).step_by(2) {
        for y in (0..shape[1]).step_by(2) {
            let slice: Array2<u32> = image.clone().slice_move(s![x..x + 2, y..y + 2]);
            if !tileset.values().any(|t| t.tile == slice) {
                tileset.insert(id, Tile::new(id, slice));
                id += 1;
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
