import init, * as gravitle from "../slingshot/gravitle_slingshot.js";
import { ViewWebGPU } from "../slingshot/view_webgpu.js";
import { setup_1 } from "./setup_1.js";
import { has_webgpu_support } from "../slingshot/misc.js"
const main = async () => {
	try {
		const start = performance.now()
		await init();
		await has_webgpu_support();
		const wasm_engine = gravitle
		const wasm_memory = gravitle.initSync().memory
		const world_1 = wasm_engine.setup();
		const world_2 = wasm_engine.setup();
		const world_3 = wasm_engine.setup();
		await setup_1(
			world_1,
			0.0116,
			0.01121
		)
		await setup_1(
			world_2,
			0.0116,
			0.011201
		)
		await setup_1(
			world_3,
			0.0116,
			0.0112
		)
		const view = new ViewWebGPU("canvas")
		view.set_zoom(5)
		world_1.save_positions(6)
		world_3.save_positions(6)
		const aa = 10000
		world_1.tick_n(aa)
		world_3.tick_n(aa)
		world_2.save_positions(6)
		for (let index = 0; index < aa; index++) {
			document.getElementById("info").innerHTML = index
			world_2.tick_n(1)
		}
		await view.setup(wasm_engine);
		view.render_2(
			[
				world_1, 
				world_2,
				world_3,
			],
			wasm_engine, 
			wasm_memory,
		);
		const duration = performance.now() - start
		document.getElementById("info").innerHTML = `duration ${duration.toFixed(2)} ms`	
	} catch (error) {
		document.getElementById("info").innerHTML = `error: ${error}`	
		console.log(error)
	}
};
main();
