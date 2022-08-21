use rand::{rngs::ThreadRng, thread_rng};
use raylib::prelude::*;
use std::{collections::HashMap, env};

mod wfc;
use wfc::WaveFunction;

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
    config.insert(
        "testing".to_string(),
        args.iter().any(|s| s.eq("-T") || s.eq("--testing")),
    );
    config
}

const OUTPUT_SHAPE: (usize, usize) = (10, 10);
const INPUT_PATH: &str = "images/green_cross.png";
fn main() {
    let rng: ThreadRng = thread_rng();
    let wave_function: WaveFunction = WaveFunction::from_png(OUTPUT_SHAPE, INPUT_PATH);
    let config: Config = get_config();

    let &debug = config.get("debug").unwrap_or(&false);
    let &animated = config.get("animated").unwrap_or(&false);
    let &testing = config.get("testing").unwrap_or(&false);

    type Runner = fn(ThreadRng, WaveFunction) -> ();
    let runner: Runner = if testing {
        testing_runner
    } else if animated {
        animated_runner
    } else if debug {
        debug_runner
    } else {
        default_runner
    };

    runner(rng, wave_function);
}

fn default_runner(mut rng: ThreadRng, mut wave_function: WaveFunction) -> () {
    while !wave_function.done() {
        wave_function.collapse(&mut rng);
    }
}

#[allow(unused_mut)]
fn debug_runner(rng: ThreadRng, mut wave_function: WaveFunction) -> () {
    animated_runner(rng, wave_function)
}

fn animated_runner(mut rng: ThreadRng, mut wave_function: WaveFunction) -> () {
    use wfc::tileset::TILE_SIZE;
    const SHOW_SCALE: usize = 10;
    let canvas_shape = (
        OUTPUT_SHAPE.0 * TILE_SIZE * SHOW_SCALE,
        OUTPUT_SHAPE.1 * TILE_SIZE * SHOW_SCALE,
    );
    let (mut rl, thread): (RaylibHandle, RaylibThread) = raylib::init()
        .size((canvas_shape.0) as i32, (canvas_shape.1) as i32)
        .title("Non-Tiling WFC")
        .build();

    while !rl.window_should_close() {
        wave_function.show(&mut rl, &thread, SHOW_SCALE);
        if !wave_function.done() {
            wave_function.collapse(&mut rng);
        }
    }
}

#[allow(unused_mut)]
fn testing_runner(mut rng: ThreadRng, mut wave_function: WaveFunction) -> () {
    println!("testing");
    wave_function.print_tileset();
    wave_function.collapse(&mut rng)
}
