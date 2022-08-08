use std::{collections::HashSet, env};

use ndarray::{arr2, Array2, Array3};

type Tile = Array2<u32>;
type TileSet = HashSet<Tile>;
type Image = Array2<u32>;
type Wave = Array3<bool>;

fn main() {
    let (debug, animated): (bool, bool) = parse_args();

    let tileset: TileSet = get_tiles(get_input());
    
    let mut res: Option<Image> = None;
    while res.is_none() {
        if animated {
            res = wfc_animated(&tileset);
        } else {
            res = wfc_from_tileset(&tileset);
        }
    }
    let out_image: Image = res.unwrap();
    
    // TODO: show out_image
}

fn parse_args() -> (bool, bool) {
    let args: Vec<String> = env::args().collect();

    let debug: bool = args.iter().any(|s| s.eq("--debug") || s.eq("-D"));
    let animated: bool = debug || args.iter().any(|s| s.eq("--animated") || s.eq("-A"));

    (debug, animated)
}

fn wfc_animated(tileset: &TileSet) -> Option<Image> {
    wfc_from_tileset(tileset)
}

fn wfc_from_tileset(tileset: &TileSet) -> Option<Image> {
    fn create_wave(tileset: &TileSet) -> Wave {
        todo!();
    }

    let mut wave: Wave = create_wave(tileset);

    todo!();
}

fn get_tiles(image: Image) -> TileSet {
    // TODO: extract tileset from input image
    todo!()
}

fn get_input() -> Image {
    // FIXME: use png as input instead of hardcoding

    const RED: u32 = 0xFF0000FF;
    const LIME: u32 = 0x00FF00FF;
    const BLUE: u32 = 0x0000FF00;

    arr2(&[
        [BLUE, BLUE, BLUE, BLUE, RED, RED, BLUE, BLUE, BLUE, BLUE],
        [BLUE, BLUE, BLUE, BLUE, RED, RED, BLUE, BLUE, BLUE, BLUE],
        [BLUE, BLUE, BLUE, BLUE, RED, RED, BLUE, BLUE, BLUE, BLUE],
        [BLUE, BLUE, BLUE, BLUE, RED, RED, BLUE, BLUE, BLUE, BLUE],
        [RED, RED, RED, RED, LIME, LIME, RED, RED, RED, RED],
        [RED, RED, RED, RED, LIME, LIME, RED, RED, RED, RED],
        [BLUE, BLUE, BLUE, BLUE, RED, RED, BLUE, BLUE, BLUE, BLUE],
        [BLUE, BLUE, BLUE, BLUE, RED, RED, BLUE, BLUE, BLUE, BLUE],
        [BLUE, BLUE, BLUE, BLUE, RED, RED, BLUE, BLUE, BLUE, BLUE],
        [BLUE, BLUE, BLUE, BLUE, RED, RED, BLUE, BLUE, BLUE, BLUE],
    ])
}
