import { compress, decompress, from_ron, to_ron } from "./testwasm.ts"

const textDecoder = new TextDecoder('utf-8');
const textEncoder = new TextEncoder();


const ron = await to_ron({
    "data": 123, nested: {
        key2: {
            key3: {
                some_more: "data"
            },
            array: [1, 2, 4, "123"]
        }
    }
});

console.log("to_ron", ron)
console.log("from_ron", await from_ron(ron))

const encoded = textEncoder.encode(ron)
const compressed = await compress(encoded, 10);

console.log("compress: before", encoded.length, "after", compressed.length);

const decompressed = await decompress(compressed);

console.log("decompressed ", ron == textDecoder.decode(decompressed));