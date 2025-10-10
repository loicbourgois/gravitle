import {
  last,
  len,
} from "./util";

function update_fps(x) {
  if (len(x.fps_counter) > 0) {
    while (len(x.fps_counter) > x.fps_counter_length) {
      x.fps_counter.shift()
    }
    let render_duration = 0.0;
    for (let f of x.fps_counter) {
      render_duration += f.duration;
    }
    render_duration /= len(x.fps_counter)
    document.getElementById("p_render_duration").innerHTML = `Render: ${render_duration.toFixed(2)}ms`
    document.getElementById("p_fps").innerHTML = `FPS: ${ (len(x.fps_counter) / (last(x.fps_counter).end - x.fps_counter[0].start) * 1000).toFixed(1) } `
  }
}

export {
  update_fps
}
