extern crate ndarray;
extern crate rand;

mod wave_funcion_collapse {
    use rand::rngs::ThreadRng;

    pub type Coordinates = [usize; 2];
    pub struct WaveFunction {
        pub done: bool,
    }

    impl WaveFunction {
        pub fn from_file() -> Self {
            todo!()
        }

        fn get_min_entropy(&self, rng: &mut ThreadRng) -> Coordinates {
            todo!()
        }

        pub fn collapse(&self, rng: &mut ThreadRng) -> () {
            let coords: Coordinates = self.get_min_entropy(rng);
            todo!()
        }

        pub fn show(&self) -> () {
            todo!()
        }
    }
}

use std::{collections::HashMap, env};

use rand::{rngs::ThreadRng, thread_rng};

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
    

    while !wave_function.done {
        if animated || debug {
            wave_function.show();
        }
        wave_function.collapse(&mut rng);
    }
    wave_function.show();
}
