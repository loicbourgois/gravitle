import init, {Gravithrust} from "./gravithrust/gravithrust.js";
import {
  resize_square,
  fill_circle_2,
  clear,
} from "./canvas.js"
import {body} from "./body.js"
import {
  colors,
  colors2,
} from "./colors.js"
import {
  setup_audio,
} from "./sound.js"


let context
let canvas
let context_2
let canvas_2
let gravithrust
let ZOOM = 2.0
let zen_mode_active = false
let ups = []
let particles
let particle_size = null
let ships
let ship_size = null
let wasm = null
let start
let ppms = [] 
let ppms_count = 400
let target_ups = 100
let timeout = 0


const P = (id) => {
  return {
    x: particles.getFloat32(id*particle_size, true),
    y: particles.getFloat32(id*particle_size + 4, true),
    k: particles.getInt32(id*particle_size + 4*9, true),
    dx: particles.getFloat32(id*particle_size + 4*6, true),
    dy: particles.getFloat32(id*particle_size + 4*7, true),
    a: particles.getInt32(id*particle_size + 4*10, true),
  }
}

const Ship = (id) => {
  return {
    p: {
        x: ships.getFloat32(id*ship_size, true),
        y: ships.getFloat32(id*ship_size + 4, true),
    },
    v: {
      x: ships.getFloat32(id*ship_size + 4 * 4, true),
      y: ships.getFloat32(id*ship_size + 4 * 5, true),
    },
    t: {
      x: ships.getFloat32(id*ship_size + 4 * 6, true),
      y: ships.getFloat32(id*ship_size + 4 * 7, true),
    },
    td: {
      x: ships.getFloat32(id*ship_size + 4 * 8, true),
      y: ships.getFloat32(id*ship_size + 4 * 9, true),
    },
    orientation: {
      x: ships.getFloat32(id*ship_size + 4 * 10, true),
      y: ships.getFloat32(id*ship_size + 4 * 11, true),
    },
    vt: {
      x: ships.getFloat32(id*ship_size + 4 * 12, true),
      y: ships.getFloat32(id*ship_size + 4 * 13, true),
    },
    cross: {
      x: ships.getFloat32(id*ship_size + 4 * 14, true),
      y: ships.getFloat32(id*ship_size + 4 * 15, true),
    },
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
  resize_square(canvas, ZOOM*0.9)
  const dimension = Math.min(window.innerWidth, window.innerHeight)
  canvas.style.width = `${dimension*0.9}px`
  canvas.style.height = `${dimension*0.9}px`
}


const draw = () => {
  clear(context)
  const data_ptr = gravithrust.particles();
  particles = new DataView(wasm.memory.buffer, data_ptr, gravithrust.particles_size());
  const ships_data_ptr = gravithrust.ships();
  ships = new DataView(wasm.memory.buffer, ships_data_ptr, gravithrust.ships_size());
  for (let i = 0; i < gravithrust.particles_count(); i++) {
    const p = P(i);
    fill_circle_2(context, p, gravithrust.diameter*1.1, colors[p.k].low)
    if (p.k == 3 && p.a == 1) {
      fill_circle_2(context, p, gravithrust.diameter*1.1, colors2['boost'])
    }
  }
  for (let i = 0; i < gravithrust.particles_count(); i++) {
    const p = P(i);
    fill_circle_2(context, p, gravithrust.diameter * 0.5, colors[p.k].high)
  }
  for (let i = 0; i < gravithrust.ships_count(); i++) {
    const ship = Ship(i);
    fill_circle_2(context, ship.t, gravithrust.diameter * 2.0, '#ff02')
  }
  // for (let i = 0; i < gravithrust.ships_count(); i++) {
  //   const ship = Ship(i);
  //   fill_circle_2(context, ship.p, gravithrust.diameter * 0.5, colors2['ship_center'].low)
  //   const d = normalize(ship.v)
  //   fill_circle_2(context, {
  //     x:  ship.p.x + d.x*0.05,
  //     y:  ship.p.y + d.y*0.05,
  //   }, gravithrust.diameter * 0.5, colors2['ship_center'].low)
  //   const td_n = normalize(ship.td)
  //   fill_circle_2(context, {
  //     x:  ship.p.x + td_n.x*0.05,
  //     y:  ship.p.y + td_n.y*0.05,
  //   }, gravithrust.diameter * 1., colors2['target'])
  //   const orientation_n = normalize(ship.orientation)
  //   fill_circle_2(context, {
  //     x:  ship.p.x + orientation_n.x*0.05,
  //     y:  ship.p.y + orientation_n.y*0.05,
  //   }, gravithrust.diameter * 0.5, colors2['orientation'])
  //   const cross_n = normalize(ship.cross)
  //   fill_circle_2(context, {
  //     x:  ship.p.x + cross_n.x*0.05,
  //     y:  ship.p.y + cross_n.y*0.05,
  //   }, gravithrust.diameter * 1., "#f4f")
  // }
  document.querySelector("#points").innerHTML = gravithrust.points
  const duration = (( performance.now() - start) / 1000 )
  document.querySelector("#mpps").innerHTML = (gravithrust.points * 1000000 / gravithrust.step).toFixed(1)
  document.querySelector("#duration").innerHTML = parseInt(duration)
  document.querySelector("#step").innerHTML = gravithrust.step
  if (ppms.length) {
    canvas_2.width = 400
    clear(context_2)
    let ppms_max = ppms[0].high
    for (const x of ppms) {
      ppms_max = Math.max(x.high, ppms_max)
    }
    for (let i = 0; i < ppms.length; i++) { 
      const x = (i+1) / ppms.length * canvas_2.width
      const y_low = ppms[i].low   / ppms_max * canvas_2.height
      const y_high = ppms[i].high / ppms_max * canvas_2.height
      context_2.beginPath();
      context_2.fillStyle = "#ff0d";
      context_2.strokeStyle = "#ff0d";
      context_2.rect(x, canvas_2.height-y_high, 1, Math.max(y_high-y_low, 1));
      context_2.fill();
      context_2.stroke();
    }
  }
  
  // console.log(speed * 100000)
  if (audioCtx) {
    for (let sid = 0; sid < gravithrust.ships_count(); sid++) {
      const ss = ship_sounds[sid]
      const ship = Ship(sid);
      const speed = Math.sqrt(ship.v.x * ship.v.x + ship.v.y * ship.v.y)
      ss.osc_2.frequency.linearRampToValueAtTime(
        ss.osc_2_base_frequency * (0.9 + speed * 10),
        audioCtx.currentTime + 0.01
      )
      ss.gain_2.gain.linearRampToValueAtTime(
        ss.gain_2_base_gain * (0.9 + speed * 150000),
        audioCtx.currentTime + 0.01
      )

      ss.osc_3.frequency.linearRampToValueAtTime(
        ss.osc_3_base_frequency * (0.9 + speed * 10),
        audioCtx.currentTime + 0.01
      )
      ss.gain_3.gain.linearRampToValueAtTime(
        ss.gain_3_base_gain * (0.9 + speed * 150000),
        audioCtx.currentTime + 0.01
      )

      ss.gain_1.gain.linearRampToValueAtTime(
        ss.gain_1_base_gain * (0.0 + speed * 10000),
        audioCtx.currentTime + 0.01
      )
      ss.stereo_1.pan.linearRampToValueAtTime(
        (ship.p.x - 0.5) * 3,
        audioCtx.currentTime + 0.01
      )

      // ss.osc_4.stop()
      // ss.osc_3.stop()

    }
  }
  requestAnimationFrame(draw)
}


const run = () => {
  ups.push(performance.now())
  gravithrust.ticks()
  ppms.push({
    high: gravithrust.points * 1000000 / gravithrust.step,
    low: gravithrust.points * 1000000 / gravithrust.step,
    step_high: gravithrust.step,
    step_low: gravithrust.step,
  })
  if (ppms.length >= ppms_count) {
    for (let i = 0; i < ppms_count/2; i++) {
      ppms[i] = {
        high: Math.max(ppms[i*2].high, ppms[i*2+1].high),
        low: Math.min(ppms[i*2].low, ppms[i*2+1].low),
        step_high: Math.max(ppms[i*2].step_high, ppms[i*2+1].step_high),
        step_low: Math.min(ppms[i*2].step_low, ppms[i*2+1].step_low),
      }
    }
    ppms.length = ppms_count/2;
  }
  while (ups.length > 100) {
    ups.shift()
  }
  if (ups.length > 2) {
    const ups_ = 1000 / ( ups[ups.length-1] - ups[0]  ) * ups.length
    document.querySelector("#particles_count").innerHTML = gravithrust.particles_count()
    document.querySelector("#ups").innerHTML = parseInt(ups_)
    timeout = 1000 / target_ups - ( ups[ups.length-1] - ups[ups.length-2] )
    timeout = Math.max(0,timeout)
    timeout = 0
  }
  setTimeout( run, timeout )
}


init().then( wasm_ => {
  wasm = wasm_
  document.body.innerHTML = body
  window.go_fullscreen = go_fullscreen
  window.exit_fullscreen = exit_fullscreen
  window.zen_mode = zen_mode
  window.addEventListener("resize", resize)
  window.addEventListener("click", unzen_mode)
  window.addEventListener("keydown", unzen_mode)
  gravithrust = Gravithrust.new(
    0.0025, // diameter
    5, // substep per tick
    0.000000004, // max_rotation_speed
    128, // grid_side
    0.00001, // max_speed_at_target
    0.0001, // forward_max_speed
    30, // forward_max_angle
    35,  // slow_down_max_angle
    0.00025, // slow_down_max_speed_to_target_ratio
    20, // ship_count
  );
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
  document.getElementById("sound_slider").addEventListener("input", (v) => {
    if (!started_sound){
      started_sound = true
      start_sound(gravithrust.ships_count())
    }
    master.gain.linearRampToValueAtTime(parseFloat(v.target.value) / 100, audioCtx.currentTime + 0.1)
  });
  canvas = document.querySelector("#canvas");
  context = canvas.getContext('2d')
  canvas_2 = document.querySelector("#canvas_2");
  context_2 = canvas_2.getContext('2d')
  resize_square(canvas, ZOOM * 0.9)
  particle_size = gravithrust.particle_size()
  ship_size = gravithrust.ship_size()
  requestAnimationFrame(draw)
  run()
  start = performance.now()
});


let started_sound = false
let audioCtx
let master
let ship_sounds = []
const start_sound = (ship_count) => {
  const r = setup_audio(ship_count)
  audioCtx = r.audioCtx
  master = r.master
  ship_sounds = r.ship_sounds
}