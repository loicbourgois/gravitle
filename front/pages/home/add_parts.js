import {
  attributs_count,
  float_size,
  little_endian,
} from "./constants";
import {
  rotate,
} from "./maths";



const anchors = (centers, size) => {
  let xys = []
  for (var center of centers) {
    xys.push(center)
    for (var i = 1; i < size; i++) {
      let right = [center[0]+i, center[1]]
      xys.push( right );
      xys.push( rotate(right, center, 1.0/6.0) );
      xys.push( rotate(right, center, 2.0/6.0) );
      xys.push( rotate(right, center, 3.0/6.0) );
      xys.push( rotate(right, center, 4.0/6.0) );
      xys.push( rotate(right, center, 5.0/6.0) );
    }
  }
  return xys
}


const add_parts = async (_) => {
  for (var i = 0; i < 512; i++) {
    let x = Math.random() * _.map_width
    let y = Math.random() * _.map_width
    let static_ = true;
    await add_part({
      xy: [x,y],
      dxy: [0.0, 0.0],
      static: false,
      mass: 1.0,
      gpu: _.gpu,
      map_width: _.map_width
    })
  }
  let xys = anchors(
    [[-16.0,-8.0], [16.0, 8.0]],
    8
  )
  for (let xy of xys) {
    await add_part({
      xy: xy,
      dxy: [0.0, 0.0],
      static: true,
      mass: 1.0,
      gpu: _.gpu,
      map_width: _.map_width
    })
  }
}


const add_part = async (_) => {
  await _.gpu.buffers.write.mapAsync(GPUMapMode.WRITE)
  const buffer_write = new DataView(_.gpu.buffers.write.getMappedRange())
  const p = [
    (_.xy[0] + _.map_width)%_.map_width,
    (_.xy[1] + _.map_width)%_.map_width
  ]
  const pp = [
    p[0] - _.dxy[0],
    p[1] - _.dxy[1],
  ]
  const cell_id_ = cell_id(p, _.map_width)
  const buffer_id = cell_id_ * attributs_count * float_size
  const x_id = buffer_id
  buffer_write.setFloat32(x_id,           p[0] , little_endian)
  buffer_write.setFloat32(x_id+float_size*1,  p[1] , little_endian)
  buffer_write.setFloat32(x_id+float_size*2,    pp[0] , little_endian)
  buffer_write.setFloat32(x_id+float_size*3,    pp[1] , little_endian)
  buffer_write.setInt32(x_id+float_size*4, 1, little_endian)
  buffer_write.setInt32(x_id+float_size*6, _.static, little_endian)
  buffer_write.setFloat32(x_id+float_size*7, _.mass, little_endian)
  _.gpu.buffers.write.unmap()
}


const cell_id = (xy, map_width) =>  {
  return Math.floor(xy[1] * 2 )* 2 * map_width + Math.floor(xy[0] * 2)
}


export {
  add_parts
}
