
import { setup_1 } from "./setup_1.js";
import { 
    get_avg_fps,
    as_frame_budget,
    pad5,
    as_percent_str, 
} from "./misc.js"

function Game({ wasm_memory, wasm_engine, view }) {
	this.wasm_memory = wasm_memory;
	this.wasm_engine = wasm_engine;
	this.view = view;
	this.tick_starts = [];
	this.render_durations = [];
	this.tick_durations = [];
	this.stats_durations = [];
	this.logic_durations = [];
    this.tick_per_tack = 10
}

Game.prototype.setup = async function () {
	await this.view.setup(this.wasm_engine);
	const world = this.wasm_engine.setup();
	setup_1(world)
	this.worlds = [world];
	return this;
};

Game.prototype.update_stats = function () {
    document.getElementById("fps").innerHTML = get_avg_fps(this.tick_starts);
	document.getElementById("frame_budget_logic").innerHTML = as_frame_budget(this.worlds[0].get_stats("logic").avg)
	document.getElementById("frame_budget_render").innerHTML = as_frame_budget(this.worlds[0].get_stats("render").avg)
	document.getElementById("frame_budget_stats").innerHTML = as_frame_budget(this.worlds[0].get_stats("stats").avg)
	document.getElementById("frame_budget_total").innerHTML = as_frame_budget(this.worlds[0].get_stats("tick").avg)
	document.getElementById("collision_count").innerHTML = pad5(this.worlds[0].collision_count)
	document.getElementById("pairs_count").innerHTML = pad5(this.worlds[0].pairs_count)
	document.getElementById("zones_count").innerHTML = pad5(this.worlds[0].zones_count)
	document.getElementById("cell_count").innerHTML = pad5(this.worlds[0].cells_count())
	document.getElementById("link_count").innerHTML = pad5(this.worlds[0].links_count())
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
	{
        const b = 'tick_links';
		const a = `frame_budget_${b}`;
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

export {
    Game,
}