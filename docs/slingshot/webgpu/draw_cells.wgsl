
// include file://./cell.wgsl
// include file://./uniforms.wgsl
// include file://./disk.wgsl
// include file://./material.wgsl
// include file://./storage.wgsl


struct VSOutput {
  @builtin(position) position: vec4f,
  @location(0) color: vec4f,
}


@group(0) @binding(0) var<storage, read> cells: array<Cell>;
@group(0) @binding(1) var<storage, read> materials: array<Material>;
@group(0) @binding(3) var<uniform> uniforms: Uniforms;
@group(0) @binding(5) var<storage, read> storages: array<Storage>;


@vertex fn vs(
  @builtin(vertex_index) vertexIndex : u32,
  @builtin(instance_index) cell_idx: u32,
) -> VSOutput {
  let particle = cells[cell_idx];
  let center = vec2f( 0.0,  0.0 );
  var vsOut: VSOutput;
  vsOut.position = vec4f(
    ( disk_positions[vertexIndex]*particle.diameter*0.51 + particle.ap - center )
      * uniforms.zoom,
    0.0, 1.0
  );
  let material = materials[particle.material_idx];
  let color = material.color;
  vsOut.color = vec4f(
    color.r, color.g, color.b, 1.0
  );
  vsOut.color.r = f32(storages[cell_idx].value) / 1000.0;
  return vsOut;
}


@fragment fn fs(vsOut: VSOutput) -> @location(0) vec4f {
  return vsOut.color;
}
