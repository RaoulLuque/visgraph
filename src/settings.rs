use std::{error::Error, fmt::Display};

pub const DEFAULT_WIDTH: f32 = 1000.0;
pub const DEFAULT_HEIGHT: f32 = 1000.0;
pub const DEFAULT_RADIUS: f32 = 25.0;
pub const DEFAULT_FONT_SIZE: f32 = 16.0;
pub const DEFAULT_STROKE_WIDTH: f32 = 5.0;
pub const DEFAULT_MARGIN: f32 = 0.05;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SettingsError {
    InvalidDimensions((f32, f32)),
    InvalidRadius(f32),
    InvalidFontSize(f32),
    InvalidStrokeWidth(f32),
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
                    "Invalid margins: margin_x={} and margin_y={} must be between 0.0 and 0.5.",
                    mx, my
                )
            }
        }
    }
}

impl Error for SettingsError {}

pub struct Settings {
    width: f32,
    height: f32,
    radius: f32,
    font_size: f32,
    stroke_width: f32,
    margin_x: f32,
    margin_y: f32,
}

impl Default for Settings {
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
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn width(&self) -> f32 {
        self.width
    }

    pub(crate) fn height(&self) -> f32 {
        self.height
    }

    pub(crate) fn radius(&self) -> f32 {
        self.radius
    }

    pub(crate) fn font_size(&self) -> f32 {
        self.font_size
    }

    pub(crate) fn stroke_width(&self) -> f32 {
        self.stroke_width
    }

    pub(crate) fn margin_x(&self) -> f32 {
        self.margin_x
    }

    pub(crate) fn margin_y(&self) -> f32 {
        self.margin_y
    }
}

pub struct SettingsBuilder {
    settings: Settings,
}

impl SettingsBuilder {
    pub fn new() -> Self {
        SettingsBuilder {
            settings: Settings::default(),
        }
    }

    pub fn width(mut self, width: f32) -> Self {
        self.settings.width = width;
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.settings.height = height;
        self
    }

    pub fn radius(mut self, radius: f32) -> Self {
        self.settings.radius = radius;
        self
    }

    pub fn font_size(mut self, font_size: f32) -> Self {
        self.settings.font_size = font_size;
        self
    }

    pub fn stroke_width(mut self, stroke_width: f32) -> Self {
        self.settings.stroke_width = stroke_width;
        self
    }

    pub fn margin_x(mut self, margin_x: f32) -> Self {
        self.settings.margin_x = margin_x;
        self
    }

    pub fn margin_y(mut self, margin_y: f32) -> Self {
        self.settings.margin_y = margin_y;
        self
    }

    pub fn build(self) -> Result<Settings, SettingsError> {
        if self.settings.width <= 0.0 || self.settings.height <= 0.0 {
            return Err(SettingsError::InvalidDimensions((
                self.settings.width,
                self.settings.height,
            )));
        } else if self.settings.radius <= 0.0 {
            return Err(SettingsError::InvalidRadius(self.settings.radius));
        } else if self.settings.font_size <= 0.0 {
            return Err(SettingsError::InvalidFontSize(self.settings.font_size));
        } else if self.settings.stroke_width <= 0.0 {
            return Err(SettingsError::InvalidStrokeWidth(
                self.settings.stroke_width,
            ));
        } else if self.settings.margin_x < 0.0
            || self.settings.margin_x > 0.5
            || self.settings.margin_y < 0.0
            || self.settings.margin_y > 0.5
        {
            return Err(SettingsError::InvalidMargin((
                self.settings.margin_x,
                self.settings.margin_y,
            )));
        }

        Ok(self.settings)
    }
}
