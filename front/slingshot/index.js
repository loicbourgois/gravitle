import init, * as gravitle from "./gravitle_slingshot.js";
import { ViewWebGPU } from "./view_webgpu.js";
import { Game } from "./game.js";
import { has_webgpu_support } from "./misc.js"
import { setup } from "./setup.js"
const main = async () => {
	await init();
	await has_webgpu_support();
	const view = new ViewWebGPU("canvas");
	view.set_zoom(5);
	(
		await new Game({
			wasm_memory: gravitle.initSync().memory,
			wasm_engine: gravitle,
			view: view,
			setup: setup,
		}).setup()
	).start();
};
main();
