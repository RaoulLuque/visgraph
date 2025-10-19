use resvg::{render, tiny_skia};

use crate::errors::SvgToImageError;

/// Convert SVG data to a png image and save it to the specified path.
///
/// The provided width and height should match those used to generate the SVG data and should be
/// strictly positive. Otherwise, an appropriate error will be returned.
pub fn svg_to_img(
    svg_data: &str,
    width: f32,
    height: f32,
    path: impl AsRef<std::path::Path>,
) -> Result<(), SvgToImageError> {
    // Setup usvg options
    let mut opt = resvg::usvg::Options::default();
    opt.fontdb_mut().load_system_fonts();
    opt.default_size =
        resvg::usvg::Size::from_wh(width, height).expect("Provided dimensions should be strictly positive, as Settings struct is validated on creation.");

    let svg_tree = resvg::usvg::Tree::from_data(svg_data.as_bytes(), &opt)?;

    // Render to pixmap
    let mut pixmap = tiny_skia::Pixmap::new(width as u32, height as u32).unwrap();
    render(
        &svg_tree,
        tiny_skia::Transform::identity(),
        &mut pixmap.as_mut(),
    );
    // Create directory if it doesn't exist
    if let Some(parent) = path.as_ref().parent() {
        std::fs::create_dir_all(parent)?;
    }

    pixmap.save_png(path).map_err(std::io::Error::from)?;

    Ok(())
}
