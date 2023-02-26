import init, {Gravithrust} from "./gravithrust/gravithrust.js";
import {
  resize_square,
  fill_circle_2,
  clear,
} from "./canvas.js"
let particles
let context
let canvas
let gravithrust
let ZOOM = 1.0
let zen_mode_active = false
let ups = []
let particle_size = null
const P = (id) => {
  return {
    x: particles.getFloat32(id*particle_size, true),
    y: particles.getFloat32(id*particle_size + 4, true),
  }
}
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
  const dimension = Math.min(window.innerWidth, window.innerHeight)
  canvas.style.width = `${dimension*0.9}px`
  canvas.style.height = `${dimension*0.9}px`
}
const draw = () => {
  clear(context)
  for (let i = 0; i < gravithrust.particles_count(); i++) {
    const p = P(i);
    fill_circle_2(context, p, gravithrust.diameter, "#fc0")
  }
  for (let i = 0; i < gravithrust.particles_count(); i++) {
    const p = P(i);
    fill_circle_2(context, p, gravithrust.diameter * 0.5, "#ff4")
  }
  requestAnimationFrame(draw)
}
let target_ups = 100
let timeout = 0
const run = () => {
  ups.push(performance.now())
  for (let _ = 0; _ < 10; _++) {
      gravithrust.tick()
  }
  while (ups.length > 100) {
    ups.shift()
  }
  if (ups.length > 2) {
    timeout = 1000 / target_ups - ( ups[ups.length-1] - ups[ups.length-2] )
    timeout = Math.max(0,timeout)
  }
  setTimeout( run, timeout )
}
init().then( wasm => {
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
  window.go_fullscreen = go_fullscreen
  window.exit_fullscreen = exit_fullscreen
  window.zen_mode = zen_mode
  window.addEventListener("resize", resize)
  window.addEventListener("click", unzen_mode)
  window.addEventListener("keydown", unzen_mode)
  canvas = document.querySelector("#canvas");
  context = canvas.getContext('2d')
  resize_square(canvas, ZOOM * 0.9)
  gravithrust = Gravithrust.new();
  particle_size = gravithrust.particle_size()
  const data_ptr = gravithrust.particles();
  particles = new DataView(wasm.memory.buffer, data_ptr, gravithrust.particles_size());
  requestAnimationFrame(draw)
  run()
});
