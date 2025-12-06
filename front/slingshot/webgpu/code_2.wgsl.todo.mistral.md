I'll address the TODO comment in the code to make the links thicker. Currently, the `vs_2` shader renders links as simple 1-pixel wide lines. To make them thicker, we need to implement a technique that can render wider lines.

Here's the improved implementation for the link shader:

```wgsl
// Improved link shader with thickness
@vertex fn vs_2(
  @builtin(vertex_index) vertexIndex : u32,
  @builtin(instance_index) instanceIndex: u32,
) -> VSOutput {
  let link = links[instanceIndex];
  let cell_a = cells[link.a];
  let cell_b = cells[link.b];

  // Calculate the direction vector of the link
  let link_dir = cell_b.ap - cell_a.ap;
  // Calculate a perpendicular vector for offsetting
  let perp_dir = normalize(vec2f(-link_dir.y, link_dir.x)) * 0.005; // Adjust 0.005 for thickness

  var position_to_render: vec2f;
  let center = vec2f(0.0, 0.0);

  // Create a quad for the link by offsetting perpendicular to the link direction
  switch(vertexIndex) {
    case 0u: { // Bottom-left vertex
      position_to_render = cell_a.ap - perp_dir;
    }
    case 1u: { // Top-left vertex
      position_to_render = cell_a.ap + perp_dir;
    }
    case 2u: { // Bottom-right vertex
      position_to_render = cell_b.ap - perp_dir;
    }
    default: { // Top-right vertex (case 3u)
      position_to_render = cell_b.ap + perp_dir;
    }
  }

  var vsOut: VSOutput;
  vsOut.position = vec4f(
    (position_to_render - center) * uniforms.zoom,
    0.0, 1.0
  );
  vsOut.color = vec4f(
    1.0, 1.0, 0.0, 1.0 // Yellow color for links
  );
  return vsOut;
}

@fragment fn fs_2(vsOut: VSOutput) -> @location(0) vec4f {
  return vsOut.color;
}
```

This implementation:

1. Calculates the direction vector between the two linked cells
2. Creates a perpendicular vector to that direction
3. Uses the vertex index to position four vertices that form a quad (rectangle) around the link
4. The thickness can be adjusted by changing the multiplier (currently 0.005)

Note that you'll also need to update the render pipeline to use a triangle strip topology instead of a line list for this shader to work properly. The vertex count per instance should be 4 instead of 2.

The value 0.005 is a starting point for thickness - you may need to adjust this based on your specific needs and coordinate system scale. If you want the thickness to scale with zoom, you could divide it by `uniforms.zoom`.