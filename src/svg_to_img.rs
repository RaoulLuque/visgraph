//! SVG to image conversion utilities.
//!
//! This module provides functions to convert SVG data into image formats (currently PNG only)
//! using the `resvg` crate.
//!
//! The main functions are [`svg_to_pixmap`], which converts SVG data to a [`tiny_skia::Pixmap`],
//! and [`svg_to_img`], which saves the SVG data as a PNG image to a specified path.
//!
//! For more information on usage, see the function documentation.

use resvg::{
    render,
    tiny_skia::{self, Pixmap},
};

use crate::errors::SvgToImageError;

/// Convert SVG data to a pixmap image.
///
/// The provided width and height should match those used to generate the SVG data and should be
/// strictly positive. Otherwise, an appropriate error will be returned.
pub fn svg_to_pixmap(svg_data: &str, width: f32, height: f32) -> Result<Pixmap, SvgToImageError> {
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

    Ok(pixmap)
}

/// Convert SVG data to a png image and save it to the specified path.
///
/// The provided width and height should match those used to generate the SVG data and should be
/// strictly positive. Otherwise, an appropriate error will be returned.
///
/// Calls [`svg_to_pixmap`] internally.
pub fn svg_to_img(
    svg_data: &str,
    width: f32,
    height: f32,
    path: impl AsRef<std::path::Path>,
) -> Result<(), SvgToImageError> {
    let pixmap = svg_to_pixmap(svg_data, width, height)?;

    // Create target directory if it doesn't exist
    if let Some(parent) = path.as_ref().parent() {
        std::fs::create_dir_all(parent)?;
    }

    pixmap.save_png(path).map_err(std::io::Error::from)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use image::{DynamicImage, GenericImageView, ImageReader};
    use resvg::tiny_skia::Pixmap;

    use crate::graph_to_svg::graph_to_svg_with_layout;
    use crate::graph_to_svg::graph_to_svg_with_positions;
    use crate::tests::{test_graph_with_custom_labels, test_square_graph_with_position_map};
    use crate::Layout::Circular;

    const MSE_ERROR_TOLERANCE: f64 = 100.0;

    /// Helper function to convert a tiny-skia Pixmap to an image::DynamicImage for easier testing.
    fn image_from_pixmap(pixmap: &Pixmap) -> DynamicImage {
        let data = pixmap.data();
        // tiny-skia pixmap is RGBA8
        DynamicImage::ImageRgba8(
            image::RgbaImage::from_raw(pixmap.width(), pixmap.height(), data.to_vec()).unwrap(),
        )
    }

    /// Compute mean squared error (MSE) between two RGBA images.
    /// Returns the average squared pixel difference in [0.0, 65025.0].
    fn mean_squared_error(image_1: &DynamicImage, image_2: &DynamicImage) -> f64 {
        let (width_1, height_1) = image_1.dimensions();

        let pixels_1 = image_1.to_rgba8();
        let pixels_2 = image_2.to_rgba8();

        let mut sum = 0.0;
        for (pixel_1, pixel_2) in pixels_1.pixels().zip(pixels_2.pixels()) {
            for i in 0..4 {
                let diff = pixel_1.0[i] as f64 - pixel_2.0[i] as f64;
                sum += diff * diff;
            }
        }

        sum / ((width_1 * height_1 * 4) as f64)
    }

    /// Asserts that the generated image matches the reference image at the given path.
    fn assert_images_equal(generated: &Pixmap, reference_path: &Path) {
        let generated = image_from_pixmap(generated);
        let reference = ImageReader::open(reference_path)
            .expect("Reference path should be valid")
            .decode()
            .expect("Reference image should be decodable");

        assert_eq!(
            generated.dimensions(),
            reference.dimensions(),
            "Image dimensions differ: generated: {:?}, reference: {:?}",
            generated.dimensions(),
            reference.dimensions()
        );

        let mse = mean_squared_error(&generated, &reference);
        assert!(
            mse < MSE_ERROR_TOLERANCE,
            "Mean squared error exceeds tolerance of {MSE_ERROR_TOLERANCE}: {mse}",
        );
    }

    #[test]
    fn test_svg_to_image_on_graph_with_custom_labels() {
        let (graph, settings) = test_graph_with_custom_labels();

        let svg_data = graph_to_svg_with_layout(&graph, Circular, &settings);
        let pixmap = super::svg_to_pixmap(&svg_data, settings.width, settings.height)
            .expect("SVG to pixmap conversion should succeed.");

        assert_images_equal(
            &pixmap,
            "examples/results/graph_with_custom_labels.png".as_ref(),
        );
    }

    #[test]
    fn test_svg_to_image_on_square_graph_with_position_map() {
        let (graph, settings, position_map) = test_square_graph_with_position_map();

        let svg_data = graph_to_svg_with_positions(&graph, position_map, &settings);
        let pixmap = super::svg_to_pixmap(&svg_data, settings.width, settings.height)
            .expect("SVG to pixmap conversion should succeed.");

        assert_images_equal(
            &pixmap,
            "examples/results/square_graph_with_position_map.png".as_ref(),
        );
    }
}
