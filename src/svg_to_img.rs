use resvg::{render, tiny_skia};

use crate::errors::SvgToImageError;
use crate::settings::Settings;

pub fn svg_to_img<FnNodeLabel, FnEdgeLabel>(
    svg_data: &str,
    settings: &Settings<FnNodeLabel, FnEdgeLabel>,
    path: impl AsRef<std::path::Path>,
) -> Result<(), SvgToImageError> {
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
    // Create directory if it doesn't exist
    if let Some(parent) = path.as_ref().parent() {
        std::fs::create_dir_all(parent)?;
    }

    pixmap.save_png(path).map_err(|e| std::io::Error::from(e))?;

    Ok(())
}
