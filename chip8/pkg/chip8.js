/* tslint:disable */
import * as wasm from './chip8_bg';
import { setMainLoop } from '../www/index';

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

export function __wbg_setMainLoop_784f7d3a90ae3108(arg0, arg1) {
    let cbarg0 = function() {
        let a = this.a;
        this.a = 0;
        try {
            return this.f(a, this.b);

        } finally {
            this.a = a;

        }

    };
    cbarg0.f = wasm.__wbg_function_table.get(3);
    cbarg0.a = arg0;
    cbarg0.b = arg1;
    try {
        setMainLoop(cbarg0.bind(cbarg0));
    } finally {
        cbarg0.a = cbarg0.b = 0;

    }
}

const heap = new Array(32);

heap.fill(undefined);

heap.push(undefined, null, true, false);

let heap_next = heap.length;

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}
/**
* @param {any} arg0
* @returns {void}
*/
export function run(arg0) {
    return wasm.run(addHeapObject(arg0));
}

function getObject(idx) { return heap[idx]; }

export function __wbg_forEach_b66b0db0fe3d89ad(arg0, arg1, arg2) {
    let cbarg1 = function(arg0, arg1, arg2) {
        let a = this.a;
        this.a = 0;
        try {
            return this.f(a, this.b, arg0, arg1, addHeapObject(arg2));

        } finally {
            this.a = a;

        }

    };
    cbarg1.f = wasm.__wbg_function_table.get(21);
    cbarg1.a = arg1;
    cbarg1.b = arg2;
    try {
        getObject(arg0).forEach(cbarg1.bind(cbarg1));
    } finally {
        cbarg1.a = cbarg1.b = 0;

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

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

export function __wbindgen_object_drop_ref(i) { dropObject(i); }

let cachedTextDecoder = new TextDecoder('utf-8');

function getStringFromWasm(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}

export function __wbindgen_throw(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
}

