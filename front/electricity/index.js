import init, * as gravitle from "../slingshot/gravitle_slingshot.js";
import { ViewWebGPU } from "../slingshot/view_webgpu.js";
import { has_webgpu_support } from "../slingshot/misc.js"
import { Game } from "../slingshot/game.js";
import { setup } from "./setup.js";
import { padn } from "../slingshot/misc.js";


const setup_slider = (id, min, max, initialValue) => {
	const input = document.getElementById(`${id}-slider-input`)
	const value = document.getElementById(`${id}-slider-value`)
	input.min = min;
	input.max = max;
	input.value = initialValue;
	const update = () => {
		value.textContent = padn(input.value, 5);
	}
	input.addEventListener("input",  update)
	update()
}


const get_slider_value = (id) => {
	const input = document.getElementById(`${id}-slider-input`)
	return input.value
}


const main = async () => {
	await init();
	await has_webgpu_support();
	const view = new ViewWebGPU("canvas");
	view.set_zoom(28);
	const game = await new Game({
		wasm_memory: gravitle.initSync().memory,
		wasm_engine: gravitle,
		view: view,
		setup: setup,
	}).setup();
	setup_slider("tick", 0, 1000, 999)
	setup_slider("zoom", 0, 40, 28)
	const render_loop = () => {
		game.render()
		requestAnimationFrame(() => {
			render_loop();
		});
	};
	const slider_loop = () => {
		// view.tick = get_slider_value("tick")
		view.set_zoom(get_slider_value("zoom"))
		requestAnimationFrame(() => {
			slider_loop();
		});
		document.getElementById("debug").innerHTML = game.worlds[0].debug_storage()
	}
	game.start()
	slider_loop()
	// render_loop()
	// game.worlds[0].tick_n(1000)
	// game.worlds[0].tick()
	// game.worlds[0].tick()
};


main();
