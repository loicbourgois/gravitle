import { ZOOM } from "./consts.js";


// Rust `Particle` is #[repr(C)] with 4-byte-aligned fields; the WGSL
// `GpuParticle` struct mirrors it exactly, so the raw wasm bytes can be
// copied to the GPU as-is.
const COLOR_STRIDE = 4; // floats per particle color (vec4f)
const LINK_STRIDE = 3; // u32 per Link (idx, a, b)

// Passed through firstInstance to the shader's instance_index.
const DRAW_DEBUG = 0;
const DRAW_PARTICLES = 1;
const DRAW_LINKS = 2;
const DRAW_INDICATORS = 3;


const color_cache = new Map();


const parse_color = (s) => {
    if (color_cache.has(s)) {
        return color_cache.get(s);
    }
    let rgba;
    if (s.startsWith("#")) {
        const hex = s.slice(1);
        const n = hex.length === 3
            ? hex.split("").map((c) => c + c).join("")
            : hex;
        rgba = [
            parseInt(n.slice(0, 2), 16) / 255,
            parseInt(n.slice(2, 4), 16) / 255,
            parseInt(n.slice(4, 6), 16) / 255,
            1.0,
        ];
    } else {
        const m = s.match(/[\d.]+/g);
        rgba = [
            parseFloat(m[0]) / 255,
            parseFloat(m[1]) / 255,
            parseFloat(m[2]) / 255,
            1.0,
        ];
    }
    color_cache.set(s, rgba);
    return rgba;
}


const resolve_shader_imports = async (src) => {
    const import_re = /^\s*import\s+([\w/]+)\s*;/gm;
    const parts = [];
    let last = 0;
    let match;
    while ((match = import_re.exec(src)) !== null) {
        parts.push(src.slice(last, match.index));
        const file = `./shader/${match[1]}.wgsl`;
        const text = await (
            await fetch(file, { cache: "no-store" })
        ).text();
        // Recursively resolve imports inside the dependency too.
        parts.push(await resolve_shader_imports(text));
        last = import_re.lastIndex;
    }
    parts.push(src.slice(last));
    return parts.join("\n");
}


const make_triangle_pipeline = async (device, format) => {
    const shader_src = await (
        await fetch("./shader/main.wgsl", { cache: "no-store" })
    ).text();
    const merged_src = await resolve_shader_imports(shader_src);
    const module = device.createShaderModule({ code: merged_src });
    const pipeline = device.createRenderPipeline({
        layout: 'auto',
        vertex: { module, entryPoint: 'vs_main' },
        fragment: {
            module,
            entryPoint: 'fs_main',
            targets: [{
                format,
                blend: {
                    color: {
                        srcFactor: 'src-alpha',
                        dstFactor: 'one-minus-src-alpha',
                        operation: 'add',
                    },
                    alpha: {
                        srcFactor: 'one',
                        dstFactor: 'one-minus-src-alpha',
                        operation: 'add',
                    },
                },
            }],
        },
        primitive: { topology: 'triangle-list' },
    });
    const uniform_buffer = device.createBuffer({
        size: 32,
        usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
    });
    return {
        pipeline,
        uniform_buffer,
        particle_buffer: null,
        color_buffer: null,
        link_buffer: null,
        particle_capacity: 0,
        link_capacity: 0,
        bind_group_with_particles: null,
    };
}


// Lazily (re)allocate the storage buffers when the count changes,
// and rebuild the bind group accordingly.
const ensure_storage = (device, gpu_pipeline, count, particle_size, link_count) => {
    const particle_ok =
        gpu_pipeline.particle_buffer !== null &&
        gpu_pipeline.particle_capacity >= count;
    const link_ok =
        gpu_pipeline.link_buffer !== null && gpu_pipeline.link_capacity >= link_count;
    if (particle_ok && link_ok) {
        return;
    }
    if (gpu_pipeline.particle_buffer !== null) {
        gpu_pipeline.particle_buffer.destroy();
        gpu_pipeline.color_buffer.destroy();
        gpu_pipeline.link_buffer.destroy();
    }
    const n = Math.max(1, count);
    gpu_pipeline.particle_buffer = device.createBuffer({
        size: n * particle_size,
        usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST,
    });
    gpu_pipeline.color_buffer = device.createBuffer({
        size: n * COLOR_STRIDE * 4,
        usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST,
    });
    const l = Math.max(1, link_count);
    gpu_pipeline.link_buffer = device.createBuffer({
        size: l * LINK_STRIDE * 4,
        usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST,
    });
    gpu_pipeline.particle_capacity = n;
    gpu_pipeline.link_capacity = l;
    gpu_pipeline.bind_group_with_particles = device.createBindGroup({
        layout: gpu_pipeline.pipeline.getBindGroupLayout(0),
        entries: [
            {
                binding: 0,
                resource: { buffer: gpu_pipeline.uniform_buffer },
            },
            {
                binding: 1,
                resource: { buffer: gpu_pipeline.particle_buffer },
            },
            {
                binding: 2,
                resource: { buffer: gpu_pipeline.color_buffer },
            },
            {
                binding: 3,
                resource: { buffer: gpu_pipeline.link_buffer },
            },
        ],
    });
}


const draw_triangle_webgpu = (
    device,
    context,
    gpu_pipeline,
    world,
    engine,
    memory,
) => {
    const height = context.canvas.height;
    const width = context.canvas.width;

    const particle_count = world.particles_count();
    const link_count = world.links_count();

    device.queue.writeBuffer(
        gpu_pipeline.uniform_buffer,
        0,
        new Float32Array([
            width,
            height,
            ZOOM,
            performance.now() / 1000,
            world.completed_steps,
            Math.random(),
            link_count,
            particle_count,
        ]),
    );

    const particle_size = engine.Particle.size();
    ensure_storage(device, gpu_pipeline, particle_count, particle_size, link_count);
    if (gpu_pipeline.bind_group_with_particles === null) {
        return;
    }

    // Raw Particle structs, mirrored 1:1 by the WGSL `GpuParticle` struct:
    // upload the wasm bytes directly, no per-field reads or staging copy.
    const f32_per_particle = particle_size / 4;
    const particles_data = new Float32Array(
        memory.buffer,
        world.particles(),
        particle_count * f32_per_particle,
    );
    
    device.queue.writeBuffer(
        gpu_pipeline.particle_buffer,
        0,
        particles_data,
        0,
        particle_count * f32_per_particle,
    );

    const links_data = new Uint32Array(memory.buffer, world.links(), link_count * LINK_STRIDE);
    device.queue.writeBuffer(gpu_pipeline.link_buffer, 0, links_data);

    // Colors must be computed in JS (engine.base_color); `kind` is read
    // directly from the same wasm memory (offset 76 bytes = 19 floats).
    const u32_per_particle = particle_size / 4;
    const kinds = new Uint32Array(
        memory.buffer,
        world.particles(),
        particle_count * u32_per_particle,
    );
    const colors = new Float32Array(particle_count * COLOR_STRIDE);
    for (let i = 0; i < particle_count; i++) {
        const kind = kinds[i * u32_per_particle + 19];
        const [r, g, b, a] = parse_color(engine.base_color(kind));
        const o = i * COLOR_STRIDE;
        colors[o + 0] = r;
        colors[o + 1] = g;
        colors[o + 2] = b;
        colors[o + 3] = a;
    }
    device.queue.writeBuffer(gpu_pipeline.color_buffer, 0, colors);

    const encoder = device.createCommandEncoder();
    const pass = encoder.beginRenderPass({
        colorAttachments: [{
            view: context.getCurrentTexture().createView(),
            clearValue: { r: 0, g: 0, b: 0, a: 0 },
            loadOp: 'clear',
            storeOp: 'store',
        }],
    });

    pass.setPipeline(gpu_pipeline.pipeline);
    pass.setBindGroup(0, gpu_pipeline.bind_group_with_particles);

    pass.draw(6 * particle_count, 1, 0, DRAW_PARTICLES);
    pass.draw(6 * link_count, 1, 0, DRAW_LINKS);
    pass.draw(6 * particle_count, 1, 0, DRAW_INDICATORS);
    pass.draw(9, 1, 0, DRAW_DEBUG);

    pass.end();
    device.queue.submit([encoder.finish()]);
}


export {
    draw_triangle_webgpu,
    make_triangle_pipeline,
}
