const setup_webgpu = async (canvas, source_canvas) => {
    if (!navigator.gpu) {
        throw new Error("WebGPU requires a supported browser and secure context.");
    }

    const adapter = await navigator.gpu.requestAdapter();
    if (!adapter) {
        throw new Error("No WebGPU adapter is available.");
    }

    const device = await adapter.requestDevice();

    try {
        const context = canvas.getContext("webgpu");
        if (!context) {
            throw new Error("Could not create a WebGPU canvas context.");
        }

        const format = navigator.gpu.getPreferredCanvasFormat();
        context.configure({
            device,
            format,
            alphaMode: "premultiplied",
        });

        const shader = device.createShaderModule({
            label: "Overlay triangle shader",
            code: `
                @vertex
                fn vertex_main(
                    @builtin(vertex_index) index: u32,
                ) -> @builtin(position) vec4f {
                    let positions = array<vec2f, 3>(
                        vec2f(0.0, 0.6),
                        vec2f(-0.6, -0.6),
                        vec2f(0.6, -0.6),
                    );

                    return vec4f(positions[index], 0.0, 1.0);
                }

                @fragment
                fn fragment_main() -> @location(0) vec4f {
                    return vec4f(0.2, 0.7, 1.0, 1.0);
                }
            `,
        });

        const pipeline = await device.createRenderPipelineAsync({
            label: "Overlay triangle pipeline",
            layout: "auto",
            vertex: {
                module: shader,
                entryPoint: "vertex_main",
            },
            fragment: {
                module: shader,
                entryPoint: "fragment_main",
                targets: [{ format }],
            },
            primitive: {
                topology: "triangle-list",
            },
        });

        const draw = () => {
            const limit = device.limits.maxTextureDimension2D;
            canvas.width = Math.max(1, Math.min(source_canvas.width, limit));
            canvas.height = Math.max(1, Math.min(source_canvas.height, limit));

            const encoder = device.createCommandEncoder();
            const pass = encoder.beginRenderPass({
                colorAttachments: [{
                    view: context.getCurrentTexture().createView(),
                    clearValue: { r: 0, g: 0, b: 0, a: 0 },
                    loadOp: "clear",
                    storeOp: "store",
                }],
            });

            pass.setPipeline(pipeline);
            pass.draw(3);
            pass.end();

            device.queue.submit([encoder.finish()]);
        };

        draw();

        // Observe backing-size attributes as well as CSS layout changes.
        const observer = new MutationObserver(draw);
        observer.observe(source_canvas, {
            attributes: true,
            attributeFilter: ["width", "height"],
        });

        device.lost.then((info) => {
            observer.disconnect();
            context.unconfigure();
            canvas.remove();
            console.warn("WebGPU overlay device lost:", info.message);
        });
    } catch (error) {
        device.destroy();
        throw error;
    }
};


export {
    setup_webgpu,
};