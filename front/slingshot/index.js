import init, * as gravitle from "./gravitle_slingshot.js";
import { fetch_as_text, fetch_as_json_string } from "./fetch.js";
import { ViewWebGPU } from "./view_webgpu.js";

const has_webgpu_support = async () => {
	const adapter = await navigator.gpu?.requestAdapter();
	const device = await adapter?.requestDevice();
	if (device) {
		return true;
	} else {
		return false;
	}
};

function Game({ wasm_memory, wasm_engine, view }) {
	this.wasm_memory = wasm_memory;
	this.wasm_engine = wasm_engine;
	this.view = view;
	this.tick_starts = [];
	this.render_durations = [];
	this.tick_durations = [];
	this.stats_durations = [];
	this.logic_durations = [];
}

Game.prototype.setup = async function () {
	await this.view.setup(this.wasm_engine);
	const world = this.wasm_engine.setup();
	for (const url of [
		"/slingshot/material/steel.json",
		"/slingshot/material/launcher.json",
	]) {
		world.add_material(url, await fetch_as_json_string(url));
	}
	world.add_from_blueprint(
		await fetch_as_text("/slingshot/blueprint/slingshot/material.txt"),
		0.0,
		0.0,
	);
	this.worlds = [world];
	return this;
};

const get_stats = (durations) => {
	const sorted = [...durations].sort((a, b) => a - b);
	const avg = durations.reduce((sum, d) => sum + d, 0) / durations.length;
	const p99_index = Math.floor(0.99 * (sorted.length - 1));
	return {
		avg: avg,
		p99: sorted[p99_index],
	};
};

const get_avg_fps = (tick_starts) => {
	return Math.round(
		1000 / ((tick_starts.at(-1) - tick_starts[0]) / (tick_starts.length - 1)),
	);
};

const perf_array_len = 100;
Game.prototype.tick = function () {
	const now_00 = performance.now();
	this.worlds[0].tick();
	this.logic_durations.push(performance.now() - now_00);

	const now_01 = performance.now();
	this.view.render(this.worlds, this.wasm_engine, this.wasm_memory);
	this.render_durations.push(performance.now() - now_01);
	const now_02 = performance.now();
	this.tick_starts.push(now_01);
	document.getElementById("fps").innerHTML = get_avg_fps(this.tick_starts);
	const stats = get_stats(this.render_durations);
	const stats_2 = get_stats(this.tick_durations);
	const stats_3 = get_stats(this.stats_durations);
	document.getElementById("render_duration_avg").innerHTML =
		stats.avg.toFixed(3);
	document.getElementById("render_duration_p99").innerHTML =
		stats.p99.toFixed(3);
	document.getElementById("tick_duration_avg").innerHTML =
		stats_2.avg.toFixed(3);
	document.getElementById("tick_duration_p99").innerHTML =
		stats_2?.p99?.toFixed(3);
	document.getElementById("stats_duration_avg").innerHTML =
		stats_3.avg.toFixed(3);
	document.getElementById("stats_duration_p99").innerHTML =
		stats_3?.p99?.toFixed(3);
	for (const x of [
		this.render_durations,
		this.tick_starts,
		this.tick_durations,
		this.stats_durations,
		this.logic_durations,
	]) {
		while (x.length > perf_array_len) {
			x.shift();
		}
	}
	this.stats_durations.push(performance.now() - now_02);
	this.tick_durations.push(performance.now() - now_00);
	requestAnimationFrame(() => {
		this.tick();
	});
};

Game.prototype.start = function () {
	this.tick();
};

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
