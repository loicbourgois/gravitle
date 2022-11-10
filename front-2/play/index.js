import {
  resize_square,
  fill_circle_2,
  fill_text,
  clear,
} from "../canvas.js"
import {
  uuid,
} from '../utils.js'
import {
  rotate,
} from '../math.js'
const ZOOM = 2
const DELTA_DRAW = 0.001/ZOOM
// const ip = '136.243.64.165'
const ip = 'localhost'
const url = `ws://${ip}:8000`
document.body.innerHTML = `
  <div id="left">
    <canvas id="canvas"></canvas>
  </div>
  <div id="right">
    <button id="go_fullscreen" onclick="go_fullscreen()">Fullscreen</button>
    <button id="exit_fullscreen" onclick="exit_fullscreen()" style="display:none">Exit Fullscreen</button>
    <button id="zen_mode" onclick="zen_mode()">Zen</button>
    <div id="texts"></div>
    <div>
      <label>collide color:</label>
      <input id="color_0" value="#ff4" />
    </div>
    <div>
      <label>base color:   </label>
      <input id="color_1" value="#fc0" />
    </div>
    <div>
      <label>edge color:   </label>
      <input id="color_2" value="#e80" />
    </div>
  </div>
`
const go_fullscreen = () => {
  const elem = document.body
  if (elem.requestFullscreen) {
    elem.requestFullscreen();
  } else if (elem.webkitRequestFullscreen) { /* Safari */
    elem.webkitRequestFullscreen();
  } else if (elem.msRequestFullscreen) { /* IE11 */
    elem.msRequestFullscreen();
  }
  document.querySelector("#go_fullscreen").style.display = "none"
  document.querySelector("#exit_fullscreen").style.display = ""
}
const exit_fullscreen = () => {
  const docElm = document
  if (docElm.exitFullscreen) {
			docElm.exitFullscreen();
	} else if (docElm.webkitExitFullscreen) {
		docElm.webkitExitFullscreen();
	} else if (docElm.mozCancelFullScreen) {
		docElm.mozCancelFullScreen();
	} else if (docElm.msExitFullscreen) {
		docElm.msExitFullscreen();
	}
  document.querySelector("#go_fullscreen").style.display = ""
  document.querySelector("#exit_fullscreen").style.display = "none"
}
let zen_mode_active = false
const zen_mode = () => {
  document.querySelector("#right").style.display = "none"
  zen_mode_active = true
  document.querySelector("#canvas").style.cursor = "none"
  event.stopPropagation()
}
const unzen_mode = () => {
  if (zen_mode_active) {
    document.querySelector("#right").style.display = ""
    document.querySelector("#canvas").style.cursor = ""
    zen_mode_active = false;
  }
}
const resize = () => {
  resize_square(canvas,ZOOM*0.9)
  const aa = Math.min(window.innerWidth, window.innerHeight)
  canvas.style.width = `${aa*0.9}px`
  canvas.style.height = `${aa*0.9}px`
  image = context.createImageData(canvas.width, canvas.height);
  dim = canvas.width
  // document.querySelector("#right").innerHTML = ""
}
const expand = () => {
  document.querySelector("#expand").style.display = "none"
  document.querySelector("#square").style.display = ""
  document.querySelector("#canvas").style.flexGrow = "1"
}
const square = () => {
  document.querySelector("#expand").style.display = ""
  document.querySelector("#square").style.display = "none"
  document.querySelector("#canvas").style.flexGrow = ""
}
window.go_fullscreen = go_fullscreen
window.exit_fullscreen = exit_fullscreen
window.zen_mode = zen_mode
window.square = square
window.expand = expand
window.addEventListener("resize", resize)
window.addEventListener("click", unzen_mode)
window.addEventListener("keydown", unzen_mode)
const texts = document.querySelector("#texts");
const canvas = document.querySelector("#canvas");
const context = canvas.getContext('2d')
resize_square(canvas, ZOOM * 0.9)
const socket = new WebSocket(url);
const uuid_ = uuid()
socket.addEventListener('open', (event) => {
  socket.send(`request ship ${uuid_}`);
});
socket.binaryType = "arraybuffer";
let image = context.createImageData(canvas.width, canvas.height);
let data = image.data;
let dim = canvas.width;
const drawPixel = (x, y, c) => {
  	let roundedX = Math.round(x*dim);
  	let roundedY = Math.round(y*dim);
    if (roundedX > 0 && roundedX < dim && roundedY > 0 && roundedY < dim) {
      let index = 4 * (canvas.width * roundedY + roundedX);
      data[index + 0] = c[0];
      data[index + 1] = c[1];
      data[index + 2] = c[2];
      data[index + 3] = c[3];
    }
}
const to_rgb = (str_) => {
  str_ = str_.replace("#", "")
  if(str_.length == 3){
    const aRgbHex = str_.match(/.{1}/g);
    return [
        parseInt(aRgbHex[0], 16)*16,
        parseInt(aRgbHex[1], 16)*16,
        parseInt(aRgbHex[2], 16)*16,
        255,
    ];
  }
  if(str_.length == 6){
    const aRgbHex = str_.match(/.{1,2}/g);
    return [
        parseInt(aRgbHex[0], 16)*16,
        parseInt(aRgbHex[1], 16)*16,
        parseInt(aRgbHex[2], 16)*16,
        255,
    ];
  }
  return [
      120,
      120,
      120,
      255,
  ];
}
let refreshing = false
let render_duration_total = 0;
let render_step = 0;
let start_step = undefined
// console.log(socket)
socket.addEventListener('message', (event) => {
  if (event.data instanceof ArrayBuffer) {
    const start = performance.now()
    render_step += 1
    const view = new DataView(event.data);
    const colors = [
      to_rgb(document.querySelector("#color_0").value),
      to_rgb(document.querySelector("#color_1").value),
      to_rgb(document.querySelector("#color_2").value),
    ];
    let ii = 0;
    const server_timestamp = view.getBigInt64(ii) ; ii+=8
    const client_timestamp = BigInt( (new Date()).getTime() );
    const lag = client_timestamp - server_timestamp;
    if (lag > 100 && render_step%2 == 0) {
      return
    } 
    const step = view.getFloat32(ii) ; ii+=4
    if (start_step == undefined) {
      start_step = step
    }
    const messages = step-start_step+1;
    const elapsed = view.getFloat32(ii) ; ii+=4
    const elapsed_compute = view.getFloat32(ii) ; ii+=4
    const elapsed_compute_total = view.getFloat32(ii) ; ii+=4
    const clients = view.getInt32(ii) ; ii+=4
    const collisions = view.getInt32(ii) ; ii+=4
    const diameter = view.getFloat32(ii) ; ii+=4
    const particle_count = view.getInt32(ii) ; ii+=4
    const ratio = 1.0 / view.getFloat32(ii)  ; ii+=4
    image = context.createImageData(canvas.width, canvas.height);
    data = image.data;
    // const diameter = 0.001 * 0.5;
    if (ratio <= 0.5) {
      const oi = 2+2+1
      for (var i = 0; i < particle_count ; i++) {
        const idx = ii + oi*i
        const x = view.getUint16(idx) * ratio
        const y = view.getUint16(idx+2) * ratio
        const colliding = ( view.getInt8(idx+4) != 0)
        const color = {
          true: colors[0],
          false: colors[1],
        }[colliding]
        drawPixel(x, y, color);
      }
    } else {
      const oi = 4+4+1
      let x_0 = view.getFloat32(ii)
      let y_0 = view.getFloat32(ii+4)
      for (var i = 0; i < particle_count ; i++) {
        const idx = ii + oi*i
        const x = view.getFloat32(idx)
        const y = view.getFloat32(idx+4)
        const colliding = ( view.getInt8(idx+8) != 0)
        const color = {
          true: colors[0],
          false: colors[1],
        }[colliding]
        const zoom = 20
        const x2 = (x - x_0)*zoom + 0.5
        const y2 = (y - y_0)*zoom + 0.5
        const x3 = x2 + diameter * 0.5 * zoom
        const y3 = y2
        drawPixel(
          x2,
          y2,
          color
        );
        const reso = 20
        for (var u = 0; u < reso; u++) {
          const p3 = rotate({
            x: x2,
            y: y2,
          }, {
            x: x3,
            y: y3,
          }, u/reso)
            // Rotates p2 around p1
            drawPixel(
              p3.x,
              p3.y,
              color
            );
        }
      }
    }
    context.putImageData(image, 0, 0);
    const render_duration = performance.now() - start
    let render_duration_str = `${render_duration.toFixed(3)}`
    render_duration_str = Array.apply(null, Array(  Math.max(0, 7-render_duration_str.length)  )).map(x => " ").join("") + render_duration_str
    let avg_render_duration_str = `${(render_duration_total/render_step).toFixed(3)}`
    avg_render_duration_str = Array.apply(null, Array(  Math.max(0, 7-avg_render_duration_str.length)  )).map(x => " ").join("") + avg_render_duration_str
   
    let instant_compute_str = `${(elapsed_compute/1000).toFixed(3)}`
    instant_compute_str = Array.apply(null, Array(  Math.max(0, 6-instant_compute_str.length)  )).map(x => " ").join("") + instant_compute_str
    texts.innerHTML = `
      <p>server time: ${server_timestamp}</p>
      <p>client time: ${client_timestamp}</p>
      <p>lag: ${lag}</p>
      <p>step: ${step}</p>
      <p>time: ${(elapsed/1000000).toFixed(1)} s</p>
      <p>instant compute: ${instant_compute_str} ms</p>
      <p>average compute: ${(elapsed_compute_total/step/1000).toFixed(3)} ms</p>
      <p>instant render: ${render_duration_str} ms</p>
      <p>average render: ${avg_render_duration_str} ms</p>
      <p>particles: ${particle_count}</p>
      <p>collisions: ${collisions}</p>
      <p>clients: ${clients}</p>
      <p>bytes: ${event.data.byteLength}</p>
      <p>skipped render: ${parseInt((1.0-render_step/messages)*100)}%</p>
    `
    render_duration_total += performance.now() - start
  } else {  }
});
