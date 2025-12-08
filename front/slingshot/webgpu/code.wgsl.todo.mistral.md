## Reasoning

The TODO in the code is located in the fragment shader `fs_2`, which is responsible for rendering links between cells. The comment indicates that when `link_t == 0` (the start of the link), the other end should light up as well, creating a continuous lighting effect along the link.

Currently, the shader only applies a lighting effect based on the distance between the current `link_t` position and a progress value that moves along the link. This creates a unidirectional light effect from start to end. To make the light appear to "go around" or create a bidirectional effect, we should modify the distance calculation to handle both ends of the link symmetrically.

The solution involves:
1. Calculating the distance from the current `link_t` to the progress value, but also considering the "wrap-around" distance (i.e., the distance from the other end of the link).
2. Using the minimum of these distances to determine the lighting intensity, which will make both ends light up when the progress is near either end.

## Updates

### ~/github.com/loicbourgois/gravitle/front/slingshot/webgpu/code.wgsl

#### Change to `fs_2` fragment shader
```wgsl
@fragment fn fs_2(vsOut: VSOutputLinks) -> @location(0) vec4f {
  let progress = f32(uniforms.tick % 1000) / 1000.0;
  let width = 0.15;

  // Calculate direct distance
  let dist_direct = abs(vsOut.link_t - progress);
  // Calculate wrap-around distance (distance from the other end)
  let dist_wrap = abs(1.0 - vsOut.link_t - progress);

  // Use the minimum distance to create a bidirectional effect
  let dist = min(dist_direct, dist_wrap);

  let intensity = smoothstep(width, 0.0, dist);
  return vec4f(
    intensity, intensity, 0.0, 1.0
  );
}
```

This change ensures that the lighting effect appears at both ends of the link when the progress is near either end, creating a more continuous and visually appealing effect. The `min` function is used to select the closest distance (either direct or wrap-around), which will make the light appear to "go around" the link.