## Init
* rustc: 1.82.0 (stable)
* wasm-pack: v0.13.1

## How to run 

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

