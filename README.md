# WebGL brownian tree visualization

This is a work-in-progress, toy project aimed at creating a 3D visualization of a brownian tree using
Rust WebAsm and WebGL.

# Prerequisites

* Install Rust (eg.: [using rustup](https://www.rust-lang.org/tools/install))
* Install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

## Building

```
npm install
cargo build
```

## Running

```
npm run serve
```

Then open `http://localhost:8080` on your brower. You should see a rendering like the one below:

![screenshot-brownian-tree](https://raw.githubusercontent.com/felipecsl/brownian-tree-webgl/master/screenshot.png)

## Input

The brownian tree input data is consumed in CSV format from a file under `src/input.csv`.
The CSV file is generated using https://github.com/fogleman/dlaf.

## License

MIT