# Chess_browser

A simple program to play chess in the browser.

## Building

### Requirements

- [`cargo`](https://doc.rust-lang.org/cargo/index.html)
- [`wasm-pack`](https://rustwasm.github.io/wasm-pack/)
- [`npm`](https://docs.npmjs.com/)

### Running

```
npm install
npm run build
npm start
```

The server should be accessible at http://localhost:8080.


## How does it work?

This application is made of four parts:
- `chess_std`, an internal library written in Rust that provides an efficient game representation and file parsing
- WebAssembly bindings to make `chess_std` accessible from Javascript
- An browser frontend written in TypeScript.

## Work in progress

Objective: play in the browser with a fast chess engine written in Rust.