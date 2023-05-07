struct Particle {
  p: vec2f,
  v: vec2f,
  pp: vec2f,
  direction: vec2f,
  m: f32,
  k: i32,
  a: u32, // activated, usefull for boosters
  quantity: u32,
  live: u32,
  grid_id: i32,
  idx: i32,
  packer: f32,
};
struct Duration {
  a: f32,
  b: f32,
  c: f32,
  d: f32,
  e: f32,
  f: f32,
};
struct VSOutput {
  @builtin(position) position: vec4f,
  @location(0) color: vec4f,
}
//__DISK_GENERATED__//
//__KIND_GENERATED__//
const ZOOM = 3.0;
@group(0) @binding(0) var<storage, read> particles: array<Particle>;
@group(0) @binding(1) var<storage, read> avg_durations: array<Duration>;
@vertex fn vs(
  @builtin(vertex_index) vertexIndex : u32,
  @builtin(instance_index) instanceIndex: u32,
) -> VSOutput {
  let particle = particles[instanceIndex];
  let center = vec2f( 0.0,  0.0);
  var vsOut: VSOutput;
  vsOut.position = vec4f(
      (positions[vertexIndex]*0.0025 + particle.p * 2.0 - vec2f( 1.0,  1.0))* ZOOM,
      0.0, 1.0
    );
  if (particle.live != 1) {
    vsOut.position.z = 100.0;
  }
  vsOut.color = vec4f(1.0, 0.0, 1.0, 1.0);
  switch particle.k {
    case KIND_booster: {
      vsOut.color = vec4f(0.85, 0.5, 0.0, 1.0);
    }
    case KIND_armor: {
      vsOut.color = vec4f(0.85, 0.5, 0.0, 1.0);
    }
    case KIND_core: {
      vsOut.color = vec4f(0.85, 0.5, 0.0, 0.5);
    }
    case KIND_plasma_collector: {
      if (particle.a == 1) {
        vsOut.color = vec4f(0.0, 0.8, 0.8, 1.0);
      } else {
        vsOut.color = vec4f(0.8, 0.8, 0.0, 1.0);
      }
    }
    case KIND_plasma_cargo: {
      vsOut.color = vec4f(0.85, 0.5, 0.0, 0.5);
    }
    case KIND_electro_field: {
      vsOut.color = vec4f(0.0, 0.75, 1.0, 1.0);
    }
    case KIND_sun: {
      vsOut.color = vec4f(1.0, 0.9, 0.0, 1.0);
    }
    case KIND_sun_core: {
      vsOut.color = vec4f(1.0, 0.9, 0.0, 1.0);
    }
    case KIND_plasma_electro_field: {
      vsOut.color = vec4f(1.0, 0.8, 0.0, 1.0);
    }
    case KIND_plasma_depot: {
      vsOut.color = vec4f(1.0, 0.8, 0.0, 1.0);
    }
    case KIND_static: {
      vsOut.color = vec4f(1.0, 0.8, 0.0, 1.0);
    }
    case KIND_anchor: {
      vsOut.color = vec4f(0.0, 0.5, 1.0, 1.0);
    }
    case KIND_ray: {
      let quantity = f32(particle.quantity)/2500.0;
      vsOut.color = vec4f(0.0, quantity*0.5+0.35, quantity*0.5+0.25, 1.0);
    }
    
    default: {}
  }
  return vsOut;
}
@fragment fn fs(vsOut: VSOutput) -> @location(0) vec4f {
  return vsOut.color;
}
fn rand(v: vec2f) -> f32 {
    return fract(sin(dot(v, vec2(12.9898, 78.233))) * 43758.5453);
}
@vertex fn vs_2(
  @builtin(vertex_index) vertexIndex : u32,
  @builtin(instance_index) instanceIndex: u32,
) -> VSOutput {
  let particle = particles[instanceIndex];
  let center = vec2f( 0.0,  0.0);
  var vsOut : VSOutput = VSOutput(
    vec4f(
      (positions[vertexIndex]*0.0025*1.75 + particle.p * 2.0 - vec2f( 1.0,  1.0))* ZOOM,
      0.0, 1.0),
    vec4f(1.0, 0.0, 1.0, 1.0),
  );
  
  switch particle.k {
    case KIND_plasma_electro_field: {
      vsOut.color = vec4f(0.0, 0.5, 1.0, 1.0);
    }
    case KIND_sun: {
      vsOut.color = vec4f(1.0, 0.5, 0.0, 1.0);
    }
    default: {
      vsOut.position.z = 100.0;
    }
  }
  
  // if (particle.k == KIND_booster && particle.a == 1 && particle.live == 1) {
  //   let pout = vec2f(
  //       particle.p.x + particle.direction.x * 0.0018 * rand(vec2f(particle.p.x, particle.direction.x)),
  //       particle.p.y + particle.direction.y * 0.0018 * rand(vec2f(particle.p.y, particle.direction.y)),
  //   );
  //   vsOut.position = vec4f(
  //     positions[vertexIndex] * 0.0025 * 0.7
  //     + pout * 2.0 
  //     - vec2f( 1.0,  1.0), 0.0, 1.0
  //   );
  //   vsOut.position.x = vsOut.position.x * ZOOM; 
  //   vsOut.position.y = vsOut.position.y * ZOOM;
  //   vsOut.color = vec4f(1.0, 0.0, 0.0, 1.0);
  // }
  if (particle.live != 1) {
    vsOut.position.z = 100.0;
  }
  return vsOut;
}
@fragment fn fs_2(vsOut: VSOutput) -> @location(0) vec4f {
  return vsOut.color;
}


@vertex fn vs_3(
  @builtin(vertex_index) vertexIndex : u32,
  @builtin(instance_index) instanceIndex: u32,
) -> VSOutput {
  let particle = particles[instanceIndex];
  let center = vec2f( 0.0,  0.0);
  var vsOut : VSOutput = VSOutput(
    vec4f(0.0, 0.0, 0.0, 10.0),
    vec4f(0.0, 0.0, 0.0, 0.0),
  );
  let pout = vec2f(
      particle.p.x + particle.direction.x * 0.002 * rand(vec2f(particle.p.y, particle.direction.y)),
      particle.p.y + particle.direction.y * 0.002 * rand(vec2f(particle.p.x, particle.direction.x)),
  );
  if (particle.k == KIND_booster && particle.a == 1) {
    vsOut.position = vec4f(
      positions[vertexIndex] * 0.0025 * .6
      + pout * 2.0 
      - vec2f( 1.0,  1.0), 0.0, 1.0
    );
    vsOut.position.x = vsOut.position.x * ZOOM; 
    vsOut.position.y = vsOut.position.y * ZOOM;
    if (particle.live != 1) {
      vsOut.position.z = 100.0;
    }
    vsOut.color = vec4f(1.0, 0.25, 0.5, 1.0);
  }
  return vsOut;
}
@fragment fn fs_3(vsOut: VSOutput) -> @location(0) vec4f {
  return vsOut.color;
}



@vertex fn vs_4(
  @builtin(vertex_index) vertexIndex : u32,
  @builtin(instance_index) instanceIndex: u32,
) -> VSOutput {
  let i = instanceIndex / 6;
  let aa = avg_durations[i];
  let uu = 20000.0;
  var color = vec3f(1.0, 0.0, 1.0);
  var p = positions[vertexIndex]*0.01;
  p.x += (f32(i) / 1000.0)*2.0 - 1.0;
  if ( instanceIndex % 6 == 0) {
    p.y += aa.a * uu - 1.0;
    color = vec3f(1.0, 0.0, 0.0);
  }
  if ( instanceIndex % 6 == 1) {
    p.y += aa.b * uu - 1.0;
    color = vec3f(1.0, 1.0, 0.0);
  }
  if ( instanceIndex % 6 == 2) {
    p.y += aa.c * uu - 1.0;
    color = vec3f(1.0, 0.5, 0.0);
  }
  if ( instanceIndex % 6 == 3) {
    p.y += aa.d * uu - 1.0;
    color = vec3f(0.0, 1.0, 0.5);
  }
  if ( instanceIndex % 6 == 4) {
    p.y += aa.e * uu - 1.0;
    color = vec3f(0.0, 1.0, 0.0);
  }
  if ( instanceIndex % 6 == 5) {
    p.y += aa.f * uu - 1.0;
    color = vec3f(0.0, 0.5, 1.0);
  }
  var vsOut : VSOutput = VSOutput(
    vec4f(p, 0.0, 1.0),
    vec4f(color, 1.0),
  );
  return vsOut;
}
@fragment fn fs_4(vsOut: VSOutput) -> @location(0) vec4f {
  return vsOut.color;
}