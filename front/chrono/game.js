import { cyrb128, random_seed } from "./random.js";
import { ship } from "./ship.js";
import { json_to_short, short_to_json } from "./misc.js";
import { game_setup } from "./game_setup.js";
import { Audio } from "./sound.js";
const kb = {
	s: [],
	d: [],
};
function Game({ gravitle, memory, seed_input, stars_count, ghost, view }) {
	console.log("game - start");
	this.gravitle = gravitle;
	this.memory = memory;
	this.view = view;
	this.stars_count = stars_count;
	this.ghost = ghost;
	this.ghosts = [];
	this.audio = null;
	this.seed = cyrb128(seed_input);
	this.seed_input = seed_input;
	this.last_frame = null;
	this.tick_starts = [];
	this.debug_durations = {
		engine: [],
		draw: [],
	};
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
	document.addEventListener("keydown", async (e) => {
		if (this.audio === null) {
			this.audio = new Audio();
			await this.audio.setup();
		}
		if (this.key_bindings.get(e.key)) {
			for (let idx of this.key_bindings.get(e.key)) {
				this.worlds[0].set_cell_activated(idx, 1);
				this.audio.activate(idx);
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
			this.new_level();
		}
	});
	document.addEventListener("keyup", (e) => {
		if (this.key_bindings.get(e.key)) {
			for (let idx of this.key_bindings.get(e.key)) {
				this.worlds[0].set_cell_activated(idx, 0);
				this.audio.deactivate(idx);
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
	document.getElementById("again").addEventListener("click", () => {
		this.try_again();
	});
	document.getElementById("new").addEventListener("click", () => {
		this.new_level();
	});
	this.restart();
	this.tick();
}
Game.prototype.new_level = function () {
	const url = new URL(window.location.href);
	const url2 = new URL(url.origin + url.pathname);
	url2.searchParams.append("seed", random_seed());
	url2.searchParams.append("stars", this.stars_count);
	window.location.href = url2;
};
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
	});
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
	this.debug_durations.engine.at(-1).duration =
		performance.now() - this.debug_durations.engine.at(-1).start;
	this.debug_durations.draw.push({
		start: performance.now(),
	});
	this.view.render(this.worlds, this.ghosts, this.gravitle, this.memory);
	this.debug_durations.draw.at(-1).duration =
		performance.now() - this.debug_durations.draw.at(-1).start;
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
		//
		const fuel_useds = [];
		for (const world of this.worlds) {
			if (world.victory_end) {
				fuel_useds.push(world.victory_fuel_used);
			}
		}
		const fuel_used_0 = this.worlds[0].victory_fuel_used;
		const fuel_useds_left = fuel_useds.filter((x) => x < fuel_used_0);
		const fuel_useds_right = fuel_useds.filter((x) => x > fuel_used_0);
		fuel_useds_left.sort((a, b) => b - a);
		fuel_useds_right.sort((a, b) => a - b);
		let fuel_useds_left_str = fuel_useds_left.join(" · ");
		if (fuel_useds_left_str) {
			fuel_useds_left_str = " · " + fuel_useds_left_str;
		}
		let fuel_useds_right_str = fuel_useds_right.join(" · ");
		if (fuel_useds_right_str) {
			fuel_useds_right_str = " · " + fuel_useds_right_str;
		}
		document.getElementById("fuel_used_me").innerHTML = fuel_used_0;
		document.getElementById("fuel_used_lower").innerHTML = fuel_useds_left_str;
		document.getElementById("fuel_used_higher").innerHTML =
			fuel_useds_right_str;
	}
	while (this.debug_durations.engine.length > 200) {
		this.debug_durations.engine.shift();
	}
	while (this.debug_durations.draw.length > 200) {
		this.debug_durations.draw.shift();
	}
	const debug_durations_engine =
		this.debug_durations.engine.reduce((sum, obj) => sum + obj.duration, 0) /
		this.debug_durations.engine.length;
	const debug_durations_draw =
		this.debug_durations.draw.reduce((sum, obj) => sum + obj.duration, 0) /
		this.debug_durations.draw.length;
	try {
		document.getElementById("debug_durations_engine").innerHTML =
			debug_durations_engine.toFixed(5);
		document.getElementById("debug_durations_draw").innerHTML =
			debug_durations_draw.toFixed(5);
	} catch (error) {}
	requestAnimationFrame(() => {
		this.tick();
	});
};
Game.prototype.setup = game_setup;
Game.prototype.restart = function () {
	this.victory_celebrated = false;
	this.started = false;
	this.switch_cell_limit = -1;
	this.worlds = [this.gravitle.World.new()];
	for (const _ghost of this.ghosts) {
		this.worlds.push(this.gravitle.World.new());
	}
	this.setup();
};
Game.prototype.try_again = function () {
	this.ghosts.push({
		kind: "me",
		o: JSON.parse(this.worlds[0].get_activation_events()),
	});
	this.ghost_to_share = null;
	document.getElementById("victory").classList.remove("yes");
	document.getElementById("vic_text").classList.add("hide");
	this.restart();
};
export { Game };
