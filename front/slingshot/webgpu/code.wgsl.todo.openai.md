## Reasoning

The `fs_2` fragment shader controls the visual appearance of links connecting cells. A `TODO` comment indicated that when `link_t == 0`, the "other end" (where `link_t == 1`) should also light up, giving the impression that the illumination travels around in a loop.

Currently, the shader computes intensity based on the absolute difference between `link_t` (ranging from 0 to 1 along the link) and a progress parameter (which moves from 0 to 1 over time). However, when progress approaches 1, `link_t` near 0 will appear dark — the light doesn’t wrap around properly. 

The intuitive fix is to make the distance wrap in a circular manner, by taking the minimum of the direct distance and the wrapped-around distance (i.e., `min(|t - p|, 1 - |t - p|)`).

That way, when the progress nears the end of the range, both ends of the link light up smoothly, preserving continuity as if the light loops around.

## Updates

### `~/github.com/loicbourgois/gravitle/front/slingshot/webgpu/code.wgsl`

#### Fixed link glow wrapping behavior
```wgsl
@fragment fn fs_2(vsOut: VSOutputLinks) -> @location(0) vec4f {
  // Makes the lighting loop around the link for continuity
  let progress = f32(uniforms.tick % 1000) / 1000.0;
  let width = 0.15;
  let base_dist = abs(vsOut.link_t - progress);
  let dist = min(base_dist, 1.0 - base_dist); // wrap-around correction
  let intensity = smoothstep(width, 0.0, dist);
  return vec4f(
    intensity, intensity, 0.0, 1.0
  );
}
```