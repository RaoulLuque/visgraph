use resvg::{render, tiny_skia};

use crate::errors::SvgToImageError;

pub fn parse_svg_to_img(
    svg_data: &str,
    width: f32,
    height: f32,
) -> Result<tiny_skia::Pixmap, SvgToImageError> {
    // Setup usvg options
    let mut opt = resvg::usvg::Options::default();
    opt.fontdb_mut().load_system_fonts();
    opt.default_size = resvg::usvg::Size::from_wh(width, height)
        .ok_or_else(|| SvgToImageError::ProvidedDimensionsAreZero((width, height)))?;

    let svg_tree = resvg::usvg::Tree::from_data(svg_data.as_bytes(), &opt)?;

    // Render to pixmap
    let mut pixmap = tiny_skia::Pixmap::new(width as u32, height as u32).unwrap();
    render(
        &svg_tree,
        tiny_skia::Transform::identity(),
        &mut pixmap.as_mut(),
    );
    Ok(pixmap)
}
