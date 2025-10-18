import { ship } from "./ship.js";
import { sfc32 } from "./random.js";
import { get_cell } from "./get_cell.js";
import { wrap_around } from "./math.js";

function game_setup() {
	const kinds = {
		armor: this.gravitle.Kind.Armor,
		booster: this.gravitle.Kind.Booster,
		core: this.gravitle.Kind.Core,
	};
	for (let idx = 0 ; idx < this.worlds.length ; idx++) {
		const world = this.worlds[idx]
		const ghost = this.ghosts[idx-1]
		let user_kind = 1
		if (ghost && ghost.kind == "me") {
			user_kind = 2
		}
		if (ghost && ghost.kind == "other") {
			user_kind = 3
		}
		for (const e of ship.parts) {
			world.add_cell(
				e.p.x - 0.3, e.p.y - 0.3, e.d, kinds[e.kind],
				user_kind,
			);
		}
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
					world.add_cell(x, y, diameter, 4, user_kind);
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
					world.add_cell(x, y, diameter, 5, user_kind);
					break;
				}
			}
		}
		for (const l of ship.links) {
			world.add_link(l.a, l.b);
		}
	}
}
export { game_setup };
