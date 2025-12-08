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
	this.link_line_width = 0.0005;
}


ViewWebGPU.prototype.set_zoom = function (zoom) {
	this.zoom = zoom
}


ViewWebGPU.prototype.setup_uniform = function (binding) {
	const uniformBufferSize =
    	4 // zoom: f32
		+ 4 // line_width: f32
		+ 4 // line_width: u32
		+ 4 // padding
	;
	this.buffer_uniform = this.device.createBuffer({
		size: uniformBufferSize,
		usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
	});
	this.uniformBufferArray = new ArrayBuffer(uniformBufferSize);
	this.uniformValues = new DataView(this.uniformBufferArray);
	// this.uniformValuesF32 = new Float32Array(this.uniformBufferArray);
	// this.uniformValuesU32 = new Uint32Array(this.uniformBufferArray);
	this.bind_group_layout_entries.push(
		{
			binding: binding,
			visibility: GPUShaderStage.VERTEX | GPUShaderStage.FRAGMENT,
			buffer: {
				type: "uniform",
			},
		}
	)
	this.bind_group_entries.push(bind_group_entry(binding, this.buffer_uniform))
}


ViewWebGPU.prototype.setup_links = function (gravitle, binding) {
	this.buffer_links = create_buffer(this.device, 500000, gravitle.Link);
	this.bind_group_layout_entries.push(bind_group_layout_entry(binding))
	this.bind_group_entries.push(bind_group_entry(binding, this.buffer_links))
}


ViewWebGPU.prototype.setup_cells = function (gravitle, binding) {
	this.buffer_cells = create_buffer(this.device, 100000, gravitle.Cell);
	this.bind_group_layout_entries.push(bind_group_layout_entry(binding))
	this.bind_group_entries.push(bind_group_entry(binding, this.buffer_cells))
}


ViewWebGPU.prototype.setup_positions = function (gravitle, binding) {
	this.buffer_positions = create_buffer(this.device, 200000, gravitle.Point);
	this.bind_group_layout_entries.push(bind_group_layout_entry(binding))
	this.bind_group_entries.push(bind_group_entry(binding, this.buffer_positions))
}


ViewWebGPU.prototype.setup_materials = function (gravitle, binding) {
	this.buffer_materials = create_buffer(this.device, 40, gravitle.Material);
	this.bind_group_layout_entries.push(bind_group_layout_entry(binding))
	this.bind_group_entries.push(bind_group_entry(binding, this.buffer_materials))
}


ViewWebGPU.prototype.add_new_step = function (x) {
	const bindGroupLayout = this.device.createBindGroupLayout({
		entries: this.bind_group_layout_entries,
	});
	const pipelineLayout = this.device.createPipelineLayout({
		bindGroupLayouts: [bindGroupLayout],
	});
	const pipeline = this.device.createRenderPipeline({
		label: x.label,
		layout: pipelineLayout,
		vertex: {
			module: this.module,
			entryPoint: x.vertex_entryPoint,
		},
		fragment: {
			module: this.module,
			entryPoint: x.fragment_entryPoint,
			targets: [{ format: this.presentation_format }],
		},
		primitive: {
			// topology: "line-strip",
			topology: x.topology,
			cullMode: "back",
		},
		depthStencil: {
			depthWriteEnabled: true,
			depthCompare: "less",
			format: "depth24plus",
		},
	});
	this.steps.push({
		bindGroup: this.device.createBindGroup({
			layout: pipeline.getBindGroupLayout(0),
			entries: this.bind_group_entries,
		}),
		pipeline: pipeline,
	})
}


ViewWebGPU.prototype.setup = async function (gravitle) {
	this.resize();
	this.steps = []
	this.adapter = await navigator.gpu?.requestAdapter();
	this.device = await this.adapter?.requestDevice();
	this.presentation_format = navigator.gpu.getPreferredCanvasFormat();
	this.context.configure({
		device: this.device,
		format: this.presentation_format,
		alphaMode: "premultiplied",
	});
	const code = await (
		await fetch(`/slingshot/webgpu/code.wgsl`, { cache: "no-store" })
	).text();
	this.module = this.device.createShaderModule({
		label: "shaders",
		code: code,
	});
	this.bind_group_entries = []
	this.bind_group_layout_entries = []
	this.setup_cells(gravitle, 0)
	this.setup_materials(gravitle, 1)
	this.setup_positions(gravitle, 2)
	this.setup_uniform(3)
	this.setup_links(gravitle, 4)
	this.add_new_step({
		label: "pipeline_1",
		vertex_entryPoint: "vs_1",
		fragment_entryPoint: "fs_1",
		topology: "triangle-list",
	})
	this.add_new_step({
		label: "pipeline_0",
		vertex_entryPoint: "vs_0",
		fragment_entryPoint: "fs_0",
		topology: "triangle-list",
	})
	this.add_new_step({
		label: "pipeline_2",
		vertex_entryPoint: "vs_2",
		fragment_entryPoint: "fs_2",
		// topology: "line-list",
		topology: "triangle-strip",
	})
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


ViewWebGPU.prototype.render = function (worlds, gravitle, memory) {
	this.render_2(worlds, gravitle, memory)
}


ViewWebGPU.prototype.update_uniforms = function (tick) {
	const uniforms_html = document.getElementById("uniforms")
	if (uniforms_html) {
		uniforms_html.innerHTML = JSON.stringify( {
			zoom: this.zoom,
			link_line_width: this.link_line_width,
			tick: tick,
		}, null, 2)
	}	
	this.uniformValues.setFloat32(0, this.zoom, true); // offset 0: zoom (f32)
	this.uniformValues.setFloat32(4, this.link_line_width, true); // offset 4: line_width (f32)
	this.uniformValues.setUint32(8, tick, true); // offset 8: tick (u32)
}


ViewWebGPU.prototype.render_2 = function (worlds, gravitle, memory) {
	const canvas_texture = this.context.getCurrentTexture();
	this.update_uniforms(
		this.tick || worlds[0].get_tick()
	);
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

	let countr_links = 0;
	for (const world of worlds) {
		this.device.queue.writeBuffer(
			this.buffer_links,
			countr_links,
			memory.buffer,
			world.links(), 
			world.links_count() * gravitle.Link.size(), 
		);
		countr_links += world.links_count() * gravitle.Link.size();
	}

	this.device.queue.submit([
		(() => {
			const encoder = this.device.createCommandEncoder({ label: "encoder 1" });
			const pass = encoder.beginRenderPass(this.renderPassDescriptor);
			const step_positions = this.steps[0]
			pass.setPipeline(step_positions.pipeline);
			pass.setBindGroup(0, step_positions.bindGroup);
			pass.draw(
				// edge per cell model
				16 * 3,
				// total cell count
				worlds.reduce((sum, world) => sum + world.positions_count(), 0),
			);
			const linksStep = this.steps[2];
			pass.setPipeline(linksStep.pipeline);
			pass.setBindGroup(0, linksStep.bindGroup);
			pass.draw(
				// 4 vertices for a quad (triangle-strip)
				4,
				// Total number of links
				worlds.reduce((sum, world) => sum + world.links_count(), 0), 
			);
			const step_cells = this.steps[1]
			pass.setPipeline(step_cells.pipeline);
			pass.setBindGroup(0, step_cells.bindGroup);
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

// TODO: draw links
// links should be drawn using a segment like primitive, not triangles
