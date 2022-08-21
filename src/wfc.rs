use ndarray::{s, Array2, Array3, ArrayView1, ArrayView2};
use rand::{rngs::ThreadRng, seq::SliceRandom};
use raylib::prelude::*;

pub(crate) mod tileset;
use tileset::Tileset;

use crate::wfc::tileset::{WfcColor, TILE_SIZE};

type Coordinates = (usize, usize);
type Entropy = usize;
type EntropyField = Array2<Entropy>;
type WaveField = Array3<bool>;

pub struct WaveFunction {
    done: bool,
    tileset: Tileset,
    entropy_field: EntropyField,
    wave_field: WaveField,
    shape: [usize; 2],
}

impl WaveFunction {
    pub fn from_png(out_shape: (usize, usize), path: &str) -> Self {
        fn generate_wave_field((shape_x, shape_y): (usize, usize), tileset: &Tileset) -> WaveField {
            let num_tiles = tileset.len();
            let mut wave_field: WaveField = Array3::from_elem((shape_x, shape_y, num_tiles), true);

            for x in 0..shape_x {
                for y in 0..shape_y {
                    for tile in 0..num_tiles {
                        if x == 0 {
                            // left border
                            wave_field[[x, y, tile]] &= tileset[tile].is_left_border();
                        }
                        if x == shape_x - 1 {
                            // right border
                            wave_field[[x, y, tile]] &= tileset[tile].is_right_border();
                        }
                        if y == 0 {
                            // upper border
                            wave_field[[x, y, tile]] &= tileset[tile].is_upper_border();
                        }
                        if y == shape_y - 1 {
                            // lower border
                            wave_field[[x, y, tile]] &= tileset[tile].is_lower_border();
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

        let tileset: Tileset = Tileset::from_png(path);
        let wave_field: WaveField = generate_wave_field(out_shape, &tileset);
        let entropy_field: EntropyField = generate_entropy_field(&wave_field);
        let temp_shape = entropy_field.shape();
        let shape = [temp_shape[0], temp_shape[1]];

        Self {
            done: false,
            tileset,
            entropy_field,
            wave_field,
            shape,
        }
    }
    pub fn done(&self) -> bool {
        self.done
    }
    fn get_min_entropy(&mut self, rng: &mut ThreadRng) -> Option<Coordinates> {
        if self.done() {
            return None;
        }
        let min_opt = self.entropy_field.iter().filter(|&&x| x != 1).min();

        if min_opt.is_none() {
            self.done = true;
            return None;
        }

        let min = min_opt.unwrap();

        assert_ne!(*min, 0, "minimum entropy is 0");
        let res = *self
            .entropy_field
            .indexed_iter()
            .filter(|(_, v)| v == &min)
            .map(|v| v.0)
            .collect::<Vec<Coordinates>>()
            .choose(rng)
            .unwrap();

        Some(res)
    }
    pub fn print_tileset(&self) {
        self.tileset.print();
    }

    pub fn collapse(&mut self, rng: &mut ThreadRng) -> () {
        fn is_valid_coords(coords: [isize; 2], shape: &[usize]) -> bool {
            let x = coords[0] as usize;
            let y = coords[1] as usize;
            return x < shape[0] && y < shape[1];
        }

        let coords_opt: Option<Coordinates> = self.get_min_entropy(rng);
        if coords_opt.is_none() {
            return;
        }

        let (x, y): Coordinates = coords_opt.unwrap();
        let local_superposition: ArrayView1<bool> = self.wave_field.slice(s![x, y, ..]);

        let &chosen_tile = local_superposition
            .indexed_iter()
            .filter(|(_, &b)| b)
            .map(|(cord, _)| cord)
            .collect::<Vec<_>>()
            .choose(rng)
            .unwrap();
        for tile_id in 0..self.tileset.len() {
            if tile_id != chosen_tile {
                self.wave_field[[x, y, tile_id]] = false;
            }
        }

        self.entropy_field[[x, y]] = 1;

        let mut stack: Vec<(usize, usize)> = vec![(x, y)];
        let shape = self.shape;

        // push initial neighbours onto the stack
        {
            if is_valid_coords([x as isize + 1, y as isize], &shape) && !stack.contains(&(x + 1, y))
            {
                stack.push((x + 1, y));
            }
            if is_valid_coords([x as isize - 1, y as isize], &shape) && !stack.contains(&(x - 1, y))
            {
                stack.push((x - 1, y));
            }
            if is_valid_coords([x as isize, y as isize + 1], &shape) && !stack.contains(&(x, y + 1))
            {
                stack.push((x, y + 1));
            }
            if is_valid_coords([x as isize, y as isize - 1], &shape) && !stack.contains(&(x, y - 1))
            {
                stack.push((x, y - 1));
            }
        }

        while let Some((x, y)) = stack.pop() {
            let local_entropy = self.entropy_field[[x, y]];
            assert_ne!(local_entropy, 0, "entropy reached 0 at ({x}, {y})");
            let num_tiles = self.tileset.len();
            let mut local_superposition = vec![true; num_tiles];

            // calculate new `local_superposition`
            // go up
            if is_valid_coords([x as isize, y as isize - 1], &shape) {
                let mut temp_superposition = vec![false; num_tiles];
                for other_tile in 0..num_tiles {
                    if self.wave_field[[x, y - 1, other_tile]] {
                        for i in self.tileset[other_tile].get_down() {
                            if i.is_some() {
                                temp_superposition[i.unwrap()] = true;
                            }
                        }
                    }
                }
                for i in 0..num_tiles {
                    local_superposition[i] &= temp_superposition[i];
                }
            }

            // go down
            if is_valid_coords([x as isize, y as isize + 1], &shape) {
                let mut temp_superposition = vec![false; num_tiles];
                for other_tile in 0..num_tiles {
                    if self.wave_field[[x, y + 1, other_tile]] {
                        for i in self.tileset[other_tile].get_up() {
                            if i.is_some() {
                                temp_superposition[i.unwrap()] = true;
                            }
                        }
                    }
                }
                for i in 0..num_tiles {
                    local_superposition[i] &= temp_superposition[i];
                }
            }

            // go left
            if is_valid_coords([x as isize - 1, y as isize], &shape) {
                let mut temp_superposition = vec![false; num_tiles];
                for other_tile in 0..num_tiles {
                    if self.wave_field[[x - 1, y, other_tile]] {
                        for i in self.tileset[other_tile].get_right() {
                            if i.is_some() {
                                temp_superposition[i.unwrap()] = true;
                            }
                        }
                    }
                }
                for i in 0..num_tiles {
                    local_superposition[i] &= temp_superposition[i];
                }
            }

            // go right
            if is_valid_coords([x as isize + 1, y as isize], &shape) {
                let mut temp_superposition = vec![false; num_tiles];
                for other_tile in 0..num_tiles {
                    if self.wave_field[[x + 1, y, other_tile]] {
                        for i in self.tileset[other_tile].get_left() {
                            if i.is_some() {
                                temp_superposition[i.unwrap()] = true;
                            }
                        }
                    }
                }
                for i in 0..num_tiles {
                    local_superposition[i] &= temp_superposition[i];
                }
            }

            // if any changes happened add neighbours to stack
            let mut changes = false;
            for i in 0..num_tiles {
                if !local_superposition[i] && self.wave_field[[x, y, i]] {
                    changes = true;
                    break;
                }
            }
            if changes {
                if is_valid_coords([x as isize + 1, y as isize], &shape)
                    && !stack.contains(&(x + 1, y))
                {
                    stack.push((x + 1, y));
                }
                if is_valid_coords([x as isize - 1, y as isize], &shape)
                    && !stack.contains(&(x - 1, y))
                {
                    stack.push((x - 1, y));
                }
                if is_valid_coords([x as isize, y as isize + 1], &shape)
                    && !stack.contains(&(x, y + 1))
                {
                    stack.push((x, y + 1));
                }
                if is_valid_coords([x as isize, y as isize - 1], &shape)
                    && !stack.contains(&(x, y - 1))
                {
                    stack.push((x, y - 1));
                }
            }
            // update `wave_field` and `entropy_field`
            if changes {
                let mut count = 0;
                for i in 0..num_tiles {
                    self.wave_field[[x, y, i]] &= local_superposition[i];
                    if self.wave_field[[x, y, i]] {
                        count += 1;
                    }
                }
                self.entropy_field[[x, y]] = count;
            }
        }
    }

    pub fn show(&self, rl: &mut RaylibHandle, thread: &RaylibThread, scale: usize) -> () {
        fn convert_coords(
            wave_function_coords: (usize, usize),
            tile_coords: (usize, usize),
        ) -> (usize, usize) {
            let (wf_x, wf_y) = wave_function_coords;
            let (tile_x, tile_y) = tile_coords;

            (TILE_SIZE * wf_x + tile_x, TILE_SIZE * wf_y + tile_y)
        }

        fn wfc_color_to_rl_color(wfc_color: WfcColor) -> Color {
            let (r, g, b) = wfc_color.get();
            Color::new(r, g, b, 255)
        }
        let mut draw_handle = rl.begin_drawing(thread);

        draw_handle.clear_background(Color::GRAY);

        for x in 0..self.shape[0] {
            for y in 0..self.shape[1] {
                if self.entropy_field[[x, y]] == 1 {
                    let tile_id = self
                        .wave_field
                        .slice(s![x, y, ..])
                        .indexed_iter()
                        .filter(|(_, &b)| b)
                        .map(|(id, _)| id)
                        .collect::<Vec<usize>>()[0];
                    let tile = &self.tileset[tile_id];
                    let tile_im: ArrayView2<WfcColor> = tile.get_image();
                    for tile_x in 0..TILE_SIZE {
                        for tile_y in 0..TILE_SIZE {
                            let (canvas_x, canvas_y) = convert_coords((x, y), (tile_x, tile_y));
                            for draw_x in 0..scale {
                                for draw_y in 0..scale {
                                    draw_handle.draw_pixel(
                                        (canvas_y * scale + draw_y) as i32,
                                        (canvas_x * scale + draw_x) as i32,
                                        // yes i know x and y are mixxed up, but turns out i'm stupid and this is the easiest fix :)
                                        wfc_color_to_rl_color(tile_im[[tile_x, tile_y]]),
                                    )
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
