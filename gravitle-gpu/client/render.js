const render = () => {
  if (document.getElementById('get_true_ping').checked) {
    return
  }
  const zoom = parseFloat(document.querySelector("#slider_1").value) / 1000.0 * 9.0 + 1.0
  if (!chunk.tick) {
    return
  }
  document.getElementById('step').innerHTML = chunk.tick;
  document.getElementById('tick_duration').innerHTML = `${(chunk.average_duration / 1000.0).toFixed(2)}ms`;
  document.getElementById('momentum_x').innerHTML = chunk.momentum.x ? chunk.momentum.x.toFixed(8) : "00";
  document.getElementById('momentum_y').innerHTML = chunk.momentum.y ? chunk.momentum.y.toFixed(8) : "00";
  document.getElementById('absolute_momentum_x').innerHTML = chunk.absolute_momentum.x ? chunk.absolute_momentum.x.toFixed(8) : "00";
  document.getElementById('absolute_momentum_y').innerHTML = chunk.absolute_momentum.y ? chunk.absolute_momentum.y.toFixed(8) : "00";
  document.getElementById('kinetic_energy').innerHTML = chunk.kinetic_energy ? chunk.kinetic_energy.toFixed(8) : "00";
  // fps
  last_render_time_ms = now_ms
  now_ms = Date.now()
  const elapsed_ms = now_ms - last_render_time_ms
  fps_list.push(1.0 / (elapsed_ms / 1000.0))
  while (fps_list.length > 10) {
    fps_list.shift()
  }
  let fps_sum = 0;
  for (let i = 0 ; i < fps_list.length ; i += 1) {
    fps_sum += fps_list[i]
  }
  const fps = fps_sum / fps_list.length;
  document.getElementById('fps').innerHTML = fps.toFixed(2);
  //
  context_1.clearRect(0, 0, canvas_1.width, canvas_1.height)
  context_2.clearRect(0, 0, canvas_2.width, canvas_2.height)
  for (let particle_id in chunk.particles) {
    const particle = chunk.particles[particle_id]
    if (particle.a) {
      //draw_body(canvas_1, x, y, d, zoom, center_x, center_y)
      //draw_inactive(canvas_1, x, y, d, zoom, center_x, center_y)
      draw_particle(particle, chunk, canvas_1, zoom, center_x, center_y);
      //draw_body(canvas_2, x, y, d, 1.0, 0.5, 0.5)
    } else {
      if (conf.draw_inactive) {
        const x = particle.x / chunk.constants.width;
        const y = particle.y / chunk.constants.height;
        const d = particle.d / chunk.constants.width;
        draw_inactive(canvas_1, x, y, d, zoom, center_x, center_y)
      }
    }
  }
}
const draw_particle = (p, chunk, canvas, zoom, center_x, center_y) => {
  const x = p.x / chunk.constants.width;
  const y = p.y / chunk.constants.height;
  const d = p.d / chunk.constants.width;
  const pdid_str = chunk.pdid_to_string[p.pdid];
  const c = conf.colors[pdid_str];
  if (!c) {
    log_x_time(`no color for ${pdid_str}`);
  }
  draw_disk(canvas, x, y, d, zoom, center_x, center_y, `rgba(${c.r*255.0}, ${c.g*255.0}, ${c.b*255.0}, 0.5)`)
}

const render_stats_distance = () => {
  let l = chunk.stats.length;
  let max_distance = chunk.best_dna_ever_by_distance_traveled.distance_traveled;
  max_distance = max_distance ? max_distance : 0.0;
  let last_distance_alive = chunk.stats[l-1].best_dna_alive_by_age.distance_traveled;
  document.getElementById('best_ever_distance_traveled').innerHTML = max_distance.toFixed(5);
  document.getElementById('best_alive_distance_traveled').innerHTML = last_distance_alive.toFixed(5);
  const resolution = parseFloat(document.querySelector("#resolution").value)
  let step = l / resolution;
  for (let i = 0; i < l ; i += step) {
      let stat = chunk.stats[Math.trunc(i)]
      const x = stat.step / chunk.step * canvas_3.width;
      [
        'best_dna_alive_by_age',
        'best_dna_ever_by_age',
        'best_dna_alive_by_distance_traveled',
        'best_dna_ever_by_distance_traveled',
        'averages',
      ].forEach(element => {
        let p = {
          x: x,
          y: (1.0 - stat[element].distance_traveled  / max_distance) * canvas_3.height
        }
        draw_stat_point(canvas_3, p, conf.colors[element])
      });
  }
}
const render_stats_age = () => {
  let l = chunk.stats.length;
  let max_age = chunk.best_dna_ever_by_age.age_in_ticks;
  let max_age_alive = chunk.best_dna_alive_by_age.age_in_ticks;
  document.getElementById('best_ever_age_in_ticks').innerHTML = max_age;
  document.getElementById('best_alive_age_in_ticks').innerHTML = max_age_alive;
  const resolution = parseFloat(document.querySelector("#resolution").value)
  let step = l / resolution;
  for (let i = 0; i < l ; i += step) {
    let stat = chunk.stats[Math.trunc(i)]
    const x = stat.step / chunk.step * canvas_4.width;
    [
      'best_dna_alive_by_distance_traveled',
      'best_dna_ever_by_distance_traveled',
      'best_dna_alive_by_age',
      'best_dna_ever_by_age',
      'averages',
    ].forEach(element => {
      let p = {
        x: x,
        y: (1.0 - stat[element].age_in_ticks / max_age) * canvas_4.height
      }
      draw_stat_point(canvas_4, p, conf.colors[element])
    });
  }
}
const draw_stat_point = (canvas, p, color) => {
  const radius_canvas = 0.01 * 0.5 * canvas.width;
  const startAngle = 0;
  const endAngle = Math.PI + (Math.PI * 360) * 0.5;
  const context = canvas.getContext('2d')
  context.beginPath();
  context.arc(p.x, p.y, radius_canvas, startAngle, endAngle);
  context.fillStyle = color;
  context.fill();
}
const get_canvas_cursor_position = (canvas, event) => {
    const rect = canvas.getBoundingClientRect()
    const x = event.clientX - rect.left
    const y = event.clientY - rect.top
    return {
      x: x,
      y: y
    }
}
const draw_line = (x1, y1, x2, y2, zoom, color) => {
  const p1 = get_canvas_coord(canvas_1, x1, y1, zoom, center_x, center_y)
  const p2 = get_canvas_coord(canvas_1, x2, y2, zoom, center_x, center_y)
  context_1.beginPath()
  context_1.moveTo(p1.x, p1.y)
  context_1.lineTo(p2.x, p2.y)
  context_1.lineWidth = 2;
  context_1.strokeStyle = color
  context_1.stroke()
}
const draw_dotted_line = (x1, y1, x2, y2, zoom, color) => {
  context_1.setLineDash([1, 10]);
  draw_line(x1, y1, x2, y2, zoom, color);
  context_1.setLineDash([1, 0]);
}
const draw_link = (x1, y1, x2, y2, zoom) => {
  draw_line(x1, y1, x2, y2, zoom, conf.colors.link)
}
const draw_disk = (canvas, x, y, diameter, zoom, center_x, center_y, color) => {
  const p = get_canvas_coord(canvas, x, y, zoom, center_x, center_y)
  const radius_canvas = diameter * 0.5 * canvas.width * zoom;
  if (p.x > canvas.width + radius_canvas
    || p.x < - radius_canvas
    || p.y > canvas.height + radius_canvas
    || p.y < - radius_canvas)  {
    return
  }
  const startAngle = 0;
  const endAngle = Math.PI + (Math.PI * 360) * 0.5;
  const context = canvas.getContext('2d')
  context.beginPath();
  context.arc(p.x, p.y, radius_canvas, startAngle, endAngle);
  context.fillStyle = color;
  context.fill();
}
const draw_eye = (canvas, x, y, diameter, zoom, center_x, center_y, particle_output) => {
  let g = 255.0;
  let r = 255.0 - 255.0 *  particle_output * 0.75;
  let b = 255.0 - 255.0 * particle_output * 0.5;
  draw_disk(canvas, x, y, diameter * 0.65, zoom, center_x, center_y, `rgb(${r}, ${g}, ${b})`)
  draw_disk(canvas, x, y, diameter * 0.45, zoom, center_x, center_y, conf.colors.eye.black)
}
const draw_turbo = (canvas, x, y, diameter, zoom, center_x, center_y, particle_output) => {
  let r = conf.colors.turbo.back.r * particle_output;
  let g = conf.colors.turbo.back.g * particle_output;
  let b = conf.colors.turbo.back.b * particle_output;
  draw_disk(canvas, x, y, diameter * 0.7, zoom, center_x, center_y, `rgb(${r}, ${g}, ${b})`)
  r = conf.colors.turbo.top.r * particle_output;
  g = conf.colors.turbo.top.g * particle_output;
  b = conf.colors.turbo.top.b * particle_output;
  draw_disk(canvas, x, y, diameter * 0.55, zoom, center_x, center_y, `rgb(${r}, ${g}, ${b})`)
}
const draw_mouth = (canvas, x, y, diameter, zoom, center_x, center_y, particle_output) => {
  let r = conf.colors.mouth.back.r * (particle_output* 0.75 + 0.5);
  let g = conf.colors.mouth.back.g * (particle_output* 0.75 + 0.5);
  let b = conf.colors.mouth.back.b * (particle_output* 0.75 + 0.5);
  draw_disk(canvas, x, y, diameter * 0.7, zoom, center_x, center_y, `rgb(${r}, ${g}, ${b})`)
  r = conf.colors.mouth.top.r * (particle_output* 0.5 + 0.5);
  g = conf.colors.mouth.top.g * (particle_output* 0.5 + 0.5);
  b = conf.colors.mouth.top.b * (particle_output* 0.5 + 0.5);
  draw_disk(canvas, x, y, diameter * 0.55, zoom, center_x, center_y, `rgb(${r}, ${g}, ${b})`)

}
const draw_vision_point = (canvas, p, zoom, center_x, center_y) => {
  draw_disk(canvas, p.x, p.y, 0.005, zoom, center_x, center_y, conf.colors.vision_points)
}
const draw_body = (canvas, x, y, diameter, zoom, center_x, center_y) => {
  draw_disk(canvas, x, y, diameter, zoom, center_x, center_y, conf.colors.body)
}
const draw_inactive = (canvas, x, y, diameter, zoom, center_x, center_y) => {
  draw_disk(canvas, x, y, diameter, zoom, center_x, center_y, "rgba(255.0, 255.0, 255.0, 0.2)")
}
const draw_egg = (canvas, x, y, diameter, zoom, center_x, center_y) => {
  draw_disk(canvas, x, y, diameter, zoom, center_x, center_y, conf.colors.egg)
}
const draw_body_up = (canvas, x, y, diameter, zoom, center_x, center_y) => {
  draw_disk(canvas, x, y, diameter, zoom, center_x, center_y, conf.colors.body_up)
}
const draw_plant = (canvas, x, y, diameter, zoom, center_x, center_y, color_rgb) => {
  draw_disk(canvas, x, y, diameter, zoom, center_x, center_y,
    `rgb(${Math.trunc(color_rgb.r*255.0)}, ${Math.trunc(color_rgb.g*255.0)}, ${Math.trunc(color_rgb.b*255.0)})`
  )
}
const draw_energy = (canvas, x, y, diameter, zoom, center_x, center_y, energy) => {
  diameter = Math.max(0.0, diameter * ( energy / chunk.constants.energy_max ))
  draw_disk(canvas, x, y, diameter, zoom, center_x, center_y, conf.colors.health)
}
const draw_output = (canvas, x, y, diameter, zoom, center_x, center_y, output) => {
  const r = 255.0 * output;
  const g = 255.0 * output;
  const b = 0.0;//(1.0 - output) * 255.0;
  // const a = 0.5;
  const color = `rgba(${r}, ${g}, ${b})`
  draw_disk(canvas, x, y, diameter*0.8, zoom, center_x, center_y, color)
}
const get_canvas_coord = (canvas, x, y, zoom, center_x, center_y) => {
  y = 1.0 - y
  x = x * zoom
  x = x - center_x * zoom + 0.5
  y = y * zoom
  y = y - center_y * zoom + 0.5
  return {
    x: canvas.width * x,
    y: canvas.height * y
  }
}
