# Intro to WebAssembly

## 1. What is WebAssembly

WebAssembly is a new type of code that can be run in modern web browsers — it is a low-level assembly-like language with a compact binary format that runs with near-native performance and provides languages such as C/C++, C#, and Rust with a compilation target so that they can run on the web. It is also designed to run alongside JavaScript, allowing both to work together.

## 2. Use WebAssembly in JS

WebAssembly types: i32, i64, f32, f64

Example of WebAssembly code:

```=
(module
    (func $sum (param $a i32) (param $b i32) (result i32)
        local.get $a
        local.get $b 
        i32.add
    )
    
    (export "sum" (func $sum))
)
```

After build a WebAssembly code, we will get binary code from a `.wasm` file (binary file). Now we can import and use it in JS

```html=
<body>
    <scripts>
        async function init() {
            // get the binary code of `.wasm`
            const byteArray = new Int8Array([/* binary code of `.wasm` file here */]);
        
            // init a WebAssembly instance
            const wasm = await WebAssembly.instantiate(byteArray.buffer);
            
            // get functions exported from `.wasm`
            const sumFunction = wasm.instance.exports.sum;
        
            // use that function as a normal one in JS
            const result = sumFunction(5,6);

            console.log(result);
        }

        init();
    </scripts>
</body>
```

## 3. Memory

### a. Create in webassembly and export to JS

#### webassembly

```=
(module
    (memory $mem 1)
    (data (i32.const 0) "Hi")
    (export "mem" (memory 0))
)
```

* `(memory $mem 1)`: create 1 page of memory (~64KB) in webassambly
* `(data (i32.const 0) "Hi")`: init the value ("Hi") for the first memory (`$mem`)
* `(export "mem" (memory 0))`: export the first memory in webassambly, so that js can use that memory (~variable)

#### JS

```javascript=
async function init() {
    // fetch binary code of `.wasm` file
    const response = await fetch("sum.wasm");
    const buffer = await response.arrayBuffer();

    // get the memory $mem in wasm and use it in js
    const wasmMemory = wasm.instance.exports.mem;
    // convert 2 bytes, from byte 0 of wasmMemory into an array
    const uint8Array = new Uint8Array(wasmMemory.buffer, 0, 2); 
    const hiText = new TextDecoder().decode(uint8Array);
    console.log(hiText);
}

init();
```

### b. Create in JS and import to webassembly 

#### JS

```javascript=
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
```

#### webassembly

```=
(module
    (import "console" "log" (func $log))
    (import "console" "error" (func $error))
    (memory (import "js" "mem") 1)
    (data (i32.const 0) "Hi")
    (func $sum (param $a i32) (param $b i32) (result i32)
        call $log
        call $error
        local.get $a
        local.get $b
        i32.add
    )
    (export "sum" (func $sum))
)
```

* `(memory (import "js" "mem") 1)`: import memory (`mem`) from JS
* `(data (i32.const 0) "Hi")`: init the value ("Hi") for the first memory (`$mem`)
* `$log`, `$error` are functions import from JS

## Sources
* [wat2wasm](https://webassembly.github.io/wabt/demo/wat2wasm/): a demo website that converts WebAssembly from the text format to the binary format online, download `.wasm` from text format...
* [WebAssembly Docs](https://developer.mozilla.org/en-US/docs/WebAssembly)