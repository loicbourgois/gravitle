import init, * as gravitle from "../slingshot/gravitle_slingshot.js";
import { ViewWebGPU } from "../slingshot/view_webgpu.js";
import { setup_1 } from "./setup_1.js";

const has_webgpu_support = async () => {
	try {
		const adapter = await navigator.gpu?.requestAdapter();
		const device = await adapter?.requestDevice();
		if (device) {
			console.log("aa")
			return true;
		} else {
			return false;
		}
	} catch (error) {
		console.error(error)
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
    this.tick_per_tack = 1
}

Game.prototype.setup = async function () {
	await this.view.setup(this.wasm_engine);
	const world = this.wasm_engine.setup();
	setup_1(world)
	this.worlds = [world];
	return this;
};

const get_avg_fps = (tick_starts) => {
	return Math.round(
		1000 / ((tick_starts.at(-1) - tick_starts[0]) / (tick_starts.length - 1)),
	);
};

const as_frame_budget = (value) => {
	return as_percent_str( value / (1000/120) )
}

const as_percent_str = (value) => {
	const s = `${(value * 100 )?.toFixed(1)}`
    return s.padStart(4);
}

Game.prototype.update_stats = function () {
	const render = this.worlds[0].get_stats("render");
	const total = this.worlds[0].get_stats("tick");
	const stats = this.worlds[0].get_stats("stats");
	const logic = this.worlds[0].get_stats("logic");
    document.getElementById("fps").innerHTML = get_avg_fps(this.tick_starts);
	document.getElementById("frame_budget_logic").innerHTML = as_frame_budget(logic.avg)
	document.getElementById("frame_budget_render").innerHTML = as_frame_budget(render.avg)
	document.getElementById("frame_budget_stats").innerHTML = as_frame_budget(stats.avg)
	document.getElementById("frame_budget_total").innerHTML = as_frame_budget(total.avg)
	document.getElementById("cell_count").innerHTML = this.worlds[0].cells_count()
	document.getElementById("buffer_cell_budget").innerHTML = as_percent_str(
		this.worlds[0].cells_count()*this.wasm_engine.Cell.size()/this.view.buffer_cells.size
	)
    for (let index = 1; index < 7; index++) {
        const a = `frame_budget_logic_0${index}`;
        const b = `tick_0${index}`;
        document.getElementById(a).innerHTML = as_frame_budget(
            this.worlds[0].get_stats(b).avg * 1000 * this.tick_per_tack
        )
    }
}

Game.prototype.tack = function () {
	const now_00 = performance.now();
	for (let index = 0; index < this.tick_per_tack; index++) {
		this.worlds[0].tick();
	}
    this.worlds[0].add_duration("logic", performance.now() - now_00)
	const now_01 = performance.now();
	this.view.render(this.worlds, this.wasm_engine, this.wasm_memory);
    this.worlds[0].add_duration("render", performance.now() - now_01)
	const now_02 = performance.now();
	this.tick_starts.push(now_00);
	this.update_stats()
    this.worlds[0].add_duration("stats", performance.now() - now_02)
	requestAnimationFrame(() => {
		this.tack();
	});
    this.worlds[0].add_duration("tick", performance.now() - now_00)
};

Game.prototype.start = function () {
	this.tack();
};

const main = async () => {
	await init();
	if (await has_webgpu_support()) {
		(
			await new Game({
				wasm_memory: gravitle.initSync().memory,
				wasm_engine: gravitle,
				view: new ViewWebGPU("canvas"),
			}).setup()
		).start();
	} else {
		console.error("WebGPU not supported")
	}
};
main();
