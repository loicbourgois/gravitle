struct Particle {
  // pos: vec2f,
  // kind: i32,
  // live: i32,
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
@group(0) @binding(0) var<storage, read> particles: array<Particle>;
@vertex fn vs(
  @builtin(vertex_index) vertexIndex : u32,
  @builtin(instance_index) instanceIndex: u32,
) -> VSOutput {
  let particle = particles[instanceIndex];
  let center = vec2f( 0.0,  0.0);
  //__DISK_GENERATED__//
  var vsOut: VSOutput;
  vsOut.position = vec4f(
      positions[vertexIndex]*0.0025 + particle.p * 2.0 - vec2f( 1.0,  1.0), 0.0, 1.0);
  vsOut.color = vec4f(1.0, 1.0, 0.0, f32(particle.live));
  return vsOut;
}
@fragment fn fs(vsOut: VSOutput) -> @location(0) vec4f {
  return vsOut.color;
}
