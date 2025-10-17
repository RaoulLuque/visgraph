use petgraph::visit::{IntoNodeReferences, NodeIndexable, NodeRef};

pub enum Layout {
    Circular,
    Hierarchical,
}
