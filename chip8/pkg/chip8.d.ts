/* tslint:disable */
export enum RunMode {Legacy,SuperChip,}
export function get_chip(arg0: number): Chip;

export class Chip {
free(): void;

static  new(arg0: number): Chip;

 print_mem(arg0: boolean): void;

 load_rom(arg0: Uint8Array): void;

 emulate_cycle(): void;

 decode_DXYN(arg0: number): void;

 debug_print(arg0: number): void;

}
export class Ram {
free(): void;

static  new(): Ram;

 get_length(): number;

 print(arg0: boolean): void;

 write_rom(arg0: Uint8Array): void;

 read(arg0: number): number;

 write(arg0: number, arg1: number): void;

}
