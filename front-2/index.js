import {
  resize_square,
  fill_circle_2,
  clear,
} from "./canvas.js"

console.log("plouf")

document.body.innerHTML = `
  <canvas id="canvas"></canvas>
  <div id="right"></div>
`
const right = document.querySelector("#right");
const canvas = document.querySelector("#canvas");
const context = canvas.getContext('2d')

resize_square(canvas)



const socket = new WebSocket('ws://localhost:8080');

// Connection opened
socket.addEventListener('open', (event) => {
    socket.send('Hello Server!');
});

socket.binaryType = "arraybuffer";

// Listen for messages
socket.addEventListener('message', (event) => {
  if (event.data instanceof ArrayBuffer) {
    const view = new DataView(event.data);
    const step = view.getInt32(0)
    const elapsed = view.getInt32(4)
    const collisions = view.getInt32(8)
    const diameter = view.getFloat32(12)
    const particle_count = view.getInt32(16)
    clear(context)
    for (var i = 0; i < particle_count; i++) {
      const x = view.getFloat32(20 + 9*i)
      const y = view.getFloat32(24 + 9*i)
      const colliding = view.getInt8(28 + 9*i)
      const color = {
        0: '#dd8',
        1: '#d88'
      }[colliding]
      fill_circle_2(context, {
        x: x,
        y: y,
      }, diameter, color)
    }
    right.innerHTML = `
      <p>bytes: ${event.data.byteLength}</p>
      <p>step: ${step}</p>
      <p>elapsed: ${elapsed} μs</p>
      <p>collisions: ${collisions}</p>
      <p>particle_count: ${particle_count}</p>
    `
  } else {
    // text frame
    console.log(event.data);
  }
});
