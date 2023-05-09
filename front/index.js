import init, {Gravithrust} from "./gravithrust/gravithrust.js";
import {
  resize_square,
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


const exit_fullscreen = () => {
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
  const yml_blueprints = {}
  const json_jobs = {}
  for (const x of ['small_ship', 'plasma_collector', 'sun', 'plasma_depot', 
    'harvester', 'plasma_refinery', 'plasma_transporter']
  ) {
    yml_blueprints[x] = await (await fetch(`./blueprint/${x}.yml`, {cache: "no-store"})).text();
  }
  for (const x of ['plasma_collector', 'plasma_transporter']
  ) {
    json_jobs[x] = await (await fetch(`./job/${x}.json`, {cache: "no-store"})).text();
  }
  setup(wasm, yml_blueprints, json_jobs)
});


const start_sound = (ship_count, simulation) => {
  const r = setup_audio(ship_count)
  simulation.audioCtx = r.audioCtx
  simulation.master = r.master
  simulation.ship_sounds = r.ship_sounds
}


const setup = async (wasm, yml_blueprints, json_jobs) => {
  const gravithrust = Gravithrust.new(
    0.0025, // diameter
    16*2*1, // substep per tick
    0.000000004, // max_rotation_speed
    128, // grid_side
    0.00001, // max_speed_at_target
    0.0001, // forward_max_speed
    30, // forward_max_angle
    35,  // slow_down_max_angle
    0.00025, // slow_down_max_speed_to_target_ratio
    0.00005, // booster_acceleration
  );
  const world_blueprint = {
    structures: [
      {
        blueprint: "sun",
        x: 0.5,
        y: 0.5,
      },{
        blueprint: "plasma_depot",
        x: 0.6,
        y: 0.6,
      },{
        blueprint: "plasma_refinery",
        x: 0.5,
        y: 0.6,
      }
    ],
    particles: [
      {
        kind: "anchor",
        x: 0.525,
        y: 0.5,
      }
    ],
    ships: [
      {
        blueprint: "harvester",
        x: 0.525,
        y: 0.5,
        anchor: {k:'particles', id:0},
        target: {k:'structures', id:0},
      }, 
      {
        blueprint: "plasma_collector",
        x: 0.6,
        y: 0.4,
        job: 'plasma_collector',
      }, 
      {
        blueprint: "plasma_collector",
        x: 0.4,
        y: 0.4,
        job: 'plasma_collector',
      },
      // {
      //   blueprint: "plasma_collector",
      //   x: 0.3,
      //   y: 0.4,
      //   job: 'plasma_collector',
      // },
      // {
      //   blueprint: "plasma_collector",
      //   x: 0.3,
      //   y: 0.32,
      //   job: 'plasma_collector',
      // },
      {
        blueprint: "plasma_transporter",
        x: 0.55,
        y: 0.6,
        job: 'plasma_transporter',
      }
    ]
  }
  for (const structure of world_blueprint.structures) {
    structure.pid = gravithrust.add_structure(yml_blueprints[structure.blueprint], structure.x, structure.y)
  }
  for (const p of world_blueprint.particles) {
    p.pid = gravithrust.add_particle(p.x,  p.y, p.kind)
  }
  for (const s of world_blueprint.ships) {
    s.sid = gravithrust.add_ship(yml_blueprints[s.blueprint], s.x, s.y)
    if (s.job) {
      gravithrust.set_job(s.sid, json_jobs[s.job])
    }
    if (s.anchor) {
      gravithrust.set_anchor(s.sid, world_blueprint[s.anchor.k][s.anchor.id].pid  )
    }
    if (s.target) {
      gravithrust.set_target(s.sid, world_blueprint[s.target.k][s.target.id].pid  )
    }
  }
  const keys = [
    'forward_max_speed',
    'forward_max_angle',
    'slow_down_max_angle',
    'slow_down_max_speed_to_target_ratio',
    'max_speed_at_target',
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
  const simulation = await Simulation(
    gravithrust, 
    wasm, 
    document.querySelector("#canvas"), 
    document.querySelector("#canvas_2"), 
    document.querySelector("#canvas_trace"),
  )
  simulation.start()
  // document.getElementById("sound_slider").addEventListener("input", (v) => {
  //   if (!started_sound){
  //     started_sound = true
  //     start_sound(gravithrust.ships_count(), simulation)
  //   }
  //   simulation.master.gain.linearRampToValueAtTime(parseFloat(v.target.value) / 100, simulation.audioCtx.currentTime + 0.1)
  // });
}
