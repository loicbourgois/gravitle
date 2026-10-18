import { cyrb128, random_seed, sfc32 } from "./random.js";
import { json_to_short, short_to_json } from "./misc.js";
import { game_setup } from "./game_setup.js";
import { Audio } from "./sound.js";
const kb = {
	s: [],
	d: [],
};
function Game({
	gravitle,
	memory,
	seed_input,
	stars_count,
	ghost,
	view,
	ship,
	asteroid_count,
}) {
	console.log("game - start");
	this.gravitle = gravitle;
	this.ship = ship;
	this.asteroid_count = asteroid_count;
	this.memory = memory;
	this.view = view;
	this.stars_count = stars_count;
	this.ghost = ghost;
	this.ghosts = [];
	this.audio = null;
	this.seed = cyrb128(seed_input);
	console.log(`seed_input: ${seed_input}`);
	console.log(`seed: ${this.seed}`);
	let sfc32_ = sfc32(this.seed[0], this.seed[1], this.seed[2], this.seed[3]);
	let r1 = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"[parseInt(sfc32_() * 26 * 100) % 26];
	let r2 = "0123456789"[parseInt(sfc32_() * 10 * 100) % 10];
	let r3 = "0123456789"[parseInt(sfc32_() * 10 * 100) % 10];
	let r4 = "0123456789"[parseInt(sfc32_() * 10 * 100) % 10];
	console.log(r1, r2, r3, r4);
	try {
		document.getElementById("notice_text_2_inner_2").innerHTML =
			`Sector ${r1}${r2}${r3}-${r4} needs your help.`;
	} catch {}
	this.seed_input = seed_input;
	this.last_frame = null;
	this.tick_starts = [];
	this.debug_durations = {
		engine: [],
		draw: [],
	};
	this.debug_counts = {
		pairs: [],
		colliding: [],
		colliding_physics: [],
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
	const booster_ids = [];
	ship.parts.forEach((e, idx) => {
		if (e.binding) {
			kb[e.binding].push(idx);
		}
		if (e.kind === "booster") {
			booster_ids.push(idx);
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
			await this.audio.setup(booster_ids);
		}
		if (this.key_bindings.get(e.key)) {
			for (let idx of this.key_bindings.get(e.key)) {
				this.worlds[0].set_particle_activated(idx, 1);
				this.audio.activate(idx);
			}
			this.started = true;
			document.getElementById("notice_text").classList.add("hide");
			try {
				document.getElementById("notice_text_2").classList.add("hide");
			} catch {}
			document.getElementById("overlay").classList.add("light");
		}
		if (e.key == "a" && !this.trying_again) {
			this.trying_again = true;
			this.try_again();
		}
		if (e.key == "c" && !this.trying_again && this.victory_celebrated) {
			this.sharing = true;
			this.share();
		}
		if (e.key == " " && !this.trying_again && this.victory_celebrated) {
			this.new_level();
		}
		if (e.key == "m" && !this.switching_mode && this.victory_celebrated) {
			this.switching_mode = true;
			this.view.darkmode = !this.view.darkmode;
			const url = new URL(window.location.href);
			url.searchParams.set("darkmode", String(this.view.darkmode));
			window.history.replaceState(null, "", url);
		}
	});
	document.addEventListener("keyup", (e) => {
		if (this.key_bindings.get(e.key)) {
			for (let idx of this.key_bindings.get(e.key)) {
				this.worlds[0].set_particle_activated(idx, 0);
				this.audio.deactivate(idx);
			}
		}
		if (e.key == "a") {
			this.trying_again = false;
		}
		if (e.key == "c") {
			this.sharing = false;
		}
		if (e.key == "m") {
			this.switching_mode = false;
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
	url2.searchParams.append("darkmode", String(this.view.darkmode));
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
	document.getElementById("vic_text_2").classList.remove("hide");
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
	let budget = 1000 / 120;
	if (elapsed > 15 && this.last_frame != null) {
		steps = 4;
		budget = 1000 / 60;
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
			if (es != null && world.step > this.switch_particle_limit) {
				for (const e of es) {
					world.switch_particle_activated(e.c);
				}
			}
		}
		this.switch_particle_limit = this.worlds[0].step;
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

	// Debug push
	this.debug_counts.pairs.push(this.worlds[0].count_pairs);
	this.debug_counts.colliding.push(this.worlds[0].count_colliding);
	this.debug_counts.colliding_physics.push(
		this.worlds[0].count_colliding_physics,
	);

	// Debug shift
	while (this.debug_durations.engine.length > 200) {
		this.debug_durations.engine.shift();
	}
	while (this.debug_durations.draw.length > 200) {
		this.debug_durations.draw.shift();
	}
	while (this.debug_counts.pairs.length > 200) {
		this.debug_counts.pairs.shift();
	}
	while (this.debug_counts.colliding.length > 200) {
		this.debug_counts.colliding.shift();
	}
	while (this.debug_counts.colliding_physics.length > 200) {
		this.debug_counts.colliding_physics.shift();
	}

	// Debug avg
	const debug_durations_engine =
		this.debug_durations.engine.reduce((sum, obj) => sum + obj.duration, 0) /
		this.debug_durations.engine.length;
	const debug_durations_draw =
		this.debug_durations.draw.reduce((sum, obj) => sum + obj.duration, 0) /
		this.debug_durations.draw.length;

	const frame_percent_budget =
		((debug_durations_engine + debug_durations_draw) / budget) * 100;

	const debug_counts_pairs =
		this.debug_counts.pairs.reduce((sum, obj) => sum + obj, 0) /
		this.debug_counts.pairs.length;
	const debug_counts_colliding =
		this.debug_counts.colliding.reduce((sum, obj) => sum + obj, 0) /
		this.debug_counts.colliding.length;
	const debug_counts_colliding_physics =
		this.debug_counts.colliding_physics.reduce((sum, obj) => sum + obj, 0) /
		this.debug_counts.colliding_physics.length;

	// Debug print
	try {
		document.getElementById("frame_percent_budget").innerHTML =
			"frame budget: " + parseInt(frame_percent_budget) + "%";

		document.getElementById("debug_durations_engine").innerHTML =
			"physic: " + debug_durations_engine.toFixed(1) + "ms";
		document.getElementById("debug_durations_draw").innerHTML =
			"render: " + debug_durations_draw.toFixed(1) + "ms";
		document.getElementById("debug_pairs").innerHTML =
			"pairs: " + this.worlds[0].pairs_count();

		document.getElementById("debug_counts_pairs").innerHTML =
			"pairs: " + parseInt(debug_counts_pairs);
		document.getElementById("debug_counts_colliding").innerHTML =
			"colliding: " + parseInt(debug_counts_colliding);
		document.getElementById("debug_counts_colliding_physics").innerHTML =
			"colliding_physics: " + parseInt(debug_counts_colliding_physics);

		document.getElementById("debug_elapsed").innerHTML =
			"elapsed: " + parseInt(elapsed) + "ms";
	} catch (error) {
		// pass
	}

	// Next
	requestAnimationFrame(() => {
		this.tick();
	});
};

Game.prototype.setup = game_setup;
Game.prototype.restart = function () {
	this.sharing = false;
	document.getElementById("share_1").classList.remove("hide");
	document.getElementById("share_2").classList.add("hide");
	this.victory_celebrated = false;
	this.started = false;
	this.switch_particle_limit = -1;
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
	document.getElementById("vic_text_2").classList.add("hide");
	this.restart();
};
export { Game };
