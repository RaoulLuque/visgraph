<p align="center">
  <img src="examples/results/complete_graph_with_circular_layout.png#gh-light-mode-only"
       width="400"
       alt="Visualization of complete graph with 100 nodes in circular layout. (light mode).">
  <img src="examples/results/complete_graph_with_circular_layout.png#gh-dark-mode-only"
       width="400"
       alt="Visualization of complete graph with 100 nodes in circular layout. (dark mode).">¹
</p>


visgraph
===
visgraph is an easy-to-use Rust library for visualizing graphs
using various layout algorithms and exporting them to simple image formats like PNG or even SVG.

Supports Rust 1.68 and later. This will only change on major releases.

[![Crates.io][crates-badge]][crates-url]
[![docs.rs][docsrs-badge]][docsrs]
![MSRV][msrv-badge]
[![Build Status][build-status]][ci-url]

## Usage

For more examples, see the [examples directory](examples).

```rust
// This example is taken from examples/graph_with_default_settings.rs
use petgraph::graph::UnGraph;
use visgraph::{graph_to_img_with_layout, settings::Settings};

// Create a complete graph with 4 nodes.
let mut complete_graph = UnGraph::new_undirected();
let num_nodes = 4;
let nodes: Vec<_> = (0..num_nodes)
    .map(|_| complete_graph.add_node(()))
    .collect();

for i in 0..num_nodes {
    for j in (i + 1)..num_nodes {
        complete_graph.add_edge(nodes[i], nodes[j], ());
    }
}

// This is the actual functionality of this lib:
// Generate and save the graph image using a circular layout and default settings.
graph_to_img_with_layout(
    &complete_graph,
    visgraph::Layout::Circular,
    &Settings::default(),
    "examples/results/graph_with_default_settings.png",
)
.unwrap();
```

### Performance

visgraphs performance can be greatly improved  by enabling optimizations. To do so, either build
your entire project in release mode, or enable optimizations for just visgraph by adding the
following to your `Cargo.toml`:

```toml
[profile.dev.package.visgraph]
opt-level = 3
```

## Documentation

* [API documentation on docs.rs][docsrs]
* [Examples in the examples directory](examples)

### Crate features

visgraph currently only has a single feature which is enabled by default:

- `svg_to_img`: Enables conversion from SVG to PNG using resvg. If you
  disable this feature, only SVG output will be available, but the crate
  will be smaller, as resvg is a large dependency.

## Getting Help

First, see if the answer to your question can be found in the
[API documentation][docsrs]. If the answer is not there, feel free
to ask your question on the [discussions page][github-discussions].
I'd be happy to try to answer your question. If you find a bug,
or have a feature request, please [open an issue][github-new-issue].

## Contributing

🦕 Thanks for your help improving the project! There's no contribution guide yet, but feel free 
to open an issue if you'd like to help out or just open a PR directly and we can discuss the changes 
there.

## License

Dual-licensed to be compatible with the Rust project.

Licensed under the [Apache License, Version 2.0][apache-license] or
the [MIT license][mit-license], at your option. This file may
not be copied, modified, or distributed except according to those
terms.

¹Image generated using visgraph, although coloring edges and nodes is not part of the public API yet.

[apache-license]: http://www.apache.org/licenses/LICENSE-2.0

[build-status]: https://github.com/raoulluque/visgraph/actions/workflows/ci.yml/badge.svg

[ci-url]: https://github.com/raoulluque/visgraph/actions/workflows/ci.yml

[crates-badge]: https://img.shields.io/crates/v/visgraph.svg

[crates-url]: https://crates.io/crates/visgraph

[docsrs]: https://docs.rs/visgraph/latest/visgraph/

[docsrs-badge]: https://img.shields.io/docsrs/visgraph

[github-discussions]: https://github.com/visgraph/visgraph/discussions

[github-new-issue]: https://github.com/visgraph/visgraph/issues/new

[mit-license]: http://opensource.org/licenses/MIT

[msrv-badge]: https://img.shields.io/badge/rustc-1.68+-blue.svg