const wasmCode = await Deno.readFile("./target/wasm32-unknown-unknown/debug/wasm_client.wasm");
const wasmModule = new WebAssembly.Module(wasmCode);
const wasmInstance = new WebAssembly.Instance(wasmModule);
const { age, fate_number } = wasmInstance.exports;
const yourAge = age(2020, 1988);
const yourFateNumber = fate_number(1988, 11, 4);
console.log(yourAge);
console.log(yourFateNumber);
