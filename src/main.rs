extern crate ndarray;
extern crate rand;
extern crate raylib;

mod wave_funcion_collapse {
    use std::ops::Index;

    use ndarray::{s, Array2, Array3};
    use rand::{rngs::ThreadRng, seq::SliceRandom};
    use raylib::prelude::*;

    type Coordinates = (usize, usize);
    type Entropy = usize;
    type EntropyField = Array2<Entropy>;
    type WaveField = Array3<bool>;

    struct Tile {
        left: Vec<Tile>,
        right: Vec<Tile>,
        up: Vec<Tile>,
        down: Vec<Tile>,
    }

    struct Tileset {
        tiles: Vec<Tile>,
    }
    impl Index<usize> for Tileset {
        type Output = Tile;

        fn index(&self, index: usize) -> &Self::Output {
            &self.tiles[index]
        }
    }
    impl Tileset {
        fn from_file() -> Self {
            todo!() // TODO: implement from_file for Tileset
        }

        fn len(&self) -> usize {
            self.tiles.len()
        }
    }

    fn generate_wave_field((shape_x, shape_y): (usize, usize), tileset: &Tileset) -> WaveField {
        let num_tiles = tileset.len();
        let mut wave_field: WaveField = Array3::from_elem((shape_x, shape_y, num_tiles), true);

        for x in 0..shape_x {
            for y in 0..shape_y {
                for tile in 0..num_tiles {
                    if x != 0 {
                        // no valid left tile
                        if tileset[tile].left.is_empty() {
                            wave_field[[x, y, tile]] = false;
                        }
                    }
                    if x != shape_x - 1 {
                        // no valid right tile
                        if tileset[tile].right.is_empty() {
                            wave_field[[x, y, tile]] = false;
                        }
                    }
                    if y != 0 {
                        // no valid up tile
                        if tileset[tile].up.is_empty() {
                            wave_field[[x, y, tile]] = false;
                        }
                    }
                    if y != shape_y - 1 {
                        // no valid down tile
                        if tileset[tile].down.is_empty() {
                            wave_field[[x, y, tile]] = false;
                        }
                    }
                }
            }
        }

        wave_field
    }
    fn generate_entropy_field(wave_field: &WaveField) -> EntropyField {
        fn entropy((x, y): (usize, usize), wave_field: &WaveField) -> Entropy {
            wave_field
                .slice(s![x, y, ..])
                .iter()
                .filter(|&&b| b)
                .count()
        }

        let wave_field_shape: &[usize] = wave_field.shape();
        let entropy_shape: (usize, usize) = (wave_field_shape[0], wave_field_shape[1]);
        let entropy_field: EntropyField =
            Array2::from_shape_fn(entropy_shape, |(x, y)| entropy((x, y), wave_field));
        entropy_field
    }

    pub struct WaveFunction {
        done: bool,
        tileset: Tileset,
        entropy_field: EntropyField,
        wave_field: WaveField,
    }

    impl WaveFunction {
        pub fn from_file(out_shape: (usize, usize)) -> Self {
            let tileset: Tileset = Tileset::from_file();
            let wave_field: WaveField = generate_wave_field(out_shape, &tileset);
            let entropy_field: EntropyField = generate_entropy_field(&wave_field);

            Self {
                done: false,
                tileset,
                entropy_field,
                wave_field,
            }
        }
        pub fn done(&self) -> bool {
            self.done
        }
        fn get_min_entropy(&self, rng: &mut ThreadRng) -> Coordinates {
            let min = self.entropy_field.iter().min().unwrap();
            *self
                .entropy_field
                .indexed_iter()
                .filter(|(_, v)| v == &min)
                .map(|v| v.0)
                .collect::<Vec<Coordinates>>()
                .choose(rng)
                .unwrap()
        }

        pub fn collapse(&self, rng: &mut ThreadRng) -> () {
            if self.done {
                return;
            }
            let coords: Coordinates = self.get_min_entropy(rng);
            todo!() // TODO: implement collapse function
        }

        pub fn show(&self, rl: &mut RaylibHandle, thread: &RaylibThread) -> () {
            todo!() // TODO: implement show function
        }
    }
}

use rand::{rngs::ThreadRng, thread_rng};
use raylib::prelude::*;
use std::{collections::HashMap, env};

use wave_funcion_collapse::WaveFunction;

type Config = HashMap<String, bool>;

fn get_config() -> Config {
    let mut config: Config = HashMap::new();
    let args = env::args().collect::<Vec<String>>();
    config.insert(
        "debug".to_string(),
        args.iter().any(|s| s.eq("-D") || s.eq("--debug")),
    );
    config.insert(
        "animated".to_string(),
        args.iter().any(|s| s.eq("-A") || s.eq("--animated")),
    );
    config
}

fn main() {
    const OUTPUT_SIZE: (usize, usize) = (10, 10);

    let mut rng: ThreadRng = thread_rng();
    let wave_function = WaveFunction::from_file(OUTPUT_SIZE);
    let config: Config = get_config();

    let &debug = config.get("debug").unwrap_or(&false);
    let &animated = config.get("animated").unwrap_or(&false);

    if animated || debug {
        // FIXME: split animated ad not in a nicer way
        let (mut rl, thread): (RaylibHandle, RaylibThread) = raylib::init()
            .size((200) as i32, (200) as i32)
            .title("Non-Tiling WFC")
            .build();

        while !wave_function.done() {
            wave_function.show(&mut rl, &thread);
            wave_function.collapse(&mut rng);
        }
        wave_function.show(&mut rl, &thread);
    } else {
        while !wave_function.done() {
            wave_function.collapse(&mut rng);
        }
    }
}
