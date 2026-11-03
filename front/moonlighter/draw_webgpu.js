// TODO draw all particles

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
    return { pipeline, uniform_buffer, bind_group };
}


const draw_triangle_webgpu = (device, context, gpu_pipeline) => {
    const { pipeline, uniform_buffer, bind_group } = gpu_pipeline;
    const height = context.canvas.height;
    const width = context.canvas.width;
    device.queue.writeBuffer(
        uniform_buffer,
        0,
        new Float32Array([width, height]),
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
    pass.setBindGroup(0, bind_group);
    pass.draw(21); // 3 for the triangle + 6 for the 100x100 square + 6 for the centered square + 6 for the circle
    pass.end();
    device.queue.submit([encoder.finish()]);
}


export {
    draw_triangle_webgpu,
    make_triangle_pipeline,
}