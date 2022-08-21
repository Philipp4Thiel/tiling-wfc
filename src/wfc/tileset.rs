pub(crate) use wfc_color::WfcColor;

use image::{io::Reader, DynamicImage};
use ndarray::{s, Array2, Array3, ArrayView2};

use std::ops::{Index, IndexMut};

pub(crate) const TILE_SIZE: usize = 3;

#[derive(Clone)]
pub(crate) struct Tile {
    image: Array2<WfcColor>,
    left: Vec<Option<usize>>,
    right: Vec<Option<usize>>,
    up: Vec<Option<usize>>,
    down: Vec<Option<usize>>,
}
impl Tile {
    fn print(&self) -> () {
        println!("image:");
        println!("{:?}", self.image);
        println!("left: {:?}", self.left);
        println!("right: {:?}", self.right);
        println!("up: {:?}", self.up);
        println!("down: {:?}", self.down);
    }
    fn from_ndarry_view(im_view: ArrayView2<WfcColor>) -> Self {
        let mut image: Array2<WfcColor> =
            Array2::from_elem((TILE_SIZE, TILE_SIZE), WfcColor::black());
        for ((x, y), &c) in im_view.indexed_iter() {
            image[[x, y]] = c;
        }

        Self {
            image,
            left: Vec::new(),
            right: Vec::new(),
            up: Vec::new(),
            down: Vec::new(),
        }
    }

    fn add_left(&mut self, id: Option<usize>) {
        if !self.left.contains(&id) {
            self.left.push(id);
        }
    }
    fn add_right(&mut self, id: Option<usize>) {
        if !self.right.contains(&id) {
            self.right.push(id);
        }
    }
    fn add_up(&mut self, id: Option<usize>) {
        if !self.up.contains(&id) {
            self.up.push(id);
        }
    }
    fn add_down(&mut self, id: Option<usize>) {
        if !self.down.contains(&id) {
            self.down.push(id);
        }
    }

    pub(crate) fn is_left_border(&self) -> bool {
        self.left.contains(&None)
    }
    pub(crate) fn is_right_border(&self) -> bool {
        self.right.contains(&None)
    }
    pub(crate) fn is_upper_border(&self) -> bool {
        self.up.contains(&None)
    }
    pub(crate) fn is_lower_border(&self) -> bool {
        self.down.contains(&None)
    }

    pub(crate) fn get_left(&self) -> &Vec<Option<usize>> {
        &self.left
    }
    pub(crate) fn get_right(&self) -> &Vec<Option<usize>> {
        &self.right
    }
    pub(crate) fn get_up(&self) -> &Vec<Option<usize>> {
        &self.up
    }
    pub(crate) fn get_down(&self) -> &Vec<Option<usize>> {
        &self.down
    }

    pub(crate) fn get_image(&self) -> ArrayView2<WfcColor> {
        self.image.slice(s![.., ..])
    }
}

pub(crate) struct Tileset {
    tiles: Vec<Tile>,
}
impl IndexMut<usize> for Tileset {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.tiles[index]
    }
}
impl Index<usize> for Tileset {
    type Output = Tile;

    fn index(&self, index: usize) -> &Self::Output {
        &self.tiles[index]
    }
}
impl Tileset {
    pub(crate) fn len(&self) -> usize {
        self.tiles.len()
    }
    pub(crate) fn print(&self) -> () {
        for i in 0..self.len() {
            let tile = &self[i];
            println!("Tile nr. {i}");
            tile.print();
        }
    }
    fn empty() -> Self {
        Tileset { tiles: Vec::new() }
    }
    fn contains(&self, im_view: ArrayView2<WfcColor>) -> bool {
        for tile in &self.tiles {
            if {
                let ref this = tile;
                im_view == this.image
            } {
                return true;
            }
        }
        false
    }
    fn insert(&mut self, slice: ArrayView2<WfcColor>) -> () {
        if !self.contains(slice) {
            let id: usize = self.len();
            let tile: Tile = Tile::from_ndarry_view(slice);
            self.tiles.insert(id, tile)
        }
    }
    fn get_id(&self, im_view: ArrayView2<WfcColor>) -> Option<usize> {
        for (id, tile) in self.tiles.iter().enumerate() {
            if tile.image == im_view {
                return Some(id);
            }
        }

        None
    }
    pub(crate) fn from_png(path: &str) -> Self {
        assert!(path.ends_with(".png"), "for now only pngs are supported");

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
            if im_data.len() as u32 != width * height * 3 {
                if im_data.len() as u32 == width * height * 4 {
                    panic!("alpha channel not yet supported")
                }
                panic!("image doesn't have the format 3 bytes per pixel")
            }

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
        fn convert_array3u8_to_array2color(arr3: Array3<u8>) -> Array2<WfcColor> {
            let shape: &[usize] = arr3.shape();
            let width: usize = shape[0];
            let height: usize = shape[1];
            let mut arr2: Array2<WfcColor> = Array2::from_elem((width, height), WfcColor::black());
            for x in 0..width {
                for y in 0..height {
                    arr2[[x, y]] = WfcColor::from_ndarry_view(arr3.slice(s![x, y, ..]));
                }
            }
            arr2
        }
        fn generate_tile_set(image: Array2<WfcColor>) -> Tileset {
            fn get_im_slice([x, y]: [usize; 2], image: &Array2<WfcColor>) -> ArrayView2<WfcColor> {
                image.slice(s![x..x + 3, y..y + 3])
            }
            fn is_valid_coords(coords: [isize; 2], shape: &[usize]) -> bool {
                let x = coords[0] as usize;
                let y = coords[1] as usize;
                return x <= (shape[0] - TILE_SIZE) && y <= (shape[1] - TILE_SIZE);
            }
            let shape = image.shape();
            let width = shape[0];
            let height = shape[1];
            assert_eq!(
                width % TILE_SIZE,
                0,
                "width of input image must be multiple of tilesieze ({})",
                TILE_SIZE
            );
            assert_eq!(
                height % TILE_SIZE,
                0,
                "height of input image must be multiple of tilesieze ({})",
                TILE_SIZE
            );
            let mut tileset = Tileset::empty();

            // creating all Tiles
            for x in (0..width).step_by(TILE_SIZE) {
                for y in (0..height).step_by(TILE_SIZE) {
                    let slice: ArrayView2<WfcColor> = get_im_slice([x, y], &image);
                    tileset.insert(slice);
                }
            }

            // computing neighbours
            for x in (0..width).step_by(TILE_SIZE) {
                for y in (0..height).step_by(TILE_SIZE) {
                    let slice: ArrayView2<WfcColor> = get_im_slice([x, y], &image);
                    let tile_id = tileset.get_id(slice).unwrap();

                    // go left
                    if is_valid_coords([x as isize - TILE_SIZE as isize, y as isize], shape) {
                        let left: ArrayView2<WfcColor> = get_im_slice([x - TILE_SIZE, y], &image);
                        let left_id = tileset.get_id(left).unwrap();
                        tileset[tile_id].add_left(Some(left_id));
                    } else {
                        tileset[tile_id].add_left(None);
                    }

                    // go right
                    if is_valid_coords([x as isize + TILE_SIZE as isize, y as isize], shape) {
                        let right: ArrayView2<WfcColor> = get_im_slice([x + TILE_SIZE, y], &image);
                        let right_id = tileset.get_id(right).unwrap();
                        tileset[tile_id].add_right(Some(right_id));
                    } else {
                        tileset[tile_id].add_right(None);
                    }

                    // go up
                    if is_valid_coords([x as isize, y as isize - TILE_SIZE as isize], shape) {
                        let up: ArrayView2<WfcColor> = get_im_slice([x, y - TILE_SIZE], &image);
                        let up_id = tileset.get_id(up).unwrap();
                        tileset[tile_id].add_up(Some(up_id));
                    } else {
                        tileset[tile_id].add_up(None);
                    }

                    // go down
                    if is_valid_coords([x as isize, y as isize + TILE_SIZE as isize], shape) {
                        let down: ArrayView2<WfcColor> = get_im_slice([x, y + TILE_SIZE], &image);
                        let down_id = tileset.get_id(down).unwrap();
                        tileset[tile_id].add_down(Some(down_id));
                    } else {
                        tileset[tile_id].add_down(None);
                    }
                }
            }

            return tileset;
        }
        let image: DynamicImage = get_image(path);
        let im_shape: (u32, u32) = (image.width(), image.height());

        let image_as_array3: Array3<u8> = get_image_as_array3(im_shape, image.as_bytes());
        let image: Array2<WfcColor> = convert_array3u8_to_array2color(image_as_array3);

        generate_tile_set(image)
    }
}

mod wfc_color {
    use std::fmt::Debug;

    use ndarray::ArrayView1;

    #[derive(Clone, Copy, PartialEq, Eq)]
    pub(crate) struct WfcColor {
        r: u8,
        g: u8,
        b: u8,
    }

    impl Debug for WfcColor {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if self.r > self.g && self.r > self.b {
                write!(f, "R")
            } else if self.g > self.r && self.g > self.b {
                write!(f, "G")
            } else if self.b > self.g && self.b > self.r {
                write!(f, "B")
            } else {
                write!(f, " ")
            }
        }
    }

    impl WfcColor {
        pub(crate) fn from_ndarry_view(arr_view: ArrayView1<u8>) -> Self {
            Self {
                r: arr_view[0],
                g: arr_view[1],
                b: arr_view[2],
            }
        }

        pub(crate) fn black() -> Self {
            Self { r: 0, g: 0, b: 0 }
        }

        pub(crate) fn get(&self) -> (u8, u8, u8) {
            (self.r, self.g, self.b)
        }
    }
}
