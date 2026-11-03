struct Uniforms {
    canvas_size: vec2f,
};

// TODO setup the particle with the same shape as the rust particle 
// makes it easy to follow the code 
// makes it easy to use the data directly

struct GpuParticle {
    // x, y (world coords), radius in pixels, unused
    pos_radius: vec4f,
    // rgba, premultiplied not required (alpha mode handles it)
    color: vec4f,
};

@group(0) @binding(0) var<uniform> u: Uniforms;
@group(0) @binding(1) var<storage, read> particles: array<GpuParticle>;

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
};

@vertex
fn vs_main(@builtin(vertex_index) i: u32) -> VertexOut {
    var out: VertexOut;
    out.uv = vec2f(0.0, 0.0);
    out.color = vec4f(1.0);
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
    } else if (i < 9u) {
        // Debug 100x100 px square in the top-left corner (pixel coords, y down)
        out.pos = vec4f(px_to_ndc(quad_corners(0.0, 0.0, 100.0, 100.0)[i - 3u]), 0.0, 1.0);
        out.color = vec4f(1.0, 0.8, 0.2, 0.8);
    } else {
        // One quad (drawn as a disc) per particle
        let pi = (i - 9u) / 6u;
        let corner = quad_corners(-1.0, -1.0, 1.0, 1.0)[(i - 9u) % 6u];
        let p = particles[pi];
        let aa = min(u.canvas_size.x, u.canvas_size.y);
        let center = vec2f(
            u.canvas_size.x * 0.5 + p.pos_radius.x * aa * 0.1,
            u.canvas_size.y * 0.5 - p.pos_radius.y * aa * 0.1,
        );
        let radius = p.pos_radius.z;
        let c = center + corner * radius;
        out.pos = vec4f(px_to_ndc(c), 0.0, 1.0);
        out.color = p.color;
        out.uv = corner;
    }
    return out;
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4f {
    if (length(in.uv) > 1.0) {
        discard;
    }
    return in.color;
};
