/* tslint:disable */
import * as wasm from './chip8_bg';

/**
*/
export const RunMode = Object.freeze({ Legacy:0,SuperChip:1, });

let cachegetUint8Memory = null;
function getUint8Memory() {
    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory;
}

let WASM_VECTOR_LEN = 0;

function passArray8ToWasm(arg) {
    const ptr = wasm.__wbindgen_malloc(arg.length * 1);
    getUint8Memory().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}
/**
* @returns {void}
*/
export function execute_cycle() {
    return wasm.execute_cycle();
}

/**
* @returns {number}
*/
export function get_mem() {
    return wasm.get_mem();
}

/**
* @returns {number}
*/
export function get_vid_mem() {
    return wasm.get_vid_mem();
}

/**
* @param {number} arg0
* @returns {void}
*/
export function key_pressed(arg0) {
    return wasm.key_pressed(arg0);
}

let cachedTextDecoder = new TextDecoder('utf-8');

function getStringFromWasm(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}

let cachedGlobalArgumentPtr = null;
function globalArgumentPtr() {
    if (cachedGlobalArgumentPtr === null) {
        cachedGlobalArgumentPtr = wasm.__wbindgen_global_argument_ptr();
    }
    return cachedGlobalArgumentPtr;
}

let cachegetUint32Memory = null;
function getUint32Memory() {
    if (cachegetUint32Memory === null || cachegetUint32Memory.buffer !== wasm.memory.buffer) {
        cachegetUint32Memory = new Uint32Array(wasm.memory.buffer);
    }
    return cachegetUint32Memory;
}
/**
* @returns {string}
*/
export function dump_registers() {
    const retptr = globalArgumentPtr();
    wasm.dump_registers(retptr);
    const mem = getUint32Memory();
    const rustptr = mem[retptr / 4];
    const rustlen = mem[retptr / 4 + 1];

    const realRet = getStringFromWasm(rustptr, rustlen).slice();
    wasm.__wbindgen_free(rustptr, rustlen * 1);
    return realRet;

}

/**
* @returns {string}
*/
export function dump_key_mem() {
    const retptr = globalArgumentPtr();
    wasm.dump_key_mem(retptr);
    const mem = getUint32Memory();
    const rustptr = mem[retptr / 4];
    const rustlen = mem[retptr / 4 + 1];

    const realRet = getStringFromWasm(rustptr, rustlen).slice();
    wasm.__wbindgen_free(rustptr, rustlen * 1);
    return realRet;

}

function freeRam(ptr) {

    wasm.__wbg_ram_free(ptr);
}
/**
*/
export class Ram {

    static __wrap(ptr) {
        const obj = Object.create(Ram.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;
        freeRam(ptr);
    }

    /**
    * @returns {Ram}
    */
    static new() {
        return Ram.__wrap(wasm.ram_new());
    }
    /**
    * @returns {number}
    */
    get_meta_address() {
        return wasm.ram_get_meta_address(this.ptr);
    }
    /**
    * @returns {number}
    */
    get_length() {
        return wasm.ram_get_length(this.ptr);
    }
    /**
    * @param {boolean} arg0
    * @returns {void}
    */
    print(arg0) {
        return wasm.ram_print(this.ptr, arg0);
    }
    /**
    * @param {Uint8Array} arg0
    * @returns {void}
    */
    write_rom(arg0) {
        const ptr0 = passArray8ToWasm(arg0);
        const len0 = WASM_VECTOR_LEN;
        return wasm.ram_write_rom(this.ptr, ptr0, len0);
    }
    /**
    * @param {number} arg0
    * @returns {number}
    */
    read(arg0) {
        return wasm.ram_read(this.ptr, arg0);
    }
    /**
    * @param {number} arg0
    * @param {number} arg1
    * @returns {void}
    */
    write(arg0, arg1) {
        return wasm.ram_write(this.ptr, arg0, arg1);
    }
}

export function __wbindgen_throw(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
}

