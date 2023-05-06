const draw_gpu = (self) => {
    self.webgpu.renderPassDescriptor.colorAttachments[0].view =
        self.webgpu.context.getCurrentTexture().createView();
    const encoder = self.webgpu.device.createCommandEncoder({ label: 'our encoder' });
    const pass = encoder.beginRenderPass(self.webgpu.renderPassDescriptor);

    for (const step of self.webgpu.steps) {
        pass.setPipeline(step.pipeline);
        pass.setBindGroup(0, step.bindGroup);
        pass.draw(16*3, self.gravithrust.particles_count());
    }
    pass.end();
    self.webgpu.device.queue.writeBuffer(
        self.webgpu.buffer,
        0, 
        self.wasm.memory.buffer,
        self.gravithrust.particles(),
        self.gravithrust.particles_size(),
    );
    self.webgpu.device.queue.submit([encoder.finish()]);
}
const new_step = (x) => {
    const r = {}
    r.pipeline = x.device.createRenderPipeline({
        label: x.label,
        layout: 'auto',
        vertex: {
            module: x.module,
            entryPoint: x.vertex_entryPoint,
        },
        fragment: {
            module: x.module,
            entryPoint: x.fragment_entryPoint,
            targets: [{ format: x.presentationFormat }],
        },
        primitive: {
            topology: 'line-strip',
            topology: 'triangle-list',
        }
    });
    r.bindGroup = x.device.createBindGroup({
        layout: r.pipeline.getBindGroupLayout(0),
        entries: [
            { binding: 0, resource: { buffer: x.webgpu.buffer }},
        ],
    });
    return r
}
const setup_webgpu = async (
    device,
    gravithrust,
) => {
    const webgpu = {
        device: device,
    }
    webgpu.context = canvas.getContext('webgpu');
    const presentationFormat = navigator.gpu.getPreferredCanvasFormat();
    webgpu.context.configure({
        device,
        format: presentationFormat,
        alphaMode: "premultiplied",
    });
    const code_disk = await (await fetch(`./webgpu/disk_generated.wgsl`, {cache: "no-store"})).text()
    const code_kinds = await (await fetch(`./webgpu/kind_generated.wgsl`, {cache: "no-store"})).text()
    const module = device.createShaderModule({
        label: 'shaders',
        code: (
            await (await fetch(`./webgpu/code.wgsl`, {cache: "no-store"})).text()
        )
        .replace("//__DISK_GENERATED__//", code_disk)
        .replace("//__KIND_GENERATED__//", code_kinds),
    });
    webgpu.particle_max_count = 10000;
    webgpu.particle_size = gravithrust.particle_size()
    webgpu.buffer_size = webgpu.particle_max_count * webgpu.particle_size;  
    webgpu.buffer = device.createBuffer({
        size: webgpu.buffer_size,
        usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST,
    });
    webgpu.steps = [
        new_step({
            label: 'pipeline_3',
            vertex_entryPoint : 'vs_3',
            fragment_entryPoint: 'fs_3',
            device: device,
            webgpu: webgpu,
            module: module,
            presentationFormat: presentationFormat,
        }),
        new_step({
            label: 'pipeline_2',
            vertex_entryPoint : 'vs_2',
            fragment_entryPoint: 'fs_2',
            device: device,
            webgpu: webgpu,
            module: module,
            presentationFormat: presentationFormat,
        }),
        new_step({
            label: 'pipeline',
            vertex_entryPoint : 'vs',
            fragment_entryPoint: 'fs',
            device: device,
            webgpu: webgpu,
            module: module,
            presentationFormat: presentationFormat,
        }),
    ]
    webgpu.renderPassDescriptor = {
        label: 'renderPass',
        colorAttachments: [
            {
                clearValue: { r: 0.0, g: 0.0625, b: 0.125, a: 0.1 },
                loadOp: 'clear',
                storeOp: 'store',
            },
        ],
    };
    return webgpu
}
export {
    setup_webgpu,
    draw_gpu,
}
