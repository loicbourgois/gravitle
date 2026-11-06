@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4f {
    if (in.is_link == 1) {
        return in.color;
    }
    else if ( length(in.uv) > 1.0) {
        discard;
    }
    else if (
        (
            in.kind == 1u
            || in.kind == 2u
            || in.kind == 3u
            || in.kind == 10u
            || in.kind == 4u
        )
        && length(in.uv) < 0.75
    ) {
        let seed = vec3f(
            in.uv * 24.0 + u.canvas_size * 0.013,
            19.0,
        );
        let n = hash(floor(seed));
        var aa = select(0.0, 1.0, n < in.moon_ratio * 0.5);
        return vec4f(aa, aa, aa, 1.0);
    }
    else if (
        in.kind == 19u
        && in.state != 4
        && length(in.uv) < 0.75 * (1.0 - in.progress_ratio * 0.5)
    ) {
        let seed = vec3f(
            in.uv * 24.0 + u.canvas_size * 0.013,
            19.0,
        );
        let n = hash(floor(seed));
        var aa = select(0.0, 1.0, n < in.moon_ratio * 0.5 + in.progress_ratio * 0.5);
        return vec4f(aa, aa, aa, 1.0);
    }
    else if (
        in.kind == 19u
        && in.state == 4
        && length(in.uv) < 0.75 * 0.5
    ) {
        return vec4f(1.0, 1.0, 1.0, 1.0);
    }
    else if (
        in.kind == 19u
        && in.state == 4
        && length(in.uv) < 0.75 * (1.0 - in.progress_ratio * 0.5)
    ) {
        return vec4f(0.0, 0.0, 0.0, 0.0);
    }
    else {
        return in.color;
    }
}