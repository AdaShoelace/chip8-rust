
/**
 * Driver program for emulator
 */

import * as wasm from "../pkg/chip8";
import { memory } from '../pkg/chip8_bg'

const ROM_OFFSET = 0x200;

function loadRom() {
	console.log("Loading rom...");
	const selectedFIle = document.getElementById("input").files[0];
	const reader = new FileReader();
	reader.onload = function (evt) {
		console.log("loaded file");
		window.cb = wasm.run(new Uint8Array(evt.target.result))
	}
	reader.readAsArrayBuffer(selectedFIle);
}


window.loadRom = loadRom;

var main = () => {
	console.log("main loop not set");
}


window.main = main;

export function setMainLoop(mainLoop) {
	console.log("Setting main loop from Rust");
	window.main = mainLoop;
}

export function setVideoBuffer(buffer) {
	let vBuffer = new Uint8Array(memory.buffer, buffer, 64*32);
	console.log('Buffer: ', vBuffer);
}