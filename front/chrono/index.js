import init, * as gravitle from "./gravitle_time_trial.js";
import { test } from "./test.js";
import { View2d } from "./view_2d.js";
import { ViewWebGPU } from "./view_webgpu.js";
import { Game } from "./game.js";
import { defaults } from "./defaults.js";

const has_webgpu_support = async () => {
	const adapter = await navigator.gpu?.requestAdapter();
	const device = await adapter?.requestDevice();
	if (device) {
		return true;
	} else {
		return false;
	}
};

const main = async () => {
	await init();
	gravitle.setup();
	const memory = gravitle.initSync().memory;
	test(gravitle, memory);
	const url = new URL(window.location.href);
	const params = new URLSearchParams(url.search);
	const stars_count = params.get("stars") ?? defaults.stars_count;
	const seed_input =
		params.get("seed") ?? new Date().toISOString().split("T")[0];
	const ghost = params.get("ghost");
	const render_mode =
		params.get("render") ??
		{
			[true]: "webgpu",
			[false]: "2d",
		}[await has_webgpu_support()];
	// console.log("render_mode", render_mode)
	const view = await {
		"2d": () => {
			return new View2d("canvas");
		},
		webgpu: async () => {
			const v = new ViewWebGPU("canvas");
			await v.setup(gravitle);
			return v;
		},
	}[render_mode]();
	window.addEventListener("resize", function () {
		view.resize();
	});
	new Game({
		gravitle,
		memory,
		seed_input,
		stars_count,
		ghost,
		view,
	});
};
window.onload = () => {
	main();
};
