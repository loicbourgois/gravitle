// include file://./cell.wgsl
// include file://./link.wgsl
// include file://./uniforms.wgsl


struct VSOutput {
  @builtin(position) position: vec4f,
  @location(0) link_t: f32,
}


@group(0) @binding(0) var<storage, read> cells: array<Cell>;
@group(0) @binding(3) var<uniform> uniforms: Uniforms;
@group(0) @binding(4) var<storage, read> links: array<Link>;


@vertex fn vs(
  @builtin(vertex_index) vertexIndex : u32,
  @builtin(instance_index) instanceIndex: u32,
) -> VSOutput {
  let link = links[instanceIndex];
  let cell_a = cells[link.a];
  let cell_b = cells[link.b];
  let p1 = cell_a.ap;
  let p2 = cell_b.ap;
  let line_dir = normalize(p2 - p1);
  let perp_dir = vec2f(-line_dir.y, line_dir.x);
  let extend_up_down = perp_dir * uniforms.line_width * 0.5 * uniforms.zoom;
  let extend_left_right = line_dir * 0.0005 * uniforms.zoom * 0.0;
  var vsOut: VSOutput;
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


@fragment fn fs(vsOut: VSOutput) -> @location(0) vec4f {
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
