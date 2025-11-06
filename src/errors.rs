#[cfg(feature = "img")]
use resvg::usvg::Error as UsvgError;
use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
/// Errors that can occur in the `VisGraph` library.
///
/// Note that some error variants might be feature-gated and only available
/// when applicable features are enabled, i.e. they can actually occur.
pub enum VisGraphError {
    /// Error related to settings validation.
    #[error("Settings error: {0}")]
    Settings(#[from] InvalidSettingsError),
    /// Error while converting SVG to image.
    #[cfg(feature = "img")]
    #[error("SVG to Image conversion error: {0}")]
    SvgToImage(#[from] SvgToImageError),
    /// IO error occurred during file operations.
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
}

#[derive(Debug, Error)]
#[cfg(feature = "img")]
/// Errors that can occur when converting SVG data to an image.
pub enum SvgToImageError {
    /// Error while parsing SVG data.
    #[error("SVG parsing error: {0}")]
    SVGParserError(#[from] UsvgError),
    /// IO error occurred during image saving.
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
}

#[derive(Clone, Copy, Debug, Error, PartialEq)]
pub enum InvalidSettingsError {
    /// Invalid dimensions: width or height are not strictly positive values.
    #[error("Invalid dimensions: ({0}, {1}) must be positive values.")]
    Dimensions(f32, f32),
    /// Invalid radius: radius is not a strictly positive value.
    #[error("Invalid radius: {0} must be a positive value.")]
    Radius(f32),
    /// Invalid font size: font size is not a strictly positive value.
    #[error("Invalid font size: {0} must be a positive value.")]
    FontSize(f32),
    /// Invalid stroke width: stroke width is not a strictly positive value.
    #[error("Invalid stroke width: {0} must be a positive value.")]
    StrokeWidth(f32),
    /// Invalid margins: margins are not in the range [0.0, 0.5).
    #[error("Invalid margins: ({0}, {1}) must lie in the range [0.0, 0.5).")]
    Margin(f32, f32),
}
