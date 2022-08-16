mod wave_funcion_collapse {
    use std::{
        fmt::{Debug, Display},
        ops::Index,
    };

    use image::{io::Reader, DynamicImage};
    use ndarray::{s, Array2, Array3, ArrayView1};
    use rand::{rngs::ThreadRng, seq::SliceRandom};
    use raylib::prelude::*;

    type Coordinates = (usize, usize);
    type Entropy = usize;
    type EntropyField = Array2<Entropy>;
    type WaveField = Array3<bool>;

    #[derive(Clone, Copy)]
    struct Color(u8, u8, u8);

    impl Debug for Color {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if self.0 > self.1 && self.0 > self.2 {
                write!(f, "R")
            } else if self.1 > self.0 && self.1 > self.2 {
                write!(f, "G")
            } else if self.2 > self.1 && self.2 > self.0 {
                write!(f, "B")
            } else {
                write!(f, " ")
            }
        }
    }

    impl Color {
        fn from_ndarry_view(arr_view: ArrayView1<u8>) -> Self {
            Self(arr_view[0], arr_view[1], arr_view[2])
        }
    }

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
        fn from_file(path: &str) -> Self {
            fn get_image(path: &str) -> DynamicImage {
                let reader = match Reader::open(path) {
                    Ok(reader) => reader,
                    Err(_) => panic!("not able to open file"),
                };

                match reader.decode() {
                    Ok(image) => image,
                    Err(_) => panic!("not able to decode file"),
                }
            }
            fn get_image_as_array3((width, height): (u32, u32), im_data: &[u8]) -> Array3<u8> {
                assert_eq!(im_data.len() as u32, width * height * 3);
                // if this isn't the sae there was an error in decoding the image (3 because rgb)

                let shape: (usize, usize, usize) = (width as usize, height as usize, 3);
                let mut image: Array3<u8> = Array3::from_elem(shape, 0);

                for x in 0..width as usize {
                    for y in 0..height as usize {
                        for channel in 0..3 {
                            let index: usize = 3 * (height as usize) * x + 3 * y + channel;
                            image[[x, y, channel]] = im_data[index];
                        }
                    }
                }

                image
            }
            fn convert_array3u8_to_array2color(arr3: Array3<u8>) -> Array2<Color> {
                let shape: &[usize] = arr3.shape();
                let width: usize = shape[0];
                let height: usize = shape[1];
                let mut arr2: Array2<Color> = Array2::from_elem((width, height), Color(0, 0, 0));
                for x in 0..width {
                    for y in 0..height {
                        arr2[[x, y]] = Color::from_ndarry_view(arr3.slice(s![x, y, ..]));
                    }
                }
                arr2
            }

            let image: DynamicImage = get_image(path);
            let im_shape: (u32, u32) = (image.width(), image.height());

            let image_as_array3: Array3<u8> = get_image_as_array3(im_shape, image.as_bytes());
            let image: Array2<Color> = convert_array3u8_to_array2color(image_as_array3);

            println!("{:?}", image);
            // TODO: generte Tileset from Array2
            todo!()
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
        pub fn from_file(out_shape: (usize, usize), path: &str) -> Self {
            let tileset: Tileset = Tileset::from_file(path);
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
    const OUTPUT_SHAPE: (usize, usize) = (10, 10);
    const INPUT_PATH: &str = "images/green_cross.png";

    let mut rng: ThreadRng = thread_rng();
    let wave_function = WaveFunction::from_file(OUTPUT_SHAPE, INPUT_PATH);
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
