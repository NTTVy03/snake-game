use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

// we_alloc makes the wasm file smaller

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen]
pub fn greet(name: &str) {
    // call js function in rust code
    alert(name);
}

// import functions from js to use in rust
#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

// wasm-pack build --target web