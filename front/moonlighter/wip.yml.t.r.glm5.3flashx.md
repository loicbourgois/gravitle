# Reasoning

The WebGPU validation error comes from `shader.wgsl`: the bind group layout uses `layout: 'auto'`, and WGSL discards **unused** module-scope resources from the generated layout. The `links` storage buffer (`@binding(3)`) was declared but never referenced in the shader code, so the auto layout only contained 3 bindings, while `webgpu.js` created the bind group with 4 entries → "Number of bindings in bind group descriptor (4) does not match the number of bindings defined in the bind group layout (3)".

The TODO also asks to draw each link using the WGSL shader, as a simple pink debug line for now. So the fix is to actually **use** the `links` buffer in `vs_main`: extend the draw range (`9 + 6*count + 6*link_count` is already passed from JS) with one thin quad per link, positioned along the segment between the two linked particles, colored pink. A `is_link` flag (already declared in `VertexOut` but unused) is now set so the fragment shader doesn't discard link fragments with the disc-shape discard. Link quads get `kind = 0u` and `moon_ratio = 0.0` so they never hit the "pipe fill" branch.

`webgpu.js` needs no structural change — the upload and draw-call count already account for links; I just cleaned up the leftover commented-out guard.

# Updates

## front/moonlighter/shader.wgsl

### Use the links buffer: draw one pink debug line (quad) per link — this also fixes the bind group mismatch

Replace the whole `vs_main` and `fs_main` with the following. Removed: the old `else` branch that only handled particles, and the fragment shader discarding everything with `length(uv) > 1.0` (which would have culled link quads).

```wgsl
@vertex
fn vs_main(@builtin(vertex_index) i: u32) -> VertexOut {
    var out: VertexOut;
    out.uv = vec2f(0.0, 0.0);
    out.color = vec4f(1.0);
    out.moon_ratio = 0.0;
    out.kind = 0u;
    out.is_link = 0.0;
    if (i < 3u) {
        // Debug triangle
        var positions = array<vec2f, 3>(
            vec2f( 0.0,  0.6),
            vec2f(-0.6, -0.6),
            vec2f( 0.6, -0.6),
        );
        var colors = array<vec3f, 3>(
            vec3f(1.0, 0.2, 0.2),
            vec3f(0.2, 1.0, 0.2),
            vec3f(0.2, 0.2, 1.0),
        );
        out.pos = vec4f(positions[i], 0.0, 1.0);
        out.color = vec4f(0.0);
    } else if (i < 9u) {
        // Debug 100x100 px square in the top-left corner (pixel coords, y down)
        out.pos = vec4f(px_to_ndc(quad_corners(0.0, 0.0, 100.0, 100.0)[i - 3u]), 0.0, 1.0);
        out.color = vec4f(1.0, 0.8, 0.2, 0.8);
    } else if (i < 9u + 6u * arrayLength(&particles)) {
        // One quad (drawn as a disc) per particle
        let pi = (i - 9u) / 6u;
        let corner = quad_corners(-1.0, -1.0, 1.0, 1.0)[(i - 9u) % 6u];
        let particle = particles[pi];
        let aa = min(u.canvas_size.x, u.canvas_size.y) * u.zoom;
        let center = vec2f(
            u.canvas_size.x * 0.5 + particle.p.x * aa,
            u.canvas_size.y * 0.5 - particle.p.y * aa,
        );
        let radius = particle.diameter * 0.5 * aa;
        let c = center + corner * radius;
        out.pos = vec4f(px_to_ndc(c), 0.0, 1.0);
        out.color = colors[pi];
        out.uv = corner;
        out.kind = particle.kind;
        out.moon_ratio = f32(particle.moon_q) / f32(particle.moon_c);
    } else {
        // One thin quad per link: simple pink debug line, drawn in pixel space.
        let base = 9u + 6u * arrayLength(&particles);
        let li = (i - base) / 6u;
        let corner = quad_corners(-1.0, -1.0, 1.0, 1.0)[(i - base) % 6u];
        let link = links[li];
        let pa = particles[link.a];
        let pb = particles[link.b];
        let aa = min(u.canvas_size.x, u.canvas_size.y) * u.zoom;
        let ca = vec2f(
            u.canvas_size.x * 0.5 + pa.p.x * aa,
            u.canvas_size.y * 0.5 - pa.p.y * aa,
        );
        let cb = vec2f(
            u.canvas_size.x * 0.5 + pb.p.x * aa,
            u.canvas_size.y * 0.5 - pb.p.y * aa,
        );
        let dir = select(vec2f(1.0, 0.0), normalize(cb - ca), cb != ca);
        let perp = vec2f(-dir.y, dir.x);
        let t = corner.x * 0.5 + 0.5; // 0 at a, 1 at b
        let c = mix(ca, cb, vec2f(t)) + perp * (corner.y * LINE_HALF_WIDTH);
        out.pos = vec4f(px_to_ndc(c), 0.0, 1.0);
        out.color = vec4f(1.0, 0.2, 0.8, 1.0); // debug pink
        out.is_link = 1.0;
    }
    return out;
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4f {
    if (in.is_link < 0.5 && length(in.uv) > 1.0) {
        discard;
    }
    if (
        in.is_link < 0.5
        && (
            in.kind == 1
            || in.kind == 2
            || in.kind == 3
            || in.kind == 10
            || in.kind == 4
            || in.kind == 19
        )
        && length(in.uv) < 0.75
    ) {
        var seed = vec3f(
            in.uv * 24.0 + u.canvas_size * 0.013,
            19.0,
        );
        let n = hash(floor(seed));
        let aa = select(0.0, 1.0, n < in.moon_ratio * 0.5);
        return vec4f(aa, aa, aa, 1.0);
    } else {
        return in.color;
    }
};
```

## front/moonlighter/webgpu.js

### Clean up the link upload guard

The commented-out `if (link_count > 0)` around the link upload is replaced by an unconditional upload; the buffer is always allocated (minimum 1 link) in `ensure_storage`, and `arrayLength(&links)` of 0 is valid in WGSL. Remove:

```javascript
    // if (link_count > 0) {
        const links_data = new Uint32Array(memory.buffer, world.links(), link_count * LINK_STRIDE);
        device.queue.writeBuffer(gpu_pipeline.link_buffer, 0, links_data);
    // }
```

and replace with:

```javascript
    const links_data = new Uint32Array(memory.buffer, world.links(), link_count * LINK_STRIDE);
    device.queue.writeBuffer(gpu_pipeline.link_buffer, 0, links_data);
```

No change needed to the bind group or the draw call: `pass.draw(9 + 6 * count + 6 * link_count)` already covers the new link quads, and since the shader now *reads* `links`, the auto bind group layout will contain all 4 bindings, resolving the validation error.