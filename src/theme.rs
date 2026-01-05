use iced::{
    Border, Color, Theme,
    border::Radius,
    color,
    theme::{Palette, palette::EXTENDED_CATPPUCCIN_LATTE},
};

use crate::game::App;

pub const PRIMARY_COLOR: Color = color!(74, 74, 74);
pub const TEXT_COLOR: Color = color!(200, 200, 200);
pub const CELL_COLOR: Color = color!(95, 95, 95);
// pub const BORDER: Border = Border {
//     color: Color::from_rgb8(49, 49, 49),
//     ..Default::default()
// };

// pub fn theme(_app: &App) -> Option<Theme> {
//     Some(Theme::custom("Theme", {
//         ..Palette::CATPPUCCIN_FRAPPE.extend(EXTENDED_CATPPUCCIN_LATTE)
//     }))
// }
