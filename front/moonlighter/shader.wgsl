const LINE_HALF_WIDTH: f32 = 20.0;


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
};


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
