import init, * as gravitle from "../chrono/gravitle_time_trial.js";
import { test } from "../chrono/test.js";
import { View2d } from "../chrono/view_2d.js";
import { ViewWebGPU } from "../chrono/view_webgpu.js";
import { Game } from "../chrono/game.js";
import { ship } from "./logo.js"

const main = async () => {
	await init();
	gravitle.setup();
	const memory = gravitle.initSync().memory;
	test(gravitle, memory);
	const url = new URL(window.location.href);
	const params = new URLSearchParams(url.search);
	const seed_input =
		params.get("seed") ?? new Date().toISOString().split("T")[0];
	const ghost = params.get("ghost");
	const render_mode = params.get("render") ?? "2d";
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
    view.zoom = 24
    view.background = "#0000"
	new Game({
		gravitle,
		memory,
		seed_input,
		stars_count: 0,
		ghost,
		view,
        ship,
		asteroid_count: 0,
	});
};
window.onload = () => {
	main();
};
