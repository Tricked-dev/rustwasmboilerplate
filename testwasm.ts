const textDecoder = new TextDecoder('utf-8');


const handlers = {
    onToRon: (_data: string) => { },
    onFromRon: (_data: unknown) => { },
    onCompress: (_data: Uint8Array) => { },
    onDecompress: (_data: Uint8Array) => { },
    onError: (_message: string) => { },
}
const importObject: WebAssembly.Imports = {
    console: {
        console_log: function (ptr: number, len: number) {
            const message = uint8ToString(new Uint8Array(memory.buffer, ptr, len));
            console.log(message);
        },
    },

    resulting: {
        to_ron: function (ptr: number, len: number) {
            const savedData = uint8ToString(new Uint8Array(memory.buffer, ptr, len));
            handlers.onToRon(savedData)
            exports.__dealloc(ptr, len)
        },
        from_ron: function (ptr: number, len: number) {
            const savedData = JSON.parse(uint8ToString(new Uint8Array(memory.buffer, ptr, len)));
            handlers.onFromRon(savedData)
            exports.__dealloc(ptr, len)
        },
        compress: function (ptr: number, len: number) {
            //we need to clone it
            const savedData = new Uint8Array(new Uint8Array(memory.buffer, ptr, len));
            handlers.onCompress(savedData)
            exports.__dealloc(ptr, len)
        },
        decompress: function (ptr: number, len: number) {
            //we need to clone it
            const savedData = new Uint8Array(new Uint8Array(memory.buffer, ptr, len));
            handlers.onDecompress(savedData)
            exports.__dealloc(ptr, len)
        },
        error: function (ptr: number, len: number) {
            const message = uint8ToString(new Uint8Array(memory.buffer, ptr, len));
            handlers.onError(message)
            exports.__dealloc(ptr, len)
        }
    }
};


const wasm = await WebAssembly.instantiateStreaming(
    fetch(new URL("library.wasm", import.meta.url)), importObject)

const exports = wasm.instance.exports as {
    to_ron: (ptr: number, len: number) => void
    from_ron: (ptr: number, len: number) => void
    compress: (ptr: number, len: number, level: number) => void
    decompress: (ptr: number, len: number) => void
    error: (ptr: number, len: number) => void
    __alloc: (len: number) => number
    __dealloc: (ptr: number, len: number) => void
    memory: WebAssembly.Memory
};

const memory = exports.memory;

function uint8ToString(data: Uint8Array) {
    return textDecoder.decode(data);
}

function allocUint8Array(data: Uint8Array) {
    const len = data.length;

    const ptr = exports.__alloc(len);

    const memory = new Uint8Array(exports.memory.buffer);

    for (let i = 0; i < len; i++) {
        memory[ptr + i] = data[i];
    }

    return { ptr, len };
}

function stringToPointer(str: string) {
    const textEncoder = new TextEncoder();
    const encodedString = textEncoder.encode(str);
    const ptr = exports.__alloc(encodedString.length);
    const memoryBuffer = new Uint8Array(memory.buffer);

    for (let i = 0; i < encodedString.length; i++) {
        memoryBuffer[ptr + i] = encodedString[i];
    }

    return { ptr, len: encodedString.length };
}


export function to_ron(data: unknown): Promise<string> {
    const { ptr, len } = stringToPointer(JSON.stringify(data));

    const promise = new Promise<string>((resolve, reject) => {
        handlers.onToRon = resolve
        handlers.onError = reject
    })
    exports.to_ron(ptr, len);

    return promise
}

export function from_ron<T>(data: string): Promise<T> {
    const { ptr, len } = stringToPointer(data);

    const promise = new Promise((resolve, reject) => {
        handlers.onFromRon = resolve
        handlers.onError = reject
    })

    exports.from_ron(ptr, len);

    return promise as Promise<T>
}

export function compress(data: Uint8Array, level: 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 = 10): Promise<Uint8Array> {
    const { ptr, len } = allocUint8Array(data);

    const promise = new Promise<Uint8Array>((resolve, reject) => {
        handlers.onCompress = resolve
        handlers.onError = reject
    })

    exports.compress(ptr, len, level);

    return promise
}

export function decompress(data: Uint8Array): Promise<Uint8Array> {
    const { ptr, len } = allocUint8Array(data);

    const promise = new Promise<Uint8Array>((resolve, reject) => {
        handlers.onDecompress = resolve
        handlers.onError = reject
    })

    exports.decompress(ptr, len);

    return promise
}

export function getMemoryLength() {
    return memory.buffer.byteLength
}