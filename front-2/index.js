import {
  resize_square,
  fill_circle_2,
  fill_text,
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

let image = context.createImageData(canvas.width, canvas.height);
let data = image.data;
const dim = canvas.width;
// console.log(canvas.width)

function drawPixel(x, y, c) {
  	let roundedX = Math.round(x*dim);
  	let roundedY = Math.round(y*dim);
  	let index = 4 * (canvas.width * roundedY + roundedX);
  	data[index + 0] = c.r;
    data[index + 1] = c.g;
    data[index + 2] = c.b;
    data[index + 3] = c.a;
}


const colors = [
  	{r: 220, g: 220, b:   0, a: 255},
  	{r: 220, g: 0,   b:   0, a: 255},
];

// Listen for messages
let refreshing = false
console.log(socket)
socket.addEventListener('message', (event) => {
  if (!refreshing && event.data instanceof ArrayBuffer) {
    refreshing = true

    const view = new DataView(event.data);
    let ii = -4;
    const step = view.getInt32(ii+=4)
    const elapsed = view.getInt32(ii+=4)
    const elapsed_compute = view.getInt32(ii+=4)
    const elapsed_compute_total = view.getInt32(ii+=4)
    const collisions = view.getInt32(ii+=4)
    const diameter = view.getFloat32(ii+=4)
    const particle_count = view.getInt32(ii+=4)
    ii += 4
    right.innerHTML = `
      <p>bytes: ${event.data.byteLength}</p>
      <p>step: ${step}</p>
      <p>compute: ${elapsed_compute} μs</p>
      <p>compute avg: ${parseInt(elapsed_compute_total/step)} μs</p>
      <p>elapsed: ${elapsed} μs</p>
      <p>collisions: ${collisions}</p>
      <p>particle_count: ${particle_count}</p>
    `
    image = context.createImageData(canvas.width, canvas.height);
    data = image.data;
    for (var i = 0; i < Math.min(particle_count, 100000); i++) {
      const oi = 12
      const x = view.getFloat32(ii + oi*i)
      const y = view.getFloat32(ii + 4 + oi*i)
      const colliding = view.getInt32(ii + 8 + oi*i)
      // const thid = view.getInt32(ii + 12 + oi*i)
      // const fidx = view.getInt32(ii + 16 + oi*i)
      let color = colors[0]
        if (colliding) {
          color = colors[1]
        }
      drawPixel(x,y, color);
    }
    clear(context)
    context.putImageData(image, 0, 0);
    refreshing = false
  } else {
    // text frame
    // console.log(event.data);
  }
});
