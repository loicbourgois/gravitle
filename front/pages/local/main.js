import {
  resize_square,
  stroke_circle,
  stroke_circle_2,
  fill_circle_2,
  clear,
  line,
  fill_text,
} from "../canvas"
import {
  collision_response,
  distance_sqrd,
  wrap_around,
  normalize,
  delta,
  rotate,
  add,
  del,
  mul,
  mod,
<<<<<<< Updated upstream
} from "../math"
=======
} from "./math"
>>>>>>> Stashed changes
import {
  get_fps,
  update_fps,
  get_ups,
  update_ups,
  get_ups_avg_delta,
} from "./perf"
<<<<<<< Updated upstream
import {
  ship_0,
  ship_2,
  ship_1,
} from "./ship"


const LINK_STRENGH = 0.2
=======
>>>>>>> Stashed changes


const html = () => {
  return `
    <div>
<<<<<<< Updated upstream
      <p id="move_with_instructions"></p>
      <p> <a href="garage">Go to Garage</a> </p>
=======
      <p>Move with F, J</p>
>>>>>>> Stashed changes
    </div>
    <canvas id="canvas"></canvas>
    <div>
      <p>FPS: <span id="fps">...</span></p>
      <p>UPS: <span id="ups">...</span></p>
    </div>
  `
}


const style = () => {
  return `
    #content {
      display: flex;
      width: 100%;
      height: 100%;
      align-content: center;
      align-items: center;
      flex-direction: row;
    }
    a {
      color: #ffa;
      text-decoration: none;
      background-color: #fff0;
      padding: 0.8rem;
    }
    a:hover {
      background-color: #fff2;
    }
    #content > div {
      width: 0;
      flex-grow: 1;
      display: flex;
      justify-content: space-around;
      flex-direction: column;
      height: 100%;
    }
    p {
      text-align: center;
      color: #ffa;
      font-family: monospace;
    }
    p span {
      color: #ffa;
    }
    #canvas {
        background: #113;
        display:flex;
        position: unset;
    }
    body {
      background: #113;
    }
  `
}


const DIAM = 0.0125


const parts = []
const links = []
const links_set = new Set


const add_part = (x,y,dx,dy, kind) => {
  const idx = parts.length
  parts.push({
    idx: idx,
    kind: kind,
    d: DIAM,
    dp: {
      x: dx,
      y: dy,
    },
    pp: {
      x: x-dx,
      y: y-dy,
    },
    p: {
      x: x,
      y: y,
    },
    np: {
      x: x,
      y: y,
    },
    collision_response: {
      x: 0,
      y: 0,
      count: 0,
    },
    link_response: {
      x: 0,
      y: 0,
    },
    links: new Set(),
    direction: {x:0, y:0},
  })
  return idx
}


const add_link = (a_idx, b_idx) => {
  const link_id = a_idx < b_idx ? `${a_idx}|${b_idx}`:`${b_idx}|${a_idx}`
  if (! links_set.has(link_id)) {
    links.push({
      a: a_idx,
      b: b_idx,
    })
    links_set.add(link_id)
    parts[a_idx].links.add(b_idx)
    parts[b_idx].links.add(a_idx)
  }
}


<<<<<<< Updated upstream
const key_bindings = new Map()
=======
const local_main = () => {
  document.querySelector('#content').innerHTML = html()
  const style_element = document.createElement('style')
  document.head.appendChild(style_element)
>>>>>>> Stashed changes


const add_ship_2 = (ship, x, y) => {
  const p1_idx = parts.length
  for (let part of ship.parts) {
    const idx = add_part(
      part.p.x-ship.center.x+x,
      part.p.y-ship.center.y+y,
      0,
      0,
      part.kind
    )
    if (part.binding) {
      if (!key_bindings.has(part.binding)) {
        key_bindings.set(part.binding, new Set())
      }
      key_bindings.get(part.binding).add(idx)
    }
  }
<<<<<<< Updated upstream
  for (let link of ship.links) {
    add_link(link.a+p1_idx, link.b+p1_idx)
  }
=======




  const canvas = document.querySelector('#canvas')
  resize_square(canvas)
  const context = canvas.getContext('2d')
  const ship = {
    p1: 'core',
    p2: 'core',
    parts: [
      [0,1, 'armor'],
      [0,2, 'gun'],
      [0,3, 'armor'],
      [0,4, 'armor'],
      [0,5, 'armor'],
      [2,1, 'gun'],
      [7,1, 'armor'],
      [8,1, 'armor'],
      [6,5, 'armor'],
      [9,6, 'armor'],
      [8,9, 'armor'],
      [5,4, 'armor'],
      [5,13, 'booster'],
      [12,9, 'booster'],
      [4,3, 'armor'],
      [7,8, 'armor'],
    ],
    links: [
      [1,6],
      [6,9],
      [10,11],
    ],
    key_bindings: {
      'f': [14],
      'j': [15],
    },
  }
  const ship_2 = {
    p1: 'core',
    p2: 'armor',
    parts: [
      [0,1, 'armor'],
      [0,2, 'armor'],
      [0,3, 'armor'],
      [0,4, 'armor'],
      [0,5, 'armor'],
    ],
    links: [
      [1,6]
    ],
    key_bindings: {},
  }
  add_ship(ship, 0.5, 0.5)
  add_ship(ship_2, 0.25, 0.5)
  add_ship(ship_2, 0.5, 0.25)
  add_ship(ship_2, 0.75, 0.5)
  add_ship(ship_2, 0.5, 0.75)
  add_ship(ship_2, 0.8, 0.8)
  add_ship(ship_2, 0.2, 0.8)
  add_ship(ship_2, 0.8, 0.2)
  add_ship(ship_2, 0.2, 0.2)
  render(context)
  compute()
  document.addEventListener("keydown", (e) => {
    if (key_bindings.get(e.key)) {
      for (let idx of key_bindings.get(e.key)) {
        parts[idx].activated = true
      }
    }
  });
  document.addEventListener("keyup", (e) => {
    if (key_bindings.get(e.key)) {
      for (let idx of key_bindings.get(e.key)) {
        parts[idx].activated = false
      }
    }
  });
>>>>>>> Stashed changes
}


const add_ship = (ship, x, y) => {
  const core_1_idx = parts.length
  const core_2_idx = parts.length + 1
  add_part(x - DIAM*0.5, y, 0, 0, ship.p1)
  add_part(x + DIAM*0.5, y, 0, 0, ship.p2)
  for (let part of ship.parts) {
    const p1 = parts[core_1_idx + part[0]]
    const p2 = parts[core_1_idx + part[1]]
    const pos = rotate(p1.p, p2.p, 1/6)
    const idx = add_part(pos.x, pos.y, 0, 0, part[2])
    add_link(idx, p1.idx)
    add_link(idx, p2.idx)
  }
  add_link(core_1_idx, core_2_idx)
  for (let linki of ship.links) {
    add_link(linki[0]+core_1_idx, linki[1]+core_1_idx)
  }
  for (let k of Object.keys(ship.key_bindings)) {
    if (!key_bindings.has(k)) {
      key_bindings.set(k, new Set())
    }
    for (let idx of ship.key_bindings[k]) {
      key_bindings.get(k).add(idx+core_1_idx)
    }
  }
}


const average_color = (c1,c2) => {
  c1 = {
    r: parseInt(c1[1], 16),
    g: parseInt(c1[2], 16),
    b: parseInt(c1[3], 16),
  }
  c2 = {
    r: parseInt(c2[1], 16),
    g: parseInt(c2[2], 16),
    b: parseInt(c2[3], 16),
  }
  const c3 = {
    r: (c1.r+c2.r)*0.5*17,
    g: (c1.g+c2.g)*0.5*17,
    b: (c1.b+c2.b)*0.5*17,
  }
  return `rgb(${c3.r},${c3.g},${c3.b})`
}


const render = (context) => {
  update_fps()
  clear(context)
  const colors = {
    'glass': {
      value: '#aaf3',
      score: 4,
    },
    'booster': {
      value: '#fb0',
      value_1: '#fa0',
      value_2: '#f80',
      value_3: '#f00',
      score: 2,
    },
    'core': {
      value: '#ffa',
      score: 4,
    },
    'armor': {
      value: '#aaf',
      score: 3,
    },
    'gun': {
      value: '#f88',
      score: 1,
    },
  }
  for (let p of parts) {
    if (p.activated && p.kind == 'booster')
    {
      fill_circle_2(context, add(p.p, mul(p.direction, 0.007+Math.random()*0.003)), p.d*0.7, colors[p.kind].value_3)
      fill_circle_2(context, add(p.p, mul(p.direction, 0.005+Math.random()*0.001)), p.d*0.9, colors[p.kind].value_2)
      fill_circle_2(context, p.p, p.d, colors[p.kind].value_1)
    } else {
      fill_circle_2(context, p.p, p.d, colors[p.kind].value)
    }

  }
  for (let c_ of Object.keys(colors) ) {
    for (let l of links) {
      const p1 = parts[l.a]
      const p2 = parts[l.b]
      const wa = wrap_around(p1.np, p2.np)
      const delt = mul(delta(wa.a, wa.b), 0.5)
      const color_id = colors[p1.kind].score > colors[p2.kind].score ? p1.kind : p2.kind
      if (c_ == color_id) {
        const color = colors[color_id].value
        const aa = 0.75
        fill_circle_2(context, add(p1.p, delt), p1.d*aa, color)
        fill_circle_2(context, del(p2.p, delt), p2.d*aa, color)
      }
      // line(context, p2.p, del(p2.p, delt), "grey")
    }
  }
  for (let l of links) {
    const p1 = parts[l.a]
    const p2 = parts[l.b]
    const wa = wrap_around(p1.np, p2.np)
    const delt = delta(wa.a, wa.b)
    // line(context, p1.p, add(p1.p, delt), "grey")
    // line(context, p2.p, del(p2.p, delt), "grey")
  }
  for (let p of parts) {
    // fill_text(context, p.p, p.idx, )
    //line(context, p.p, add(p.p, mul(p.direction, 0.02)), "red")
  }
  document.getElementById("fps").innerHTML = get_fps()
  document.getElementById("ups").innerHTML = get_ups()
  window.requestAnimationFrame(()=>{
    render(context)
  })
}


const compute = () => {
  let dp = 0
  for (let p of parts) {
    p.direction = {x:0,y:0}
    for (let p2_idx of p.links) {
      const p2 = parts[p2_idx]
      const wa = wrap_around(p.p, p2.p)
      p.direction = add(p.direction, delta(wa.b, wa.a))
    }
    p.direction = normalize(p.direction)
    p.dp.x = p.p.x - p.pp.x
    p.dp.y = p.p.y - p.pp.y
    if (p.kind == 'booster' && p.activated) {
      p.dp.x -= p.direction.x * 0.0001
      p.dp.y -= p.direction.y * 0.0001
    }
    p.np.x = p.p.x + p.dp.x
    p.np.y = p.p.y + p.dp.y
    p.link_response.x = 0
    p.link_response.y = 0
    p.collision_response.x = 0
    p.collision_response.y = 0
    p.collision_response.count = 0
    dp += distance_sqrd(p.dp)
  }
  for (let p1 of parts) {
    for (let p2 of parts) {
      if (p1.idx < p2.idx ) {
        const wa = wrap_around(p1.np, p2.np)
        wa.a.np = {
          x: wa.a.x,
          y: wa.a.y,
        }
        wa.b.np = {
          x: wa.b.x,
          y: wa.b.y,
        }
        wa.a.dp = p1.dp
        wa.b.dp = p2.dp
        const d = wa.d_sqrd
        const diams = (p1.d + p2.d)*0.5
        const diams_sqrd = diams*diams
        if ( d < diams_sqrd ) {
          let cr = collision_response(wa.a, wa.b)
          if (links_set.has(`${p1.idx}|${p2.idx}`)) {
            cr.x *= 0.5;
            cr.y *= 0.5;
          }
          p1.collision_response.x -= cr.x
          p1.collision_response.y -= cr.y
          p1.collision_response.count += 1
          p2.collision_response.x += cr.x
          p2.collision_response.y += cr.y
          p2.collision_response.count += 1
        }
      }
    }
  }
  for (let link of links) {
    const p1 = parts[link.a]
    const p2 = parts[link.b]
    const wa = wrap_around(p1.np, p2.np)
    const d = Math.sqrt(wa.d_sqrd)
    const n = normalize(delta(wa.a, wa.b), d)
    const ds = (p1.d + p2.d) * 0.5
    const factor = (ds - d) * LINK_STRENGH
    p1.link_response.x -= n.x * factor * 0.5
    p1.link_response.y -= n.y * factor * 0.5
    p2.link_response.x += n.x * factor * 0.5
    p2.link_response.y += n.y * factor * 0.5
  }
  for (let p of parts) {
    if (p.collision_response.count) {
      p.collision_response.x /= p.collision_response.count
      p.collision_response.y /= p.collision_response.count
      p.np.x += p.collision_response.x
      p.np.y += p.collision_response.y
      p.np.x += p.link_response.x
      p.np.y += p.link_response.y
    }
    p.p.x = (p.np.x + 1) % 1.0
    p.p.y = (p.np.y + 1) % 1.0
    p.pp.x = p.p.x - p.dp.x - p.collision_response.x - p.link_response.x
    p.pp.y = p.p.y - p.dp.y - p.collision_response.y - p.link_response.y
  }
  update_ups()
  window.setTimeout(() => {
    compute()
  }, 10-get_ups_avg_delta())
<<<<<<< Updated upstream
}


const local_main = () => {
  document.querySelector('#content').innerHTML = html()
  const style_element = document.createElement('style')
  document.head.appendChild(style_element)
  for (let x of style().split('}')) {
      try {
        style_element.sheet.insertRule(x+'}');
      } catch(e) {

      }
  }
  const canvas = document.querySelector('#canvas')
  resize_square(canvas)
  const context = canvas.getContext('2d')
  if (!localStorage.getItem('ship')) {
    localStorage.setItem('ship', ship_1)
  }
  add_ship_2(JSON.parse(localStorage.getItem('ship')), 0.5, 0.5)
  add_ship(ship_2, 0.27, 0.5)
  add_ship(ship_2, 0.5, 0.27)
  add_ship(ship_2, 0.73, 0.5)
  add_ship(ship_2, 0.5, 0.73)
  add_ship(ship_2, 0.8, 0.8)
  add_ship(ship_2, 0.2, 0.8)
  add_ship(ship_2, 0.8, 0.2)
  add_ship(ship_2, 0.2, 0.2)
  render(context)
  compute()
  document.addEventListener("keydown", (e) => {
    if (key_bindings.get(e.key)) {
      for (let idx of key_bindings.get(e.key)) {
        parts[idx].activated = true
      }
    }
  });
  document.addEventListener("keyup", (e) => {
    if (key_bindings.get(e.key)) {
      for (let idx of key_bindings.get(e.key)) {
        parts[idx].activated = false
      }
    }
  });
  const move_with_keys = new Set()
  for (let kv of key_bindings ) {
    const key = kv[0]
    const idxs = kv[1]
    for (let idx of idxs) {
      if (parts[idx].kind == 'booster' ) {
        move_with_keys.add(key)
      }
    }
  }
  if (move_with_keys.size) {
    document.querySelector("#move_with_instructions").innerHTML =
      `Move with ${Array.from(move_with_keys).map(x=>x.toUpperCase()).join(", ")}`
  }
=======
>>>>>>> Stashed changes
}


export {
  local_main
}
