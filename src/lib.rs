mod errors;
mod graph_to_svg;
mod layout;
mod svg_to_img;

const DEFAULT_RADIUS: f32 = 20.0;
const DEFAULT_FONT_SIZE: f32 = 14.0;

pub struct Settings {
    pub width: f32,
    pub height: f32,
    pub radius: f32,
    pub font_size: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            radius: DEFAULT_RADIUS,
            font_size: DEFAULT_FONT_SIZE,
        }
    }
}

pub use graph_to_svg::graph_to_svg_with_layout;
pub use graph_to_svg::graph_to_svg_with_positions;

pub use svg_to_img::parse_svg_to_img;

pub use layout::Layout;
