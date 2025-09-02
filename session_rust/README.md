
# Rust

TODO: ear clipping algorithm

## Installation 

### Step 1 - Install Rust via Terminal
Install Rust via [RustUp](rust-lang.org/tools/install) Package Manager.

```bash
sudo curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sudo sh
source "$HOME/.cargo/env"
```


### Step 2 - Verify Installation

```bash
cargo --version
```

## Commands

### Create a new project
```bash
cargo new <projectname>
```

### Run a project or specific binary
```bash
cargo run
cargo run --bin openmodel_bin
```

### Run a project without Debug Info
```bash
cargo run -q
```

### Generate Documentation
```bash
# Generate documentation
cargo doc

# Generate documentation and open it in your browser
cargo doc --open

# Include private items in the documentation
cargo doc --document-private-items --open
```

This will create HTML documentation in the `target/doc` directory based on the doc comments in your code. The documentation includes all of your public items (functions, structs, etc.) along with their doc comments.

### Step 3 - VSCode

Extensions: [rust analyzer](https://code.visualstudio.com/docs/languages/rust)


### Step  4 - Example

```rust
mod point;
use point::Point;

fn main() {
    let p0 = Point::new(0.0, 0.0, 0.0);
    let p1 = Point::new(1.0, 1.0, 1.0);
    println!("Distance: {}", p0.distance(&p1));
    println!("Hello, world!");
}
```

### Step 5 - Publish Create


```bash
cargo login
cargo publish
```

### Step 6 -  Formatting
```bash
rustup component add rustfmt
cargo fmt
```

### Step 7 - Test
```bash
cargo test
```

### Step 8 - Submodules


```bash
openmodel/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── main.rs
    ├── geometry/
    │   ├── mod.rs
    │   ├── point.rs
    │   └── vector.rs
```

Declare the geometry module in the main library file: `src/lib.rs`


```rust
pub mod geometry;
```

Declare the submodules point and vector in the geometry module: `src/geometry/mod.rs`


```rust
pub mod point;
pub mod vector;

pub use point::Point;
pub use vector::Vector;
```

Define the Point and Vector struct and its implementation: `src/geometry/point.rs`, `src/geometry/vector.rs`

Use the Point and Vector structs from the geometry module:

```rust
use openmodel::geometry::{Point, Vector};
```
