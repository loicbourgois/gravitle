import init, * as gravitle from "./gravitle_time_trial.js";
import { test } from "./test.js";
import { View } from "./view.js";
import { test_wrap_around } from "./math.js";
import { Game } from "./game.js";
import { defaults } from "./defaults.js";
const main = async () => {
	test_wrap_around();
	await init();
	gravitle.setup();
	const memory = gravitle.initSync().memory;
	test(gravitle, memory);
	const url = new URL(window.location.href);
	const params = new URLSearchParams(url.search);
	const stars_count = params.get("stars") ?? defaults.stars_count;
	const seed_input =
		params.get("seed") ?? new Date().toISOString().split("T")[0];
	let ghost = params.get("ghost");
	const view = new View("canvas");
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
