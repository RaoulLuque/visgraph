//! Settings for graph visualization.
//!
//! For the details on the different settings, see the fields of the [`SettingsBuilder`] struct.
//! One can either create a [`Settings`] instance directly using [`Settings::default()`] or
//! [`Settings::new()`], which will use default values, or use the [`SettingsBuilder`] struct to
//! customize specific settings. The latter will validate the provided values upon calling `build()`.

use crate::{
    errors::InvalidSettingsError,
    layout::{DefaultPositionMapFn, LayoutOrPositionMap},
    Layout,
};

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
/// Default layout algorithm for graph visualization.
pub const DEFAULT_LAYOUT_OR_POS_MAP: LayoutOrPositionMap<DefaultPositionMapFn> =
    LayoutOrPositionMap::Layout(Layout::Circular);
/// Default function to generate node labels. Uses node indexes as labels.
pub const DEFAULT_NODE_LABEL_FN: DefaultNodeLabelFn = |node_id| format!("Node {}", node_id.index());
/// Default function to generate edge labels. No (empty) edge labels are drawn.
pub const DEFAULT_EDGE_LABEL_FN: DefaultEdgeLabelFn = |_| "".to_string();
/// Default function to generate node colors. All nodes are colored black.
pub const DEFAULT_NODE_COLORING_FN: DefaultNodeColoringFn = |_| "black".to_string();
/// Default function to generate edge colors. All edges are colored black.
pub const DEFAULT_EDGE_COLORING_FN: DefaultEdgeColoringFn = |_| "black".to_string();

pub(crate) type DefaultNodeLabelFn = fn(petgraph::prelude::NodeIndex) -> String;
pub(crate) type DefaultEdgeLabelFn = fn(petgraph::prelude::EdgeIndex) -> String;
pub(crate) type DefaultNodeColoringFn = fn(petgraph::prelude::NodeIndex) -> String;
pub(crate) type DefaultEdgeColoringFn = fn(petgraph::prelude::EdgeIndex) -> String;

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
    PositionMapFn = DefaultPositionMapFn,
    NodeLabelFn = DefaultNodeLabelFn,
    EdgeLabelFn = DefaultEdgeLabelFn,
    NodeColoringFn = DefaultNodeColoringFn,
    EdgeColoringFn = DefaultEdgeColoringFn,
> {
    pub(crate) width: f32,
    pub(crate) height: f32,
    pub(crate) radius: f32,
    pub(crate) font_size: f32,
    pub(crate) stroke_width: f32,
    pub(crate) margin_x: f32,
    pub(crate) margin_y: f32,
    pub(crate) layout_or_pos_map: LayoutOrPositionMap<PositionMapFn>,
    pub(crate) node_label_fn: NodeLabelFn,
    pub(crate) edge_label_fn: EdgeLabelFn,
    pub(crate) node_coloring_fn: NodeColoringFn,
    pub(crate) edge_coloring_fn: EdgeColoringFn,
}

impl Default for Settings<DefaultPositionMapFn, DefaultNodeLabelFn, DefaultEdgeLabelFn> {
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
            layout_or_pos_map: DEFAULT_LAYOUT_OR_POS_MAP,
            node_label_fn: DEFAULT_NODE_LABEL_FN,
            edge_label_fn: DEFAULT_EDGE_LABEL_FN,
            node_coloring_fn: DEFAULT_NODE_COLORING_FN,
            edge_coloring_fn: DEFAULT_EDGE_COLORING_FN,
        }
    }
}

impl Settings<DefaultPositionMapFn, DefaultNodeLabelFn, DefaultEdgeLabelFn> {
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
pub struct SettingsBuilder<PositionMapFn, NodeLabelFn, EdgeLabelFn, NodeColoringFn, EdgeColoringFn>
{
    /// Width of the SVG and output image in pixels.
    ///
    /// **Valid values**: strictly positive f32
    pub width: f32,

    /// Height of the SVG and output image in pixels.
    ///
    /// **Valid values**: strictly positive f32
    pub height: f32,

    /// Radius of the nodes in pixels.
    ///
    /// **Valid values**: strictly positive f32
    pub node_radius: f32,

    /// Font size for labels in pixels.
    ///
    /// **Valid values**: strictly positive f32
    pub font_size: f32,

    /// Stroke width for edges in pixels.
    ///
    /// **Valid values**: strictly positive f32
    pub stroke_width: f32,

    /// Horizontal margin as a fraction of the width.
    /// That is, 0.1 means 10% margin on left and right, leaving 80% of the width for drawing.
    ///
    /// **Valid values**: f32 in range [0.0, 0.5)
    pub margin_x: f32,

    /// Vertical margin as a fraction of the height.
    /// That is, 0.1 means 10% margin on top and bottom, leaving 80% of the height for drawing.
    ///
    /// **Valid values**: f32 in range [0.0, 0.5)
    pub margin_y: f32,

    /// Layout algorithm for graph visualization. If none is provided, the [`DEFAULT_LAYOUT`] will be used.
    ///
    /// **Valid values**: If a `PositionMap` is used, the provided function must implement
    /// `impl Fn(G::NodeId) -> (f32, f32)`. Furthermore, the position map should return normalized
    /// positions in the range [0.0, 1.0].
    pub layout_or_pos_map: LayoutOrPositionMap<PositionMapFn>,

    /// Function to generate node labels. If none is provided, node indexes will be used as labels.
    ///
    /// **Valid values**: Functions that implement `impl Fn(G::NodeId) -> String`.
    pub node_label_fn: NodeLabelFn,

    /// Function to generate edge labels. If none is provided, no edge labels will be drawn.
    ///
    /// **Valid values**: Functions that implement `impl Fn(G::EdgeId) -> String`.
    pub edge_label_fn: EdgeLabelFn,

    /// Function to generate node colors. If none is provided, all nodes will be colored black.
    ///
    /// **Valid values**: Functions that implement `impl Fn(G::NodeId) -> String`.
    /// The returned string should be a valid SVG color (e.g., "red", "#ff0000", "rgb(255,0,0)").
    /// See [https://graphviz.org/doc/info/colors.html#svg](https://graphviz.org/doc/info/colors.html#svg)
    /// for a list of valid SVG color names.
    pub node_coloring_fn: NodeColoringFn,

    /// Function to generate edge colors. If none is provided, all edges will be colored black.
    ///
    /// **Valid values**: Functions that implement `impl Fn(G::EdgeId) -> String`.
    /// The returned string should be a valid SVG color (e.g., "red", "#ff0000", "rgb(255,0,0)").
    /// See [https://graphviz.org/doc/info/colors.html#svg](https://graphviz.org/doc/info/colors.html#svg)
    /// for a list of valid SVG color names.
    pub edge_coloring_fn: EdgeColoringFn,
}

impl Default
    for SettingsBuilder<
        DefaultPositionMapFn,
        DefaultNodeLabelFn,
        DefaultEdgeLabelFn,
        DefaultNodeColoringFn,
        DefaultEdgeColoringFn,
    >
{
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
            layout_or_pos_map: DEFAULT_LAYOUT_OR_POS_MAP,
            node_label_fn: DEFAULT_NODE_LABEL_FN,
            edge_label_fn: DEFAULT_EDGE_LABEL_FN,
            node_coloring_fn: DEFAULT_NODE_COLORING_FN,
            edge_coloring_fn: DEFAULT_EDGE_COLORING_FN,
        }
    }
}

impl
    SettingsBuilder<
        DefaultPositionMapFn,
        DefaultNodeLabelFn,
        DefaultEdgeLabelFn,
        DefaultNodeColoringFn,
        DefaultEdgeColoringFn,
    >
{
    /// Creates a new `SettingsBuilder` instance with default values.
    ///
    /// For default values, see the `DEFAULT_*` constants.
    pub fn new() -> Self {
        SettingsBuilder::default()
    }
}

impl<PositionMapFn, NodeLabelFn, EdgeLabelFn, NodeColoringFn, EdgeColoringFn>
    SettingsBuilder<PositionMapFn, NodeLabelFn, EdgeLabelFn, NodeColoringFn, EdgeColoringFn>
{
    /// Sets the width of the SVG canvas and returns the modified [`SettingsBuilder`].
    ///
    /// For valid values, see the field documentation.
    ///
    /// The default width is [`DEFAULT_WIDTH`].
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the SVG canvas and returns the modified [`SettingsBuilder`].
    ///
    /// For valid values, see the field documentation.
    ///
    /// The default height is [`DEFAULT_HEIGHT`].
    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// Sets the radius of the nodes in pixels and returns the modified [`SettingsBuilder`].
    ///
    /// For valid values, see the field documentation.
    ///
    /// The default radius is [`DEFAULT_RADIUS`].
    pub fn node_radius(mut self, radius: f32) -> Self {
        self.node_radius = radius;
        self
    }

    /// Sets the font size for labels in pixels and returns the modified [`SettingsBuilder`].
    ///
    /// For valid values, see the field documentation.
    ///
    /// The default font size is [`DEFAULT_FONT_SIZE`].
    pub fn font_size(mut self, font_size: f32) -> Self {
        self.font_size = font_size;
        self
    }

    /// Sets the stroke width for edges in pixels and returns the modified [`SettingsBuilder`].
    ///
    /// For valid values, see the field documentation.
    ///
    /// The default stroke width is [`DEFAULT_STROKE_WIDTH`].
    pub fn stroke_width(mut self, stroke_width: f32) -> Self {
        self.stroke_width = stroke_width;
        self
    }

    /// Sets the horizontal margin as a fraction of the width and returns the modified [`SettingsBuilder`].
    ///
    /// For valid values, see the field documentation.
    ///
    /// The default margin is [`DEFAULT_MARGIN`].
    pub fn margin_x(mut self, margin_x: f32) -> Self {
        self.margin_x = margin_x;
        self
    }

    /// Sets the vertical margin as a fraction of the height and returns the modified [`SettingsBuilder`].
    ///
    /// For valid values, see the field documentation.
    ///
    /// The default margin is [`DEFAULT_MARGIN`].
    pub fn margin_y(mut self, margin_y: f32) -> Self {
        self.margin_y = margin_y;
        self
    }

    /// Sets the layout algorithm and returns the modified [`SettingsBuilder`].
    ///
    /// Note that this overrides any position map previously set using the
    /// [`SettingsBuilder::position_map`] method.
    ///
    /// To provide a custom position map, use the [`SettingsBuilder::position_map`] method instead.
    pub fn layout(
        self,
        layout: Layout,
    ) -> SettingsBuilder<
        DefaultPositionMapFn,
        NodeLabelFn,
        EdgeLabelFn,
        NodeColoringFn,
        EdgeColoringFn,
    > {
        SettingsBuilder {
            width: self.width,
            height: self.height,
            node_radius: self.node_radius,
            font_size: self.font_size,
            stroke_width: self.stroke_width,
            margin_x: self.margin_x,
            margin_y: self.margin_y,
            layout_or_pos_map: LayoutOrPositionMap::Layout(layout),
            node_label_fn: self.node_label_fn,
            edge_label_fn: self.edge_label_fn,
            node_coloring_fn: self.node_coloring_fn,
            edge_coloring_fn: self.edge_coloring_fn,
        }
    }

    /// Sets the custom position map function and returns the modified [`SettingsBuilder`].
    ///
    /// For valid position map functions, see the field documentation.
    ///
    /// Note that this overrides any layout algorithm previously set using the
    /// [`SettingsBuilder::layout`] method.
    ///
    /// To use a predefined layout algorithm, use the [`SettingsBuilder::layout`] method instead.
    pub fn position_map<NewPositionMapFn>(
        self,
        position_map: NewPositionMapFn,
    ) -> SettingsBuilder<NewPositionMapFn, NodeLabelFn, EdgeLabelFn, NodeColoringFn, EdgeColoringFn>
    where
        NewPositionMapFn: Fn(petgraph::prelude::NodeIndex) -> (f32, f32),
    {
        SettingsBuilder {
            width: self.width,
            height: self.height,
            node_radius: self.node_radius,
            font_size: self.font_size,
            stroke_width: self.stroke_width,
            margin_x: self.margin_x,
            margin_y: self.margin_y,
            layout_or_pos_map: LayoutOrPositionMap::PositionMap(position_map),
            node_label_fn: self.node_label_fn,
            edge_label_fn: self.edge_label_fn,
            node_coloring_fn: self.node_coloring_fn,
            edge_coloring_fn: self.edge_coloring_fn,
        }
    }

    /// Sets the node label function and returns the modified [`SettingsBuilder`].
    ///
    /// For valid node label functions, see the field documentation.
    pub fn node_label_fn<NewNodeLabelFn>(
        self,
        node_label: NewNodeLabelFn,
    ) -> SettingsBuilder<PositionMapFn, NewNodeLabelFn, EdgeLabelFn, NodeColoringFn, EdgeColoringFn>
    where
        NewNodeLabelFn: Fn(petgraph::prelude::NodeIndex) -> String,
    {
        SettingsBuilder {
            width: self.width,
            height: self.height,
            node_radius: self.node_radius,
            font_size: self.font_size,
            stroke_width: self.stroke_width,
            margin_x: self.margin_x,
            margin_y: self.margin_y,
            layout_or_pos_map: self.layout_or_pos_map,
            node_label_fn: node_label,
            edge_label_fn: self.edge_label_fn,
            node_coloring_fn: self.node_coloring_fn,
            edge_coloring_fn: self.edge_coloring_fn,
        }
    }

    /// Sets the edge label function and returns the modified [`SettingsBuilder`].
    ///
    /// For valid edge label functions, see the field documentation.
    pub fn edge_label_fn<NewEdgeLabelFn>(
        self,
        edge_label: NewEdgeLabelFn,
    ) -> SettingsBuilder<PositionMapFn, NodeLabelFn, NewEdgeLabelFn, NodeColoringFn, EdgeColoringFn>
    where
        NewEdgeLabelFn: Fn(petgraph::prelude::EdgeIndex) -> String,
    {
        SettingsBuilder {
            width: self.width,
            height: self.height,
            node_radius: self.node_radius,
            font_size: self.font_size,
            stroke_width: self.stroke_width,
            margin_x: self.margin_x,
            margin_y: self.margin_y,
            layout_or_pos_map: self.layout_or_pos_map,
            node_label_fn: self.node_label_fn,
            edge_label_fn: edge_label,
            node_coloring_fn: self.node_coloring_fn,
            edge_coloring_fn: self.edge_coloring_fn,
        }
    }

    /// Sets the node coloring function and returns the modified [`SettingsBuilder`].
    ///
    /// For valid node coloring functions, see the field documentation.
    pub fn node_coloring_fn<NewNodeColoringFn>(
        self,
        node_coloring: NewNodeColoringFn,
    ) -> SettingsBuilder<PositionMapFn, NodeLabelFn, EdgeLabelFn, NewNodeColoringFn, EdgeColoringFn>
    where
        NewNodeColoringFn: Fn(petgraph::prelude::NodeIndex) -> String,
    {
        SettingsBuilder {
            width: self.width,
            height: self.height,
            node_radius: self.node_radius,
            font_size: self.font_size,
            stroke_width: self.stroke_width,
            margin_x: self.margin_x,
            margin_y: self.margin_y,
            layout_or_pos_map: self.layout_or_pos_map,
            node_label_fn: self.node_label_fn,
            edge_label_fn: self.edge_label_fn,
            node_coloring_fn: node_coloring,
            edge_coloring_fn: self.edge_coloring_fn,
        }
    }

    /// Sets the edge coloring function and returns the modified [`SettingsBuilder`].
    ///
    /// For valid edge coloring functions, see the field documentation.
    pub fn edge_coloring_fn<NewEdgeColoringFn>(
        self,
        edge_coloring: NewEdgeColoringFn,
    ) -> SettingsBuilder<PositionMapFn, NodeLabelFn, EdgeLabelFn, NodeColoringFn, NewEdgeColoringFn>
    where
        NewEdgeColoringFn: Fn(petgraph::prelude::EdgeIndex) -> String,
    {
        SettingsBuilder {
            width: self.width,
            height: self.height,
            node_radius: self.node_radius,
            font_size: self.font_size,
            stroke_width: self.stroke_width,
            margin_x: self.margin_x,
            margin_y: self.margin_y,
            layout_or_pos_map: self.layout_or_pos_map,
            node_label_fn: self.node_label_fn,
            edge_label_fn: self.edge_label_fn,
            node_coloring_fn: self.node_coloring_fn,
            edge_coloring_fn: edge_coloring,
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
    pub fn build(
        self,
    ) -> Result<
        Settings<PositionMapFn, NodeLabelFn, EdgeLabelFn, NodeColoringFn, EdgeColoringFn>,
        InvalidSettingsError,
    >
    where
        PositionMapFn: Fn(petgraph::prelude::NodeIndex) -> (f32, f32),
        NodeLabelFn: Fn(petgraph::prelude::NodeIndex) -> String,
        EdgeLabelFn: Fn(petgraph::prelude::EdgeIndex) -> String,
        NodeColoringFn: Fn(petgraph::prelude::NodeIndex) -> String,
        EdgeColoringFn: Fn(petgraph::prelude::EdgeIndex) -> String,
    {
        self.validate()?;
        let settings = Settings {
            width: self.width,
            height: self.height,
            radius: self.node_radius,
            font_size: self.font_size,
            stroke_width: self.stroke_width,
            margin_x: self.margin_x,
            margin_y: self.margin_y,
            layout_or_pos_map: self.layout_or_pos_map,
            node_label_fn: self.node_label_fn,
            edge_label_fn: self.edge_label_fn,
            node_coloring_fn: self.node_coloring_fn,
            edge_coloring_fn: self.edge_coloring_fn,
        };
        Ok(settings)
    }
}
