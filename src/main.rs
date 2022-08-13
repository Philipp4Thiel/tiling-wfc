extern crate ndarray;
extern crate rand;
extern crate raylib;

mod wave_funcion_collapse {
    use ndarray::{s, Array2, Array3};
    use rand::{rngs::ThreadRng, seq::SliceRandom};
    use raylib::prelude::*;

    type Coordinates = (usize, usize);
    type Entropy = usize;
    type EntropyField = Array2<Entropy>;
    type WaveField = Array3<bool>;

    struct Tileset {}

    impl Tileset {
        fn from_file() -> Self {
            todo!()
        }
    }

    fn generate_wave_field(tileset: &Tileset) -> WaveField {
        todo!()
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
        pub fn from_file() -> Self {
            let tileset: Tileset = Tileset::from_file();
            let wave_field: WaveField = generate_wave_field(&tileset);
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
                .filter(|(_, v)| *v == min)
                .map(|v| v.0)
                .collect::<Vec<(usize, usize)>>()
                .choose(rng)
                .unwrap()
        }

        pub fn collapse(&self, rng: &mut ThreadRng) -> () {
            if self.done {
                return;
            }
            let coords: Coordinates = self.get_min_entropy(rng);
            todo!()
        }

        pub fn show(&self, rl: &mut RaylibHandle, thread: &RaylibThread) -> () {
            todo!()
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
    let mut rng: ThreadRng = thread_rng();
    let wave_function = WaveFunction::from_file();
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
