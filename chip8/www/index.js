
/**
 * Driver program for emulator
 */

import * as wasm from "../pkg/chip8";
import { memory } from '../pkg/chip8_bg'

const ROM_OFFSET = 0x200;


const run = async (rom) => {
	const ptrMainMem = wasm.get_mem();
	const programMemory = new Uint8Array(memory.buffer, ptrMainMem, 4096);

	const vidPtr = wasm.get_vid_mem();
	const vidMem = new Uint8Array(memory.buffer, vidPtr, 64*32);

	document.addEventListener('keypress', event => {
    if(event.keyCode == 37) {
		wasm.key_pressed(1);
    }
    else if(event.keyCode == 39) {
		alert(wasm.dump_key_mem());
    }
});

	console.log('Index 0: ' + programMemory[0]);
	await fetch(`roms/${rom.toUpperCase()}.ch8`)
		.then(i => i.arrayBuffer())
		.then(buffer => {
			const rom = new DataView(buffer, 0, buffer.byteLength);
			// console.log(rom.getUint8(2));
			for(let i = 0; i < rom.byteLength; i++) {
				programMemory[ROM_OFFSET + i] = rom.getUint8(i);
			}
		}).catch(err => {throw err});
		console.log(wasm.dump_registers());
		for (let i = 0; i < 50; i++) {
			wasm.execute_cycle();
		}
		console.log(vidMem);
		//console.log(wasm.dump_registers());

}
run('maze');

