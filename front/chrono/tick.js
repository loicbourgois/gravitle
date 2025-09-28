import { draw_cells, draw_ship_only } from "./draw_cells.js";
import { json_to_short } from "./misc.js";

let last_frame;
let ghost_cells = new Set();
let share_activation_events = null;
let ghost_deactivated = false;
let victory_celebrated = 0;
let victory_celebrated_1 = 0;

const tick = (gravitle, view, worlds, memory, ghost_json, context) => {
	const now = performance.now();
	const elapsed = now - last_frame;
	let steps = 2;
	if (elapsed > 15 && last_frame != null) {
		steps = 4;
	}
	last_frame = now;
	for (let index = 0; index < steps; index++) {
		if (ghost_json) {
			const es = ghost_json[worlds[1].step];
			if (es != null) {
				for (const e of es) {
					worlds[1].switch_cell_activated(e.c);
					ghost_cells.add(e.c);
				}
				delete ghost_json[worlds[1].step];
			}
			if (
				!Object.keys(ghost_json).length &&
				!ghost_deactivated &&
				worlds[1].victory
			) {
				ghost_deactivated = true;
				for (const c of ghost_cells) {
					worlds[1].set_cell_activated(c, 0);
				}
			}
		}
		worlds[0].run_step();
		worlds[1].run_step();
		if (worlds[1].victory == 1 && !victory_celebrated_1) {
			victory_celebrated_1 = 1;
			console.log(worlds[1].victory_duration);
			document.getElementById("victory_duration_1").innerHTML =
				worlds[1].victory_duration;
		}
		if (worlds[0].victory == 1 && !victory_celebrated) {
			victory_celebrated = 1;
			document.getElementById("victory_duration").innerHTML =
				worlds[0].victory_duration;
			document.getElementById("victory").classList.add("yes");
			share_activation_events = worlds[0].get_activation_events();
			console.log(share_activation_events);
			context.ghost_to_share = json_to_short(
				JSON.parse(share_activation_events),
			);
		}
	}
	view.set_backgound("#102");
	if (ghost_json) {
		draw_ship_only(gravitle, worlds[1], memory, view);
	}
	draw_cells(gravitle, worlds[0], memory, view);
	requestAnimationFrame(() => {
		tick(gravitle, view, worlds, memory, ghost_json, context);
	});
};

export { tick };
