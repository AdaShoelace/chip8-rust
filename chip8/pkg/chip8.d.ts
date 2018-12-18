/* tslint:disable */
export enum RunMode {Legacy,SuperChip,}
export function execute_cycle(): void;

export function get_mem(): number;

export function get_vid_mem(): number;

export function key_pressed(arg0: number): void;

export function dump_registers(): string;

export function dump_key_mem(): string;

export class Ram {
free(): void;

static  new(): Ram;

 get_meta_address(): number;

 get_length(): number;

 print(arg0: boolean): void;

 write_rom(arg0: Uint8Array): void;

 read(arg0: number): number;

 write(arg0: number, arg1: number): void;

}
