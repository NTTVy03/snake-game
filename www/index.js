import init from "snake_game";

// because the println!() in Rust does not print anything to the console
// we can not see the greeting message
// however, this code still works
init().then( wasm => {
    wasm.greet("Vy");
})