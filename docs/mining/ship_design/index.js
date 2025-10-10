import init, {Gravithrust} from "../gravithrust/gravithrust.js";
import {
  resize_square,
  set_draw_zoom,
} from "../canvas.js"
import {body} from "../body.js"
import {
  Simulation,
} from "../simulation.js"


let context
let canvas
let context_2
let canvas_2
let ZOOM = 2.0


init().then( wasm => {
  document.body.innerHTML = body
  const gravithrust = Gravithrust.new(
    0.0025, // diameter
    5, // substep per tick
    0.000000004, // max_rotation_speed
    128, // grid_side
    0.00001, // max_speed_at_target
    0.0001, // forward_max_speed
    30, // forward_max_angle
    35,  // slow_down_max_angle
    0.00025, // slow_down_max_speed_to_target_ratio
    0, // ship_count
  );
  gravithrust.ticks()
  canvas = document.querySelector("#canvas");
  context = canvas.getContext('2d')
  canvas_2 = document.querySelector("#canvas_2");
  context_2 = canvas_2.getContext('2d')
  resize_square(canvas, ZOOM * 0.9)
  const simulation = Simulation(gravithrust, wasm, context, context_2)
  simulation.start()
  simulation.stop_physics()
  set_draw_zoom(15)
});
