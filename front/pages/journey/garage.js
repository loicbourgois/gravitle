import {
  set_css,
  set_html,
} from "../utils"
import {
  render,
  render_skeleton,
} from "../render"
import {
  resize_square,
} from "../canvas"
import {
  rotate,
  add,
  distance_sqrd,
} from "../math"
import {
  default_ship_journey,
} from "../ship"


const html = () => {
  return `
    <div>
      <p>Build and customize ships in the garage.</p>
      <p>Big circles are for parts.</p>
      <p>Small circles are for links.</p>
      <p>Hover on a part and press a key to bind it. Press again to unbind.</p>
    </div>
    <canvas id="canvas"></canvas>
    <div>
      <p><select id="select_kind">
        <option value="armor">Armor</option>
        <option value="booster">Booster</option>
        <option value="core">Core</option>
      </select></p>
      <p>Parts: <span id="parts_count">...</span></p>
      <p>Links: <span id="links_count">...</span></p>
      <p><button id="go_button">Let's Go</button></p>
    </div>
  `
}


const css = () => {
  return `
    * {
      color: #ffa;
      background: transparent;
    }
    #content {
      display: flex;
      width: 100%;
      height: 100%;
      align-content: center;
      align-items: center;
      flex-direction: row;
    }
    button {
      border: none;
      padding: 0.8rem;
      background: #fff1;
    }
    button:hover {
      cursor: pointer;
      background: #fff2;
    }
    #content > div {
      width: 0;
      flex-grow: 1;
    }
    p {
      text-align: center;
      color: #ffa;
      font-family: monospace;
      margin: 2rem;
    }
    #canvas {
        background: #113;
        display:flex;
        position: unset;
        cursor: none;
    }
    body {
      background: #113;
    }
    select {
      border: none;
      background: #fff2;
      padding: 0.8rem;
    }
  `
}


const parts = []
const links = []
const links_map = new Map()
const options = []
const link_options = []


const is_inside = (point, part, precision=1.0) => {
  return distance_sqrd(point, part.p) < part.d*part.d*0.25*precision
}


const add_part = (x,y,dx,dy, kind, player_id, d=DIAM) => {
  const idx = parts.length
  parts.push({
    idx: idx,
    kind: kind,
    d: d,
    p: {
      x: x,
      y: y,
    },
    player_id:player_id,
    links: new Set(),
    direction: {x:0, y:0},
  })
  add_links(parts[idx])
  return idx
}


const add_link = (a_idx, b_idx) => {
  const link_id = a_idx < b_idx ? `${a_idx}|${b_idx}`:`${b_idx}|${a_idx}`
  let p1 = parts[a_idx]
  let p2 = parts[b_idx]
  if (! links_map.has(link_id)) {
    const link_idx = links.length
    links.push({
      idx: link_idx,
      a: a_idx,
      b: b_idx,
      d: DIAM * 0.5,
      p: {
        x: (p1.p.x + p2.p.x)*0.5,
        y: (p1.p.y + p2.p.y)*0.5,
      }
    })
    links_map.set(link_id, link_idx)
    parts[a_idx].links.add({
      part_idx: b_idx,
      link_idx: link_idx,
    })
    parts[b_idx].links.add({
      part_idx: a_idx,
      link_idx: link_idx,
    })
    return link_idx
  } else {
    return links_map.get(link_id)
  }
}


const add_links = (p1) => {
  const p2_p = add(p1.p, {x:DIAM*1,y:0})
  for (var i = 0; i < 6; i++) {
    const pos_2 = rotate(p1.p, p2_p, i/6)
    for (let p2 of parts) {
      if (is_inside(pos_2, p2) && !p2.deleted) {
        const lidx = add_link(p1.idx, p2.idx)
        links[lidx].deleted = false
      }
    }
  }
}


const DIAM = 0.04


const add_option = (x,y,d=DIAM) => {
  for (let option of options) {
    if (
      is_inside({x:x,y:y}, option)
    ) {
      return
    }
  }
  const idx = options.length
  options.push({
    idx: idx,
    d: d,
    p: {
      x: x,
      y: y,
    },
    kind: 'option'
  })
}


const add_link_option = (x,y,d=DIAM) => {
  for (let link_option of link_options) {
    if (
      is_inside({x:x,y:y}, link_option)
    ) {
      return
    }
  }
  const parts_ = []
  const part_ = add({x:x,y:y}, {x:DIAM*0.5,y:0})
  for (var i = 0; i < 6; i++) {
    const pos = rotate({x:x,y:y}, part_, i/6)
    for (let part of parts) {
      if (
        !part.deleted
        && is_inside(pos, part, 0.2)
      ) {
        parts_.push(part.idx)
      }
    }
  }
  if (parts_.length != 2) {
    return
  }
  const idx = link_options.length
  link_options.push({
    idx: idx,
    d: d,
    p: {
      x: x,
      y: y,
    },
    kind: 'link-option'
  })
}


const add_options = (p1) => {
  const option_ = add(p1.p, {x:DIAM,y:0})
  const link_option_ = add(p1.p, {x:DIAM*0.5,y:0})
  for (var i = 0; i < 6; i++) {
    const pos = rotate(p1.p, option_, i/6)
    add_option(pos.x, pos.y)
    const pos2 = rotate(p1.p, link_option_, i/6)
    add_link_option(pos2.x, pos2.y, DIAM*0.5)
  }
}


const reset_options = () => {
  options.length = 0
  link_options.length = 0
  for (let part of parts) {
    if (!part.deleted) {
      add_options(part)
    }
  }
  save_ship()
}


const mouse_position = {
  x:-1,
  y:-1,
}


const save_ship = () => {
  localStorage.setItem('ship_journey', JSON.stringify(small_ship(true_ship())));
  console.log("Ship saved")
}


const true_ship = () => {
  const links_mieux = JSON.parse(JSON.stringify(links))
  const parts_mieux = JSON.parse(JSON.stringify(parts))
  let idx = 0
  for (let part of parts_mieux) {
    if (!part.deleted) {
      parts_mieux[part.idx].idx_mieux = idx
      idx += 1
    }
  }
  const parts_f = []
  const links_f = []
  for (let part of parts_mieux) {
    if (!part.deleted) {
      parts_f.push({
        p: {
          x: (part.p.x - 0.5)*0.25+0.8,
          y: (part.p.y - 0.5)*0.25+0.8,
        },
        d: part.d*0.25,
        np: {
          x: (part.p.x - 0.5)*0.25+0.8,
          y: (part.p.y - 0.5)*0.25+0.8,
        },
        player_id: part.player_id,
        kind: part.kind,
        binding: part.binding,
      })
    }
  }
  for (let link of links_mieux) {
    if (!link.deleted) {
      links_f.push({
        a: parts_mieux[link.a].idx_mieux,
        b: parts_mieux[link.b].idx_mieux,
      })
    }
  }
  return {
    links: links_f,
    parts: parts_f,
    center: {
      x: 0.8,
      y: 0.8,
    },
    DIAM: DIAM*0.25,
  }
}


const small_ship = (ship) => {
  const s_ship = JSON.parse(JSON.stringify(ship))
  for (let part of s_ship.parts) {
    delete part.np
  }
  return s_ship
}


const render_loop = (context) => {
  render_skeleton(context, parts, options, link_options, mouse_position, links)
  document.querySelector('#parts_count').innerHTML = parts.filter(x=>!x.deleted).length
  document.querySelector('#links_count').innerHTML = links.filter(x=>!x.deleted).length
  const ship = true_ship()
  render(context, ship.parts, ship.links)
  window.requestAnimationFrame(()=>{
    render_loop(context)
  })
}


const journey_garage = () => {
  set_html(html())
  set_css(css())
  const canvas = document.querySelector('#canvas')
  canvas.addEventListener("click", (e) => {
    const x = (e.x - e.target.offsetLeft)/e.target.width
    const y = 1.0 - (e.y - e.target.offsetTop)/e.target.height
    for (let part of parts) {
      if (is_inside({x:x,y:y}, part)) {
        {
          part.deleted = !part.deleted
          if (part.deleted) {
            for (var l_ of part.links) {
              links[l_.link_idx].deleted = true
            }
          } else {
            add_links(part)
            part.kind = document.querySelector("#select_kind").value
            part.player_id = 0
          }
          reset_options()
        }
        return
      }
    }
    for (let option of options) {
      if (is_inside({x:x,y:y}, option)) {
        const aa = add_part(
          option.p.x,
          option.p.y,
          0, 0,
          document.querySelector("#select_kind").value,
          0)
        reset_options()
        return
      }
    }
  })
  canvas.addEventListener("mousemove", (e) => {
    const x = (e.x - e.target.offsetLeft)/e.target.width
    const y = 1.0 - (e.y - e.target.offsetTop)/e.target.height
    mouse_position.x = x
    mouse_position.y = y
  })
  document.addEventListener("keydown", (e) => {
    for (let part of parts) {
      if (is_inside(mouse_position, part)) {
        if (part.binding === e.key) {
          part.binding = null
        } else {
          part.binding = e.key
        }
      }
    }
    save_ship()
  });
  document.querySelector("#go_button").addEventListener("click", () => {
    save_ship()
    window.location.href = "/journey"
  })
  const context = canvas.getContext('2d')
  resize_square(canvas)
  render_loop(context)
  let ship = JSON.parse(localStorage.getItem('ship_journey'))
  ship = ship ? ship : JSON.parse(default_ship_journey)
  const factor = ship.DIAM / DIAM
  if (ship && ship.parts.length) {
    for (let part of ship.parts) {
      parts.push({
        idx: parts.length,
        p: {
          x: (part.p.x - ship.center.x)/factor+0.5,
          y: (part.p.y - ship.center.y)/factor+0.5,
        },
        d: part.d/factor,
        kind: part.kind,
        binding: part.binding,
        player_id: part.player_id,
        links: new Set(),
        direction: {x:0, y:0},
      })
    }
    for (let link of ship.links) {
      add_link(link.a, link.b)
    }
  }
  else
  {
    add_part(0.5, 0.5, 0, 0, 'core')
  }
  reset_options()
}


export {
  journey_garage
}
