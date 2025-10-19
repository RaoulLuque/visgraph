use crate::errors::SettingsError;

pub const DEFAULT_WIDTH: f32 = 1000.0;
pub const DEFAULT_HEIGHT: f32 = 1000.0;
pub const DEFAULT_RADIUS: f32 = 25.0;
pub const DEFAULT_FONT_SIZE: f32 = 16.0;
pub const DEFAULT_STROKE_WIDTH: f32 = 5.0;
pub const DEFAULT_MARGIN: f32 = 0.05;

/// Settings for SVG graph rendering.
///
/// For the details on the different settings, see the fields of the [`SettingsBuilder`] struct.
///
/// One can either create a [`Settings`] instance directly using [`Settings::default()`] or `Settings::new()`,
/// which will use default values, or use the [`SettingsBuilder`] struct to customize specific settings.
/// The latter will validate the provided values upon calling `build()`.
///
/// For default values, see the `DEFAULT_*` constants.
///
/// /// Example usage:
/// ```rust
/// use visgraph::settings::SettingsBuilder;
/// // All values we don't explicitly set will use their default values.
/// let settings = SettingsBuilder::new()
///     .width(800.0)
///     .height(600.0)
///     .build()
///     .expect("Provided values should be valid.");
/// ```
#[derive(Debug)]
pub struct Settings<
    FnNodeLabel = fn(petgraph::prelude::EdgeIndex) -> String,
    FnEdgeLabel = fn(petgraph::prelude::NodeIndex) -> String,
> {
    pub(crate) width: f32,
    pub(crate) height: f32,
    pub(crate) radius: f32,
    pub(crate) font_size: f32,
    pub(crate) stroke_width: f32,
    pub(crate) margin_x: f32,
    pub(crate) margin_y: f32,
    pub(crate) node_label: Option<FnNodeLabel>,
    pub(crate) edge_label: Option<FnEdgeLabel>,
}

impl Default
    for Settings<
        fn(petgraph::prelude::NodeIndex) -> String,
        fn(petgraph::prelude::EdgeIndex) -> String,
    >
{
    /// Creates a new [`Settings`] instance with default values.
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
            node_label: None,
            edge_label: None,
        }
    }
}

impl
    Settings<fn(petgraph::prelude::NodeIndex) -> String, fn(petgraph::prelude::EdgeIndex) -> String>
{
    /// Creates a new [`Settings`] instance with default values.
    ///
    /// Use the [`SettingsBuilder`] struct to customize specific settings and for details on the
    /// different settings.
    /// For default values, see the `DEFAULT_*` constants.
    pub fn new() -> Self {
        Self::default()
    }
}

impl<FnNodeLabel, FnEdgeLabel> Settings<FnNodeLabel, FnEdgeLabel> {
    pub(crate) fn with_node_label<NewFnNodeLabel>(
        self,
        node_label: NewFnNodeLabel,
    ) -> Settings<NewFnNodeLabel, FnEdgeLabel> {
        Settings {
            width: self.width,
            height: self.height,
            radius: self.radius,
            font_size: self.font_size,
            stroke_width: self.stroke_width,
            margin_x: self.margin_x,
            margin_y: self.margin_y,
            node_label: Some(node_label),
            edge_label: self.edge_label,
        }
    }
}

pub struct SettingsBuilder<FnNodeLabel, FnEdgeLabel> {
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
    pub node_label: Option<FnNodeLabel>,
    pub edge_label: Option<FnEdgeLabel>,
}

impl<FnNodeLabel, FnEdgeLabel> SettingsBuilder<FnNodeLabel, FnEdgeLabel> {
    /// Creates a new `SettingsBuilder` instance with default values.
    ///
    /// For default values, see the `DEFAULT_*` constants.
    pub fn new() -> Self {
        SettingsBuilder {
            width: DEFAULT_WIDTH,
            height: DEFAULT_HEIGHT,
            radius: DEFAULT_RADIUS,
            font_size: DEFAULT_FONT_SIZE,
            stroke_width: DEFAULT_STROKE_WIDTH,
            margin_x: DEFAULT_MARGIN,
            margin_y: DEFAULT_MARGIN,
            node_label: None,
            edge_label: None,
        }
    }

    /// Sets the width of the SVG canvas and returns the modified [`SettingsBuilder`].
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the SVG canvas and returns the modified [`SettingsBuilder`].
    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// Sets the radius of the nodes in pixels and returns the modified [`SettingsBuilder`].
    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }

    /// Sets the font size for labels in pixels and returns the modified [`SettingsBuilder`].
    pub fn font_size(mut self, font_size: f32) -> Self {
        self.font_size = font_size;
        self
    }

    /// Sets the stroke width for edges in pixels and returns the modified [`SettingsBuilder`].
    pub fn stroke_width(mut self, stroke_width: f32) -> Self {
        self.stroke_width = stroke_width;
        self
    }

    /// Sets the horizontal margin as a fraction of the width and returns the modified [`SettingsBuilder`].
    pub fn margin_x(mut self, margin_x: f32) -> Self {
        self.margin_x = margin_x;
        self
    }

    /// Sets the vertical margin as a fraction of the height and returns the modified [`SettingsBuilder`].
    pub fn margin_y(mut self, margin_y: f32) -> Self {
        self.margin_y = margin_y;
        self
    }

    pub fn node_label<NewFnNodeLabel>(
        self,
        node_label: NewFnNodeLabel,
    ) -> SettingsBuilder<NewFnNodeLabel, FnEdgeLabel> {
        SettingsBuilder {
            width: self.width,
            height: self.height,
            radius: self.radius,
            font_size: self.font_size,
            stroke_width: self.stroke_width,
            margin_x: self.margin_x,
            margin_y: self.margin_y,
            node_label: Some(node_label),
            edge_label: self.edge_label,
        }
    }

    pub fn edge_label<NewFnEdgeLabel>(
        self,
        edge_label: NewFnEdgeLabel,
    ) -> SettingsBuilder<FnNodeLabel, NewFnEdgeLabel> {
        SettingsBuilder {
            width: self.width,
            height: self.height,
            radius: self.radius,
            font_size: self.font_size,
            stroke_width: self.stroke_width,
            margin_x: self.margin_x,
            margin_y: self.margin_y,
            node_label: self.node_label,
            edge_label: Some(edge_label),
        }
    }

    /// Validates the settings.
    ///
    /// Checks that all settings are within acceptable ranges. If not, returns a corresponding [`SettingsError`].
    fn validate(&self) -> Result<(), SettingsError> {
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

    /// Builds the [`Settings`] instance after validating the provided values.
    pub fn build(self) -> Result<Settings<FnNodeLabel, FnEdgeLabel>, SettingsError> {
        self.validate()?;
        let settings = Settings {
            width: self.width,
            height: self.height,
            radius: self.radius,
            font_size: self.font_size,
            stroke_width: self.stroke_width,
            margin_x: self.margin_x,
            margin_y: self.margin_y,
            node_label: self.node_label,
            edge_label: self.edge_label,
        };
        Ok(settings)
    }
}
