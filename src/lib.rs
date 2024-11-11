use wasm_bindgen::prelude::*;

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