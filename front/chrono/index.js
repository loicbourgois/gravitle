import init, * as gravitle from "./gravitle_time_trial.js";
import { test } from "./test.js";
import { View } from "./view.js";
import { cyrb128, sfc32, random_seed } from "./random.js";
import { test_wrap_around } from "./math.js";
import { short_to_json } from "./misc.js";
import { Game } from "./game.js";


// let ghost_json = null;
let share_link = null;
let seed_input
let stars_count
let ghost
// const context = {
// 	ghost_to_share: null,
// };


const try_again = () => {
	let ref = `./?seed=${seed_input}&stars=${stars_count}`;
	if (ghost) {
		ref += `&ghost=${ghost}`;
	}
	window.location.href = ref;
}


const getQueryParam = (params, param) => {
	return params.get(param);
}


const restart = (memory) => {
	
	
	document.getElementById("again").addEventListener("click", () => {
		try_again()
	});
	document.getElementById("new").addEventListener("click", () => {
		window.location.href = `./?seed=${random_seed()}&stars=${stars_count}`;
	});
	document.getElementById("share").addEventListener("click", () => {
		const fullUrl = window.location.href;
		const url = new URL(fullUrl);
		const url2 = new URL(url.origin + url.pathname);
		url2.searchParams.append("seed", seed_input);
		url2.searchParams.append("stars", stars_count);
		url2.searchParams.append("ghost", context.ghost_to_share);
		share_link = url2.href;
		console.log(share_link);
		navigator.clipboard
			.writeText(share_link)
			.then(() => {
				console.log("Text copied to clipboard");
			})
			.catch((err) => {
				console.error("Failed to copy text: ", err);
			});
	});

	// document.addEventListener("click", (event) => {
	// 	const x = event.clientX;
	// 	const y = event.clientY;
	// 	view.set_mouse(x, y);
	// });
	// document.addEventListener("mousemove", (e) => {
	// 	const x = e.offsetX;
	// 	const y = e.offsetY;
	// 	view.set_mouse(x, y);
	// });
	// document
	// 	.getElementById("update_config")
	// 	.addEventListener("click", () => update_config(world));
	// update_config(world);
	
}


const main = async () => {
	test_wrap_around();
	await init();
	gravitle.setup();
	const memory = gravitle.initSync().memory;
	test(gravitle, memory);
	const url = new URL(window.location.href);
	const params = new URLSearchParams(url.search);
	seed_input = getQueryParam(params, "seed");
	if (seed_input == null) {
		seed_input = "gravitle";
	}
	stars_count = getQueryParam(params, "stars");
	if (stars_count == null) {
		stars_count = 4;
	}
	ghost = getQueryParam(params, "ghost");
	if (ghost != null) {
		ghost_json = short_to_json(ghost);
	}
	const view = new View("canvas");
	window.addEventListener("resize", function () {
		view.resize();
	});
	const game = new Game({
		gravitle, 
		memory, 
		seed_input,
		stars_count,
		ghost,
		view,
	})
	// game.start()
};
window.onload = () => {
	main();
};
