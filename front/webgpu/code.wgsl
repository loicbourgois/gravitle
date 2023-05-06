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
struct VSOutput {
  @builtin(position) position: vec4f,
  @location(0) color: vec4f,
}
//__DISK_GENERATED__//
//__KIND_GENERATED__//
const ZOOM = 3.0;
@group(0) @binding(0) var<storage, read> particles: array<Particle>;
@vertex fn vs(
  @builtin(vertex_index) vertexIndex : u32,
  @builtin(instance_index) instanceIndex: u32,
) -> VSOutput {
  let particle = particles[instanceIndex];
  let center = vec2f( 0.0,  0.0);
  var vsOut: VSOutput;
  vsOut.position = vec4f(
      positions[vertexIndex]*0.0025 + particle.p * 2.0 - vec2f( 1.0,  1.0), 0.0, 1.0);
  vsOut.position.x = vsOut.position.x * ZOOM; 
  vsOut.position.y = vsOut.position.y * ZOOM;
  if (particle.live != 1) {
    vsOut.position.z = 100.0;
  }
  vsOut.color = vec4f(1.0, 0.9, 0.0, 1.0);
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
    vec4f(0.0, 0.0, 0.0, 10.0),
    vec4f(0.0, 0.0, 0.0, 0.0),
  );
  let pout = vec2f(
      particle.p.x + particle.direction.x * 0.0018 * rand(vec2f(particle.p.x, particle.direction.x)),
      particle.p.y + particle.direction.y * 0.0018 * rand(vec2f(particle.p.y, particle.direction.y)),
  );
  if (particle.k == KIND_booster && particle.a == 1) {
    vsOut.position = vec4f(
      positions[vertexIndex] * 0.0025 * 0.7
      + pout * 2.0 
      - vec2f( 1.0,  1.0), 0.0, 1.0
    );
    vsOut.position.x = vsOut.position.x * ZOOM; 
    vsOut.position.y = vsOut.position.y * ZOOM;
    if (particle.live != 1) {
      vsOut.position.z = 100.0;
    }
    vsOut.color = vec4f(1.0, 0.0, 0.0, 1.0);
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