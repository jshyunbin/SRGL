# Simple Rust Graphic Library [SRGL]

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**Simple Graphic Library in Rust.**

```rust
use srgl::CanvasBuilder;

fn main() -> Result<(), srgl::Error> {
    CanvasBuilder::new()
        .with_size(400, 300)
        .build().run()
}
```

## Usage

**SRGL** is a Rust based graphic rendering library.
It is designed to be easy, fast and fun!

## Features

- 2D rendering
- anti-aliasing (wip)
- 3D rendering (wip)
- shading (Flat, Gouraud, Phong) (wip)
- ray tracing
- save img files (wip)

## Examples

Examples are available in `/examples`.

- [Simple Window](./examples/simple-window)
- [Simple2D](./examples/simple2d)
- [Sphere Ray Tracing](./examples/sphere-rt)
