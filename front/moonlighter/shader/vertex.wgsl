@vertex
fn vs_main(@builtin(vertex_index) i: u32) -> VertexOut {
    var out: VertexOut;
    out.uv = vec2f(0.0, 0.0);
    out.color = vec4f(1.0);
    out.moon_ratio = 0.0;
    out.kind = 0u;
    out.is_link = 0;
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
        out.state = particle.state;
        out.moon_ratio = f32(particle.moon_q) / f32(particle.moon_c);
        out.progress_ratio = f32(particle.progress_q) / f32(particle.progress_c);
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
        out.color = mix(colors[link.a], colors[link.b], t);
        out.is_link = 1;
    }
    return out;
}