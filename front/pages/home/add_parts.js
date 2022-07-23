import {
  attributs_count,
  float_size,
  little_endian,
  kind,
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


const random_part = () => {
  // for (var i = 0; i < 0; i++) {
  //   let x = Math.random() * _.map_width
  //   let y = Math.random() * _.map_width
  //   let static_ = true;
  //   let kinds = [
  //     kind.carbon,
  //     kind.water,
  //   ]
  //   await add_part({
  //     xy: [x,y],
  //     dxy: [0.0, 0.0],
  //     static: false,
  //     mass: 1.0,
  //     gpu: _.gpu,
  //     map_width: _.map_width,
  //     kind: kinds[parseInt(Math.random()*2.0)]
  //   })
  // }
}


const add_ball = async (_) => {
  for (let xy of anchors(
    [_.xy],
    2
  )) {
    await add_part({
      xy: xy,
      dxy: [0.0, 0.0],
      static: true,
      mass: 2.0,
      gpu: _.gpu,
      map_width: _.map_width,
      kind: _.kind,
    })
  }
}


const add_parts = async (_) => {
  for (var xyk of [
    [[0,0], kind.iron],
    [[7,5], kind.carbon],
    [[-3,8], kind.ice],
    [[-7,-4], kind.stone],
    [[-7,-8], kind.water],
  ]) {
    await add_ball({
      gpu: _.gpu,
      map_width: _.map_width,
      kind: xyk[1],
      xy: xyk[0]
    })
  }


  // await add_part({
  //   xy: [2.5,9.0],
  //   dxy: [0.0, 0.0],
  //   static: false,
  //   mass: 1.0,
  //   gpu: _.gpu,
  //   map_width: _.map_width,
  //   kind: kind.miner
  // })

  await add_part({
    xy: [-2,7.0],
    dxy: [0.0, 0.0],
    static: false,
    mass: 1.0,
    gpu: _.gpu,
    map_width: _.map_width,
    kind: kind.miner
  })

  await add_part({
    xy: [2,-1.0],
    dxy: [0.0, 0.0],
    static: false,
    mass: 1.0,
    gpu: _.gpu,
    map_width: _.map_width,
    kind: kind.launcher
  })

  await add_part({
    xy: [2,2],
    dxy: [0.0, 0.0],
    static: false,
    mass: 1.0,
    gpu: _.gpu,
    map_width: _.map_width,
    kind: kind.launcher
  })

  await add_part({
    xy: [3, 1.0],
    dxy: [0.0, 0.0],
    static: false,
    mass: 1.0,
    gpu: _.gpu,
    map_width: _.map_width,
    kind: kind.launcher
  })


  await add_part({
    xy: [7, 8],
    dxy: [0.0, 0.0],
    static: false,
    mass: 1.0,
    gpu: _.gpu,
    map_width: _.map_width,
    kind: kind.heater
  })

  await add_part({
    xy: [8, 8],
    dxy: [0.0, 0.0],
    static: false,
    mass: 1.0,
    gpu: _.gpu,
    map_width: _.map_width,
    kind: kind.launcher
  })
  await add_part({
    xy: [10, 8],
    dxy: [0.0, 0.0],
    static: false,
    mass: 1.0,
    gpu: _.gpu,
    map_width: _.map_width,
    kind: kind.launcher
  })

  await add_part({
    xy: [7, 9],
    dxy: [0.0, 0.0],
    static: false,
    mass: 1.0,
    gpu: _.gpu,
    map_width: _.map_width,
    kind: kind.heater
  })
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
  buffer_write.setInt32(x_id+float_size*6, _.static ? 1 : 0 , little_endian)
  buffer_write.setFloat32(x_id+float_size*7, _.mass, little_endian)
  buffer_write.setInt32(x_id+float_size*8, _.kind, little_endian)
  _.gpu.buffers.write.unmap()
}


const add_part_2 = (_) => {
  const buffer_write = _.buffer_write
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
  buffer_write.setInt32(x_id+float_size*6, _.static ? 1 : 0 , little_endian)
  buffer_write.setFloat32(x_id+float_size*7, _.mass, little_endian)
  buffer_write.setInt32(x_id+float_size*8, _.kind, little_endian)
}


const cell_id = (xy, map_width) =>  {
  return Math.floor(xy[1] * 2 )* 2 * map_width + Math.floor(xy[0] * 2)
}


export {
  add_parts,
  add_part_2,
}
