import init, * as gravitle from "./gravitle_slingshot.js";
import { ViewWebGPU } from "./view_webgpu.js";
import { Game } from "./game.js";
import { has_webgpu_support } from "./misc.js"
const main = async () => {
	await init();
	await has_webgpu_support();
	(
		await new Game({
			wasm_memory: gravitle.initSync().memory,
			wasm_engine: gravitle,
			view: new ViewWebGPU("canvas"),
		}).setup()
	).start();
};
main();
