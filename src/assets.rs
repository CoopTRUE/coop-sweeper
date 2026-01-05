use std::sync::LazyLock;

use iced::widget::{Image, image};

const CLOCK_BYTES: [&[u8]; 10] = [
    include_bytes!("../assets/c0.png"),
    include_bytes!("../assets/c1.png"),
    include_bytes!("../assets/c2.png"),
    include_bytes!("../assets/c3.png"),
    include_bytes!("../assets/c4.png"),
    include_bytes!("../assets/c5.png"),
    include_bytes!("../assets/c6.png"),
    include_bytes!("../assets/c7.png"),
    include_bytes!("../assets/c8.png"),
    include_bytes!("../assets/c9.png"),
];

const CELL_BYTES: [&[u8]; 9] = [
    include_bytes!("../assets/0.png"),
    include_bytes!("../assets/1.png"),
    include_bytes!("../assets/2.png"),
    include_bytes!("../assets/3.png"),
    include_bytes!("../assets/4.png"),
    include_bytes!("../assets/5.png"),
    include_bytes!("../assets/6.png"),
    include_bytes!("../assets/7.png"),
    include_bytes!("../assets/8.png"),
];

const FACE_BYTES: [&[u8]; 2] = [
    include_bytes!("../assets/happy.png"),
    include_bytes!("../assets/surpised.png"),
];

// const IMAGE_SCALE: f32 = 4.0;

fn create_handle(bytes: &'static [u8]) -> image::Handle {
    image::Handle::from_bytes(bytes)
}

fn create_image(handle: image::Handle) -> Image {
    image(handle).filter_method(image::FilterMethod::Nearest)
}

static CLOCK_HANDLES: LazyLock<[image::Handle; 10]> =
    LazyLock::new(|| CLOCK_BYTES.map(create_handle));
static CELL_HANDLES: LazyLock<[image::Handle; 9]> = LazyLock::new(|| CELL_BYTES.map(create_handle));
static UNREVEALED_CELL_HANDLE: LazyLock<image::Handle> =
    LazyLock::new(|| create_handle(include_bytes!("../assets/tile.png")));
static MINE_HANDLE: LazyLock<image::Handle> =
    LazyLock::new(|| create_handle(include_bytes!("../assets/mine.png")));
static FLAG_HANDLE: LazyLock<image::Handle> =
    LazyLock::new(|| create_handle(include_bytes!("../assets/flag.png")));
static FACE_HANDLES: LazyLock<[image::Handle; 2]> = LazyLock::new(|| FACE_BYTES.map(create_handle));

pub fn get_clock_image(number: u8) -> Image {
    create_image(CLOCK_HANDLES[number as usize].clone())
}

pub fn get_cell_image(number: u8) -> Image {
    create_image(CELL_HANDLES[number as usize].clone())
}

pub fn get_unrevealed_cell_image() -> Image {
    create_image(UNREVEALED_CELL_HANDLE.clone())
}

pub fn get_mine_image() -> Image {
    create_image(MINE_HANDLE.clone())
}

pub fn get_flag_image() -> Image {
    create_image(FLAG_HANDLE.clone())
}

#[derive(Default, Debug, Clone, Copy)]
pub enum Face {
    #[default]
    Happy = 0,
    Surprised = 1,
}

pub fn get_face_image(face: Face) -> Image {
    create_image(FACE_HANDLES[face as usize].clone())
}
