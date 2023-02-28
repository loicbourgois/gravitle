import init, {Gravithrust} from "./gravithrust/gravithrust.js";
import {
  resize_square,
  fill_circle_2,
  clear,
} from "./canvas.js"
import {
  normalize,
} from "./math.js"
let context
let canvas
let gravithrust
let ZOOM = 1.0
let zen_mode_active = false
let ups = []

let particles
let particle_size = null
let ships
let ship_size = null

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

const colors = [
  {
    'low': "#d0d",
    'high': "#F0F",
  },
  {
    'low': "#da0",
    'high': "#dd4",
  }, 
  {
    'low': "#da0",
    'high': "#dd4",
  },
  {
    'low': "#d80",
    'high': "#da4",
  }
]

const colors2 = {
  'ship_center': {
    'low': "#8f8",
    'high': "#F0F",
  },
  'target': "#f00",
  'orientation': "#88f",
  'boost': "#f00",
  'vt': "#ff0",
}

const draw = () => {
  clear(context)
  for (let i = 0; i < gravithrust.particles_count(); i++) {
    const p = P(i);
    fill_circle_2(context, p, gravithrust.diameter*1.1, colors[p.k].low)
    // if (p.k == 3 )
    // console.log(p.a)
    if (p.k == 3 && p.a == 1) {
      fill_circle_2(context, p, gravithrust.diameter*1.1, colors2['boost'])
    }
  }
  for (let i = 0; i < gravithrust.particles_count(); i++) {
    const p = P(i);
    fill_circle_2(context, p, gravithrust.diameter * 0.5, colors[p.k].high)
  }

  // for (let i = 0; i < gravithrust.particles_count(); i++) {
  //   const p = P(i);
  //   if (p.k == 3) {
  //     // console.log(p.dx)
  //     fill_circle_2(context, {
  //       x: p.x + p.dx * gravithrust.diameter,
  //       y: p.y + p.dy * gravithrust.diameter,
  //     }, gravithrust.diameter * 0.5, "#888")
  //   }
  // }

  for (let i = 0; i < gravithrust.ships_count(); i++) {
    const ship = Ship(i);
    // console.log(ship.center)
    fill_circle_2(context, ship.p, gravithrust.diameter * 0.5, colors2['ship_center'].low)
    // console.log(ship.v)
    
    const d = normalize(ship.v)
    fill_circle_2(context, {
      x:  ship.p.x + d.x*0.05,
      y:  ship.p.y + d.y*0.05,
    }, gravithrust.diameter * 0.5, colors2['ship_center'].low)


    fill_circle_2(context, ship.t, gravithrust.diameter * 0.5, colors2['target'])
    const td_n = normalize(ship.td)
    fill_circle_2(context, {
      x:  ship.p.x + td_n.x*0.05,
      y:  ship.p.y + td_n.y*0.05,
    }, gravithrust.diameter * 0.5, colors2['target'])


    // fill_circle_2(context, ship.t, gravithrust.diameter * 0.5, colors2['target'])
    const orientation_n = normalize(ship.orientation)
    fill_circle_2(context, {
      x:  ship.p.x + orientation_n.x*0.05,
      y:  ship.p.y + orientation_n.y*0.05,
    }, gravithrust.diameter * 0.5, colors2['orientation'])


    fill_circle_2(context, {
      x:  ship.p.x + ship.vt.x*0.05,
      y:  ship.p.y + ship.vt.y*0.05,
    }, gravithrust.diameter * 0.5, colors2['vt'])


    const target_ship_center = {
      x: (ship.p.x + ship.t.x) * 0.5,
      y: (ship.p.y + ship.t.y) * 0.5,
    }

    fill_circle_2(context, target_ship_center, gravithrust.diameter * 1.5, "#f0F")


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
  ship_size = gravithrust.ship_size()
  const data_ptr = gravithrust.particles();
  particles = new DataView(wasm.memory.buffer, data_ptr, gravithrust.particles_size());
  const ships_data_ptr = gravithrust.ships();
  ships = new DataView(wasm.memory.buffer, ships_data_ptr, gravithrust.ships_size());
  // console.log(ships.getFloat32(4, true),)
  requestAnimationFrame(draw)
  run()
});
