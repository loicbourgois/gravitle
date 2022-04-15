import {
  attributs_count,
  float_size,
  little_endian,
} from "./constants";
import {
  rotate,
} from "./maths";
const add_parts = async (_) => {
  //let xys = [[0.0, 0.0]]
  // xys.push( [1.0, 0.0] );
  // xys.push( rotate(xys[1], xys[0], 1.0/6.0) );
  // xys.push( rotate(xys[1], xys[0], 2.0/6.0) );
  // xys.push( rotate(xys[1], xys[0], 3.0/6.0) );
  // xys.push( rotate(xys[1], xys[0], 4.0/6.0) );
  // xys.push( rotate(xys[1], xys[0], 5.0/6.0) );
  //
  // const aa = [2.0, 0.0]
  // xys.push(aa)
  // xys.push( rotate(aa, xys[0], 1.0/6.0) );
  // xys.push( rotate(aa, xys[0], 2.0/6.0) );
  // xys.push( rotate(aa, xys[0], 3.0/6.0) );
  // xys.push( rotate(aa, xys[0], 4.0/6.0) );
  // xys.push( rotate(aa, xys[0], 5.0/6.0) );
  //
  // const bb = [3.0, 0.0]
  // xys.push(bb)
  // xys.push( rotate(bb, xys[0], 1.0/6.0) );
  // xys.push( rotate(bb, xys[0], 2.0/6.0) );
  // xys.push( rotate(bb, xys[0], 3.0/6.0) );
  // xys.push( rotate(bb, xys[0], 4.0/6.0) );
  // xys.push( rotate(bb, xys[0], 5.0/6.0) );



  for (var i = 0; i < 500; i++) {
    let x = Math.random() * _.map_width
    let y = Math.random() * _.map_width
    let static_ = true;
    // if (x > _.map_width * 0.25 && y > _.map_width * 0.25) {
    //   static_ = false
    // }
    await add_part({
      xy: [x,y],
      dxy: [0.0, 0.0],
      static: false,
      mass: 1.0,
      gpu: _.gpu,
      map_width: _.map_width
    })
  }

  // for (let xy of xys) {
  //   await add_part({
  //     xy: xy,
  //     dxy: [0.0, 0.0],
  //     static: true,
  //     mass: 1.0,
  //     gpu: _.gpu,
  //     map_width: _.map_width
  //   })
  // }

  for (var i = 0; i < 5; i++) {
    await add_part({
      xy: [i, 0.0],
      dxy: [0.0, 0.0],
      static: true,
      mass: 1.0,
      gpu: _.gpu,
      map_width: _.map_width
    })
    await add_part({
      xy: [-i, 0.0],
      dxy: [0.0, 0.0],
      static: true,
      mass: 1.0,
      gpu: _.gpu,
      map_width: _.map_width
    })
    // await add_part({
    //   xy: [0.0, i],
    //   dxy: [0.0, 0.0],
    //   static: true,
    //   mass: 1.0,
    //   gpu: _.gpu,
    //   map_width: _.map_width
    // })
    // await add_part({
    //   xy: [0.0, -i],
    //   dxy: [0.0, 0.0],
    //   static: true,
    //   mass: 1.0,
    //   gpu: _.gpu,
    //   map_width: _.map_width
    // })
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
