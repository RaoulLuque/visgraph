use resvg::usvg::Error as UsvgError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum VisGraphError {
    /// Error related to settings validation.
    #[error("Settings error: {0}")]
    SettingsError(#[from] SettingsError),
    /// Error while converting SVG to image.
    #[error("SVG to Image conversion error: {0}")]
    SvgToImageError(#[from] SvgToImageError),
}

#[derive(Debug, Error)]
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
pub enum SettingsError {
    /// Invalid dimensions: width or height are not strictly positive values.
    #[error("Invalid dimensions: {0:?} must be positive values.")]
    InvalidDimensions((f32, f32)),
    /// Invalid radius: radius is not a strictly positive value.
    #[error("Invalid radius: {0} must be a positive value.")]
    InvalidRadius(f32),
    /// Invalid font size: font size is not a strictly positive value.
    #[error("Invalid font size: {0} must be a positive value.")]
    InvalidFontSize(f32),
    /// Invalid stroke width: stroke width is not a strictly positive value.
    #[error("Invalid stroke width: {0} must be a positive value.")]
    InvalidStrokeWidth(f32),
    /// Invalid margins: margins are not in the range [0.0, 0.5).
    #[error("Invalid margins: {0:?} must lie in the range [0.0, 0.5).")]
    InvalidMargin((f32, f32)),
}
