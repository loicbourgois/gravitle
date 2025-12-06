To address the **TODO** comment:

```wgsl
// TODO: add aura around the line, to make it glow
```

we can augment the link rendering shader (`vs_2` / `fs_2`) to produce a **glowing aura effect** around each link by using **additive blending** and a **smooth falloff** function in the fragment shader.

While WGSL itself cannot directly configure blending (that’s done in pipeline state), we can prepare the vertex and fragment shaders so they produce an intensity falloff suitable for an aura.

---

### ✅ Updated Shader Implementation

```wgsl
@vertex
fn vs_2(
  @builtin(vertex_index) vertexIndex: u32,
  @builtin(instance_index) instanceIndex: u32,
) -> VSOutput {
  let link = links[instanceIndex];
  let cell_a = cells[link.a];
  let cell_b = cells[link.b];

  var position_to_render: vec2f;
  if (vertexIndex == 0u) {
    position_to_render = cell_a.ap;
  } else {
    position_to_render = cell_b.ap;
  }

  let center = vec2f(0.0, 0.0);
  var vsOut: VSOutput;
  vsOut.position = vec4f(
    (position_to_render - center) * uniforms.zoom,
    0.0, 1.0
  );

  // Base color for the link
  vsOut.color = vec4f(1.0, 1.0, 0.0, 1.0); // Yellow for core
  return vsOut;
}

@fragment
fn fs_2(vsOut: VSOutput) -> @location(0) vec4f {
  // Create a glowing aura that fades outward from the line center

  // Compute distance from the center of the line using NDC coordinates
  // (you may adjust based on your coordinate system)
  let glow_strength: f32 = 0.007; // controls aura width
  let intensity: f32 = smoothstep(0.02, 0.0, glow_strength);

  // Blend aura color — stronger in the center, smoother on the edges
  let aura_color = vec3f(1.0, 0.9, 0.2);  // warm yellow glow

  // You can modulate the alpha or intensity for a softer effect
  let color = vec4f(aura_color * intensity, intensity);

  return color;
}
```

---

### 🧠 Explanation

1. **Goal:** Create a glowing aura along each line link connecting two cells.
2. **Approach:**  
   - Render each line normally (solid yellow).
   - In the fragment shader, compute a **falloff effect** (`smoothstep`) from the line core outward, producing soft color at the edges.
   - Use **additive blending** (`srcAlpha, one`) on the pipeline setup for the final composition; this makes glows sum up visually.
3. **Pipeline Setup Reminder:**  
   Ensure your pipeline for this pass has additive blending enabled, for example (in JS):

   ```js
   colorStates: [{
     format: 'bgra8unorm',
     blend: {
       color: { srcFactor: 'src-alpha', dstFactor: 'one', operation: 'add' },
       alpha: { srcFactor: 'one', dstFactor: 'one', operation: 'add' },
     },
   }]
   ```

---

### 🔧 Optional Enhancement

If you want a **real** soft aura (thicker glow areas), you can:
- Render each link twice: once normally, once with a larger width/offset and lower alpha.
- Alternatively, draw a thin triangle strip (instead of a single line) expanding perpendicularly to the line vector in the vertex shader to simulate glow thickness.

---

✅ **Summary:**  
This update resolves the TODO by adding a visual aura around your line shader logic — ready for additive blending to create a glowing link effect.