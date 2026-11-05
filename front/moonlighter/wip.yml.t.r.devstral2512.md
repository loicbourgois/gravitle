# Reasoning

The TODO mentions two main issues to address:

1. Drawing each link using the WGSL shader (currently only drawing a simple pink debug line)
2. Fixing a WebGPU validation error about bind group descriptor mismatch (4 bindings vs 3 in layout)

Looking at the code:

1. In `webgpu.js`, the bind group is created with 4 entries (uniform buffer, particle buffer, color buffer, link buffer), but the shader only defines 3 bindings in group 0 (uniforms, particles, colors). This causes the validation error.

2. The shader currently only draws particles (as quads) but doesn't draw links. The TODO wants links to be drawn using the shader.

3. The `draw_triangle_webgpu` function in `webgpu.js` is already uploading link data to the GPU, but the shader isn't using it yet.

To fix this:

1. First, we need to update the shader to include the link buffer binding and implement link drawing.

2. The vertex shader needs to generate vertices for links (as lines) in addition to particles (as quads).

3. The fragment shader needs to handle link fragments appropriately.

4. We need to ensure the bind group layout in the shader matches the number of bindings we're creating in JavaScript.

Looking at the current shader:
- It has bindings 0, 1, 2 (uniforms, particles, colors)
- We need to add binding 3 for links
- The vertex shader currently handles:
  - 0-2: debug triangle
  - 3-8: debug square
  - 9+: particles (6 vertices per particle)
- We need to add link vertices after the particle vertices

For links, we'll draw them as simple lines. Each link will need 2 vertices (start and end), but to draw a line with width, we'll need to expand it to a quad (6 vertices per link, similar to particles).

The current draw call is:
```javascript
pass.draw(9 + 6 * count + 6 * link_count);
```
This suggests the shader is already expecting to process link vertices (6 per link), but the vertex shader doesn't implement this yet.

Here's the plan:

1. Update the shader to:
   - Add the link buffer binding
   - Handle link vertices in the vertex shader
   - Update the fragment shader to handle links (probably just return a color)

2. The vertex index calculation needs to account for:
   - 0-2: debug triangle (3 vertices)
   - 3-8: debug square (6 vertices)
   - 9 to 9+6*count-1: particles (6 vertices each)
   - 9+6*count to 9+6*count+6*link_count-1: links (6 vertices each)

3. For links, we'll draw them as quads (2 triangles) representing a line with width.

4. We need to pass link colors to the fragment shader. We can either:
   - Compute link colors in JS and store in a buffer (like particle colors)
   - Compute link colors in the shader based on the particles they connect

For simplicity, we'll compute link colors in the shader based on the connected particles.

# Updates

## file://./../../../gravitle/front/moonlighter/shader.wgsl

### Add link buffer binding and update vertex/fragment shaders

```wgsl
const LINE_HALF_WIDTH: f32 = 2.0;

struct Uniforms {
    canvas_size: vec2f,
    zoom: f32,
    time: f32,
    step: f32,
    rnd: f32,
    link_count: f32,
    _pad: f32,
};

// Mirrors `Particle` in engine/src/particle.rs (`#[repr(C)]`, 4-byte aligned).
// Fields and offsets must be kept in sync with the Rust struct.
struct GpuParticle {
    idx: u32,
    diameter: f32,
    p: vec2f,                 // position
    pp: vec2f,                // previous position
    np: vec2f,                // new position
    dp: vec2f,                // delta position
    direction: vec2f,
    dv: vec2f,                // delta velocity
    link_response: vec2f,
    collision_response: vec2f,
    collision_response_count: u32,
    kind: u32,
    pressure_c: i32,
    pressure_q: i32,
    pressure_qn: i32,
    moon_c: i32,
    moon_q: i32,
    moon_qd: i32,
    electricity_c: i32,
    electricity_q: i32,
    electricity_qd: i32,
    activation: i32,
    signal_q: i32,
    signal_qn: i32,
    solidmoon_c: i32,
    solidmoon_q: i32,
    solidmoon_qd: i32,
    padding: u32,
};

struct GpuLink {
    idx: u32,
    a: u32,
    b: u32,
};

@group(0) @binding(0) var<uniform> u: Uniforms;
@group(0) @binding(1) var<storage, read> particles: array<GpuParticle>;
// rgba per particle, computed in JS from engine.base_color
@group(0) @binding(2) var<storage, read> colors: array<vec4f>;
@group(0) @binding(3) var<storage, read> links: array<GpuLink>;

fn px_to_ndc(p: vec2f) -> vec2f {
    let norm = p / u.canvas_size;
    return vec2f(norm.x * 2.0 - 1.0, 1.0 - norm.y * 2.0);
}

fn quad_corners(x1: f32, y1: f32, x2: f32, y2: f32) -> array<vec2f, 6> {
    return array<vec2f, 6>(
        vec2f(x1, y1),
        vec2f(x2, y1),
        vec2f(x2, y2),
        vec2f(x1, y1),
        vec2f(x2, y2),
        vec2f(x1, y2),
    );
}

struct VertexOut {
    @builtin(position) pos: vec4f,
    @location(0) color: vec4f,
    // Local coordinates of the fragment relative to the center of the shape,
    // expressed in units of the shape's radius. Fragments with
    // `length(uv) > 1.0` are discarded for round shapes.
    @location(1) uv: vec2f,
    @location(2) moon_ratio: f32, // quantity / capacity
    @location(3) @interpolate(flat) kind: u32,
    // 1.0 for link vertices: disables the disc discard in fs_main
    @location(4) @interpolate(flat) is_link: f32,
    // For links, the color computed from the connected particles
    @location(5) link_color: vec4f,
};

fn get_link_color(link: GpuLink) -> vec4f {
    let a = particles[link.a];
    let b = particles[link.b];

    // Check if this is a pipe link
    let is_pipe = (a.kind == 1u || b.kind == 1u) &&  // Pipe
                 (a.kind == 1u || a.kind == 2u || a.kind == 3u || a.kind == 4u || a.kind == 10u || a.kind == 11u || a.kind == 19u ||
                  b.kind == 1u || b.kind == 2u || b.kind == 3u || b.kind == 4u || b.kind == 10u || b.kind == 11u || b.kind == 19u);

    // Check if this is an electricity link
    let is_electricity = (a.kind == 5u || a.kind == 6u || a.kind == 7u || a.kind == 8u || a.kind == 9u || a.kind == 10u ||
                          b.kind == 5u || b.kind == 6u || b.kind == 7u || b.kind == 8u || b.kind == 9u || b.kind == 10u);

    // Check if this is a signal link
    let is_signal = (a.kind == 12u || a.kind == 13u || a.kind == 14u || a.kind == 15u || a.kind == 16u || a.kind == 17u || a.kind == 18u ||
                     b.kind == 12u || b.kind == 13u || b.kind == 14u || b.kind == 15u || b.kind == 16u || b.kind == 17u || b.kind == 18u);

    // Check if this is a solid moon link
    let is_solid_moon = (a.kind == 19u || a.kind == 21u || a.kind == 22u ||
                         b.kind == 19u || b.kind == 21u || b.kind == 22u);

    if (is_pipe) {
        // For pipe links, use a gray color with intensity based on moon ratio
        let moon_ratio_a = f32(a.moon_q) / f32(max(a.moon_c, 1));
        let moon_ratio_b = f32(b.moon_q) / f32(max(b.moon_c, 1));
        let avg_moon_ratio = (moon_ratio_a + moon_ratio_b) * 0.5;
        let intensity = 0.3 + 0.7 * avg_moon_ratio;
        return vec4f(intensity, intensity, intensity, 1.0);
    } else if (is_electricity) {
        // For electricity links, use a yellowish color
        return vec4f(0.9, 0.8, 0.2, 1.0);
    } else if (is_signal) {
        // For signal links, use a purple color
        return vec4f(0.8, 0.2, 0.8, 1.0);
    } else if (is_solid_moon) {
        // For solid moon links, use a light gray color
        return vec4f(0.7, 0.7, 0.7, 1.0);
    } else {
        // Default to pink debug color
        return vec4f(1.0, 0.5, 0.8, 1.0);
    }
}

// Cheap hash-based value noise in [0, 1), seeded by time, step and
// a per-frame random number coming from JS.
fn hash(p: vec3f) -> f32 {
    var v = fract(sin(dot(p, vec3f(127.1, 311.7, 74.7))) * 43758.5453123);
    return v;
}

@vertex
fn vs_main(@builtin(vertex_index) i: u32) -> VertexOut {
    var out: VertexOut;
    out.uv = vec2f(0.0, 0.0);
    out.color = vec4f(1.0);
    out.is_link = 0.0;
    out.link_color = vec4f(0.0);

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
        out.color = vec4f(colors[i], 0.8);
        out.color = vec4f(0.0);
    } else if (i < 9u) {
        // Debug 100x100 px square in the top-left corner (pixel coords, y down)
        out.pos = vec4f(px_to_ndc(quad_corners(0.0, 0.0, 100.0, 100.0)[i - 3u]), 0.0, 1.0);
        out.color = vec4f(1.0, 0.8, 0.2, 0.8);
    } else {
        let particle_count = u32(arrayLength(&particles));
        let link_count = u32(arrayLength(&links));

        // Calculate how many vertices are used for particles
        let particle_vertices = 6u * particle_count;

        if (i < 9u + particle_vertices) {
            // Particle vertices
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
            out.moon_ratio = f32(particle.moon_q) / f32(max(particle.moon_c, 1));
        } else {
            // Link vertices
            let li = (i - (9u + particle_vertices)) / 6u;
            let corner_idx = (i - (9u + particle_vertices)) % 6u;
            let link = links[li];
            let link_color = get_link_color(link);

            // Get the two particles connected by this link
            let a = particles[link.a];
            let b = particles[link.b];

            let aa = min(u.canvas_size.x, u.canvas_size.y) * u.zoom;
            let pos_a = vec2f(
                u.canvas_size.x * 0.5 + a.p.x * aa,
                u.canvas_size.y * 0.5 - a.p.y * aa,
            );
            let pos_b = vec2f(
                u.canvas_size.x * 0.5 + b.p.x * aa,
                u.canvas_size.y * 0.5 - b.p.y * aa,
            );

            // Calculate the direction and length of the link
            let dir = normalize(pos_b - pos_a);
            let perpendicular = vec2f(-dir.y, dir.x);

            // Line width in pixels
            let line_width = LINE_HALF_WIDTH * 2.0;

            // Get the quad corners for this link segment
            let corners = quad_corners(-1.0, -1.0, 1.0, 1.0);
            let corner = corners[corner_idx];

            // Calculate the position along the line (from -1 to 1)
            let t = corner.x;

            // Calculate the position perpendicular to the line (from -1 to 1)
            let u = corner.y;

            // Calculate the actual position
            let center_pos = pos_a + (pos_b - pos_a) * (t + 1.0) * 0.5;
            let offset = perpendicular * u * line_width * 0.5;
            let c = center_pos + offset;

            out.pos = vec4f(px_to_ndc(c), 0.0, 1.0);
            out.color = link_color;
            out.uv = vec2f(0.0, 0.0); // Not used for links
            out.kind = 0u; // Not used for links
            out.moon_ratio = 0.0; // Not used for links
            out.is_link = 1.0;
            out.link_color = link_color;
        }
    }
    return out;
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4f {
    // For links, just return the link color
    if (in.is_link > 0.5) {
        return in.link_color;
    }

    // For particles, apply the disc discard
    if (length(in.uv) > 1.0) {
        discard;
    }

    // Special handling for certain particle types to show fill level
    if (
        (
            in.kind == 1u  // Pipe
            || in.kind == 2u  // Tank
            || in.kind == 3u  // Filter
            || in.kind == 10u // Blower
            || in.kind == 4u  // Pump
            || in.kind == 19u // Press
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
}
```

The changes made to the shader include:

1. Added the link buffer binding (`@group(0) @binding(3) var<storage, read> links: array<GpuLink>`)
2. Added a `get_link_color` function that determines the color of a link based on the types of particles it connects
3. Updated the `VertexOut` struct to include a `link_color` field
4. Modified the vertex shader to:
   - Calculate the number of particle and link vertices
   - Handle link vertices after particle vertices
   - For links, create a quad representing a line with width between the two connected particles
   - Set the `is_link` flag and `link_color` for link vertices
5. Modified the fragment shader to:
   - Return the link color directly for link fragments
   - Keep the existing particle rendering logic for particle fragments

This should now properly draw all links using the WGSL shader with appropriate colors based on the link type, while also fixing the bind group validation error by ensuring the shader has all 4 bindings that are being used in the JavaScript code.