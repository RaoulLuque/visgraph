# Lists all available recipes
default:
    just --list

# Builds the project in development mode
build:
    cargo build

# Tests with all features enabled
test:
    cargo test --all --all-features

# Miri with all tests (this might take very long). Consider the fast-miri recipe instead or specify individual tests
miri:
    cargo miri test

# Fmt with the same configuration as in CI
fmt:
    cargo fmt --all -- --check

# Clippy with the same configuration as in CI
clippy:
    cargo clippy --all-features --lib --bins --examples --tests -- -D warnings

# Runs all linting checks that are run in CI
lint: fmt clippy

# Runs all tests, linting and examples that are run in CI
ci: fmt clippy test examples

# Builds the documentation
docs:
    cargo doc --no-deps --all-features --open

# Runs all examples
examples:
    cargo run --example bipartite_layout --release --features "img"
    cargo run --example circular_layout --release --features "img"
    cargo run --example custom_colors --release --features "img"
    cargo run --example custom_labels --release --features "img"
    cargo run --example default_settings --release --features "img"
    cargo run --example force_directed_layout --release --features "img"
    cargo run --example force_directed_layout_big --release --features "img"
    cargo run --example graph_to_svg --release --features "img"
    cargo run --example hierarchical_layout --release --features "img"
    cargo run --example position_map --release --features "img"