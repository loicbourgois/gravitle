import init, {Gravithrust} from "./gravithrust/gravithrust.js";
import {
  resize_square,
  fill_circle_2,
} from "./canvas.js"
import {body} from "./body.js"
import {
  setup_audio,
} from "./sound.js"
import {
  Simulation,
} from "./simulation.js"


let RESOLUTION = 1
let zen_mode_active = false
let started_sound = false


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
  resize()
}


const unzen_mode = () => {
  if (zen_mode_active) {
    document.querySelector("#right").style.display = ""
    document.querySelector("#canvas").style.cursor = ""
    zen_mode_active = false;
    resize()
  }
}


const resize = () => {
  const context_trace = document.querySelector("#canvas_trace").getContext('2d')
  const canvas = document.querySelector("#canvas")
  resize_square(canvas, RESOLUTION*0.9)
  const dimension = Math.min(window.innerWidth, window.innerHeight)
  canvas.style.width = `${dimension*0.9}px`
  canvas.style.height = `${dimension*0.9}px`
  context_trace.canvas.style.left = (canvas.offsetLeft - canvas.offsetTop) + "px"
}


init().then( async (wasm) => {
  document.body.innerHTML = body
  window.go_fullscreen = go_fullscreen
  window.exit_fullscreen = exit_fullscreen
  window.zen_mode = zen_mode
  window.addEventListener("resize", resize)
  window.addEventListener("click", unzen_mode)
  window.addEventListener("keydown", unzen_mode)
  try {
    const responses = await Promise.all([
        fetch('./blueprint/blueprint_01.yml'),
        fetch('./blueprint/blueprint_02.yml')
    ]);
    const yml_blueprints = await Promise.all(responses.map(r => r.text()))
    setup(wasm, yml_blueprints)
  } catch (err) {
    throw err;
  }
});


const start_sound = (ship_count, simulation) => {
  const r = setup_audio(ship_count)
  simulation.audioCtx = r.audioCtx
  simulation.master = r.master
  simulation.ship_sounds = r.ship_sounds
}


const setup = (wasm, yml_blueprints) => {
  const gravithrust = Gravithrust.new(
    0.0025, // diameter
    32, // substep per tick
    0.000000004, // max_rotation_speed
    128, // grid_side
    0.00001, // max_speed_at_target
    0.0001, // forward_max_speed
    30, // forward_max_angle
    35,  // slow_down_max_angle
    0.00025, // slow_down_max_speed_to_target_ratio
    0.00005, // booster_acceleration
  );
  gravithrust.add_ship(yml_blueprints[1], 0.55, 0.45)
  gravithrust.add_ship(yml_blueprints[1], 0.52, 0.52)
  for (let index = 0; index < 20; index++) {
    gravithrust.add_ship(yml_blueprints[Math.floor(Math.random()*2)], Math.random(), Math.random())
  }
  const keys = [
    'forward_max_speed',
    'forward_max_angle',
    'slow_down_max_angle',
    'slow_down_max_speed_to_target_ratio',
    'max_rotation_speed',
  ]
  for (const k of keys) {
    const l = "slow_down_max_speed_to_target_ratio".length - k.length
    document.getElementById("right").innerHTML += `
      <div>
        <label>${k}: ${" ".repeat(l)}</label>
        <input id="input_${k}" value="${gravithrust[k].toFixed(9)}"></input>
      </div>
    `
  }
  document.getElementById("right").innerHTML += `
    <div class="slidecontainer">
      <label>sound: </label>
      <input type="range" min="0" max="100" value="0" class="slider" id="sound_slider">
    </div>
  `
  for (const k of keys) {
    document.getElementById(`input_${k}`).addEventListener("change", (v) => {
      gravithrust[k] = parseFloat(v.target.value)
    });
  }
  const context_trace = document.querySelector("#canvas_trace").getContext('2d')
  const context = document.querySelector("#canvas").getContext('2d')
  const context_2 = document.querySelector("#canvas_2").getContext('2d')
  resize_square(context.canvas, RESOLUTION * 0.9)
  resize_square(context_trace.canvas, RESOLUTION * 0.9 )
  resize()
  const simulation = Simulation(gravithrust, wasm, context, context_2, context_trace)
  simulation.start()
  document.getElementById("sound_slider").addEventListener("input", (v) => {
    if (!started_sound){
      started_sound = true
      start_sound(gravithrust.ships_count(), simulation)
    }
    simulation.master.gain.linearRampToValueAtTime(parseFloat(v.target.value) / 100, simulation.audioCtx.currentTime + 0.1)
  });
}