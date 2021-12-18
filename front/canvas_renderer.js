import {
  last,
  len,
  assert,
} from "./util";
import {
  update_fps,
} from "./renderer_util";
// import * as render_shader from "./shaders/render";
// import {materials} from "./materials";
function render(x) {
  const start = performance.now();
  // Render
  // const canvas = document.getElementById("canvas");
  // canvas.width  = x.image_width;
  // canvas.height = x.image_height;
  // canvas.style.width = window.innerWidth + "px";
  // canvas.style.height = window.innerHeight + "px";
  // const ctx = canvas.getContext("2d");

  const minimap = document.getElementById("minimap");
  minimap.width  = x.image_width;
  minimap.height = x.image_height;
  // minimap.style.width = window.innerWidth + "px";
  // minimap.style.height = window.innerHeight + "px";
  const ctx_minimap = minimap.getContext("2d");

  const server_data = x.pull();
  if (len(server_data)) {
    const d = JSON.parse(server_data)
    if (d.step) {
      document.getElementById("p_step").innerHTML = `Step: ${ d.step } `
    }
    const w = (minimap.width/d.width);
    const h = (minimap.height/d.height);
    for (let i in d.parts) {
      let p = d.parts[i];
      let x_ = p.x * w;
      let y_ = p.y * h;
      //console.log(x_, y_)
      ctx_minimap.beginPath();
      ctx_minimap.arc(x_, y_, w, 0, 2 * Math.PI);
      ctx_minimap.fill();
    }
  }
  update_fps(x)
  const end = performance.now();
  x.fps_counter.push({
    start: start,
    end: end,
    duration: end - start
  })
  loop(x)
}
function loop(x) {
  window.requestAnimationFrame(function () {
    render(x)
  })
}
export {
  render
}
