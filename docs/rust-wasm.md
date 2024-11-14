# Set up Rust Wasm

## 1. Rust to JS package

1. Create a library project in Rust
    ```
    cargo new <project-name> --lib
    ```
2. Config `Cargo.toml`
    ```toml=
    // default information
    
    [dependencies]
    // make Rust can be touched from JS (use Wasm)
    wasm-bindgen = "0.2.63"
    // minimized the size of WebAssembly
    wee_alloc = "0.4.5"

    [lib]
    // C dynamic library
    // allows compile Rust into Wasm library
    crate-type = ["cdylib"]

    [package.metadata.wasm-pack.profile.release]
    wasm-opt = false
    ```
3. Install `wasm-pack`:
    ```
    cargo install wasm-pack
    ```
4. Compile Rust code into JS package: 
    ```
    wasm-pack build --target web
    ```
    A `pkg/` directory will be created:
    ```
    pkg/
    ├── <package-name>.js         # Main JavaScript wrapper
    ├── <package-name>_bg.wasm    # Compiled WebAssembly binary
    ├── <package-name>_bg.js      # JavaScript bindings for Wasm
    ├── package.json              # Package metadata for npm
    ├── README.md                 # Usage instructions
    └── <package-name>.d.ts       # TypeScript declarations (optional)
    ```
5. Add JS package to `package.json`:
    ```json
    "dependencies": {
        "<rust-wasm-pkg-name>": "file:<path-to-pkg>",
        // other packages
      },
    ```
    Note that no whilespaces between `file:` and `<path-to-pkg>`
6. Use that package (in `index.js`):
    ```typescript=
    import init, {World} from "<rust-wasm-pkg-name>";
    
    init().then(wasm => {
        const world = World.new();
        // ...
    }
    ```
    Note that `wasm.World.new()` is a WebAssembly function, so import `World` from `<rust-wasm-pkg-name>` to use a normal JS function!
    
That all you need to set up a Rust project that will create a JS package. Anytime you change your Rust library, run `wasm-pack build --target web` to update JS package.

## 2. Rust Wasm project

In Rust file (`src/lib.rs`), we need to follow some rules to make a function in Rust can be used in JS.

1. Import `wasm-bindgen`:
    ```rust
    use wasm_bindgen::prelude::*;
    ```
2. Make a Rust function/struct/... can be used in JS package
    ```rust=
    #[wasm_bindgen]
    /* your function/struct goes here */
    ```
    Example:
    ```rust=
    #[wasm_bindgen]
    pub struct RewardCell(usize);
    ```
    Note that these part of code need to be public (`pub`)
3. Use functions from JS
    ```rust=
    #[wasm_bindgen(module = "<path-to-JS-module>")]
    extern {
        /* list of function signatures */
    }
    ```
    Example:
    ```rust=
    #[wasm_bindgen(module = "/www/utils/rand.js")]
    extern {
        fn random(max: usize) -> usize;
        // other functions
    }
    ```
    
## 3. wee_alloc
[Github - wee_alloc](https://github.com/rustwasm/wee_alloc)