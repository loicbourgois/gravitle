
// const compute = () => {
//   computes.push({
//     time: performance.now()
//   })
//   while (computes.length > 100) {
//     computes.shift()
//   }
//   const cps_value = 1000/((computes[computes.length-1].time - computes[0].time) / computes.length)
//   byid("value_cps").innerHTML = cps_value.toFixed(1)
//   for (let i = 0 ; i < map_size*gravity_field_density ; i+=1) {
//     for (let j = 0 ; j < map_size*gravity_field_density ; j+=1) {
//       const gf_x = i / gravity_field_density;
//       const gf_y = j / gravity_field_density;
//       const id = gf_id(i,j)
//       gravity_field[id][0] = 0.0;
//       gravity_field[id][1] = 0.0;
//       const a = [gf_x, gf_y]
//       for (let part of parts) {
//         const b = [part.x, part.y]
//         const dv = min_dv(dvs(a,b))
//         const d_sqrd = distance_sqrd(dv)
//         const G = 1.0;
//         const f = G * mass * mass / d_sqrd
//         const d = Math.sqrt(d_sqrd)
//         const n = normalized(dv, d)
//         gravity_field[id][0] += f * n[0];
//         gravity_field[id][1] += f * n[1];
//       }
//     }
//   }
//   setTimeout(compute, 0);
// }


// const draw = async () => {
//   frames.push({
//     time: performance.now()
//   })
//   while (frames.length > 100) {
//     frames.shift()
//   }
//   const fps_value = 1000/((frames[frames.length-1].time - frames[0].time) / frames.length)
//   byid("value_fps").innerHTML = fps_value.toFixed(1)
//   context.fillStyle = "#222"
//
//   context.fillRect(0, 0, canvas.width, canvas.height);
//   const unit = canvas.width/map_size;
//
//
//
//   const buffer_read = new DataView(data_out_buffer.buffer)
//   if (DRAW_CIRCLES) {
//     for (var j = 0; j < grid_width; j++) {
//       for (var i = 0; i < grid_width; i++) {
//         const cell_id = j * grid_width + i;
//         const buffer_id = cell_id * attributs_count * float_size
//         // const x = buffer_read.getFloat32(buffer_id,little_endian)
//         // const y = buffer_read.getFloat32(buffer_id+float_size,little_endian)
//         const p = {
//           x: buffer_read.getFloat32(buffer_id,little_endian),
//           y: buffer_read.getFloat32(buffer_id+float_size,little_endian),
//           xx: buffer_read.getFloat32(buffer_id+float_size*2,little_endian),
//           yy: buffer_read.getFloat32(buffer_id+float_size*3,little_endian),
//         }
//         p.x = (p.x + p.xx)*0.5
//         p.y = (p.y + p.yy)*0.5
//         // const pp = {
//         //   x: buffer_read.getFloat32(buffer_id,little_endian),
//         //   y: buffer_read.getFloat32(buffer_id+float_size,little_endian)
//         // }
//         const enabled = buffer_read.getInt32(buffer_id+float_size*4,little_endian)
//         const debug = buffer_read.getInt32(buffer_id+float_size*5,little_endian)
//         if (enabled === 1) {
//           context.beginPath();
//           context.arc(
//             (canvas.width + canvas.width*0.5 + p.x*unit)%canvas.width,
//             (canvas.width + canvas.width*0.5 - p.y*unit)%canvas.width,
//             unit*0.5, 0, Math.PI * 2, true);
//           context.fillStyle = "#8808"
//           if (debug && DRAW_COLLISIONS) {
//             context.fillStyle = "#f80"
//           }
//           context.fill()
//         }
//       }
//     }
//   }
//
//   if (DRAW_SQUARES) {
//     for (var j = 0; j < grid_width; j++) {
//       for (var i = 0; i < grid_width; i++) {
//         const cell_id = j * grid_width + i;
//         const buffer_id = cell_id * attributs_count * float_size
//         const x = buffer_read.getFloat32(buffer_id, little_endian)
//         const y = buffer_read.getFloat32(buffer_id+float_size, little_endian)
//         const x2 = i * 0.5 + 0.25
//         const y2 = j * 0.5 + 0.25
//         const enabled = buffer_read.getInt32(buffer_id+float_size*4,little_endian)
//         const xx = (canvas.width + canvas.width*0.5 + x*unit)%canvas.width
//         const yy = (canvas.width + canvas.width*0.5 - y*unit)%canvas.width
//         const xx2 = (canvas.width + canvas.width*0.5 + x2*unit)%canvas.width
//         const yy2 = (canvas.width + canvas.width*0.5 - y2*unit)%canvas.width
//         if (enabled === 1) {
//           context.fillStyle = "#a00"
//           context.strokeStyle = "#d00"
//           context.beginPath();
//           context.arc(
//             xx,
//             yy,
//             unit*0.05, 0, Math.PI * 2, true);
//           context.fill()
//           context.beginPath();
//           context.moveTo(xx, yy);
//           context.lineTo(xx2,yy2);
//           context.stroke();
//         }
//         if (enabled === 2) {
//           context.fillStyle = "#0f02"
//           context.fillRect(xx2-unit*0.25, yy2-unit*0.25, unit*0.5, unit*0.5);
//         }
//       }
//     }
//   }
//
//
//
//   context.strokeStyle = "#480"
//   context.fillStyle = "#480"
//   for (let i = 0 ; i < map_size*gravity_field_density ; i+=1) {
//     for (let j = 0 ; j < map_size*gravity_field_density ; j+=1) {
//       const x = i/gravity_field_density;
//       const y = j/gravity_field_density;
//       const id = gf_id(i,j)
//       const xc = (canvas.width + canvas.width*0.5 + x*unit)%canvas.width
//       const yc = (canvas.width + canvas.width*0.5 - y*unit)%canvas.width
//       context.beginPath();
//       context.moveTo(
//         xc,
//         yc);
//       const dgf_nd = normalized(gravity_field[id])
//       const dgfx = dgf_nd[0]*unit/(gravity_field_density+1)
//       const dgfy = dgf_nd[1]*unit/(gravity_field_density+1)
//       context.lineTo(
//         xc + dgfx,
//         yc - dgfy);
//       context.stroke();
//       context.beginPath();
//       context.arc(
//         xc + dgfx,
//         yc - dgfy,
//         unit*0.03, 0, Math.PI * 2, true);
//       context.fill()
//     }
//   }
//
//
//   context.fillStyle = "#333"
//   for (let x = 1; x < grid_width; x++) {
//     // context.fillRect(x/grid_width*canvas.width, 0, 1, canvas.height);
//     // context.fillRect(0, x/grid_width*canvas.width, canvas.width, 1  );
//   }
//   context.fillStyle = "#666"
//   for (let x = 1; x < map_width; x++) {
//     // context.fillRect(x/map_width*canvas.width, 0, 1, canvas.height);
//     // context.fillRect(0, x/map_width*canvas.width, canvas.width, 1  );
//   }
//   context.fillStyle = "#840"
//   if (DRAW_ORIGIN) {
//     context.fillRect(0.5*canvas.width, 0, 1, canvas.height);
//     context.fillRect(0, 0.5*canvas.width, canvas.height, 1);
//   }
//
//
//   if (LOOP_DRAW) {
//     requestAnimationFrame(() => {
//       requestAnimationFrame(() => {
//         draw()
//       })
//     })
//   }
// }
