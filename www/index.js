async function init() {
    // init memory in js to use in webassembly
    const memory = new WebAssembly.Memory({initial: 1});

    // import some JS code that we want to use in WebAssembly code
    const importObject = {
        js: {
            mem: memory
        },
        console: {
            log: () => {
                console.log("Just logging something!");
            },
            error: () => {
                console.log("I am an error");
            }
        }
    }

    // fetch binary code of `.wasm` file
    const response = await fetch("sum.wasm");
    const buffer = await response.arrayBuffer();

    const wasm = await WebAssembly.instantiate(buffer, importObject);
    
    // note that, the .wasm has changed the memory 
    
    const uint8Array = new Uint8Array(memory.buffer, 0, 2); 
    const hiText = new TextDecoder().decode(uint8Array);
    console.log(hiText);
}

init();