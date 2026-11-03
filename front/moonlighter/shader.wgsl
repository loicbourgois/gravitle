struct Uniforms {
    canvas_size: vec2f,
};
@group(0) @binding(0) var<uniform> u: Uniforms;

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
    @location(0) color: vec3f,
    // Local coordinates of the fragment relative to the center of the shape,
    // expressed in units of the shape's radius:
    //   - `(0, 0)` for the triangle and squares (no discard applies),
    //   - `(corner - circle_center) / radius` for the circle, so that
    //     fragments with `length(uv) > 1.0` (outside the circle) are discarded
    //     in the fragment shader.
    @location(1) uv: vec2f,
};

@vertex
fn vs_main(@builtin(vertex_index) i: u32) -> VertexOut {
    var out: VertexOut;
    out.uv = vec2f(0.0, 0.0);
    if (i < 3u) {
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
        out.color = colors[i];
    } else if (i < 9u) {
        // 100x100 px square in the top-left corner (pixel coords, y down)
        out.pos = vec4f(px_to_ndc(quad_corners(0.0, 0.0, 100.0, 100.0)[i - 3u]), 0.0, 1.0);
        out.color = vec3f(1.0, 0.8, 0.2);
    } else if (i < 15u) {
        // Square centered on screen, side = 0.1 * min(canvas_width, canvas_height)
        let side = 0.05 * min(u.canvas_size.x, u.canvas_size.y);
        let center = u.canvas_size * 0.5;
        let half = side * 0.5;
        out.pos = vec4f(px_to_ndc(quad_corners(
            center.x - half, center.y - half,
            center.x + half, center.y + half,
        )[i - 9u]), 0.0, 1.0);
        out.color = vec3f(0.2, 0.8, 1.0);
    } else {
        let x = 0.5;
        let y = 0.5;
        let m = min(u.canvas_size.x, u.canvas_size.y);
        let circle_center = u.canvas_size * 0.5 + vec2f(0.5 * m * x, -0.5 * m * y);
        let radius = 0.05 * m;
        let half = radius;
        let corners = quad_corners(
            circle_center.x - half, circle_center.y - half,
            circle_center.x + half, circle_center.y + half,
        );
        let c = corners[i - 15u];
        out.pos = vec4f(px_to_ndc(c), 0.0, 1.0);
        out.color = vec3f(1.0, 0.2, 1.0);
        out.uv = (c - circle_center) / radius;
    }
    return out;
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4f {
    if (length(in.uv) > 1.0) {
        discard;
    }
    return vec4f(in.color, 0.8);
};