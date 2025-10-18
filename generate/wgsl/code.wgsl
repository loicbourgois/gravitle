const USER_KIND_USER = 1;
const USER_KIND_GHOST = 2;
const USER_KIND_OTHER = 3;
{kind}
{cell}
struct VSOutput {
  @builtin(position) position: vec4f,
  @location(0) color: vec4f,
}
{disk}
const ZOOM = 1.5;
@group(0) @binding(0) var<storage, read> particles: array<Cell>;
@vertex fn vs_0(
  @builtin(vertex_index) vertexIndex : u32,
  @builtin(instance_index) instanceIndex: u32,
) -> VSOutput {
  let particle = particles[instanceIndex];
  let center = vec2f( 0.5,  0.5);
  var vsOut: VSOutput;
  var aa = 1.0;
  vsOut.position = vec4f(
    (disk_positions[vertexIndex]*particle.diameter*0.55*aa + particle.p*1.1 - center )* ZOOM,
    0.0, 1.0
  );
  {colors}
  if (particle.user_kind == USER_KIND_USER) {
    if (particle.kind == KIND_ASTEROID) {
      vsOut.position.z = 0.0;
    } else {
      vsOut.position.z = 0.1;
    }
  } else {
    vsOut.position.z = 0.2;
  }
  return vsOut;
}
@fragment fn fs_0(vsOut: VSOutput) -> @location(0) vec4f {
  return vsOut.color;
}
