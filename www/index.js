import init, {greet} from "snake_game";

// note that the init() return a wasm
// wasm.greet() is a webassembly function
// so, you must pass number as parameter 
// (webassembly does not have complex data types)
// use greet from "snake_game", it is a js function
init().then( _ => {
    greet("Vy");
})