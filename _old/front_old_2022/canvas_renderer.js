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
function render(a) {
  const start = performance.now();
  // Render
  const definition = 0.5;
  const canvas = document.getElementById("canvas");
  canvas.width  = window.innerWidth*definition;
  canvas.height = window.innerHeight*definition;
  canvas.style.width = window.innerWidth + "px";
  canvas.style.height = window.innerHeight + "px";
  const canvas_min = Math.min(canvas.width, canvas.height);
  const canvas_max = Math.max(canvas.width, canvas.height);
  const canvas_ratio = canvas_max / canvas_min;
  const ctx = canvas.getContext("2d");
  const minimap = document.getElementById("minimap");
  minimap.width  = a.image_width;
  minimap.height = a.image_height;
  const ctx_minimap = minimap.getContext("2d");
  const server_data = a.pull();
  //console.log(server_data)
  if (len(server_data)) {
    const d = JSON.parse(server_data)
    if (d.step) {
      document.getElementById("p_step").innerHTML = `Step: ${ d.step } `
    }
    document.getElementById("p_pids").innerHTML = `Particles: ${ d.pids.length } `
    //console.log("oo")
    // console.log(d.pids.length)
    let zoom = .5 * (d.blocks/d.client_blocks)*canvas_ratio;
    for (let i in d.parts) {
      let p = d.parts[i];
      let x = p.x * minimap.width;
      let y = p.y * minimap.height;
      
      ctx_minimap.beginPath();
      ctx_minimap.arc(x, y, p.d*a.image_width*0.5, 0, 2 * Math.PI);
      ctx_minimap.fill();
      let center = {
        x: 0.5,
        y: 0.5,
      };
      let x_ = (p.x - center.x) * zoom + center.x
      let y_ = (p.y - center.y) * zoom + center.y
      if (p.colissions == 0) {
        ctx.fillStyle = "black";
      } else {
        // ctx.fillStyle = "red";
      }
      ctx.beginPath();
      ctx.arc(
        x_  *  canvas_min + (canvas.width -  canvas_min) * 0.5 ,
        y_  *  canvas_min + (canvas.height - canvas_min) * 0.5 ,
        zoom * canvas_min * p.d * 0.5, 0, 2 * Math.PI);
      ctx.fill();
    }
  }
  update_fps(a)
  const end = performance.now();
  a.fps_counter.push({
    start: start,
    end: end,
    duration: end - start
  })
  loop(a)
}
function loop(x) {
  window.requestAnimationFrame(function () {
    render(x)
  })
}
export {
  render
}
