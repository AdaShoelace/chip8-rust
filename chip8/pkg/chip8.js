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

function passArray8ToWasm(arg) {
    const ptr = wasm.__wbindgen_malloc(arg.length * 1);
    getUint8Memory().set(arg, ptr / 1);
    return [ptr, arg.length];
}
/**
* @param {number} arg0
* @returns {Chip}
*/
export function get_chip(arg0) {
    return Chip.__wrap(wasm.get_chip(arg0));
}

function freeChip(ptr) {

    wasm.__wbg_chip_free(ptr);
}
/**
*/
export class Chip {

    static __wrap(ptr) {
        const obj = Object.create(Chip.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;
        freeChip(ptr);
    }

    /**
    * @param {number} arg0
    * @returns {Chip}
    */
    static new(arg0) {
        return Chip.__wrap(wasm.chip_new(arg0));
    }
    /**
    * @param {boolean} arg0
    * @returns {void}
    */
    print_mem(arg0) {
        return wasm.chip_print_mem(this.ptr, arg0);
    }
    /**
    * @param {Uint8Array} arg0
    * @returns {void}
    */
    load_rom(arg0) {
        const [ptr0, len0] = passArray8ToWasm(arg0);
        return wasm.chip_load_rom(this.ptr, ptr0, len0);
    }
    /**
    * @returns {void}
    */
    emulate_cycle() {
        return wasm.chip_emulate_cycle(this.ptr);
    }
    /**
    * @param {number} arg0
    * @returns {void}
    */
    decode_DXYN(arg0) {
        return wasm.chip_decode_DXYN(this.ptr, arg0);
    }
    /**
    * @param {number} arg0
    * @returns {void}
    */
    debug_print(arg0) {
        return wasm.chip_debug_print(this.ptr, arg0);
    }
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
        const [ptr0, len0] = passArray8ToWasm(arg0);
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

let cachedTextDecoder = new TextDecoder('utf-8');

function getStringFromWasm(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}

export function __wbindgen_throw(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
}

