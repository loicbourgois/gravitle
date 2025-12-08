struct Cell {
     p: vec2f, // position
     pp: vec2f, // previous position
     ap: vec2f, // average position
     dp: vec2f, // delta position
     dv: vec2f, // delta velocity
     material_idx: u32,
     mass: f32,
     diameter: f32,
     fixed: u32,
     collision_count: f32,
     padding: u32,
}


struct Position {
     p: vec2f, // position
}


struct Link {
  // reference to first cell
  a: u32,
  // reference to second cell
  b: u32,
}


struct Material {
    color: vec3f,
    density: f32,
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


struct Uniforms {
    zoom: f32,
    line_width: f32,
    tick: u32,
}


struct VSOutput {
  @builtin(position) position: vec4f,
  @location(0) color: vec4f,
}


struct VSOutputLinks {
  @builtin(position) position: vec4f,
  @location(0) link_t: f32,
}


@group(0) @binding(0) var<storage, read> cells: array<Cell>;
@group(0) @binding(1) var<storage, read> materials: array<Material>;
@group(0) @binding(2) var<storage, read> positions: array<Position>;
@group(0) @binding(3) var<uniform> uniforms: Uniforms;
@group(0) @binding(4) var<storage, read> links: array<Link>;


@vertex fn vs_0(
  @builtin(vertex_index) vertexIndex : u32,
  @builtin(instance_index) instanceIndex: u32,
) -> VSOutput {
  let particle = cells[instanceIndex];
  let center = vec2f( 0.0,  0.0 );
  var vsOut: VSOutput;
  vsOut.position = vec4f(
    ( disk_positions[vertexIndex]*particle.diameter*0.51 + particle.ap - center )
      * uniforms.zoom,
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


@vertex fn vs_1(
  @builtin(vertex_index) vertexIndex : u32,
  @builtin(instance_index) instanceIndex: u32,
) -> VSOutput {
  let position = positions[instanceIndex];
  let center = vec2f( 0.0,  0.0);
  var vsOut: VSOutput;
  vsOut.position = vec4f(
    (disk_positions[vertexIndex]*0.0002 + position.p - center ) 
      * uniforms.zoom,
    0.0, 1.0
  );
  vsOut.color = vec4f(
    1.0, 1.0, 0.0, 1.0
  );
  return vsOut;
}


@fragment fn fs_1(vsOut: VSOutput) -> @location(0) vec4f {
  return vsOut.color;
}


@vertex fn vs_2(
  @builtin(vertex_index) vertexIndex : u32,
  @builtin(instance_index) instanceIndex: u32,
) -> VSOutputLinks {
  let link = links[instanceIndex];
  let cell_a = cells[link.a];
  let cell_b = cells[link.b];
  let p1 = cell_a.ap;
  let p2 = cell_b.ap;
  let line_dir = normalize(p2 - p1);
  let perp_dir = vec2f(-line_dir.y, line_dir.x);
  let extend_up_down = perp_dir * uniforms.line_width * 0.5 * uniforms.zoom;
  let extend_left_right = line_dir * 0.0005 * uniforms.zoom * 0.0;
  var vsOut: VSOutputLinks;
  //  0───2
  //  │ / │
  //  1───3
  switch vertexIndex {
    // top left
    case 0u: { 
      vsOut.position = vec4f(
        p1*uniforms.zoom + extend_up_down - extend_left_right, 
        0.0, 1.0,
      );
      vsOut.link_t = 0.0;
    }
    // bottom left
    case 1u: { 
      vsOut.position = vec4f(
        p1*uniforms.zoom - extend_up_down - extend_left_right, 
        0.0, 1.0
      );
      vsOut.link_t = 0.0;
    }
    // top right
    case 2u: { 
      vsOut.position = vec4f(
        p2*uniforms.zoom + extend_up_down + extend_left_right, 
        0.0, 1.0
      );
      vsOut.link_t = 1.0;
    }
    // bottom right
    case 3u: { 
      vsOut.position = vec4f(
        p2*uniforms.zoom - extend_up_down + extend_left_right, 
        0.0, 1.0
      );
      vsOut.link_t = 1.0;
    }
    default: {}
  }
  return vsOut;
}


@fragment fn fs_2(vsOut: VSOutputLinks) -> @location(0) vec4f {
  let progress = f32(uniforms.tick % 1000) / 1000.0;
  let width = 0.1;
  let dist_direct = abs(vsOut.link_t - progress);
  let dist_wrap = abs(1.0 - vsOut.link_t + progress);
  let dist_start = (vsOut.link_t + 1) % 1 + (1-progress);
  let dist = min(dist_direct, min(dist_wrap, dist_start));
  let intensity = smoothstep(width, 0.0, dist);
  return vec4f(
    intensity, intensity, 0.0, intensity
  );
}
