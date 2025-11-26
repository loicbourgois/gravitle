import init, * as gravitle from "../slingshot/gravitle_slingshot.js";
import { ViewWebGPU } from "../slingshot/view_webgpu.js";
// import { Game } from "../slingshot/game.js";
import { setup_1 } from "./setup_1.js";
import { has_webgpu_support } from "../slingshot/misc.js"
const main = async () => {
	await init();
	await has_webgpu_support();
	const wasm_engine = gravitle
	const wasm_memory = gravitle.initSync().memory
	const world_1 = wasm_engine.setup();
	const world_2 = wasm_engine.setup();
	await setup_1(
		world_1, 
		0.0116, 
		0.011
	)
	await setup_1(
		world_2, 
		0.0116, 
		0.0113
	)
	const view = new ViewWebGPU("canvas")
	world_1.save_positions(6)
	world_1.tick_n(8000)
	world_2.save_positions(6)
	world_2.tick_n(8000)
	const ps = world_1.get_positions(6)
	console.log(ps[0].x)
	await view.setup(wasm_engine);
	view.render([world_1, world_2], wasm_engine, wasm_memory);
	view.render_2(
		[world_1, world_2],
		wasm_engine, 
		wasm_memory,
	);
};
main();
