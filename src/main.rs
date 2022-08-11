extern crate ndarray;
extern crate rand;
extern crate raylib;

mod wave_funcion_collapse {
    use rand::rngs::ThreadRng;
    use raylib::prelude::*;

    pub type Coordinates = [usize; 2];
    pub struct WaveFunction {
        done: bool,
    }

    impl WaveFunction {
        pub fn from_file() -> Self {
            todo!()
        }

        pub fn done(&self) -> bool {
            self.done
        }

        fn get_min_entropy(&self, rng: &mut ThreadRng) -> Coordinates {
            todo!()
        }

        pub fn collapse(&self, rng: &mut ThreadRng) -> () {
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
