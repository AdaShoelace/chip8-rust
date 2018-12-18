/* tslint:disable */
export const memory: WebAssembly.Memory;
export function __wbg_ram_free(a: number): void;
export function ram_new(): number;
export function ram_get_meta_address(a: number): number;
export function ram_get_length(a: number): number;
export function ram_print(a: number, b: number): void;
export function ram_write_rom(a: number, b: number, c: number): void;
export function ram_read(a: number, b: number): number;
export function ram_write(a: number, b: number, c: number): void;
export function execute_cycle(): void;
export function get_mem(): number;
export function get_vid_mem(): number;
export function key_pressed(a: number): void;
export function dump_registers(a: number): void;
export function dump_key_mem(a: number): void;
export function __wbindgen_global_argument_ptr(): number;
export function __wbindgen_malloc(a: number): number;
export function __wbindgen_free(a: number, b: number): void;
