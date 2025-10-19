use std::{error::Error, fmt::Display};

pub const DEFAULT_WIDTH: f32 = 1000.0;
pub const DEFAULT_HEIGHT: f32 = 1000.0;
pub const DEFAULT_RADIUS: f32 = 25.0;
pub const DEFAULT_FONT_SIZE: f32 = 16.0;
pub const DEFAULT_STROKE_WIDTH: f32 = 5.0;
pub const DEFAULT_MARGIN: f32 = 0.05;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SettingsError {
    /// Invalid dimensions: width or height are not strictly positive values.
    InvalidDimensions((f32, f32)),
    /// Invalid radius: radius is not a strictly positive value.
    InvalidRadius(f32),
    /// Invalid font size: font size is not a strictly positive value.
    InvalidFontSize(f32),
    /// Invalid stroke width: stroke width is not a strictly positive value.
    InvalidStrokeWidth(f32),
    /// Invalid margins: margins are not in the range [0.0, 0.5).
    InvalidMargin((f32, f32)),
}

impl Display for SettingsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SettingsError::InvalidDimensions((w, h)) => {
                write!(
                    f,
                    "Invalid dimensions: width={} and height={} must be positive values.",
                    w, h
                )
            }
            SettingsError::InvalidRadius(r) => {
                write!(f, "Invalid radius: radius={} must be a positive value.", r)
            }
            SettingsError::InvalidFontSize(fs) => {
                write!(
                    f,
                    "Invalid font size: font_size={} must be a positive value.",
                    fs
                )
            }
            SettingsError::InvalidStrokeWidth(sw) => {
                write!(
                    f,
                    "Invalid stroke width: stroke_width={} must be a positive value.",
                    sw
                )
            }
            SettingsError::InvalidMargin((mx, my)) => {
                write!(
                    f,
                    "Invalid margins: margin_x={} and margin_y={} must lie in the range [0.0, 0.5).",
                    mx, my
                )
            }
        }
    }
}

impl Error for SettingsError {}

/// Settings for SVG graph rendering.
///
/// For the different settings, see the fields of the [`SettingsBuilder`] struct.
///
/// One can either create a `Settings` instance directly using `Settings::default()` or `Settings::new()`,
/// which will use default values, or use the `SettingsBuilder` to customize specific settings.
/// The latter will validate the provided values upon calling `build()`.
///
/// For default values, see the `DEFAULT_*` constants.
///
/// /// Example usage:
/// ```rust
/// use visgraph::settings::Settings;
/// // We overwrite only the width and height and keep other settings as default.
/// let settings = Settings::new()
///     .with_width(800.0)
///     .with_height(600.0);
/// ```
pub struct Settings {
    /// Width of the SVG and output image in pixels.
    ///
    /// *Valid values*: strictly positive f32
    pub width: f32,
    /// Height of the SVG and output image in pixels.
    ///
    /// *Valid values*: strictly positive f32
    pub height: f32,
    /// Radius of the nodes in pixels.
    ///
    /// *Valid values*: strictly positive f32
    pub radius: f32,
    /// Font size for labels in pixels.
    ///
    /// *Valid values*: strictly positive f32
    pub font_size: f32,
    /// Stroke width for edges in pixels.
    ///
    /// *Valid values*: strictly positive f32
    pub stroke_width: f32,
    /// Horizontal margin as a fraction of the width.
    /// That is, 0.1 means 10% margin on left and right, leaving 80% of the width for drawing.
    ///
    /// *Valid values*: f32 in range [0.0, 0.5)
    pub margin_x: f32,
    /// Vertical margin as a fraction of the height.
    /// That is, 0.1 means 10% margin on top and bottom, leaving 80% of the height for drawing.
    ///
    /// *Valid values*: f32 in range [0.0, 0.5)
    pub margin_y: f32,
}

impl Default for Settings {
    /// Creates a new `Settings` instance with default values.
    ///
    /// For default values, see the `DEFAULT_*` constants.
    fn default() -> Self {
        Settings {
            width: DEFAULT_WIDTH,
            height: DEFAULT_HEIGHT,
            radius: DEFAULT_RADIUS,
            font_size: DEFAULT_FONT_SIZE,
            stroke_width: DEFAULT_STROKE_WIDTH,
            margin_x: DEFAULT_MARGIN,
            margin_y: DEFAULT_MARGIN,
        }
    }
}

impl Settings {
    /// Creates a new `Settings` instance with default values.
    ///
    /// Default values can be overwritten using the `with_*` methods in a builder pattern. For a
    /// description of each settings, see the [`Settings`] struct documentation.
    ///
    /// For default values, see the `DEFAULT_*` constants.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the width of the SVG canvas and returns the modified `Settings`.
    pub fn with_width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the SVG canvas and returns the modified `Settings`.
    pub fn with_height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// Sets the radius of the nodes in pixels and returns the modified `Settings`.
    pub fn with_radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }

    /// Sets the font size for labels in pixels and returns the modified `Settings`.
    pub fn with_font_size(mut self, font_size: f32) -> Self {
        self.font_size = font_size;
        self
    }

    /// Sets the stroke width for edges in pixels and returns the modified `Settings`.
    pub fn with_stroke_width(mut self, stroke_width: f32) -> Self {
        self.stroke_width = stroke_width;
        self
    }

    /// Sets the horizontal margin as a fraction of the width and returns the modified `Settings`.
    pub fn with_margin_x(mut self, margin_x: f32) -> Self {
        self.margin_x = margin_x;
        self
    }

    /// Sets the vertical margin as a fraction of the height and returns the modified `Settings`.
    pub fn with_margin_y(mut self, margin_y: f32) -> Self {
        self.margin_y = margin_y;
        self
    }

    /// Validates the settings.
    ///
    /// Checks that all settings are within acceptable ranges. If not, returns a corresponding `SettingsError`.
    fn validate_settings(&self) -> Result<(), SettingsError> {
        if self.width <= 0.0 || self.height <= 0.0 {
            return Err(SettingsError::InvalidDimensions((self.width, self.height)));
        } else if self.radius <= 0.0 {
            return Err(SettingsError::InvalidRadius(self.radius));
        } else if self.font_size <= 0.0 {
            return Err(SettingsError::InvalidFontSize(self.font_size));
        } else if self.stroke_width <= 0.0 {
            return Err(SettingsError::InvalidStrokeWidth(self.stroke_width));
        } else if self.margin_x < 0.0
            || self.margin_x > 0.5
            || self.margin_y < 0.0
            || self.margin_y > 0.5
        {
            return Err(SettingsError::InvalidMargin((self.margin_x, self.margin_y)));
        }

        Ok(())
    }
}
