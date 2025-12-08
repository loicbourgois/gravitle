## Reasoning
The `TODO` comment in the `fs_2` fragment shader indicates that the "light" effect on the links should wrap around, meaning if the `progress` is near 0, the `link_t` values near 1 should also be illuminated, and vice-versa. The current distance calculation `abs( (vsOut.link_t - progress) )` only considers linear distance, not cyclical distance.

To address this, the distance calculation needs to be updated to account for the shortest path on a circle (or a normalized segment from 0 to 1 that wraps around). If `linear_dist` is the absolute difference between `vsOut.link_t` and `progress`, then the wrap-around distance is `min(linear_dist, 1.0 - linear_dist)`. This ensures that points near the start/end of the link (0 and 1) are considered close if the `progress` is also near those extremes.

The `TODO` comment will be removed after implementing this change.

## Updates

### `~/github.com/loicbourgois/gravitle/front/slingshot/webgpu/code.wgsl`

#### Update `fs_2` fragment shader to implement wrap-around light effect

```wgsl
@fragment fn fs_2(vsOut: VSOutputLinks) -> @location(0) vec4f {
  let progress = f32(uniforms.tick % 1000) / 1000.0;
  let width = 0.15;
  let linear_dist = abs(vsOut.link_t - progress);
  let dist = min(linear_dist, 1.0 - linear_dist);
  let intensity = smoothstep(width, 0.0, dist);
  return vec4f(
    intensity, intensity, 0.0, 1.0
  );
}
```