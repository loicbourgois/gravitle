I'll address the TODO comment in the code to add an aura/glow effect around the lines in the `vs_2`/`fs_2` shader pair. Here's the implementation:

```wgsl
// Glowing links shader
@vertex fn vs_2(
  @builtin(vertex_index) vertexIndex : u32,
  @builtin(instance_index) instanceIndex: u32,
) -> VSOutput {
  let link = links[instanceIndex];
  let cell_a = cells[link.a];
  let cell_b = cells[link.b];
  var position_to_render: vec2f;
  if (vertexIndex == 0u) {
    // First vertex of the line segment uses cell_a's average position
    position_to_render = cell_a.ap;
  } else { // vertexIndex == 1u
    // Second vertex of the line segment uses cell_b's average position
    position_to_render = cell_b.ap;
  }
  let center = vec2f(0.0, 0.0);
  var vsOut: VSOutput;
  vsOut.position = vec4f(
    (position_to_render - center) * uniforms.zoom,
    0.0, 1.0
  );

  // Store the line thickness information in the alpha channel
  // We'll use this in the fragment shader to create the glow effect
  let line_thickness = 0.005; // Base thickness
  let glow_intensity = 0.02; // Glow size
  vsOut.color = vec4f(
    1.0, 1.0, 0.0, // Yellow color
    line_thickness + glow_intensity // Store thickness info in alpha
  );
  return vsOut;
}

@fragment fn fs_2(vsOut: VSOutput) -> @location(0) vec4f {
  // Calculate distance from center of line
  let coord = vec2f(vsOut.position.x, vsOut.position.y);
  let distance = length(coord);

  // Extract thickness information from alpha channel
  let line_thickness = 0.005;
  let glow_intensity = vsOut.color.a - line_thickness;

  // Create glow effect - brighter near the line, fading out
  let glow = exp(-distance * distance / (2.0 * glow_intensity * glow_intensity));
  let glow_color = vec3f(1.0, 1.0, 0.0) * glow;

  // Create solid line
  let line = smoothstep(line_thickness * 0.5, line_thickness * 0.4, distance);
  let line_color = vec3f(1.0, 1.0, 0.0) * line;

  // Combine effects
  let final_color = glow_color + line_color;

  return vec4f(final_color, 1.0);
}
```

This implementation:

1. Modifies the vertex shader to pass thickness information through the alpha channel
2. Creates a fragment shader that:
   - Calculates distance from the line center
   - Creates a glow effect that fades out with distance
   - Adds a solid line in the center
   - Combines both effects for a nice glowing line

You can adjust these parameters to get the desired effect:
- `line_thickness` in the vertex shader controls the width of the solid line
- `glow_intensity` controls how far the glow extends
- The `smoothstep` values in the fragment shader control the line's edge softness

The result will be a yellow line with a glowing aura around it, making it more visible and visually appealing.