import { ZOOM } from "./consts.js";
import { get_particle } from "./particle.js";

// 32 bytes per particle: vec4f pos+radius, vec4f color
const PARTICLE_STRIDE = 32;

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
        size: 8, // vec2f
        usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
    });
    const bind_group = device.createBindGroup({
        layout: pipeline.getBindGroupLayout(0),
        entries: [{
            binding: 0,
            resource: { buffer: uniform_buffer },
        }],
    });
    return {
        pipeline,
        uniform_buffer,
        bind_group,
        storage_buffer: null,
        storage_capacity: 0,
        bind_group_with_particles: null,
    };
}

// Lazily (re)allocate the particle storage buffer when the count changes,
// and rebuild the bind group accordingly.
const ensure_storage = (device, gpu_pipeline, count) => {
    if (gpu_pipeline.storage_buffer !== null && gpu_pipeline.storage_capacity >= count) {
        return;
    }
    if (gpu_pipeline.storage_buffer !== null) {
        gpu_pipeline.storage_buffer.destroy();
    }
    const size = Math.max(1, count) * PARTICLE_STRIDE;
    gpu_pipeline.storage_buffer = device.createBuffer({
        size,
        usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST,
    });
    gpu_pipeline.storage_capacity = Math.max(1, count);
    gpu_pipeline.bind_group_with_particles = device.createBindGroup({
        layout: gpu_pipeline.pipeline.getBindGroupLayout(0),
        entries: [
            {
                binding: 0,
                resource: { buffer: gpu_pipeline.uniform_buffer },
            },
            {
                binding: 1,
                resource: { buffer: gpu_pipeline.storage_buffer },
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
    const { pipeline, uniform_buffer } = gpu_pipeline;
    const height = context.canvas.height;
    const width = context.canvas.width;
    device.queue.writeBuffer(
        uniform_buffer,
        0,
        new Float32Array([width, height]),
    );

    const count = world.particles_count();
    ensure_storage(device, gpu_pipeline, count);

    const diameter = Math.min(height, width) * ZOOM;
    // TODO use memory.buffer directly
    const data = new Float32Array(gpu_pipeline.storage_capacity * 8);
    const particles_ptr = world.particles();
    const particle_size = engine.Particle.size();
    const particles_view = new DataView(
        memory.buffer,
        particles_ptr,
        count * particle_size,
    );
    for (let i = 0; i < count; i++) {
        
        const particle = get_particle(particles_view, particle_size, i);
        const o = i * 8;
        // TODO: do Not copy data around
        // directly map the buffer to th gpu code
        data[o + 0] = particle.p.x;
        data[o + 1] = particle.p.y;
        // Shader multiplies world x/y by ZOOM * min(w, h); keep radius consistent
        data[o + 2] = (diameter / 2) * particle.diameter;
        data[o + 3] = 0.0;
        const [r, g, b, a] = parse_color(engine.base_color(particle.kind));
        data[o + 4] = r;
        data[o + 5] = g;
        data[o + 6] = b;
        data[o + 7] = a;
    }
    device.queue.writeBuffer(
        gpu_pipeline.storage_buffer,
        0,
        data,
        0,
        count * 8,
    );

    const encoder = device.createCommandEncoder();
    const pass = encoder.beginRenderPass({
        colorAttachments: [{
            view: context.getCurrentTexture().createView(),
            clearValue: { r: 0, g: 0, b: 0, a: 0 },
            loadOp: 'clear',
            storeOp: 'store',
        }],
    });
    pass.setPipeline(pipeline);
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