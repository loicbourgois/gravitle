struct Particle {
    idx: u32,
    diameter: f32,
    p: vec2f,
    pp: vec2f,
    np: vec2f,
    dp: vec2f,
    direction: vec2f,
    dv: vec2f,
    link_response: vec2f,
    collision_response: vec2f,
    collision_response_count: u32,
    activated: u32,
    activated_previous: u32,
    kind: u32,
}
struct VSOutput {
  @builtin(position) position: vec4f,
  @location(0) color: vec4f,
}
//__DISK_GENERATED__//
const ZOOM = 1.5;
@group(0) @binding(0) var<storage, read> particles: array<Particle>;
@vertex fn vs_0(
  @builtin(vertex_index) vertexIndex : u32,
  @builtin(instance_index) instanceIndex: u32,
) -> VSOutput {
  let particle = particles[instanceIndex];
  let center = vec2f( 0.5,  0.5);
  var vsOut: VSOutput;
  vsOut.position = vec4f(
    (disk_positions[vertexIndex]*particle.diameter*0.55 + particle.p*1.1 - center )* ZOOM,
    0.0, 1.0
  );

  // vsOut.position = vec4f(
  //   disk_positions[vertexIndex] * 0.05,
  //   0.0, 1.0
  // );

  // if (particle.live != 1) {
  //   vsOut.position.z = 100.0;
  // }
  vsOut.color = vec4f(1.0, 1.0, 0.0, 1.0);
  return vsOut;
}
@fragment fn fs_0(vsOut: VSOutput) -> @location(0) vec4f {
  return vsOut.color;
}
