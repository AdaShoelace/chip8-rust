
/**
 * Driver program for emulator
 */

import * as wasm from "../pkg/chip8";
import { memory } from '../pkg/chip8_bg'
const THREE = require('./lib/three.min');
// import { THREE as ORB } from './lib/OrbitControls';


const ROM_OFFSET = 0x200;
const ROWS = 32;
const COLUMNS = 64;

var camera, scene, renderer, grid;
var geometry, material, mesh;
var meshArray = []
var vBuffer = null;

const printBuffer = (buffer) => {
	let printOut = '';
	for(let i = 0; i < 64; i++) {
		for (let j = 0; j < 32; j++) {
			printOut += buffer[i * 32 + j];
		}
		console.log(printOut);
	}
}

export function init() {

	camera = new THREE.PerspectiveCamera( 120, window.innerWidth / window.innerHeight, 0.001, 1000 );
	camera.position.z = 15;

	scene = new THREE.Scene();

	geometry = new THREE.BoxGeometry( 0.2, 0.2, 0.2 );
	material = new THREE.MeshPhongMaterial({ color: 0x40DBDB });
	
	mesh = new THREE.Mesh( geometry, material );



	var light = new THREE.DirectionalLight( 0xf000f0 );
	light.position.set( 0, 1, 1 ).normalize();
	scene.add(light);

	const hCount = COLUMNS;
    const vCount = ROWS;
    const size = .6;
	const spacing = 1;

	grid = new THREE.Object3D(); // just to hold them all together
	for (let h=0; h<hCount; h+=1) {
		for (let v=0; v<vCount; v+=1) {
			let box = new THREE.Mesh(new THREE.BoxGeometry(size,size,size), material);
			box.position.x = (h-hCount/2) * spacing;
			box.position.y = (v-vCount/2) * spacing;
			grid.add(box);
		}
	}	
	scene.add(grid);

	renderer = new THREE.WebGLRenderer( { antialias: true } );
	renderer.setSize( window.innerWidth - 10, window.innerHeight - 10);
	// let controls = new ORB.OrbitControls(camera, renderer.domElement);
	document.body.appendChild( renderer.domElement );
}

const testMatrix = new Array(COLUMNS*ROWS).fill(1);
testMatrix[1 * ROWS - 2] = 0;

export function animate() {
	requestAnimationFrame( animate );
	if(!vBuffer) return;
	let index = 0;
	for (let i = 0; i < COLUMNS; i++) {
		for (let j = 0; j < ROWS; j++) {
			index = i * ROWS + j;	
			if(vBuffer[index] == 0) {
				grid.children[index].visible = false;
			} else {
				grid.children[index].visible = true;
			}
		}
	}
	renderer.render( scene, camera );
	window.main();
} 
function loadRom() {
	console.log("Loading rom...");
	const selectedFIle = document.getElementById("input").files[0];
	const reader = new FileReader();
	reader.onload = function (evt) {
		console.log("loaded file");
		window.cb = wasm.run(new Uint8Array(evt.target.result));
	}
	reader.readAsArrayBuffer(selectedFIle);
	animate();
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
	vBuffer = new Uint8Array(memory.buffer, buffer, COLUMNS*ROWS);
}


