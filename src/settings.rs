//! Settings for graph visualization.
//!
//! For the details on the different settings, see the fields of the [`SettingsBuilder`] struct.
//! One can either create a [`Settings`] instance directly using [`Settings::default()`] or
//! [`Settings::new()`], which will use default values, or use the [`SettingsBuilder`] struct to
//! customize specific settings. The latter will validate the provided values upon calling `build()`.

use crate::errors::InvalidSettingsError;

/// Default width of the SVG canvas and output image in pixels.
pub const DEFAULT_WIDTH: f32 = 1000.0;
/// Default height of the SVG canvas and output image in pixels.
pub const DEFAULT_HEIGHT: f32 = 1000.0;
/// Default radius of the nodes in pixels.
pub const DEFAULT_RADIUS: f32 = 25.0;
/// Default font size for labels in pixels.
pub const DEFAULT_FONT_SIZE: f32 = 16.0;
/// Default stroke width for edges in pixels.
pub const DEFAULT_STROKE_WIDTH: f32 = 5.0;
/// Default margin as a fraction of the width/height. That is, 0.05 means 5% margin on each side.
/// This leaves 90% of the width/height for drawing.
pub const DEFAULT_MARGIN: f32 = 0.05;

pub(crate) type DefaultNodeLabelFn = fn(petgraph::prelude::NodeIndex) -> String;
pub(crate) type DefaultEdgeLabelFn = fn(petgraph::prelude::EdgeIndex) -> String;

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
pub struct Settings<FnNodeLabel = DefaultNodeLabelFn, FnEdgeLabel = DefaultEdgeLabelFn> {
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

impl Default for Settings<DefaultNodeLabelFn, DefaultEdgeLabelFn> {
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

impl Settings<DefaultNodeLabelFn, DefaultEdgeLabelFn> {
    /// Creates a new [`Settings`] instance with default values.
    ///
    /// Use the [`SettingsBuilder`] struct to customize specific settings and for details on the
    /// different settings.
    /// For default values, see the `DEFAULT_*` constants.
    pub fn new() -> Self {
        Self::default()
    }
}

/// Builder for creating a [`Settings`] instance with customized values.
///
/// For details on the different settings, see the fields of this struct.
///
/// Example usage:
/// /// Example usage:
/// ```rust
/// use visgraph::settings::SettingsBuilder;
/// // All values we don't explicitly set will use their default values.
/// let settings = SettingsBuilder::new()
///     .width(500.0)
///     .height(500.0)
///     .build()
///     .expect("Provided values should be valid.");
/// ```
#[derive(Debug)]
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
    pub node_radius: f32,

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

    /// Function to generate node labels. If none is provided, node indexes will be used as labels.
    ///
    /// *Valid values*: Functions that implement `impl Fn(G::NodeId) -> String`. This is validated
    /// statically by the compiler when the settings are used as an argument.
    pub node_label_fn: Option<FnNodeLabel>,

    /// Function to generate edge labels. If none is provided, no edge labels will be drawn.
    ///
    /// *Valid values*: Functions that implement `impl Fn(G::EdgeId) -> String`. This is validated
    /// statically by the compiler when the settings are used as an argument.
    pub edge_label_fn: Option<FnEdgeLabel>,
}

impl Default for SettingsBuilder<DefaultNodeLabelFn, DefaultEdgeLabelFn> {
    /// Creates a new `SettingsBuilder` instance with default values.
    ///
    /// For default values, see the `DEFAULT_*` constants.
    fn default() -> Self {
        SettingsBuilder {
            width: DEFAULT_WIDTH,
            height: DEFAULT_HEIGHT,
            node_radius: DEFAULT_RADIUS,
            font_size: DEFAULT_FONT_SIZE,
            stroke_width: DEFAULT_STROKE_WIDTH,
            margin_x: DEFAULT_MARGIN,
            margin_y: DEFAULT_MARGIN,
            node_label_fn: None,
            edge_label_fn: None,
        }
    }
}

impl SettingsBuilder<DefaultNodeLabelFn, DefaultEdgeLabelFn> {
    /// Creates a new `SettingsBuilder` instance with default values.
    ///
    /// For default values, see the `DEFAULT_*` constants.
    pub fn new() -> Self {
        SettingsBuilder::default()
    }
}

impl<FnNodeLabel, FnEdgeLabel> SettingsBuilder<FnNodeLabel, FnEdgeLabel> {
    /// Sets the width of the SVG canvas and returns the modified [`SettingsBuilder`].
    ///
    /// The default width is [`DEFAULT_WIDTH`].
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the SVG canvas and returns the modified [`SettingsBuilder`].
    ///
    /// The default height is [`DEFAULT_HEIGHT`].
    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// Sets the radius of the nodes in pixels and returns the modified [`SettingsBuilder`].
    ///
    /// The default radius is [`DEFAULT_RADIUS`].
    pub fn node_radius(mut self, radius: f32) -> Self {
        self.node_radius = radius;
        self
    }

    /// Sets the font size for labels in pixels and returns the modified [`SettingsBuilder`].
    ///
    /// The default font size is [`DEFAULT_FONT_SIZE`].
    pub fn font_size(mut self, font_size: f32) -> Self {
        self.font_size = font_size;
        self
    }

    /// Sets the stroke width for edges in pixels and returns the modified [`SettingsBuilder`].
    ///
    /// The default stroke width is [`DEFAULT_STROKE_WIDTH`].
    pub fn stroke_width(mut self, stroke_width: f32) -> Self {
        self.stroke_width = stroke_width;
        self
    }

    /// Sets the horizontal margin as a fraction of the width and returns the modified [`SettingsBuilder`].
    ///
    /// The default margin is [`DEFAULT_MARGIN`].
    pub fn margin_x(mut self, margin_x: f32) -> Self {
        self.margin_x = margin_x;
        self
    }

    /// Sets the vertical margin as a fraction of the height and returns the modified [`SettingsBuilder`].
    ///
    /// The default margin is [`DEFAULT_MARGIN`].
    pub fn margin_y(mut self, margin_y: f32) -> Self {
        self.margin_y = margin_y;
        self
    }

    /// Sets the node label function and returns the modified [`SettingsBuilder`].
    ///
    /// The node label function should implement `impl Fn(G::NodeId) -> String`.
    pub fn node_label_fn<NewFnNodeLabel>(
        self,
        node_label: NewFnNodeLabel,
    ) -> SettingsBuilder<NewFnNodeLabel, FnEdgeLabel> {
        SettingsBuilder {
            width: self.width,
            height: self.height,
            node_radius: self.node_radius,
            font_size: self.font_size,
            stroke_width: self.stroke_width,
            margin_x: self.margin_x,
            margin_y: self.margin_y,
            node_label_fn: Some(node_label),
            edge_label_fn: self.edge_label_fn,
        }
    }

    /// Sets the edge label function and returns the modified [`SettingsBuilder`].
    ///
    /// The edge label function should implement `impl Fn(G::EdgeId) -> String`.
    pub fn edge_label_fn<NewFnEdgeLabel>(
        self,
        edge_label: NewFnEdgeLabel,
    ) -> SettingsBuilder<FnNodeLabel, NewFnEdgeLabel> {
        SettingsBuilder {
            width: self.width,
            height: self.height,
            node_radius: self.node_radius,
            font_size: self.font_size,
            stroke_width: self.stroke_width,
            margin_x: self.margin_x,
            margin_y: self.margin_y,
            node_label_fn: self.node_label_fn,
            edge_label_fn: Some(edge_label),
        }
    }

    /// Validates the settings.
    ///
    /// Checks that all settings are within acceptable ranges. If not, returns a corresponding [`SettingsError`].
    fn validate(&self) -> Result<(), InvalidSettingsError> {
        if self.width <= 0.0 || self.height <= 0.0 {
            return Err(InvalidSettingsError::Dimensions(self.width, self.height));
        } else if self.node_radius <= 0.0 {
            return Err(InvalidSettingsError::Radius(self.node_radius));
        } else if self.font_size <= 0.0 {
            return Err(InvalidSettingsError::FontSize(self.font_size));
        } else if self.stroke_width <= 0.0 {
            return Err(InvalidSettingsError::StrokeWidth(self.stroke_width));
        } else if self.margin_x < 0.0
            || self.margin_x > 0.5
            || self.margin_y < 0.0
            || self.margin_y > 0.5
        {
            return Err(InvalidSettingsError::Margin(self.margin_x, self.margin_y));
        }

        Ok(())
    }

    /// Builds the [`Settings`] instance after validating the provided values.
    pub fn build(self) -> Result<Settings<FnNodeLabel, FnEdgeLabel>, InvalidSettingsError> {
        self.validate()?;
        let settings = Settings {
            width: self.width,
            height: self.height,
            radius: self.node_radius,
            font_size: self.font_size,
            stroke_width: self.stroke_width,
            margin_x: self.margin_x,
            margin_y: self.margin_y,
            node_label: self.node_label_fn,
            edge_label: self.edge_label_fn,
        };
        Ok(settings)
    }
}
