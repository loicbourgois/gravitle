import { ZOOM } from "./consts.js";
import { get_particle } from "./particle.js";

// Rust `Particle` is #[repr(C)] with 4-byte-aligned fields; the WGSL
// `GpuParticle` struct mirrors it exactly, so the raw wasm bytes can be
// copied to the GPU as-is.
const COLOR_STRIDE = 4; // floats per particle color (vec4f)

const color_cache = new Map();

// Parse engine.base_color output: "#eee", "#48f" or "rgb(r, g, b)"
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

const make_triangle_pipeline = async (device, format) => {
    const shader_src = await (await fetch("./shader.wgsl")).text();
    const module = device.createShaderModule({ code: shader_src });
    const pipeline = device.createRenderPipeline({
        layout: 'auto',
        vertex: { module, entryPoint: 'vs_main' },
        fragment: { module, entryPoint: 'fs_main', targets: [{ format }] },
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
        particle_capacity: 0,
        bind_group_with_particles: null,
    };
}

// Lazily (re)allocate the storage buffers when the count changes,
// and rebuild the bind group accordingly.
const ensure_storage = (device, gpu_pipeline, count, particle_size) => {
    if (gpu_pipeline.particle_buffer !== null && gpu_pipeline.particle_capacity >= count) {
        return;
    }
    if (gpu_pipeline.particle_buffer !== null) {
        gpu_pipeline.particle_buffer.destroy();
        gpu_pipeline.color_buffer.destroy();
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
    gpu_pipeline.particle_capacity = n;
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
    device.queue.writeBuffer(
        gpu_pipeline.uniform_buffer,
        0,
        new Float32Array([
            width,
            height,
            ZOOM,
            performance.now() / 1000, // time in seconds
            world.completed_steps,
            Math.random(),            // fresh random per frame
            0.0,
            0.0,
        ]),
    );

    const count = world.particles_count();
    const particle_size = engine.Particle.size();
    ensure_storage(device, gpu_pipeline, count, particle_size);
    if (gpu_pipeline.bind_group_with_particles === null) {
        return;
    }

    // Raw Particle structs, mirrored 1:1 by the WGSL `GpuParticle` struct:
    // upload the wasm bytes directly, no per-field reads or staging copy.
    const f32_per_particle = particle_size / 4;
    const particles_data = new Float32Array(
        memory.buffer,
        world.particles(),
        count * f32_per_particle,
    );
    device.queue.writeBuffer(
        gpu_pipeline.particle_buffer,
        0,
        particles_data,
        0,
        count * f32_per_particle,
    );

    // Colors must be computed in JS (engine.base_color); `kind` is read
    // directly from the same wasm memory (offset 76 bytes = 19 floats).
    const u32_per_particle = particle_size / 4;
    const kinds = new Uint32Array(
        memory.buffer,
        world.particles(),
        count * u32_per_particle,
    );
    const colors = new Float32Array(count * COLOR_STRIDE);
    for (let i = 0; i < count; i++) {
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
    // 3 for the debug triangle + 6 for the 100x100 square + 6 per particle
    pass.draw(9 + 6 * count);
    pass.end();
    device.queue.submit([encoder.finish()]);
}


export {
    draw_triangle_webgpu,
    make_triangle_pipeline,
}