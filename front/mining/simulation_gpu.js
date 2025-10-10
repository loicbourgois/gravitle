const draw_gpu = (self) => {
    self.webgpu.renderPassDescriptor.colorAttachments[0].view =
        self.webgpu.context.getCurrentTexture().createView();
    const encoder = self.webgpu.device.createCommandEncoder({ label: 'encoder 1' });
    const pass = encoder.beginRenderPass(self.webgpu.renderPassDescriptor);
    for (const step of self.webgpu.steps) {
        pass.setPipeline(step.pipeline);
        pass.setBindGroup(0, step.bindGroup);
        pass.draw(16*3, self.gravithrust.particles_count());
    }
    pass.end();
    const job1 = encoder.finish();
    self.webgpu.renderPassDescriptor_2.colorAttachments[0].view =
        self.webgpu.context_2.getCurrentTexture().createView();
    const encoder_2 = self.webgpu.device.createCommandEncoder({ label: 'encoder 2' });
    const pass_2 = encoder_2.beginRenderPass(self.webgpu.renderPassDescriptor_2);
    for (const step of self.webgpu.steps_2) {
        pass_2.setPipeline(step.pipeline);
        pass_2.setBindGroup(0, step.bindGroup);
        pass_2.draw(16*3, 1000 * 6);
    }
    pass_2.end();
    const job2 = encoder_2.finish();
    self.webgpu.device.queue.writeBuffer(
        self.webgpu.buffer,
        0, 
        self.wasm.memory.buffer,
        self.gravithrust.particles(),
        self.gravithrust.particles_size(),
    );
    self.webgpu.device.queue.writeBuffer(
        self.webgpu.buffer_durations,
        0, 
        self.wasm.memory.buffer,
        self.gravithrust.average_durations_pointer(),
        self.gravithrust.average_durations_size_full(),
    );
    self.webgpu.device.queue.submit([
        job1,
        job2,
    ]);
}
const new_step = (x) => {
    const r = {}


    const bindGroupLayout = x.device.createBindGroupLayout({
        entries: [
          {
            binding: 0,
            visibility: GPUShaderStage.VERTEX ,
            buffer: {
                type: 'read-only-storage'
            },
          },
          {
            binding: 1,
            visibility: GPUShaderStage.VERTEX ,
            buffer: {
                type: 'read-only-storage'
            },
          },
        ],
      });
      const pipelineLayout = x.device.createPipelineLayout({
        bindGroupLayouts: [bindGroupLayout],
      });


    r.pipeline = x.device.createRenderPipeline({
        label: x.label,
        layout: pipelineLayout,
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
            { binding: 1, resource: { buffer: x.webgpu.buffer_durations }},
        ],
    });
    return r
}
const setup_webgpu = async (
    device,
    gravithrust,
    canvas,
    canvas_2,
) => {
    const webgpu = {
        device: device,
    }
    const presentationFormat = navigator.gpu.getPreferredCanvasFormat();
    webgpu.context = canvas.getContext('webgpu');
    webgpu.context.configure({
        device,
        format: presentationFormat,
        alphaMode: "premultiplied",
    });
    webgpu.context_2 = canvas_2.getContext('webgpu');
    webgpu.context_2.configure({
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

    webgpu.buffer_durations_size = 1000 * gravithrust.average_durations_size_unit()
    webgpu.buffer_durations = device.createBuffer({
        size: webgpu.buffer_durations_size,
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
    webgpu.steps_2 = [
        new_step({
            label: 'pipeline_4',
            vertex_entryPoint : 'vs_4',
            fragment_entryPoint: 'fs_4',
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
    webgpu.renderPassDescriptor_2 = {
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
