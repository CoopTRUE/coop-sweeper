use iced::widget::{Image, image};

pub fn get_clock_image(number: u8) -> Image {
    match number {
        0 => image("assets/c0.png"),
        1 => image("assets/c1.png"),
        2 => image("assets/c2.png"),
        3 => image("assets/c3.png"),
        4 => image("assets/c4.png"),
        5 => image("assets/c5.png"),
        6 => image("assets/c6.png"),
        7 => image("assets/c7.png"),
        8 => image("assets/c8.png"),
        9 => image("assets/c9.png"),
        _ => unreachable!("Invalid number: {}", number),
    }
    .filter_method(image::FilterMethod::Nearest)
    .scale(3.0)
}

pub fn get_mine_image() -> Image {
    image("assets/mine.png")
        .filter_method(image::FilterMethod::Nearest)
        .scale(3.0)
}

pub fn get_flag_image() -> Image {
    image("assets/flag.png")
        .filter_method(image::FilterMethod::Nearest)
        .scale(3.0)
}

pub fn get_cell_image() -> Image {
    image("assets/cell.png")
        .filter_method(image::FilterMethod::Nearest)
        .scale(3.0)
}
