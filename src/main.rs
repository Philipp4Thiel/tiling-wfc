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

const OUTPUT_SHAPE: (usize, usize) = (100, 100);
const SHOW_SCALE: usize = 2; // only needed if using animated tag
const INPUT_PATH: &str = "images/house.png"; // yes why use variable input when you can hardcode it :5head:
fn main() {
    let config: Config = get_config();

    let &debug = config.get("debug").unwrap_or(&false);
    let &animated = config.get("animated").unwrap_or(&false);
    let &testing = config.get("testing").unwrap_or(&false);

    type Runner = fn() -> WaveFunction;
    let runner: Runner = if testing {
        testing_runner
    } else if animated {
        animated_runner
    } else if debug {
        debug_runner
    } else {
        default_runner
    };
    let _res = runner();
    // FIXME: create png from `_res`
}

fn default_runner() -> WaveFunction {
    let mut res: Result<(), ()> = Err(());
    let mut wave_function: WaveFunction = WaveFunction::from_png(OUTPUT_SHAPE, INPUT_PATH);
    let mut rng: ThreadRng = thread_rng();
    let mut counter = 1;
    while let Err(_) = res {
        println!("try: {counter}");
        counter += 1;

        wave_function = WaveFunction::from_png(OUTPUT_SHAPE, INPUT_PATH);
        while !wave_function.done() {
            res = wave_function.collapse(&mut rng);
            if res.is_err() {
                break;
            }
        }
    }
    wave_function
}

#[allow(unused_mut)]
fn debug_runner() -> WaveFunction {
    animated_runner()
}

fn animated_runner() -> WaveFunction {
    use wfc::tileset::TILE_SIZE;
    let canvas_shape = (
        OUTPUT_SHAPE.0 * TILE_SIZE * SHOW_SCALE,
        OUTPUT_SHAPE.1 * TILE_SIZE * SHOW_SCALE,
    );
    let (mut rl, thread): (RaylibHandle, RaylibThread) = raylib::init()
        .size((canvas_shape.0) as i32, (canvas_shape.1) as i32)
        .title("Non-Tiling WFC")
        .build();

    let mut res: Result<(), ()> = Err(());
    let mut wave_function: WaveFunction = WaveFunction::from_png(OUTPUT_SHAPE, INPUT_PATH);
    let mut rng: ThreadRng = thread_rng();

    let mut counter = 1;

    while !rl.window_should_close() {
        if rl.is_key_down(KeyboardKey::KEY_SPACE) {
            break;
        }
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::GRAY);
    }

    while let Err(_) = res {
        println!("try: {counter}");
        counter += 1;

        wave_function = WaveFunction::from_png(OUTPUT_SHAPE, INPUT_PATH);

        while !wave_function.done() {
            if rl.window_should_close() {
                panic!("window was closed");
            }
            wave_function.show(&mut rl, &thread, SHOW_SCALE);
            res = wave_function.collapse(&mut rng);
            if res.is_err() {
                break;
            }
        }
    }

    while !rl.window_should_close() {
        wave_function.show(&mut rl, &thread, SHOW_SCALE);
    }

    wave_function
}

#[allow(unused_mut)]
fn testing_runner() -> WaveFunction {
    println!("testing");
    let mut wave_function: WaveFunction = WaveFunction::from_png(OUTPUT_SHAPE, INPUT_PATH);
    let mut rng: ThreadRng = thread_rng();
    wave_function.print_tileset();
    let _res = wave_function.collapse(&mut rng);
    wave_function
}
