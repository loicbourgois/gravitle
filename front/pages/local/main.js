import {
  resize_square,
  stroke_circle,
  stroke_circle_2,
  clear,
} from "./canvas"
import {
  collision_response,
  distance_sqrd,
  wrap_around,
  normalize,
  delta,
} from "./math"


const style = () => {
  return `
    #canvas {
        background: blue;
    }
  `
}


const parts = []
const links = []
const links_set = new Set


const add_part = (x,y,dx,dy) => {
  parts.push({
    idx: parts.length,
    d: 0.1,
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
    }
  })
}


const add_link = (a_idx, b_idx) => {
  links.push({
    a: a_idx,
    b: b_idx,
  })
  links_set.add(`${a_idx}|${b_idx}`)
}


const local_main = () => {
  document.querySelector('#content').innerHTML = `
    <canvas id="canvas"></canvas>
  `
  const style_element = document.createElement('style')
  document.head.appendChild(style_element)
  style_element.sheet.insertRule(style());
  const canvas = document.querySelector('#canvas')
  resize_square(canvas)
  const context = canvas.getContext('2d')
  stroke_circle(context, {
    x: 0.0,
    y: 0.0,
  }, 0.1, '#fff')
  stroke_circle(context, {
    x: 1.0,
    y: 0.0,
  }, 0.1, '#f00')
  stroke_circle(context, {
    x: 0.0,
    y: 1.0,
  }, 0.1, '#0f0')
  stroke_circle(context, {
    x: 1.0,
    y: 1.0,
  }, 0.1, '#ff0')
  add_part(0.5, 0.5, 0.0, 0.0)
  add_part(0.5, 0.6, 0.0, 0.0)
  add_link(0,1)
  add_part(0.2, 0.5, 0.004, 0.0)
  add_part(0.4, 0.55, 0.0, 0.0)
  render(context)
  compute()
}


const render = (context) => {
  clear(context)
  for (let p of parts) {
    stroke_circle_2(context, p.p, 0.1, '#ff0')
  }
  window.requestAnimationFrame(()=>{
    render(context)
  })
}


const compute = () => {
  let dp = 0
  for (let p of parts) {
    p.dp.x = p.p.x - p.pp.x
    p.dp.y = p.p.y - p.pp.y
    p.np.x = p.p.x + p.dp.x
    p.np.y = p.p.y + p.dp.y
    p.link_response.x = 0
    p.link_response.y = 0
    p.collision_response.x = 0
    p.collision_response.y = 0
    p.collision_response.count = 0
    dp += distance_sqrd(p.dp)
  }
  console.log(dp)
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
    const link_strength = 0.1
    const d = Math.sqrt(wa.d_sqrd)
    const n = normalize(delta(wa.a, wa.b), d)
    const ds = (p1.d + p2.d) * 0.5
    const factor = (ds - d) * link_strength
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
    p.p.x = (p.np.x + 1 ) % 1.0
    p.p.y = (p.np.y + 1 ) % 1.0
    p.pp.x = p.p.x - p.dp.x - p.collision_response.x - p.link_response.x
    p.pp.y = p.p.y - p.dp.y - p.collision_response.y - p.link_response.y
  }
  window.setTimeout(() => {
    compute()
  }, 10)
}


export {
  local_main
}
