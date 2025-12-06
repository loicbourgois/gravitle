To address the `TODO: add aura around the line, to make it glow`, we need to change how lines are rendered. Currently, `vs_2` likely renders 1-pixel wide lines using a `line-list` or `line-strip` topology. To create a glow effect, the line must be rendered as a textured quad (two triangles) for each segment.

This solution involves the following steps:
1.  **Introduce a new `VSOutputLink` struct** for the link rendering pipeline (`vs_2` and `fs_2`). This struct will carry normalized coordinates (across the line's width and along its length) to the fragment shader.
2.  **Modify `vs_2`** to generate the vertices for a quad (two triangles) for each link instead of a simple line segment. It will also calculate and pass the `line_uv_across` and `line_uv_along` coordinates. This implies that the host-side rendering code must be updated to:
    *   Set the primitive topology for links to `triangle-strip`.
    *   Draw 4 vertices per instance for each link.
3.  **Modify `fs_2`** to use these `line_uv_across` and `line_uv_along` coordinates to compute a smooth fading alpha and color blend, creating the glow effect.

Here's the updated WGSL code:

```wgsl
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

// Existing VSOutput struct for cell and position rendering
struct VSOutput {
  @builtin(position) position: vec4f,
  @location(0) color: vec4f,
}

// New struct for link rendering, to pass additional information to the fragment shader
struct VSOutputLink {
  @builtin(position) position: vec4f,
  @location(0) line_uv_across: f32, // Normalized position across the line quad, from -1.0 (left edge) to 1.0 (right edge)
  @location(1) line_uv_along: f32,  // Normalized position along the line, from 0.0 (start point) to 1.0 (end point)
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

// TODO: add aura around the line, to make it glow
// Implementation Details:
// - This vertex shader assumes the primitive topology is 'triangle-strip' and 'vertexCount' per instance is 4.
// - It generates a quad for each link segment, perpendicular to the line direction.
// - It passes normalized coordinates (across and along the line) to the fragment shader for glow calculation.
@vertex fn vs_2(
  @builtin(vertex_index) vertexIndex : u32,
  @builtin(instance_index) instanceIndex: u32,
) -> VSOutputLink { // Now returns VSOutputLink
  let link = links[instanceIndex];
  let cell_a = cells[link.a];
  let cell_b = cells[link.b];

  let p1 = cell_a.ap;
  let p2 = cell_b.ap;
  let center = vec2f( 0.0,  0.0 ); // Keeping consistent with other shaders

  let line_vec = p2 - p1;
  let line_length = length(line_vec);

  var normalized_offset_dir = vec2f(0.0, 0.0);
  // Ensure line_length is not zero to avoid division by zero for normalization.
  // If the line is a point, the quad will collapse, effectively rendering nothing or a tiny dot.
  if (line_length > 1e-6) { // Use a small epsilon for floating point comparison
      let normalized_line_vec = line_vec / line_length;
      normalized_offset_dir = vec2f(-normalized_line_vec.y, normalized_line_vec.x); // Perpendicular vector
  }

  // Total half-width of the line quad (including the glow) in world coordinates.
  // Adjust this value to control the overall thickness and extent of the glow.
  let line_total_half_width_world = 0.008;

  var base_pos: vec2f;
  var line_uv_across: f32; // -1 for left edge, 1 for right edge
  var line_uv_along: f32;  // 0 for p1 end, 1 for p2 end

  // Map vertexIndex (0, 1, 2, 3) to quad corners for a triangle strip:
  //   0: p1_left  (uv_across=-1, uv_along=0)
  //   1: p1_right (uv_across=1,  uv_along=0)
  //   2: p2_left  (uv_across=-1, uv_along=1)
  //   3: p2_right (uv_across=1,  uv_along=1)
  switch vertexIndex {
    case 0u: { // P1, left side of the line quad
      base_pos = p1;
      line_uv_across = -1.0;
      line_uv_along = 0.0;
    }
    case 1u: { // P1, right side of the line quad
      base_pos = p1;
      line_uv_across = 1.0;
      line_uv_along = 0.0;
    }
    case 2u: { // P2, left side of the line quad
      base_pos = p2;
      line_uv_across = -1.0;
      line_uv_along = 1.0;
    }
    case 3u: { // P2, right side of the line quad
      base_pos = p2;
      line_uv_across = 1.0;
      line_uv_along = 1.0;
    }
    default: {
        // Fallback for unexpected vertexIndex, though this case should be avoided by correct drawing parameters.
        base_pos = p1;
        line_uv_across = -1.0;
        line_uv_along = 0.0;
    }
  }

  // Calculate the final world position for the current vertex of the quad
  let final_pos_world = base_pos + normalized_offset_dir * line_uv_across * line_total_half_width_world;

  var vsOut: VSOutputLink;
  vsOut.position = vec4f(
    (final_pos_world - center) * uniforms.zoom,
    0.0, 1.0
  );
  vsOut.line_uv_across = line_uv_across;
  vsOut.line_uv_along = line_uv_along;

  return vsOut;
}

@fragment fn fs_2(vsOut: VSOutputLink) -> @location(0) vec4f { // Now takes VSOutputLink
  let core_color = vec3f(1.0, 1.0, 0.0); // Bright yellow for the solid core of the line
  let glow_color = vec3f(1.0, 1.0, 0.7); // Lighter yellow/white for the outer glow

  // `dist_from_center_normalized` ranges from 0.0 (at the line's center) to 1.0 (at the quad's edge).
  let dist_from_center_normalized = abs(vsOut.line_uv_across);

  // Define how much of the line's half-width is opaque core and how much is fading glow.
  let core_radius_factor = 0.2; // e.g., 20% of the total half-width is opaque
  let glow_falloff_start_factor = 0.3; // e.g., glow starts fading transparency at 30%
  let glow_falloff_end_factor = 1.0; // Glow fully transparent at 100% (edge of quad)

  var final_alpha: f32;
  var final_rgb: vec3f;

  if (dist_from_center_normalized < core_radius_factor) {
    // Inside the solid core of the line
    final_alpha = 1.0;
    final_rgb = core_color;
  } else {
    // In the glow region, blend color and alpha
    // `smoothstep` creates a smooth transition between 0 and 1.
    // `t_alpha` will be 0 up to `glow_falloff_start_factor` and 1 at `glow_falloff_end_factor`.
    let t_alpha = smoothstep(glow_falloff_start_factor, glow_falloff_end_factor, dist_from_center_normalized);
    // `final_alpha` goes from 1.0 to 0.0 as `t_alpha` goes from 0.0 to 1.0.
    final_alpha = 1.0 - t_alpha;

    // Blend the color from `core_color` towards `glow_color` in the glow region.
    // `t_color` will be 0 up to `core_radius_factor` and 1 at `glow_falloff_end_factor`.
    let t_color = smoothstep(core_radius_factor, glow_falloff_end_factor, dist_from_center_normalized);
    final_rgb = mix(core_color, glow_color, t_color);
    
    // An optional multiplier to reduce the overall opacity of the glow, if desired.
    final_alpha *= 0.8; 
  }

  return vec4f(final_rgb, final_alpha);
}
```