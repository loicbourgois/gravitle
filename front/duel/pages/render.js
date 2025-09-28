import {
  colors,
} from './colors.js'
import {
  stroke_circle_2,
  fill_circle_2,
  clear,
  fill_text,
} from "./canvas.js"
import {
  wrap_around,
  delta,
  add,
  mul,
  del,
} from "./math.js"


const render_skeleton = (context, parts, options, link_options, mouse_position, links) => {
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
    else if (p.kind == 'booster') {
      fill_circle_2(context, p.p, p.d*0.75, colors[p.kind].value)
    }
    else {
      fill_circle_2(context, p.p, p.d*0.75, colors[p.kind].value[p.player_id])
    }
    if (p.binding) {
      fill_text(context, p.p, p.binding, 20, "#222")
    }
  }
  for (let option of options) {
    stroke_circle_2(context, option.p, option.d*0.75, colors[option.kind].value[option.player_id])
  }
  for (let link_option of link_options) {
    stroke_circle_2(context, link_option.p, link_option.d*0.75, colors[link_option.kind].value[link_option.player_id])
  }
  stroke_circle_2(context, mouse_position, 0.01, "#222")
  stroke_circle_2(context, mouse_position, 0.013, "#ffa")
  for (let l of links) {
    if (l.deleted) {
      continue
    }
    fill_circle_2(context, l.p, l.d*0.75, '#8e8')
  }
}


const render = (context, parts, links) => {
  for (let p of parts) {
    if (p.deleted) {
      continue
    }
    if (p.activated && p.kind == 'booster')
    {
      fill_circle_2(context, add(p.p, mul(p.direction, 0.007+Math.random()*0.003)), p.d*0.7, colors[p.kind].value_3)
      fill_circle_2(context, add(p.p, mul(p.direction, 0.005+Math.random()*0.001)), p.d*0.9, colors[p.kind].value_2)
      fill_circle_2(context, p.p, p.d, colors[p.kind].value_1)
    } else if (p.kind == 'booster') {
      fill_circle_2(context, p.p, p.d, colors[p.kind].value)
    }
    else {
      fill_circle_2(context, p.p, p.d, colors[p.kind].value[p.player_id])
    }

  }
  for (let c_ of Object.keys(colors) ) {
    for (let l of links) {
      if (l.deleted) {
        continue
      }
      const p1 = parts[l.a]
      const p2 = parts[l.b]
      const wa = wrap_around(p1.np, p2.np)
      const delt = mul(delta(wa.a, wa.b), 0.5)
      const color_id = colors[p1.kind].score > colors[p2.kind].score ? p1.kind : p2.kind
      if (c_ == color_id) {
        const color = colors[color_id].value[p2.player_id]
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
}


export {
  render,
  render_skeleton,
}
