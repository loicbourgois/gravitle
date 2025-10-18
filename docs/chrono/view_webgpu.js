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
			cullMode: "back",
		},
		// depthStencil: {
		// 	format: "depth24plus",
		// 	depthWriteEnabled: true,
		// 	depthCompare: "less",
		// },
		// primitive: {
		// cullMode: 'front',  // note: uncommon setting. See article
		// },
		depthStencil: {
			depthWriteEnabled: true,
			depthCompare: "less",
			format: "depth24plus",
		},
	});
	r.bindGroup = x.device.createBindGroup({
		layout: r.pipeline.getBindGroupLayout(0),
		entries: [{ binding: 0, resource: { buffer: x.buffer_cells } }],
	});
	return r;
};

function ViewWebGPU(id) {
	this.canvas = document.getElementById(id);
	this.context = canvas.getContext("webgpu", {
		powerpreference: "high-performance",
	});
	this.center = {
		x: 0.5,
		y: 0.5,
	};
	this.zoom = 1;
	this.mouse = null;
}

ViewWebGPU.prototype.setup = async function (gravitle, memory) {
	this.resize();
	this.adapter = await navigator.gpu?.requestAdapter();
	this.device = await this.adapter?.requestDevice();
	const presentation_format = navigator.gpu.getPreferredCanvasFormat();
	this.context.configure({
		device: this.device,
		format: presentation_format,
		alphaMode: "premultiplied",
	});
	const code = await (
		await fetch(`./chrono/webgpu/code.wgsl`, { cache: "no-store" })
	).text();
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
		}),
	];
	// const depthTexture = device.createTexture({
	// 	size: [canvas.width, canvas.height],
	// 	format: "depth24plus",
	// 	usage: GPUTextureUsage.RENDER_ATTACHMENT,
	// });
	this.renderPassDescriptor = {
		label: "renderPass",
		colorAttachments: [
			{
				clearValue: { r: 0.0, g: 0.0625, b: 0.125, a: 0.1 },
				loadOp: "clear",
				storeOp: "store",
			},
		],
		depthStencilAttachment: {
			// view: depthTexture.createView(),
			depthClearValue: 1.0,
			depthLoadOp: "clear",
			depthStoreOp: "store",
		},
	};
	console.log(gravitle.Cell.size());
};

ViewWebGPU.prototype.render = function (worlds, ghosts, gravitle, memory) {
	// this.renderPassDescriptor.colorAttachments[0].view = this.context
	// 	.getCurrentTexture()
	// 	.createView();

	const canvasTexture = this.context.getCurrentTexture();
	this.renderPassDescriptor.colorAttachments[0].view =
		canvasTexture.createView();

	// If we don't have a depth texture OR if its size is different
	// from the canvasTexture when make a new depth texture
	if (
		!this.depthTexture ||
		this.depthTexture.width !== canvasTexture.width ||
		this.depthTexture.height !== canvasTexture.height
	) {
		if (this.depthTexture) {
			this.depthTexture.destroy();
		}
		this.depthTexture = this.device.createTexture({
			size: [canvasTexture.width, canvasTexture.height],
			format: "depth24plus",
			usage: GPUTextureUsage.RENDER_ATTACHMENT,
		});
	}
	this.renderPassDescriptor.depthStencilAttachment.view =
		this.depthTexture.createView();

	let total_count = 0;
	for (const world of worlds) {
		total_count += world.cells_count();
	}
	const encoder = this.device.createCommandEncoder({ label: "encoder 1" });
	const pass = encoder.beginRenderPass(this.renderPassDescriptor);
	for (const step of this.steps) {
		pass.setPipeline(step.pipeline);
		pass.setBindGroup(0, step.bindGroup);
		pass.draw(16 * 3, total_count);
	}
	pass.end();
	const job1 = encoder.finish();
	let countr = 0;
	for (const world of worlds) {
		this.device.queue.writeBuffer(
			this.buffer_cells,
			countr,
			memory.buffer,
			world.cells(),
			world.cells_count() * gravitle.Cell.size(),
		);
		countr += world.cells_count() * gravitle.Cell.size();
	}
	this.device.queue.submit([job1]);
};

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

export { ViewWebGPU };
