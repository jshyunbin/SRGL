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

**SRGL** is designed to be as simple and easy to use. You can open an empty window with just 
few lines of code!


## Features

**SRGL** is in a very early stage of development and many of the 
features are still work in progress.

 - 2D rendering 
 - anti-aliasing (wip)
 - 3D rendering (wip)
 - shading (Flat, Gouraud, Phong) (wip)
 - create vertices, polygons (wip)
 - ray tracing (wip)
 - keyboard/mouse interface (wip)
 - frame rate control (wip)

## Examples

Examples are available in `/examples`.

- [Simple2D](./examples/simple2d)

