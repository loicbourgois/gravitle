const new_step = (x) => {
	const r = {};
	const bindGroupLayout = x.device.createBindGroupLayout({
		entries: [
			{
				binding: 0,
				visibility: GPUShaderStage.VERTEX,
				buffer: {
					type: "read-only-storage",
				},
			},
			// {
			// 	binding: 1,
			// 	visibility: GPUShaderStage.VERTEX,
			// 	buffer: {
			// 		type: "read-only-storage",
			// 	},
			// },
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
			targets: [{ format: x.presentation_format }],
		},
		primitive: {
			topology: "line-strip",
			topology: "triangle-list",
		},
	});
	r.bindGroup = x.device.createBindGroup({
		layout: r.pipeline.getBindGroupLayout(0),
		entries: [
			{ binding: 0, resource: { buffer: x.buffer_cells } },
		],
	});
	return r;
};

function ViewWebGPU(id) {
	this.canvas = document.getElementById(id);
	this.context = canvas.getContext("webgpu");
	this.center = {
		x: 0.5,
		y: 0.5,
	};
	this.zoom = 1;
	this.mouse = null;
}

ViewWebGPU.prototype.setup = async function (gravitle, memory) {
	this.resize()
	this.adapter = await navigator.gpu?.requestAdapter();
	this.device = await this.adapter?.requestDevice();
	const presentation_format = navigator.gpu.getPreferredCanvasFormat();
	this.context.configure({
		device: this.device,
		format: presentation_format,
		alphaMode: "premultiplied",
	});
	const code_disk = await (
		await fetch(`./chrono/webgpu/disk_generated.wgsl`, { cache: "no-store" })
	).text();
	const code = (
		await (
			await fetch(`./chrono/webgpu/code.wgsl`, { cache: "no-store" })
		).text()
	).replace("//__DISK_GENERATED__//", code_disk);
	const module = this.device.createShaderModule({
		label: "shaders",
		code: code,
	});
	const cells_count_max = 1000;
	const buffer_cells_size = gravitle.Cell.size() * cells_count_max;
	this.buffer_cells = this.device.createBuffer({
		size: buffer_cells_size,
		usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST,
	});
	this.steps = [
		new_step({
			label: "pipeline_0",
			vertex_entryPoint: "vs_0",
			fragment_entryPoint: "fs_0",
			device: this.device,
			module: module,
			buffer_cells: this.buffer_cells,
			presentation_format: presentation_format,
		})
	]
	this.renderPassDescriptor = {
        label: 'renderPass',
        colorAttachments: [
            {
                clearValue: { r: 0.0, g: 0.0625, b: 0.125, a: 0.1 },
                loadOp: 'clear',
                storeOp: 'store',
            },
        ],
    };
	console.log(gravitle.Cell.size())
};

ViewWebGPU.prototype.render = function (world, memory, gravitle) {
	this.renderPassDescriptor.colorAttachments[0].view =
        this.context.getCurrentTexture().createView();
    const encoder = this.device.createCommandEncoder({ label: 'encoder 1' });
    const pass = encoder.beginRenderPass(this.renderPassDescriptor);
    for (const step of this.steps) {
        pass.setPipeline(step.pipeline);
        pass.setBindGroup(0, step.bindGroup);
        pass.draw(16*3, world.cells_count() );
    }
    pass.end();
    const job1 = encoder.finish();
    this.device.queue.writeBuffer(
        this.buffer_cells,
        0, 
        memory.buffer,
        world.cells(),
        world.cells_count() * gravitle.Cell.size(),
    );
    this.device.queue.submit([
        job1,
        // job2,
    ]);
}

ViewWebGPU.prototype.set_backgound = function (color) {
	// this.background_color = color;
	// this.context.fillStyle = color;
	// this.context.fillRect(0, 0, this.canvas.width, this.canvas.height);
};

ViewWebGPU.prototype.set_mouse = function (x, y) {
	// this.mouse = {
	// 	html: {
	// 		x: x,
	// 		y: y,
	// 	},
	// 	canvas: {
	// 		x: x * this.dpr,
	// 		y: y * this.dpr,
	// 	},
	// 	world: this.pixel_to_world({
	// 		x: x * this.dpr,
	// 		y: y * this.dpr,
	// 	}),
	// };
};

ViewWebGPU.prototype.resize = function () {
	this.dpr = window.devicePixelRatio || 1;
	const size = Math.min(window.innerWidth, window.innerHeight) * this.dpr;
	this.canvas.width = size;
	this.canvas.height = size;
};

ViewWebGPU.prototype.min_dim = function () {
	// return Math.min(this.canvas.width, this.canvas.height);
};

ViewWebGPU.prototype.world_to_pixel_2 = function (x, y) {
	// return {
	// 	x:
	// 		x * this.min_dim() * this.zoom +
	// 		this.canvas.width / 2 -
	// 		this.center.x * this.zoom * this.canvas.width,
	// 	y:
	// 		-y * this.min_dim() * this.zoom +
	// 		this.canvas.height / 2 +
	// 		this.center.y * this.zoom * this.canvas.height,
	// };
};

ViewWebGPU.prototype.world_to_pixel = function (p) {
	// return this.world_to_pixel_2(p.x, p.y);
};

ViewWebGPU.prototype.pixel_to_world = function (p) {
	// return {
	// 	x: (p.x - this.canvas.width / 2) / (this.min_dim() * this.zoom),
	// 	y: -(p.y - this.canvas.height / 2) / (this.min_dim() * this.zoom),
	// };
};

ViewWebGPU.prototype.draw_disk = function (x, y, diameter, color) {
	// const radius = diameter * this.min_dim() * 0.5 * this.zoom;
	// this.context.beginPath();
	// const vp = this.world_to_pixel_2(x, y);
	// this.context.arc(vp.x, vp.y, radius, 0, 2 * Math.PI, false);
	// this.context.fillStyle = color;
	// this.context.fill();
};

ViewWebGPU.prototype.draw_disk_multi = function (x, y, diameter, color) {
	// for (const x2 of [-1, 0, 1]) {
	// 	for (const y2 of [-1, 0, 1]) {
	// 		this.draw_disk(x + x2, y + y2, diameter, color);
	// 	}
	// }
};

ViewWebGPU.prototype.draw_square = function (x, y, size, color) {
	// const vp = this.world_to_pixel_2(x, y);
	// const size_ = this.world_to_pixel_2(size, size);
	// this.context.fillStyle = color;
	// this.context.fillRect(
	// 	vp.x,
	// 	vp.y,
	// 	size_.x - this.canvas.width / 2,
	// 	size_.x - this.canvas.width / 2,
	// );
};

ViewWebGPU.prototype.draw_line = function (a, b, color, line_width) {
	// const avp = this.world_to_pixel(a);
	// const bvp = this.world_to_pixel(b);
	// this.context.beginPath();
	// this.context.moveTo(avp.x, avp.y);
	// this.context.lineTo(bvp.x, bvp.y);
	// this.context.strokeStyle = color;
	// this.context.lineWidth = line_width ? line_width : 2;
	// this.context.stroke();
};

export { ViewWebGPU };
