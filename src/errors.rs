use resvg::usvg::Error as UsvgError;
use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum SvgToImageError {
    SVGParserError(UsvgError),
    ProvidedDimensionsAreZero((f32, f32)),
    PNGEncodingError(Box<dyn Error + Send + Sync>),
}

impl Display for SvgToImageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SvgToImageError::SVGParserError(e) => write!(f, "SVG parsing error: {}", e),
            SvgToImageError::ProvidedDimensionsAreZero((w, h)) => {
                write!(
                    f,
                    "Provided dimensions are not non-zero and thus invalid: width={}, height={}",
                    w, h
                )
            }
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
