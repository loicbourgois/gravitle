const fps_list = []
const fps_size = 100
let fps = 0
const update_fps = () => {
  fps_list.push(performance.now())
  while (fps_list.length > fps_size) {
    fps_list.shift()
  }
  const length = fps_list.length
  const last_id = length-1
  const avg_delta = (fps_list[last_id] - fps_list[0])/length
  fps = 1000/avg_delta
}
const get_fps = () => {
  return fps.toFixed(1)
}


const ups_list = []
const ups_size = 100
let ups = 0
let ups_avg_delta = 0
const update_ups = () => {
  ups_list.push(performance.now())
  while (ups_list.length > ups_size) {
    ups_list.shift()
  }
  const length = ups_list.length
  const last_id = length-1
  ups_avg_delta = (ups_list[last_id] - ups_list[0])/length
  ups = 1000/ups_avg_delta
}
const get_ups = () => {
  return ups.toFixed(1)
}
const get_ups_avg_delta = () => {
  return ups_avg_delta
}


export {
  update_fps,
  get_fps,
  update_ups,
  get_ups,
  get_ups_avg_delta,
}
