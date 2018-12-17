import * as wasm from "../pkg/chip8";

import { memory } from '../pkg/chip8_bg'

let ptr = wasm.get_mem();
let buf = new Uint8Array(memory.buffer);
let value = buf[ptr]

alert(value)

