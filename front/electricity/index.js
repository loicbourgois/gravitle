import init, * as gravitle from "../slingshot/gravitle_slingshot.js";
import { ViewWebGPU } from "../slingshot/view_webgpu.js";
import { setup } from "./setup.js";
import { has_webgpu_support } from "../slingshot/misc.js"
const main = async () => {
	try {
		const start = performance.now()
		await init();
		await has_webgpu_support();
		const wasm_engine = gravitle
		const wasm_memory = gravitle.initSync().memory
		const world_1 = wasm_engine.setup();
		await setup(
			world_1,
		)
		const view = new ViewWebGPU("canvas")
		view.set_zoom(5.0)
		world_1.tick_n(1)
		await view.setup(wasm_engine);
		view.render_2(
			[
				world_1, 
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
