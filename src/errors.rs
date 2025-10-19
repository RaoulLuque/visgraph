use resvg::usvg::Error as UsvgError;
use std::{error::Error, fmt::Display};

#[derive(Debug)]
/// Errors that can occur when converting SVG data to an image.
pub enum SvgToImageError {
    /// Error while parsing SVG data.
    SVGParserError(UsvgError),
    /// Error while encoding PNG image.
    PNGEncodingError(Box<dyn Error + Send + Sync>),
}

impl Display for SvgToImageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SvgToImageError::SVGParserError(e) => write!(f, "SVG parsing error: {}", e),
            SvgToImageError::PNGEncodingError(e) => {
                write!(f, "PNG encoding error: {}", e)
            }
        }
    }
}

impl Error for SvgToImageError {}

impl From<UsvgError> for SvgToImageError {
    fn from(err: UsvgError) -> Self {
        SvgToImageError::SVGParserError(err)
    }
}
