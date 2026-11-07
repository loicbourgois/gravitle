fn fragment_debug(in: VertexOut) -> vec4f {
    return in.color;
}

fn fragment_particles(in: VertexOut) -> vec4f {
    let radius = length(in.uv);
    if (radius > 1.0) {
        discard;
    }

    if (
        (
            in.kind == KIND_PIPE
            || in.kind == KIND_TANK
            || in.kind == KIND_FILTER
            || in.kind == KIND_BLOWER
            || in.kind == KIND_PUMP
        )
        && radius < 0.75
    ) {
        let seed = vec3f(
            in.uv * 24.0 + u.canvas_size * 0.013,
            19.0,
        );
        let n = hash(floor(seed));
        let aa = select(0.0, 1.0, n < in.moon_ratio * 0.5);
        return vec4f(aa, aa, aa, 1.0);
    }

    if (in.kind == KIND_PRESS) {
        if (in.state == STATE_READY && radius < 0.75 * 0.5) {
            return vec4f(1.0);
        }

        if (
            in.state != STATE_DONE
            && radius < 0.75 * (1.0 - in.progress_ratio * 0.5)
        ) {
            let seed = vec3f(
                in.uv * 24.0 + u.canvas_size * 0.013,
                19.0,
            );
            let n = hash(floor(seed));
            let aa = select(
                0.0,
                1.0,
                n < in.moon_ratio * 0.5 + in.progress_ratio * 0.5,
            );
            return vec4f(aa, aa, aa, 1.0);
        }

        if (in.state == STATE_DONE) {
            if (radius < 0.75 * 0.5) {
                return vec4f(1.0);
            }
            if (radius < 0.75 * (1.0 - in.progress_ratio * 0.5)) {
                return vec4f(0.0);
            }
        }
    }

    return in.color;
}

fn fragment_links(in: VertexOut) -> vec4f {
    return in.color;
}

fn fragment_indicators(in: VertexOut) -> vec4f {
    let radius = length(in.uv);
    if (radius > 1.0) {
        discard;
    }
    if (radius < 0.5) {
        return vec4f(0.8, 0.8, 1.0, 1.0);
    }
    if (in.state == STATE_MOVING_AWAY) {
        return vec4f(1.0);
    }
    return vec4f(0.0);
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4f {
    switch (in.draw_kind) {
        case DRAW_DEBUG: {
            return fragment_debug(in);
        }
        case DRAW_PARTICLES: {
            return fragment_particles(in);
        }
        case DRAW_LINKS: {
            return fragment_links(in);
        }
        case DRAW_INDICATORS: {
            return fragment_indicators(in);
        }
        default: {
            return vec4f(0.0);
        }
    }
}