import { cyrb128, sfc32, random_seed } from "./random.js";
import { ship } from "./ship.js";
import { get_cell } from "./get_cell.js";
import { wrap_around } from "./math.js";
import { draw_cells, draw_ship_only } from "./draw_cells.js";
import { json_to_short, short_to_json } from "./misc.js";
const kinds = {
	armor: 0,
	booster: 1,
	core: 2,
};
const kb = {
	s: [],
	d: [],
};
function Game({ gravitle, memory, seed_input, stars_count, ghost, view }) {
	this.gravitle = gravitle;
	this.memory = memory;
	this.view = view;
	this.stars_count = stars_count;
	this.ghost = ghost;
	this.ghosts = [];
	this.seed = cyrb128(seed_input);
	this.seed_input = seed_input;
	this.last_frame = null;
	this.tick_starts = [];
	this.debug_durations = {
		engine: [],
		draw: [],
	}
	if (this.ghost) {
		this.ghosto = short_to_json(this.ghost);
	}
	if (this.ghosto) {
		this.ghosts.push({
			kind: "other",
			o: this.ghosto,
		});
	}
	ship.parts.forEach((e, idx) => {
		if (e.binding) {
			kb[e.binding].push(idx);
		}
	});
	this.key_bindings = new Map();
	for (let k of Object.keys(kb)) {
		if (!this.key_bindings.has(k)) {
			this.key_bindings.set(k, new Set());
		}
		for (let idx of kb[k]) {
			this.key_bindings.get(k).add(idx);
		}
	}
	document.addEventListener("keydown", (e) => {
		if (this.key_bindings.get(e.key)) {
			for (let idx of this.key_bindings.get(e.key)) {
				this.worlds[0].set_cell_activated(idx, 1);
			}
			this.started = true;
			document.getElementById("notice_text").classList.add("hide");
		}
		if (e.key == "e" && !this.trying_again) {
			this.trying_again = true;
			this.try_again();
		}
		if (e.key == "r" && !this.trying_again && this.victory_celebrated) {
			this.sharing = true;
			this.share();
		}
		if (e.key == " " && !this.trying_again && this.victory_celebrated) {
			const url = new URL(window.location.href);
			const url2 = new URL(url.origin + url.pathname);
			url2.searchParams.append("seed", random_seed());
			url2.searchParams.append("stars", this.stars_count);
			window.location.href = url2;
		}
		document.querySelectorAll(".disappearable").forEach((x, i) => {
			x.classList.remove("disappear");
		});
	});
	document.addEventListener("keyup", (e) => {
		if (this.key_bindings.get(e.key)) {
			for (let idx of this.key_bindings.get(e.key)) {
				this.worlds[0].set_cell_activated(idx, 0);
			}
		}
		if (e.key == "e") {
			this.trying_again = false;
		}
		if (e.key == "r") {
			this.sharing = false;
		}
	});
	document.getElementById("share").addEventListener("click", () => {
		this.share();
	});
	this.restart();
	this.tick();
}
Game.prototype.share = function () {
	const url = new URL(window.location.href);
	const url2 = new URL(url.origin + url.pathname);
	url2.searchParams.append("seed", this.seed_input);
	url2.searchParams.append("stars", this.stars_count);
	this.ghost_to_share = json_to_short(
		JSON.parse(this.worlds[0].get_activation_events()),
	);
	url2.searchParams.append("ghost", this.ghost_to_share);
	const share_link = url2.href;
	navigator.clipboard
		.writeText(share_link)
		.then(() => {
			document.getElementById("share_1").classList.add("hide");
			document.getElementById("share_2").classList.remove("hide");
		})
		.catch((err) => {
			console.error("Failed to copy text: ", err);
		});
};
Game.prototype.celebrate_victory = function () {
	this.victory_celebrated = 1;
	document.getElementById("victory").classList.add("yes");
	document.getElementById("vic_text").classList.remove("hide");
	this.ghost_to_share = json_to_short(
		JSON.parse(this.worlds[0].get_activation_events()),
	);
};
Game.prototype.tick = function () {
	const now = performance.now();
	this.tick_starts.push(now);
	while (this.tick_starts.length > 200) {
		this.tick_starts.shift();
	}
	const fps =
		1000 /
		((this.tick_starts[this.tick_starts.length - 1] - this.tick_starts[0]) /
			(this.tick_starts.length - 1));
	document.getElementById("fps").innerHTML = Math.round(fps);
	const elapsed = now - this.last_frame;
	let steps = 2;
	if (elapsed > 15 && this.last_frame != null) {
		steps = 4;
	}
	this.last_frame = now;
	this.debug_durations.engine.push({
		start: performance.now(),
	})
	for (let index = 0; index < steps; index++) {
		for (let i = 1; i < this.worlds.length; i++) {
			const world = this.worlds[i];
			const ghost = this.ghosts[i - 1].o;
			const es = ghost[world.step];
			if (es != null && world.step > this.switch_cell_limit) {
				for (const e of es) {
					world.switch_cell_activated(e.c);
				}
			}
		}
		this.switch_cell_limit = this.worlds[0].step;
		if (this.started) {
			for (const world of this.worlds) {
				world.run_step();
			}
		}
		if (this.worlds[0].victory == 1 && !this.victory_celebrated) {
			this.celebrate_victory();
		}
	}
	this.debug_durations.engine.at(-1).duration = performance.now() - this.debug_durations.engine.at(-1).start
	this.debug_durations.draw.push({
		start: performance.now(),
	})
	this.view.set_backgound("#102");
	for (let i = 1; i < this.worlds.length; i++) {
		if (this.ghosts[i - 1].kind == "me") {
			draw_ship_only(
				this.gravitle,
				this.worlds[i],
				this.memory,
				this.view,
				"g",
			);
		}
	}
	for (let i = 1; i < this.worlds.length; i++) {
		if (this.ghosts[i - 1].kind != "me") {
			draw_ship_only(
				this.gravitle,
				this.worlds[i],
				this.memory,
				this.view,
				"o",
			);
		}
	}
	draw_cells(this.gravitle, this.worlds[0], this.memory, this.view);
	this.debug_durations.draw.at(-1).duration = performance.now() - this.debug_durations.draw.at(-1).start
	if (this.victory_celebrated) {
		const durations = [];
		for (const world of this.worlds) {
			if (world.victory_end) {
				durations.push(world.victory_end);
			}
		}
		const duration_0 = this.worlds[0].victory_end;
		const durations_left = durations.filter((x) => x < duration_0);
		const durations_right = durations.filter((x) => x > duration_0);
		durations_left.sort((a, b) => b - a);
		durations_right.sort((a, b) => a - b);
		let durations_left_str = durations_left.join(" · ");
		if (durations_left_str) {
			durations_left_str = " · " + durations_left_str;
		}
		let durations_right_str = durations_right.join(" · ");
		if (durations_right_str) {
			durations_right_str = " · " + durations_right_str;
		}
		document.getElementById("victory_duration").innerHTML = duration_0;
		document.getElementById("victory_duration_lower").innerHTML =
			durations_left_str;
		document.getElementById("victory_duration_higher").innerHTML =
			durations_right_str;
	}
	while (this.debug_durations.engine.length > 200) {
		this.debug_durations.engine.shift();
	}
	while (this.debug_durations.draw.length > 200) {
		this.debug_durations.draw.shift();
	}
	const debug_durations_engine = this.debug_durations.engine.reduce((sum, obj) => sum + obj.duration, 0) / this.debug_durations.engine.length;
	const debug_durations_draw = this.debug_durations.draw.reduce((sum, obj) => sum + obj.duration, 0) / this.debug_durations.draw.length;

	document.getElementById("debug_durations_engine").innerHTML = debug_durations_engine.toFixed(5)
	document.getElementById("debug_durations_draw").innerHTML = debug_durations_draw.toFixed(5)
	requestAnimationFrame(() => {
		this.tick();
	});
};
Game.prototype.restart = function () {
	this.victory_celebrated = false;
	this.started = false;
	this.switch_cell_limit = -1;
	this.worlds = [this.gravitle.World.new()];
	for (const _ghost of this.ghosts) {
		this.worlds.push(this.gravitle.World.new());
	}
	for (const world of this.worlds) {
		ship.parts.forEach((e, idx) => {
			world.add_cell(e.p.x - 0.3, e.p.y - 0.3, e.d, kinds[e.kind]);
		});
		const rand = sfc32(this.seed[0], this.seed[1], this.seed[2], this.seed[3]);
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
				const cell_size = this.gravitle.Cell.size();
				const cells_view = new DataView(
					this.memory.buffer,
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
		for (let index = 0; index < this.stars_count; index++) {
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
				const cell_size = this.gravitle.Cell.size();
				const cells_view = new DataView(
					this.memory.buffer,
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
};
Game.prototype.try_again = function () {
	this.ghosts.push({
		kind: "me",
		o: JSON.parse(this.worlds[0].get_activation_events()),
	});
	this.ghost_to_share = null;
	document.getElementById("victory").classList.remove("yes");
	this.restart();
};
export { Game };
