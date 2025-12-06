const bind_group_layout_entry = (idx) => {
	return {
		binding: idx,
		visibility: GPUShaderStage.VERTEX,
		buffer: {
			type: "read-only-storage",
		},
	};
};

const bind_group_entry = (idx, buffer) => {
	return { binding: idx, resource: { buffer } };
};

const create_buffer = (device, count_max, struct) => {
	return device.createBuffer({
		size: struct.size() * count_max,
		usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST,
	});
};

const new_step = (x) => {
	const r = {};
	const bindGroupLayout = x.device.createBindGroupLayout({
		entries: [
			bind_group_layout_entry(0), 
			bind_group_layout_entry(1),
			bind_group_layout_entry(2),
			{
				binding: 3,
				visibility: GPUShaderStage.VERTEX,
				buffer: {
					type: "uniform",
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
		depthStencil: {
			depthWriteEnabled: true,
			depthCompare: "less",
			format: "depth24plus",
		},
	});
	r.bindGroup = x.device.createBindGroup({
		layout: r.pipeline.getBindGroupLayout(0),
		entries: [
			bind_group_entry(0, x.buffer_cells),
			bind_group_entry(1, x.buffer_materials),
			bind_group_entry(2, x.buffer_positions),
			bind_group_entry(3, x.buffer_uniform),
		],
	});
	return r;
};

function ViewWebGPU(canvas_id) {
	this.canvas = document.getElementById(canvas_id);
	this.context = canvas.getContext("webgpu", {
		powerpreference: "high-performance",
	});
	this.center = {
		x: 0.0,
		y: 0.0,
	};
	this.zoom = 1.0;
	this.mouse = null;
}

ViewWebGPU.prototype.set_zoom = function (zoom) {
	this.zoom = zoom
}

const setup_uniform = () {

}

ViewWebGPU.prototype.setup_uniform = function () {
	const uniformBufferSize =
    	1 * 4; // 1 32bit floats
	this.buffer_uniform = this.device.createBuffer({
		size: uniformBufferSize,
		usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
	});
	this.uniformValues = new Float32Array(uniformBufferSize / 4);
}

ViewWebGPU.prototype.setup = async function (gravitle) {
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
		await fetch(`/slingshot/webgpu/code.wgsl`, { cache: "no-store" })
	).text();
	const module = this.device.createShaderModule({
		label: "shaders",
		code: code,
	});
	this.setup_uniform()
	this.buffer_cells = create_buffer(this.device, 100000, gravitle.Cell);
	this.buffer_materials = create_buffer(this.device, 40, gravitle.Material);
	this.buffer_positions = create_buffer(this.device, 200000, gravitle.Point);
	this.steps = [
		new_step({
			label: "pipeline_1",
			vertex_entryPoint: "vs_1",
			fragment_entryPoint: "fs_1",
			device: this.device,
			module: module,
			buffer_cells: this.buffer_cells,
			buffer_materials: this.buffer_materials,
			buffer_positions: this.buffer_positions,
			buffer_uniform: this.buffer_uniform,
			presentation_format: presentation_format,
		}),
		new_step({
			label: "pipeline_0",
			vertex_entryPoint: "vs_0",
			fragment_entryPoint: "fs_0",
			device: this.device,
			module: module,
			buffer_cells: this.buffer_cells,
			buffer_materials: this.buffer_materials,
			buffer_positions: this.buffer_positions,
			buffer_uniform: this.buffer_uniform,
			presentation_format: presentation_format,
		}),
	];
	this.renderPassDescriptor = {
		label: "renderPass",
		colorAttachments: [
			{
				clearValue: { g: 0.0, r: 0.0625, b: 0.125, a: 0.95 },
				// clearValue: { r: 0.0, g: 0.0625, b: 0.125, a: 0.1 },
				loadOp: "clear",
				storeOp: "store",
			},
		],
		depthStencilAttachment: {
			depthClearValue: 1.0,
			depthLoadOp: "clear",
			depthStoreOp: "store",
		},
	};
};


// ViewWebGPU.prototype.render = function (worlds, gravitle, memory) {
// 	const canvas_texture = this.context.getCurrentTexture();
// 	this.uniformValues.set([
// 		this.zoom,
// 	]);
// 	this.renderPassDescriptor.colorAttachments[0].view =
// 		canvas_texture.createView();
// 	if (
// 		!this.depthTexture ||
// 		this.depthTexture.width !== canvas_texture.width ||
// 		this.depthTexture.height !== canvas_texture.height
// 	) {
// 		if (this.depthTexture) {
// 			this.depthTexture.destroy();
// 		}
// 		this.depthTexture = this.device.createTexture({
// 			size: [canvas_texture.width, canvas_texture.height],
// 			format: "depth24plus",
// 			usage: GPUTextureUsage.RENDER_ATTACHMENT,
// 		});
// 	}
// 	this.renderPassDescriptor.depthStencilAttachment.view =
// 		this.depthTexture.createView();
// 	// for (const _world of worlds) {
// 		console.log(this.uniformValues)
// 		this.device.queue.writeBuffer(this.buffer_uniform, 3, this.uniformValues);
// 	// }
// 	let countr = 0;
// 	for (const world of worlds) {
// 		this.device.queue.writeBuffer(
// 			this.buffer_cells,
// 			countr,
// 			memory.buffer,
// 			world.cells(),
// 			world.cells_count() * gravitle.Cell.size(),
// 		);
// 		countr += world.cells_count() * gravitle.Cell.size();
// 	}
// 	let countr_2 = 0;
// 	for (const world of worlds) {
// 		this.device.queue.writeBuffer(
// 			this.buffer_materials,
// 			countr_2,
// 			memory.buffer,
// 			world.materials(),
// 			world.materials_count() * gravitle.Material.size(),
// 		);
// 		countr_2 += world.materials_count() * gravitle.Material.size();
// 	}
// 	this.device.queue.submit([
// 		(() => {
// 			const encoder = this.device.createCommandEncoder({ label: "encoder 1" });
// 			const pass = encoder.beginRenderPass(this.renderPassDescriptor);
// 			let step = this.steps[1]
// 			pass.setPipeline(step.pipeline);
// 			pass.setBindGroup(0, step.bindGroup);
// 			pass.draw(
// 				// edge per cell model
// 				16 * 3,
// 				// total cell count
// 				worlds.reduce((sum, world) => sum + world.cells_count(), 0),
// 			);
// 			pass.end();
// 			return encoder.finish();
// 		})(),
// 	]);
// };


ViewWebGPU.prototype.render_2 = function (worlds, gravitle, memory) {
	const canvas_texture = this.context.getCurrentTexture();
	this.uniformValues.set([
		this.zoom,
	]);
	this.device.queue.writeBuffer(this.buffer_uniform, 0, this.uniformValues);
	this.renderPassDescriptor.colorAttachments[0].view =
		canvas_texture.createView();
	if (
		!this.depthTexture ||
		this.depthTexture.width !== canvas_texture.width ||
		this.depthTexture.height !== canvas_texture.height
	) {
		if (this.depthTexture) {
			this.depthTexture.destroy();
		}
		this.depthTexture = this.device.createTexture({
			size: [canvas_texture.width, canvas_texture.height],
			format: "depth24plus",
			usage: GPUTextureUsage.RENDER_ATTACHMENT,
		});
	}
	this.renderPassDescriptor.depthStencilAttachment.view =
		this.depthTexture.createView();
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
	let countr_2 = 0;
	for (const world of worlds) {
		this.device.queue.writeBuffer(
			this.buffer_materials,
			countr_2,
			memory.buffer,
			world.materials(),
			world.materials_count() * gravitle.Material.size(),
		);
		countr_2 += world.materials_count() * gravitle.Material.size();
	}

	let countr_3 = 0;
	for (const world of worlds) {
		this.device.queue.writeBuffer(
			this.buffer_positions,
			countr_3,
			memory.buffer,
			world.positions(),
			world.positions_count() * gravitle.Point.size(),
		);
		countr_3 += world.positions_count() * gravitle.Point.size();
	}

	this.device.queue.submit([
		(() => {
			const encoder = this.device.createCommandEncoder({ label: "encoder 1" });
			const pass = encoder.beginRenderPass(this.renderPassDescriptor);
			let step = this.steps[0]
			pass.setPipeline(step.pipeline);
			pass.setBindGroup(0, step.bindGroup);
			pass.draw(
				// edge per cell model
				16 * 3,
				// total cell count
				worlds.reduce((sum, world) => sum + world.positions_count(), 0),
			);
			step = this.steps[1]
			pass.setPipeline(step.pipeline);
			pass.setBindGroup(0, step.bindGroup);
			pass.draw(
				// edge per cell model
				16 * 3,
				// total cell count
				worlds.reduce((sum, world) => sum + world.cells_count(), 0),
			);
			pass.end();
			return encoder.finish();
		})(),
	]);
};



ViewWebGPU.prototype.resize = function () {
	this.dpr = window.devicePixelRatio || 1;
	const size = Math.min(window.innerWidth, window.innerHeight) * this.dpr;
	this.canvas.width = size;
	this.canvas.height = size;
};

export { ViewWebGPU };
