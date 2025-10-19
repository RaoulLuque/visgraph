use std::path::Path;

use resvg::{render, tiny_skia};

use crate::errors::SvgToImageError;
use crate::settings::Settings;

pub fn parse_svg_to_img<P>(
    svg_data: &str,
    settings: Settings,
    path: P,
) -> Result<(), SvgToImageError>
where
    P: AsRef<Path>,
{
    // Setup usvg options
    let mut opt = resvg::usvg::Options::default();
    opt.fontdb_mut().load_system_fonts();
    opt.default_size =
        resvg::usvg::Size::from_wh(settings.width, settings.height).expect("Provided dimensions should be strictly positive, as Settings struct is validated on creation.");

    let svg_tree = resvg::usvg::Tree::from_data(svg_data.as_bytes(), &opt)?;

    // Render to pixmap
    let mut pixmap = tiny_skia::Pixmap::new(settings.width as u32, settings.height as u32).unwrap();
    render(
        &svg_tree,
        tiny_skia::Transform::identity(),
        &mut pixmap.as_mut(),
    );
    pixmap
        .save_png(path)
        .map_err(|e| SvgToImageError::PNGEncodingError(Box::new(e)))?;

    Ok(())
}
