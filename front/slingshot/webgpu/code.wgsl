struct Cell {
    p: vec2f,
    diameter: f32,
    material_idx: u32,
}
struct Material {
    color: vec3f,
    density: f32,
}
struct VSOutput {
  @builtin(position) position: vec4f,
  @location(0) color: vec4f,
}
const disk_positions = array<vec2f, 48>(
    vec2f( 0,  0),
    vec2f( 1,  0),
    vec2f( 0.9238795,  0.38268346),
    vec2f( 0,  0),
    vec2f( 0.9238795,  0.38268346),
    vec2f( 0.70710677,  0.70710677),
    vec2f( 0,  0),
    vec2f( 0.70710677,  0.70710677),
    vec2f( 0.38268343,  0.9238795),
    vec2f( 0,  0),
    vec2f( 0.38268343,  0.9238795),
    vec2f( -0.00000004371139,  1),
    vec2f( 0,  0),
    vec2f( -0.00000004371139,  1),
    vec2f( -0.38268352,  0.9238795),
    vec2f( 0,  0),
    vec2f( -0.38268352,  0.9238795),
    vec2f( -0.70710677,  0.70710677),
    vec2f( 0,  0),
    vec2f( -0.70710677,  0.70710677),
    vec2f( -0.9238796,  0.38268328),
    vec2f( 0,  0),
    vec2f( -0.9238796,  0.38268328),
    vec2f( -1,  -0.00000008742278),
    vec2f( 0,  0),
    vec2f( -1,  -0.00000008742278),
    vec2f( -0.9238795,  -0.38268346),
    vec2f( 0,  0),
    vec2f( -0.9238795,  -0.38268346),
    vec2f( -0.70710665,  -0.7071069),
    vec2f( 0,  0),
    vec2f( -0.70710665,  -0.7071069),
    vec2f( -0.38268313,  -0.9238797),
    vec2f( 0,  0),
    vec2f( -0.38268313,  -0.9238797),
    vec2f( 0.000000011924881,  -1),
    vec2f( 0,  0),
    vec2f( 0.000000011924881,  -1),
    vec2f( 0.3826836,  -0.92387944),
    vec2f( 0,  0),
    vec2f( 0.3826836,  -0.92387944),
    vec2f( 0.707107,  -0.70710653),
    vec2f( 0,  0),
    vec2f( 0.707107,  -0.70710653),
    vec2f( 0.92387956,  -0.38268343),
    vec2f( 0,  0),
    vec2f( 0.92387956,  -0.38268343),
    vec2f( 1,  0.00000017484555),
);
const ZOOM = 9;
@group(0) @binding(0) var<storage, read> cells: array<Cell>;
@group(0) @binding(1) var<storage, read> materials: array<Material>;
@vertex fn vs_0(
  @builtin(vertex_index) vertexIndex : u32,
  @builtin(instance_index) instanceIndex: u32,
) -> VSOutput {
  let particle = cells[instanceIndex];
  let center = vec2f( 0.0,  0.0);
  var vsOut: VSOutput;
  vsOut.position = vec4f(
    (disk_positions[vertexIndex]*particle.diameter*0.51 + particle.p - center )* ZOOM,
    0.0, 1.0
  );
  let m = materials[particle.material_idx];
  vsOut.color = vec4f(
    m.color.r, m.color.g, m.color.b, 1.0
  );
  return vsOut;
}
@fragment fn fs_0(vsOut: VSOutput) -> @location(0) vec4f {
  return vsOut.color;
}
