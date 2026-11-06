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
    progress_c: i32,
    progress_q: i32,
    progress_qd: i32,
    state: u32,
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
    @location(4) @interpolate(flat) is_link: u32,
    @location(5) progress_ratio: f32,
    @location(6) @interpolate(flat) state: u32,
};


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

// Cheap hash-based value noise in [0, 1), seeded by time, step and
// a per-frame random number coming from JS.
fn hash(p: vec3f) -> f32 {
    let v = fract(sin(dot(p, vec3f(127.1, 311.7, 74.7))) * 43758.5453123);
    return v;
}
