# Snake Game on Browser
> Course: [udemy - rust webassembly](https://bosch-bgsv.udemy.com/course/rust-webassembly-with-js-ts-the-practical-guide/)

## 1. Overview
Project: Build a Snake Game runs directly on browser

Target: Convert Rust code into WebAssembly (which can run directly on Browser)

VSCode extentions: rust-analyser, WebAssembly (from WebAssembly Foundation)

## 2. Init
* rustc: 1.82.0 (stable)
* wasm-pack: v0.13.1

## 3. How to run 

```
// You are currently at: snake_game/

// Compile the Rust library
wasm-pack build --target web

// Move to the UI:
cd www

// Update node modules (for the first time add snake_game pkg)
npm install

// Run website
npm run dev
```

