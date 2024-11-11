async function init() {
    // import some JS code that we want to use in WebAssembly code
    const importObject = {
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

    // get functions from webassembly and use it in js
    const sumFunction = wasm.instance.exports.sum;
    const result = sumFunction(5,62);
    console.log(result);
    
    // get the memory $mem in wasm and use it in js
    const wasmMemory = wasm.instance.exports.mem;
    // convert 2 bytes, from byte 0 of wasmMemory into an array
    const uint8Array = new Uint8Array(wasmMemory.buffer, 0, 2); 
    const hiText = new TextDecoder().decode(uint8Array);
    console.log(hiText);
}

init();