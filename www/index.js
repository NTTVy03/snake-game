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

    const sumFunction = wasm.instance.exports.sum;
    const result = sumFunction(5,62);

    console.log(result);
}

init();