import init, {Gravithrust} from "../gravithrust/gravithrust.js";
import {
  resize_square,
//   fill_circle_2,
//   clear,
} from "../canvas.js"
import {body} from "../body.js"
// import {
//   colors,
//   colors2,
// } from "./colors.js"
// import {
//   setup_audio,
// } from "./sound.js"
import {
  Graphics,
} from "./graphics.js"


let context
let canvas
let context_2
let canvas_2
// let gravithrust
let ZOOM = 2.0
// let zen_mode_active = false
// let ups = []
// let particles
// let particle_size = null
// let ships
// let ship_size = null
// let wasm = null
// let start
// let ppms = [] 
// let ppms_count = 400
// let target_ups = 100
// let timeout = 0


init().then( wasm => {
  // wasm = wasm_
  document.body.innerHTML = body
  const gravithrust = Gravithrust.new(
    0.0025, // diameter
    5, // substep per tick
    0.000000004, // max_rotation_speed
    128, // grid_side
    0.00001, // max_speed_at_target
    0.0001, // forward_max_speed
    30, // forward_max_angle
    35,  // slow_down_max_angle
    0.00025, // slow_down_max_speed_to_target_ratio
    0, // ship_count
  );
  // const keys = [
  //   'forward_max_speed',
  //   'forward_max_angle',
  //   'slow_down_max_angle',
  //   'slow_down_max_speed_to_target_ratio',
  //   'max_rotation_speed',
  // ]
  // for (const k of keys) {
  //   const l = "slow_down_max_speed_to_target_ratio".length - k.length
  //   document.getElementById("right").innerHTML += `
  //     <div>
  //       <label>${k}: ${" ".repeat(l)}</label>
  //       <input id="input_${k}" value="${gravithrust[k].toFixed(9)}"></input>
  //     </div>
  //   `
  // }
  // document.getElementById("right").innerHTML += `
  //   <div class="slidecontainer">
  //     <label>sound: </label>
  //     <input type="range" min="0" max="100" value="0" class="slider" id="sound_slider">
  //   </div>
  // `
  // for (const k of keys) {
  //   document.getElementById(`input_${k}`).addEventListener("change", (v) => {
  //     gravithrust[k] = parseFloat(v.target.value)
  //   });
  // }
  // document.getElementById("sound_slider").addEventListener("input", (v) => {
  //   if (!started_sound){
  //     started_sound = true
  //     start_sound(gravithrust.ships_count())
  //   }
  //   master.gain.linearRampToValueAtTime(parseFloat(v.target.value) / 100, audioCtx.currentTime + 0.1)
  // });
  canvas = document.querySelector("#canvas");
  context = canvas.getContext('2d')
  canvas_2 = document.querySelector("#canvas_2");
  context_2 = canvas_2.getContext('2d')
  resize_square(canvas, ZOOM * 0.9)
  // particle_size = gravithrust.particle_size()
  // ship_size = gravithrust.ship_size()
  // const resize = () => {
    // resize_square(canvas, ZOOM*0.9)
    // const dimension = Math.min(window.innerWidth, window.innerHeight)
    // canvas.style.width = `${dimension*0.9}px`
    // canvas.style.height = `${dimension*0.9}px`
  // }
  const graphics = Graphics(gravithrust, wasm, context)
  graphics.start()
  
  // requestAnimationFrame(draw)
  // run()
  // start = performance.now()
});


// let started_sound = false
// let audioCtx
// let master
// let ship_sounds = []
// const start_sound = (ship_count) => {
//   const r = setup_audio(ship_count)
//   audioCtx = r.audioCtx
//   master = r.master
//   ship_sounds = r.ship_sounds
// }