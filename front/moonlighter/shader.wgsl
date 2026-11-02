struct Uniforms {
    canvas_size: vec2f,
};
@group(0) @binding(0) var<uniform> u: Uniforms;

fn px_to_ndc(p: vec2f) -> vec2f {
    let norm = p / u.canvas_size;
    return vec2f(norm.x * 2.0 - 1.0, 1.0 - norm.y * 2.0);
}

struct VertexOut {
    @builtin(position) pos: vec4f,
    @location(0) color: vec3f,
};

@vertex
fn vs_main(@builtin(vertex_index) i: u32) -> VertexOut {
    var out: VertexOut;
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
        let x1 = 0.0;
        let y1 = 0.0;
        let x2 = 100.0;
        let y2 = 100.0;
        var corners = array<vec2f, 6>(
            vec2f(x1, y1),
            vec2f(x2, y1),
            vec2f(x2, y2),
            vec2f(x1, y1),
            vec2f(x2, y2),
            vec2f(x1, y2),
        );
        out.pos = vec4f(px_to_ndc(corners[i - 3u]), 0.0, 1.0);
        out.color = vec3f(1.0, 0.8, 0.2);
    } else {
        // Square centered on screen, side = 0.1 * min(canvas_width, canvas_height)
        let side = 0.05 * min(u.canvas_size.x, u.canvas_size.y);
        let center = u.canvas_size * 0.5;
        let half = side * 0.5;
        let x1 = center.x - half;
        let y1 = center.y - half;
        let x2 = center.x + half;
        let y2 = center.y + half;
        var corners = array<vec2f, 6>(
            vec2f(x1, y1),
            vec2f(x2, y1),
            vec2f(x2, y2),
            vec2f(x1, y1),
            vec2f(x2, y2),
            vec2f(x1, y2),
        );
        out.pos = vec4f(px_to_ndc(corners[i - 9u]), 0.0, 1.0);
        out.color = vec3f(0.2, 0.8, 1.0);
    }
    return out;
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4f {
    return vec4f(in.color, 0.8);
};