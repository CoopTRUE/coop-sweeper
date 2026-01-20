use iced::{Background, Color, color};

pub const PRIMARY_COLOR: Color = color!(74, 74, 74);
pub const BACKGROUND_COLOR: Background = Background::Color(PRIMARY_COLOR);
pub const TEXT_COLOR: Color = color!(200, 200, 200);
pub const GRID_CONTAINER_BACKGROUND_COLOR: Background = Background::Color(color!(48, 48, 48));
// pub const BORDER: Border = Border {
//     color: Color::from_rgb8(49, 49, 49),
//     ..Default::default()
// };

// pub fn theme(_app: &App) -> Option<Theme> {
//     Some(Theme::custom("Theme", {
//         ..Palette::CATPPUCCIN_FRAPPE.extend(EXTENDED_CATPPUCCIN_LATTE)
//     }))
// }
