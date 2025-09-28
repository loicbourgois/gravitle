import {
    resize_square,
} from "../canvas.js"
const RESOLUTION = 2.0
const main = async () => {
    document.body.innerHTML = `
        <canvas id="canvas"></canvas>
    `
    const canvas = document.querySelector('#canvas');
    resize_square(canvas, RESOLUTION*0.9)
    canvas.style.width = `${canvas.width/RESOLUTION}px`
    canvas.style.height = `${canvas.height/RESOLUTION}px`

    const adapter = await navigator.gpu?.requestAdapter();
    const device = await adapter?.requestDevice();
    if (!device) {
        fail('need a browser that supports WebGPU');
        return;
    }
    const context = canvas.getContext('webgpu');
    const presentationFormat = navigator.gpu.getPreferredCanvasFormat();
    context.configure({
        device,
        format: presentationFormat,
    });
    const code_disk = await (await fetch(`./disk_generated.wgsl`)).text()
    const module = device.createShaderModule({
        label: 'our hardcoded red triangle shaders',
        code: (await (await fetch(`./code.wgsl`)).text()).replace("//__DISK_GENERATED__//", code_disk),
    });
    const particle_count = 10000;
    const particle_size = 4 * 4;
    const uniformBufferSize = particle_count * particle_size;  
    const uniformBuffer = device.createBuffer({
        size: uniformBufferSize,
        usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST,
    });
    const js_buffer = new ArrayBuffer(uniformBufferSize)
    const view = new DataView(js_buffer);
    for (let pid = 0; pid < particle_count; pid++) {
        const bid = pid * particle_size;
        const x = pid / particle_count
        const y = x*x
        view.setFloat32(bid+0, x, true);
        view.setFloat32(bid+4, y, true);
    }
    const pipeline = device.createRenderPipeline({
        label: 'pipeline',
        layout: 'auto',
        vertex: {
          module,
          entryPoint: 'vs',
        },
        fragment: {
          module,
          entryPoint: 'fs',
          targets: [{ format: presentationFormat }],
        },
      });
      const bindGroup = device.createBindGroup({
        layout: pipeline.getBindGroupLayout(0),
        entries: [
          { binding: 0, resource: { buffer: uniformBuffer }},
        ],
      });
      const renderPassDescriptor = {
        label: 'renderPass',
        colorAttachments: [
          {
            clearValue: [0.1, 0.1, 0.1, 1],
            loadOp: 'clear',
            storeOp: 'store',
          },
        ],
      };  
    render(context,
        device,
        renderPassDescriptor,
        pipeline, 
        bindGroup,
        particle_count,
        view,
        particle_size,
        uniformBuffer,
        js_buffer,
    );
}
let last_start = undefined
function render(
    context,
    device,
    renderPassDescriptor,
    pipeline, 
    bindGroup,
    particle_count,
    view,
    particle_size,
    uniformBuffer,
    js_buffer,
) {
    const start = performance.now()
    last_start = start
    const s = performance.now()
    for (let pid = 0; pid < particle_count; pid++) {
        const bid = pid * particle_size;
        view.setFloat32(
            bid+0, 
            view.getFloat32(bid+0, true) + Math.random() * 0.001 - 0.0005, 
            true
        );
        view.setFloat32(
            bid+4, 
            view.getFloat32(bid+4, true) + Math.random() * 0.001 - 0.0005, 
            true
        );
    }
    renderPassDescriptor.colorAttachments[0].view =
        context.getCurrentTexture().createView();
    const encoder = device.createCommandEncoder({ label: 'our encoder' });
    const pass = encoder.beginRenderPass(renderPassDescriptor);
    pass.setPipeline(pipeline);
    pass.setBindGroup(0, bindGroup);
    pass.draw(16*3, particle_count);
    pass.end();
    device.queue.writeBuffer(uniformBuffer, 0, js_buffer);
    device.queue.submit([encoder.finish()]);

    // console.log( `${(performance.now() - s).toFixed(2)} ms` )

    setTimeout(()=>{
        render(context,
            device,
            renderPassDescriptor,
            pipeline, 
            bindGroup,
            particle_count,
            view,
            particle_size,
            uniformBuffer,
            js_buffer,
        );
    }, 0)

    // requestAnimationFrame(()=>{
        // requestAnimationFrame(()=>{
        //     render(context,
        //         device,
        //         renderPassDescriptor,
        //         pipeline, 
        //         bindGroup,
        //         particle_count,
        //         view,
        //         particle_size,
        //         uniformBuffer,
        //         js_buffer,
        //     );
        // })
    // })

}
main();
