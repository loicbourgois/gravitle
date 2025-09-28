import init, * as gravitle from "./gravitle_time_trial.js";
import { test } from "./test.js";
import { View } from "./view.js";
import { ship_1 } from "./ship_1.js";
import { ship_2 } from "./ship_2.js";
import { get_cell } from "./get_cell.js";
import { wrap_around, test_wrap_around } from "./math.js";
import { cyrb128, sfc32, random_seed } from "./random.js";
import { short_to_json } from "./misc.js";
import { tick } from "./tick.js";

const ship = ship_2;
let ghost_json = null;
let share_link = null;
let key_bindings = new Map();
const kinds = {
	armor: 0,
	booster: 1,
	core: 2,
};
const kb = {
	s: [],
	d: [],
};
const context = {
	ghost_to_share: null,
};

const main = async () => {
	test_wrap_around();
	await init();
	gravitle.setup();
	const memory = gravitle.initSync().memory;
	test(gravitle, memory);

	const url = new URL(window.location.href);
	const params = new URLSearchParams(url.search);
	function getQueryParam(param) {
		return params.get(param);
	}
	let seed_input = getQueryParam("seed");
	if (seed_input == null) {
		seed_input = "gravitle";
	}
	let stars_count = getQueryParam("stars");
	if (stars_count == null) {
		stars_count = 4;
	}
	let ghost = getQueryParam("ghost");
	if (ghost != null) {
		ghost_json = short_to_json(ghost);
	}
	const seed = cyrb128(seed_input);
	const worlds = [gravitle.World.new(), gravitle.World.new()];
	for (const world of worlds) {
		ship.parts.forEach((e, idx) => {
			world.add_cell(e.p.x - 0.3, e.p.y - 0.3, e.d, kinds[e.kind]);
			if (e.binding) {
				kb[e.binding].push(idx);
			}
		});
		const rand = sfc32(seed[0], seed[1], seed[2], seed[3]);
		for (let index = 0; index < 20; index++) {
			const diameter = 0.05;
			let iterations = 0;
			while (true) {
				iterations += 1;
				if (iterations > 100) {
					throw "too many iterations";
				}
				const x = rand();
				const y = rand();
				const cells_ptr = world.cells();
				const cell_size = gravitle.Cell.size();
				const cells_view = new DataView(
					memory.buffer,
					cells_ptr,
					world.cells_count() * cell_size,
				);
				let ok = true;
				for (let i = 0; i < world.cells_count(); i++) {
					const cell = get_cell(cells_view, cell_size, i);
					const wa = wrap_around(cell.p, { x, y });
					let diams = cell.diameter + diameter * 1.5;
					let colliding = wa.d_sqrd < diams * diams;
					if (colliding) {
						ok = false;
						break;
					}
				}
				if (ok) {
					world.add_cell(x, y, diameter, 4);
					break;
				}
			}
		}
		for (let index = 0; index < stars_count; index++) {
			const diameter = 0.015;
			let iterations = 0;
			while (true) {
				iterations += 1;
				if (iterations > 200) {
					throw "too many iterations";
				}
				const x = rand();
				const y = rand();
				const cells_ptr = world.cells();
				const cell_size = gravitle.Cell.size();
				const cells_view = new DataView(
					memory.buffer,
					cells_ptr,
					world.cells_count() * cell_size,
				);
				let ok = true;
				for (let i = 0; i < world.cells_count(); i++) {
					const cell = get_cell(cells_view, cell_size, i);
					const wa = wrap_around(cell.p, { x: x, y: y });
					let diams = cell.diameter * 0.5 + diameter * 5;
					let colliding = wa.d_sqrd < diams * diams;
					if (colliding) {
						ok = false;
						break;
					}
				}
				if (ok) {
					world.add_cell(x, y, diameter, 5);
					break;
				}
			}
		}
		for (const l of ship.links) {
			world.add_link(l.a, l.b);
		}
	}
	for (let k of Object.keys(kb)) {
		if (!key_bindings.has(k)) {
			key_bindings.set(k, new Set());
		}
		for (let idx of kb[k]) {
			key_bindings.get(k).add(idx);
		}
	}
	const view = new View("canvas");
	window.addEventListener("resize", function () {
		view.resize();
	});
	document.addEventListener("keydown", (e) => {
		if (key_bindings.get(e.key)) {
			for (let idx of key_bindings.get(e.key)) {
				worlds[0].set_cell_activated(idx, 1);
			}
		}
		document.querySelectorAll(".disappearable").forEach((x, i) => {
			x.classList.remove("disappear");
		});
	});
	document.addEventListener("keyup", (e) => {
		if (key_bindings.get(e.key)) {
			for (let idx of key_bindings.get(e.key)) {
				worlds[0].set_cell_activated(idx, 0);
			}
		}
	});
	document.getElementById("again").addEventListener("click", () => {
		let ref = `../gravitle-time-trial?seed=${seed_input}&stars=${stars_count}`;
		if (ghost) {
			ref += `&ghost=${ghost}`;
		}
		window.location.href = ref;
	});
	document.getElementById("new").addEventListener("click", () => {
		window.location.href = `../gravitle-time-trial?seed=${random_seed()}&stars=${stars_count}`;
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
	tick(gravitle, view, worlds, memory, ghost_json, context);
};
window.onload = () => {
	main();
};
