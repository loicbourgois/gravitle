import {
  resize_square,
  stroke_circle,
  stroke_circle_2,
  fill_circle_2,
  fill_circle,
  clear,
  line,
  fill_text,
} from "../canvas"
import {
  colors,
} from '../colors'
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
} from "../math"
import {
  get_fps,
  update_fps,
  get_ups,
  update_ups,
  get_ups_avg_delta,
} from "../perf"
import {
  ship_0,
  ship_2,
  default_ship_journey,
  emerald,
} from "../ship"


const LINK_STRENGH = 0.2
const GRID_SIDE = 20
const CELL_COUNT = GRID_SIDE * GRID_SIDE
const DIAM = 0.0125
const LOADING_AWAIT = 1
let scores = [0,0]
let CONTINUE_RENDER = true
let score_to_win
let start_time


const msToTime = (s) => {
  // Pad to 2 or 3 digits, default is 2
  function pad(n, z) {
    z = z || 2;
    return ('00' + n).slice(-z);
  }
  var ms = s % 1000;
  s = (s - ms) / 1000;
  var secs = s % 60;
  s = (s - secs) / 60;
  var mins = s % 60;
  var hrs = (s - mins) / 60;
  return pad(hrs) + ':' + pad(mins) + ':' + pad(secs) + '.' + pad(ms, 3);
}


const get_best_by_player = (player_name) => {
  const best_id = `best_${parseInt(window.location.pathname.split('journey-')[1])}_${player_name}`
  let data = localStorage.getItem(best_id)
  if (data && data.length) {
    data = JSON.parse(data)
  }
  if (!data || !data.id) {
    data = {}
  }
  data.id = best_id
  return data
}


const get_best = () => {
  const best_id = `best_${parseInt(window.location.pathname.split('journey-')[1])}`
  let data = localStorage.getItem(best_id)
  if (data && data.length) {
    data = JSON.parse(data)
  }
  if (!data || !data.id) {
    data = {}
  }
  data.id = best_id
  return data
}


const update_best = (duration, player_name) => {
  const current_best = get_best()
  if ( current_best.duration && current_best.duration <= duration ) {
    // pass
  } else {
    current_best.duration = duration
    current_best.player_name = player_name
    localStorage.setItem(current_best.id, JSON.stringify(current_best))
  }
  const best_by_player = get_best_by_player(player_name)
  if ( best_by_player.duration && best_by_player.duration <= duration ) {
    // pass
  } else {
    best_by_player.duration = duration
    best_by_player.player_name = player_name
    localStorage.setItem(best_by_player.id, JSON.stringify(best_by_player))
  }
}


const player_name = () => {
  return localStorage.getItem('player_journey_name') ? localStorage.getItem('player_journey_name') :  "Blue"
}


const update_player_info = () => {
  localStorage.setItem('player_journey_name', document.querySelector('#player_journey_name').value)
  const data_2 = get_best_by_player(player_name())
  const best_duration_current_player = data_2.duration ? msToTime(data_2.duration) : '--:--:--:---'
  document.querySelector("#best_duration_current_player").innerHTML = best_duration_current_player
}


const html = () => {
  const data = get_best()
  const best_duration_str = data.duration ? msToTime(data.duration) : '--:--:--:---'
  const best_player_name = data.player_name ? data.player_name : ''
  const data_2 = get_best_by_player(player_name())
  const best_duration_current_player = data_2.duration ? msToTime(data_2.duration) : '--:--:--:---'
  return `
    <div id="winner" class="hide">
      <p><span id="duration"></span></p>
      <div>
        <button onclick="again()">Play Again<br>[space]</button>
        <button onclick="next()">Next<br>[enter]</button>
      </div>
    </div>
    <div class="bob">
      <input class="player_name"  id="player_journey_name"
        value="${player_name()}"
        oninput="update_player_info()"></input>
      <p class="disappearable disappear" id="best_duration_current_player">${best_duration_current_player}</p>
      <p id="move_with_instructions" class="disappearable disappear">Loading...</p>
      <p class="disappearable disappear"> <a href="/journey">Levels</a> </p>
      <p class="disappearable disappear"> <a href="/journey-garage">Garage</a> </p>
    </div>
    <canvas id="canvas"></canvas>
    <div class="bob">
      <p class="disappearable disappear" id="best_name">${best_player_name}</p>
      <p class="disappearable disappear" id="best_duration">${best_duration_str}</p>
      <p class="disappearable disappear">FPS: <span id="fps">...</span></p>
      <p class="disappearable disappear">UPS: <span id="ups">...</span></p>
      <p class="disappearable disappear"> <a href="/">Home</a> </p>
    </div>
  `
}


const style = () => {
  return `
    * {
      color: #ffa;
      background: #113;
      font-size: 1.05rem;
    }
    select {
      border: none;
    }
    option {
    }
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
    .disappear, .disappear * {
      color: #0000;
      transition: color 0.2s;
    }
    a:hover {
      background-color: #fff2;
    }
    #score_player_1, #score_player_2 {
      font-size: 2rem;
    }
    #content > div.bob {
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
    #winner {
      position: absolute;
      height: 100vh;
      width: 100vw;
      background: #0000;
      display: flex;
      flex-direction: column;
      align-content: center;
      align-items: center;
      justify-content: center;
      pointer-events: none;
    }
    #winner > p {
      background: #ffaa;
      padding: 5rem;
      border-radius: 10rem;
      border: solid 5px #ffa;
    }
    #winner > p > span, #winner > p  {
      color: #ffa;
      font-size: 3rem;
    }
    #winner > p > span {
      background: transparent;
    }
    #winner button {
      border: none;
      margin: 2rem;
      cursor: pointer;
      pointer-events: all;
      padding: 1rem;
      border-radius: 10rem;
      background: #fff0;
      line-height: 1.5rem;
    }
    #winner div {
      background: #fff0;
    }
    #winner button:hover {
      background: #fff2;
    }
    .hide {
      display: none !important;
    }
    a {
      border-radius: 10rem;
    }
    .player_name {
      text-align: center;
      background: none;
      border: solid 2px transparent;
      margin-left: 1rem;
      margin-right: 1rem;
      padding: 0.5rem;
      font-size: 1.5rem;
    }
    .player_name:hover {
      border: solid 2px #ffdd;
    }
    #best_name {
      margin-left: 1rem;
      margin-right: 1rem;
      padding: 0.5rem;
      font-size: 1.5rem;
      border: solid 2px #0000;
    }
  `
}


const grid_id = (position) => {
  return (parseInt(position.y * GRID_SIDE) % GRID_SIDE) * GRID_SIDE + parseInt(position.x * GRID_SIDE) % GRID_SIDE
}
const grid_id_3 = (x,y) => {
  return (y% GRID_SIDE) * GRID_SIDE  + x % GRID_SIDE
}


const grid_ids = []
const grid = []
for (var x = 0; x < GRID_SIDE; x++) {
  for (var y = 0; y < GRID_SIDE; y++) {
    const grid_x = x
    const grid_y = y
    const grid_xs = [
      (grid_x - 1 + GRID_SIDE) % GRID_SIDE,
      (grid_x + GRID_SIDE) % GRID_SIDE,
      (grid_x + 1 + GRID_SIDE) % GRID_SIDE,
    ]
    const grid_ys = [
      (grid_y - 1 + GRID_SIDE) % GRID_SIDE,
      (grid_y + GRID_SIDE) % GRID_SIDE,
      (grid_y + 1 + GRID_SIDE) % GRID_SIDE,
    ]
    const grid_id_ = grid_id_3(x,y)
    grid_ids.push([])
    grid_ids[grid_id_] = [
      grid_id_3(grid_xs[0], grid_ys[0]),
      grid_id_3(grid_xs[0], grid_ys[1]),
      grid_id_3(grid_xs[0], grid_ys[2]),
      grid_id_3(grid_xs[1], grid_ys[0]),
      grid_id_3(grid_xs[1], grid_ys[1]),
      grid_id_3(grid_xs[1], grid_ys[2]),
      grid_id_3(grid_xs[2], grid_ys[0]),
      grid_id_3(grid_xs[2], grid_ys[1]),
      grid_id_3(grid_xs[2], grid_ys[2]),
    ]
    grid.push(new Set())
  }
}




let parts = []
let parts_deleted = new Set()
let links = []
let links_set = new Set()
let key_bindings = new Map()
let emeralds = []
let key_allowed = false
let winner = undefined


const sleep = (ms) => {
  if (ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
  } else {
    return new Promise(resolve => resolve());
  }
}


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


const add_link = (a_idx, b_idx, force) => {
  const link_id = a_idx < b_idx ? `${a_idx}|${b_idx}`:`${b_idx}|${a_idx}`
  if ( (! links_set.has(link_id)) || force ) {
    links.push({
      a: a_idx,
      b: b_idx,
    })
    links_set.add(link_id)
    parts[a_idx].links.add(b_idx)
    parts[b_idx].links.add(a_idx)
  }
}


const add_player_ship = async (ship, x, y) => {
  const p1_idx = parts.length
  for (let part of ship.parts) {
    const idx = add_part(
      (part.p.x-ship.center.x)/ship.DIAM*DIAM+x,
      (part.p.y-ship.center.y)/ship.DIAM*DIAM+y,
      0,
      0,
      part.kind
    )
    parts[idx].player_id = part.player_id
    if (part.binding) {
      if (!key_bindings.has(part.binding)) {
        key_bindings.set(part.binding, new Set())
      }
      key_bindings.get(part.binding).add(idx)
    }
    await sleep(LOADING_AWAIT)
  }
  for (let link of ship.links) {
    add_link(link.a+p1_idx, link.b+p1_idx)
    await sleep(LOADING_AWAIT)
  }
}


const add_ship = async (ship, x, y) => {
  const core_1_idx = parts.length
  const core_2_idx = parts.length + 1
  add_part(x - DIAM*0.5, y, 0, 0, ship.p1)
  await sleep(LOADING_AWAIT)
  add_part(x + DIAM*0.5, y, 0, 0, ship.p2)
  await sleep(LOADING_AWAIT)
  for (let part of ship.parts) {
    const p1 = parts[core_1_idx + part[0]]
    const p2 = parts[core_1_idx + part[1]]
    const pos = rotate(p1.p, p2.p, 1/6)
    const idx = add_part(pos.x, pos.y, 0, 0, part[2])
    add_link(idx, p1.idx)
    await sleep(LOADING_AWAIT)
    add_link(idx, p2.idx)
    await sleep(LOADING_AWAIT)
  }
  add_link(core_1_idx, core_2_idx)
  await sleep(LOADING_AWAIT)
  for (let linki of ship.links) {
    add_link(linki[0]+core_1_idx, linki[1]+core_1_idx)
    await sleep(LOADING_AWAIT)
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
  for (let p of parts) {
    if (p.deleted) {
      continue
    }
    if (p.activated && p.kind == 'booster')
    {
      fill_circle_2(context, add(p.p, mul(p.direction, 0.007+Math.random()*0.003)), p.d*0.7, colors[p.kind].value_3)
      fill_circle_2(context, add(p.p, mul(p.direction, 0.005+Math.random()*0.001)), p.d*0.9, colors[p.kind].value_2)
      fill_circle_2(context, p.p, p.d, colors[p.kind].value_1)
    }
    else if (p.kind == 'booster') {
      fill_circle_2(context, p.p, p.d, colors[p.kind].value)
    }
    else {
      fill_circle_2(context, p.p, p.d, colors[p.kind].value[p.player_id])
    }
  }
  for (let c_ of Object.keys(colors) ) {
    for (let l of links) {
      const p1 = parts[l.a]
      const p2 = parts[l.b]
      if (p1.deleted || p2.deleted || l.deleted) {
        continue
      }
      const wa = wrap_around(p1.np, p2.np)
      const delt = mul(delta(wa.a, wa.b), 0.5)
      const color_id = colors[p1.kind].score > colors[p2.kind].score ? p1.kind : p2.kind
      if (c_ == color_id) {
        const color = colors[color_id].value[p1.player_id]
        const aa = 0.75
        fill_circle_2(context, add(p1.p, delt), p1.d*aa, color)
        fill_circle_2(context, del(p2.p, delt), p2.d*aa, color)
      }
    }
  }
  document.getElementById("fps").innerHTML = get_fps()
  document.getElementById("ups").innerHTML = get_ups()
  if (CONTINUE_RENDER) {
    window.requestAnimationFrame(()=>{
      render(context)
    })
  } else {
    again_2()
  }
}


const update_grid = () => {
  for (var i = 0; i < CELL_COUNT; i++) {
    grid[i].clear()
  }
  for (let p of parts) {
    const grid_id_ = grid_id(p.p)
    grid[grid_id_].add(p.idx)
    p.grid_id = grid_id_
  }
}


const neighbours = (pos) => {
  const grid_id_ = grid_id(pos)
  return new Set([
    ...grid[grid_ids[grid_id_][0]],
    ...grid[grid_ids[grid_id_][1]],
    ...grid[grid_ids[grid_id_][2]],
    ...grid[grid_ids[grid_id_][3]],
    ...grid[grid_ids[grid_id_][4]],
    ...grid[grid_ids[grid_id_][5]],
    ...grid[grid_ids[grid_id_][6]],
    ...grid[grid_ids[grid_id_][7]],
    ...grid[grid_ids[grid_id_][8]],
  ])
}


const compute = () => {
  update_grid()
  let dp = 0
  for (let p of parts) {
    if (p.deleted) {
      continue
    }
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
    if (p1.deleted) {
      continue
    }
    for ( let idx2 of neighbours(p1.p) ) {
      const p2 = parts[idx2]
      if (p2.deleted) {
        continue
      }
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
          let emerald_idx = null
          let player_id = null
          if (p1.player_id !== undefined && p2.kind == 'emerald') {
            emerald_idx = p2.idx
            player_id = p1.player_id
          } else if (p2.player_id !== undefined && p1.kind == 'emerald') {
            emerald_idx = p1.idx
            player_id = p2.player_id
          }
          if (emerald_idx) {
            parts[emerald_idx].deleted = true
            parts_deleted.add(emerald_idx)
            scores[player_id] += 1
          }
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
    if (p1.deleted && p2.deleted) {
      link.deleted = true
    }
    if (p1.deleted || p2.deleted || link.deleted) {
      continue
    }
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
    if (p.deleted) {
      continue
    }
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
  // for (var i = 0; i < emeralds.length; i++) {
  //   const emerald = emeralds[i]
  //   let s = 0
  //   for (var idx of emerald) {
  //     if (parts[idx].deleted) {
  //       s += 1
  //     }
  //   }
  //   if (s === 4) {
  //     emeralds[i] = new_emerald()
  //   }
  // }
  update_ups()
  winning_condition()
  window.setTimeout(() => {
    compute()
  }, 10-get_ups_avg_delta())
}


const winning_condition = () => {
  if (winner != undefined) {
    return
  }
  let duration
  for (var i = 0; i < scores.length; i++) {
    if (scores[i] >= score_to_win) {
      winner = i
      duration = performance.now() - start_time
      break
    }
  }
  if (winner != undefined) {
    localStorage.setItem("progress", parseInt(Math.max(
      parseInt(window.location.pathname.split('journey-')[1]),
      parseInt(localStorage.getItem("progress"))
    )))
    update_best(duration, player_name())
    document.querySelector('#duration').innerHTML = `${msToTime(duration)}`
    document.querySelector('#winner').classList.remove("hide")
  }
}


const is_in_emerald = (idx) => {
  for (var emerald of emeralds) {
    for (var idx_2 of emerald) {
      if (idx == idx_2) {
        return true
      }
    }
  }
}


const get_free_idx = () => {
  if (parts_deleted.size) {
    const idx = parts_deleted.keys().next().value
    if (!is_in_emerald(idx)) {
      parts_deleted.delete(idx)
      return idx
    }
  }
  const idx = parts.length
  parts.push({})
  return idx
}


const new_emerald = (x,y) => {
  if (!x) {
    x = Math.random() * 0.8 + 0.1
  }
  if (!y) {
    y = Math.random() * 0.8 + 0.1
  }
  for (var part of parts) {
    const wa = wrap_around(part.p, {x:x, y:y})
    if (wa.d_sqrd < DIAM*DIAM*4*4) {
      return new_emerald()
    }
  }
  const free_ids = new Set()
  for (var i = 0; i < 4; i++) {
    free_ids.add( get_free_idx() )
  }
  add_ship_2(emerald,x, y, [...free_ids])
  return free_ids
}


const add_ship_2 = (ship, x, y, idxs) => {
  const core_1_idx = idxs[0]
  const core_2_idx = idxs[1]
  set_part(x - DIAM*0.5, y, 0, 0, ship.p1, idxs[0])
  set_part(x + DIAM*0.5, y, 0, 0, ship.p2, idxs[1])
  for (var i = 0; i < ship.parts.length; i++) {
    const part = ship.parts[i]
    const p1 = parts[idxs[part[0]]]
    const p2 = parts[idxs[part[1]]]
    const pos = rotate(p1.p, p2.p, 1/6)
    const idx = idxs[i+2]
    set_part(pos.x, pos.y, 0, 0, part[2], idx)
    add_link(idx, p1.idx, true)
    add_link(idx, p2.idx, true)
  }
  add_link(core_1_idx, core_2_idx, true)
  for (let linki of ship.links) {
    add_link(idxs[linki[0]], idxs[linki[1]], true)
  }
  for (let k of Object.keys(ship.key_bindings)) {
    if (!key_bindings.has(k)) {
      key_bindings.set(k, new Set())
    }
    for (let idx of ship.key_bindings[k]) {
      key_bindings.get(k).add(idxs[idx])
    }
  }
}


const set_part = (x,y,dx,dy, kind, idx) => {
  parts[idx] = {
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
  }
  return idx
}


const again = () => {
  CONTINUE_RENDER = false
}


const next = () => {
  const level = window.location.pathname.split('journey-')[1]
  window.location.pathname = `/journey-${parseInt(level)+1}`
}


const again_2 = async () => {
  CONTINUE_RENDER = true
  parts = []
  parts_deleted = new Set()
  links = []
  links_set = new Set()
  key_bindings = new Map()
  emeralds = []
  key_allowed = false
  winner = undefined
  scores = [-1,-1]
  document.querySelector('#content').innerHTML = html()
  const style_element = document.createElement('style')
  document.head.appendChild(style_element)
  for (let x of style().split('}')) {
      try {
        style_element.sheet.insertRule(x+'}');
      } catch(e) {}
  }
  const canvas = document.querySelector('#canvas')
  resize_square(canvas)
  const context = canvas.getContext('2d')
  const version = '2022.08.09'
  if (!localStorage.getItem('ship_journey') || localStorage.getItem('version') !== version ) {
    localStorage.setItem('ship_journey', default_ship_journey)
    localStorage.setItem('version', version)
  }
  render(context)
  key_allowed = false
  await add_player_ship(JSON.parse(localStorage.getItem('ship_journey')), 0.5, 0.5)
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
  const level = parseInt(window.location.pathname.split('journey-')[1])
  if (level === 0) {
    emeralds.push(new_emerald(0.5, 0.7))
    emeralds.push(new_emerald(0.5, 0.3))
  }
  else if (level === 1) {
    await add_ship(ship_2, 0.27, 0.5)
    await add_ship(ship_2, 0.5, 0.27)
    await add_ship(ship_2, 0.73, 0.5)
    await add_ship(ship_2, 0.5, 0.73)
    await add_ship(ship_2, 0.8, 0.8)
    await add_ship(ship_2, 0.2, 0.8)
    await add_ship(ship_2, 0.8, 0.2)
    await add_ship(ship_2, 0.2, 0.2)
    emeralds.push(new_emerald(0.33, 0.33))
    emeralds.push(new_emerald(0.67, 0.33))
    emeralds.push(new_emerald(0.33, 0.67))
    emeralds.push(new_emerald(0.67, 0.67))
  }
  else if (level === 2) {
    emeralds.push(new_emerald(0.27, 0.5))
    emeralds.push(new_emerald(0.5, 0.27))
    emeralds.push(new_emerald(0.73, 0.5))
    emeralds.push(new_emerald(0.5, 0.73))
    emeralds.push(new_emerald(0.8, 0.8))
    emeralds.push(new_emerald(0.8, 0.2))
    emeralds.push(new_emerald(0.2, 0.8))
    emeralds.push(new_emerald(0.2, 0.2))
    emeralds.push(new_emerald(0.33, 0.33))
    emeralds.push(new_emerald(0.67, 0.33))
    emeralds.push(new_emerald(0.33, 0.67))
    emeralds.push(new_emerald(0.67, 0.67))
  }
  else if (level === 3) {
    emeralds.push(new_emerald(0.27, 0.5))
    emeralds.push(new_emerald(0.73, 0.5))
    emeralds.push(new_emerald(0.8, 0.5))
    emeralds.push(new_emerald(0.2, 0.5))
    emeralds.push(new_emerald(0.33, 0.5))
    emeralds.push(new_emerald(0.67, 0.5))
  }
  else if (level === 4) {
    emeralds.push(new_emerald(0.27, 0.5))
    emeralds.push(new_emerald(0.5, 0.27))
    emeralds.push(new_emerald(0.73, 0.5))
    emeralds.push(new_emerald(0.5, 0.73))
    emeralds.push(new_emerald(0.8, 0.8))
    emeralds.push(new_emerald(0.2, 0.8))
    emeralds.push(new_emerald(0.8, 0.2))
    emeralds.push(new_emerald(0.2, 0.2))
    await add_ship(ship_2, 0.33, 0.33)
    await add_ship(ship_2, 0.67, 0.33)
    await add_ship(ship_2, 0.33, 0.67)
    await add_ship(ship_2, 0.67, 0.67)
  }
  else {
    emeralds.push(new_emerald())
    emeralds.push(new_emerald())
    emeralds.push(new_emerald())
    emeralds.push(new_emerald())
  }
  let level_max = 4
  const prog = parseInt(localStorage.getItem("progress") )
  if ( prog >= level-1 && level <= level_max) {
    console.log("ok")
    scores[0] = 0
  } else {
    console.log("not ok")
  }
  if (move_with_keys.size) {
    document.querySelector("#move_with_instructions").innerHTML =
      `Move with ${Array.from(move_with_keys).map(x=>x.toUpperCase()).join(", ")}`
  }
  key_allowed = true
  score_to_win = parts.filter(x => x.kind == 'emerald').length
  start_time = performance.now()
}


const journey_level = async (id) => {
  window.again = again
  window.next = next
  window.update_player_info = update_player_info
  document.addEventListener("keydown", (e) => {
    if (key_bindings.get(e.key)) {
      if (key_allowed) {
        document.querySelectorAll(".disappearable").forEach((x, i) => {
          x.classList.add('disappear')
        });
        for (let idx of key_bindings.get(e.key)) {
          parts[idx].activated = true
        }
      }
      return
    }
    if (e.key == " " ) {
      if (key_allowed) {
        again()
      }
      return
    }
    if (e.key == "Enter" ) {
      if (key_allowed && winner !== undefined) {
        next()
      }
      return
    }
    document.querySelectorAll(".disappearable").forEach((x, i) => {
      x.classList.remove('disappear')
    });
  });
  document.addEventListener("keyup", (e) => {
    if (key_bindings.get(e.key)) {
      for (let idx of key_bindings.get(e.key)) {
        parts[idx].activated = false
      }
    }
  });
  document.addEventListener("mousemove", (e) => {
    document.querySelectorAll(".disappearable").forEach((x, i) => {
      x.classList.remove('disappear')
    });
  })
  again_2()
  compute()
  document.querySelectorAll(".disappearable").forEach((x, i) => {
    x.classList.remove('disappear')
  });
}


export {
  journey_level
}
