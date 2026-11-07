fn vertex_debug(i: u32) -> VertexOut {
    var out: VertexOut;

    if (i < 3u) {
        let positions = array<vec2f, 3>(
            vec2f(0.0, 0.6),
            vec2f(-0.6, -0.6),
            vec2f(0.6, -0.6),
        );
        out.pos = vec4f(positions[i], 0.0, 1.0);
        out.color = vec4f(0.0);
    } else {
        let corner = quad_corners(0.0, 0.0, 100.0, 100.0)[i - 3u];
        out.pos = vec4f(px_to_ndc(corner), 0.0, 1.0);
        out.color = vec4f(1.0, 0.8, 0.2, 0.8);
    }

    return out;
}

fn vertex_particles(i: u32) -> VertexOut {
    var out: VertexOut;
    let pi = i / 6u;
    let corner = quad_corners(-1.0, -1.0, 1.0, 1.0)[i % 6u];
    let particle = particles[pi];
    let scale = min(u.canvas_size.x, u.canvas_size.y) * u.zoom;
    let center = vec2f(
        u.canvas_size.x * 0.5 + particle.p.x * scale,
        u.canvas_size.y * 0.5 - particle.p.y * scale,
    );
    let radius = particle.diameter * 0.5 * scale;

    out.pos = vec4f(px_to_ndc(center + corner * radius), 0.0, 1.0);
    out.color = colors[pi];
    out.uv = corner;
    out.moon_ratio = f32(particle.moon_q) / f32(max(particle.moon_c, 1));
    out.progress_ratio =
        f32(particle.progress_q) / f32(max(particle.progress_c, 1));
    out.kind = particle.kind;
    out.state = particle.state;
    out.solidmoon_q = particle.solidmoon_q;

    return out;
}

fn vertex_links(i: u32) -> VertexOut {
    var out: VertexOut;
    let link = links[i / 6u];
    let corner = quad_corners(-1.0, -1.0, 1.0, 1.0)[i % 6u];
    let pa = particles[link.a];
    let pb = particles[link.b];
    let scale = min(u.canvas_size.x, u.canvas_size.y) * u.zoom;
    let ca = vec2f(
        u.canvas_size.x * 0.5 + pa.p.x * scale,
        u.canvas_size.y * 0.5 - pa.p.y * scale,
    );
    let cb = vec2f(
        u.canvas_size.x * 0.5 + pb.p.x * scale,
        u.canvas_size.y * 0.5 - pb.p.y * scale,
    );

    let delta = cb - ca;
    var direction = vec2f(1.0, 0.0);
    if (dot(delta, delta) > 0.0) {
        direction = normalize(delta);
    }

    let perpendicular = vec2f(-direction.y, direction.x);
    let t = corner.x * 0.5 + 0.5;
    let position = mix(ca, cb, t)
        + perpendicular * (corner.y * LINE_HALF_WIDTH);

    out.pos = vec4f(px_to_ndc(position), 0.0, 1.0);
    out.color = mix(colors[link.a], colors[link.b], t);

    return out;
}

fn vertex_indicators(i: u32) -> VertexOut {
    var out: VertexOut;
    // Collapse unused indicator quads outside the viewport.
    out.pos = vec4f(2.0, 2.0, 0.0, 1.0);

    let pi = i / 6u;
    let particle_count = u32(u.particle_count);
    if (pi >= particle_count) {
        return out;
    }

    let particle = particles[pi];
    if (particle.kind != KIND_CONVEYOR
        || particle.input >= particle_count
        || particle.output >= particle_count) {
        return out;
    }

    let progress = clamp(
        f32(particle.progress_q) / f32(max(particle.progress_c, 1)),
        0.0,
        1.0,
    );
    let position = mix(
        particles[particle.input].p,
        particles[particle.output].p,
        progress,
    );
    let scale = min(u.canvas_size.x, u.canvas_size.y) * u.zoom;
    let center = vec2f(
        u.canvas_size.x * 0.5 + position.x * scale,
        u.canvas_size.y * 0.5 - position.y * scale,
    );
    let corner = quad_corners(-1.0, -1.0, 1.0, 1.0)[i % 6u];
    let radius = particle.diameter * 0.18 * scale;

    out.pos = vec4f(px_to_ndc(center + corner * radius), 0.0, 1.0);
    out.uv = corner;
    out.kind = particle.kind;
    out.state = particle.state;
    out.solidmoon_q = particle.solidmoon_q;

    return out;
}

@vertex
fn vs_main(
    @builtin(vertex_index) i: u32,
    @builtin(instance_index) draw_kind: u32,
) -> VertexOut {
    var out: VertexOut;

    switch (draw_kind) {
        case DRAW_DEBUG: {
            out = vertex_debug(i);
        }
        case DRAW_PARTICLES: {
            out = vertex_particles(i);
        }
        case DRAW_LINKS: {
            out = vertex_links(i);
        }
        case DRAW_INDICATORS: {
            out = vertex_indicators(i);
        }
        default: {
            out.pos = vec4f(2.0, 2.0, 0.0, 1.0);
        }
    }

    out.draw_kind = draw_kind;
    return out;
}