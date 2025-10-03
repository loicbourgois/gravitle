import { get_cell } from "./get_cell.js";
import { get_link } from "./get_link.js";
import { wrap_around, delta } from "./math.js";
import { colors } from "./colors.js";

const add_vectors = (a, b) => {
	return {
		x: a.x + b.x,
		y: a.y + b.y,
	};
};

const del_vector = (a, b) => {
	return {
		x: a.x - b.x,
		y: a.y - b.y,
	};
};

const multiply_vector = (a, k) => {
	return {
		x: a.x * k,
		y: a.y * k,
	};
};

const get_color = (cell_kind, activation, layer, player_kind) => {
	const ARMOR = 0;
	const BOOSTER = 1;
	const CORE = 2;
	const LIGHTED = 6;
	try {
		const aa = {
			[ARMOR]: {
				0: {
					0: {
						// me
						m: "#___",
						// ghost
						g: "#558",
						// other_ghost
						o: "#585",
					},
				},
			},
			[BOOSTER]: {
				0: {
					0: {
						m: "#___",
						g: "#850",
						o: "#850",
					},
				},
				1: {
					0: {
						m: "#___",
						g: "#850",
						o: "#850",
					},
					1: {
						m: "#___",
						g: "#840",
						o: "#840",
					},
					2: {
						m: "#___",
						g: "#800",
						o: "#800",
					},
				},
			},
			[CORE]: {
				0: {
					0: {
						m: "#___",
						g: "#885",
						o: "#885",
					},
				},
			},
			[LIGHTED]: {
				0: {
					0: {
						m: "#ff9d",
						g: "#0000",
						o: "#9f95",
					},
				},
			},
		};
		return aa[cell_kind][activation][layer][player_kind];
	} catch (error) {
		console.log(cell_kind, activation, layer, player_kind);
		throw error;
	}
};

const draw_ship_only = (gravitle, world, memory, view, player_kind = "g") => {
	const cells_ptr = world.cells();
	const cell_size = gravitle.Cell.size();
	const cells_view = new DataView(
		memory.buffer,
		cells_ptr,
		world.cells_count() * cell_size,
	);
	const links_ptr = world.links();
	const link_size = gravitle.Link.size();
	const links_view = new DataView(
		memory.buffer,
		links_ptr,
		world.links_count() * link_size,
	);
	const mult = 1.0;
	for (let i = 0; i < world.cells_count(); i++) {
		const cell = get_cell(cells_view, cell_size, i);
		if (cell.activated && cell.kind == 1) {
			const p = add_vectors(
				cell.ap,
				multiply_vector(cell.direction, 0.007 + Math.random() * 0.003),
			);
			const d = cell.diameter * 0.7 * mult;
			view.draw_disk_multi(
				p.x,
				p.y,
				d,
				get_color(cell.kind, cell.activated, 2, player_kind),
			);
			const p2 = add_vectors(
				cell.ap,
				multiply_vector(cell.direction, 0.005 + Math.random() * 0.001),
			);
			const d2 = cell.diameter * 0.9 * mult;
			view.draw_disk_multi(
				p2.x,
				p2.y,
				d2,
				get_color(cell.kind, cell.activated, 1, player_kind),
			);
			const p3 = cell.ap;
			const d3 = cell.diameter * mult;
			view.draw_disk_multi(
				p3.x,
				p3.y,
				d3,
				get_color(cell.kind, cell.activated, 0, player_kind),
			);
		} else if (cell.kind == 1) {
			const p3 = cell.ap;
			const d3 = cell.diameter * mult;
			view.draw_disk_multi(
				p3.x,
				p3.y,
				d3,
				get_color(cell.kind, cell.activated, 0, player_kind),
			);
		} else {
		}
	}
	for (let i = 0; i < world.cells_count(); i++) {
		const cell = get_cell(cells_view, cell_size, i);
		if ([1, 4, 5].includes(cell.kind)) {
			// pass
		} else {
			const p3 = cell.ap;
			const d3 = cell.diameter * mult;
			view.draw_disk_multi(
				p3.x,
				p3.y,
				d3,
				get_color(cell.kind, cell.activated, 0, player_kind),
			);
		}
	}
	for (let color_id_1 of [1, 2, 0]) {
		for (let i = 0; i < world.links_count(); i++) {
			const l = get_link(links_view, link_size, i);
			const p1 = get_cell(cells_view, cell_size, l.a);
			const p2 = get_cell(cells_view, cell_size, l.b);
			const kind =
				colors[p1.kind].score > colors[p2.kind].score ? p1.kind : p2.kind;
			if (color_id_1 == kind) {
				const wa = wrap_around(p1.ap, p2.ap);
				const delt = multiply_vector(delta(wa.a, wa.b), 0.5);
				const color = get_color(kind, 0, 0, player_kind);
				const aa = 0.75;
				const pos1 = add_vectors(p1.p, delt);
				const pos2 = del_vector(p2.p, delt);
				view.draw_disk_multi(pos1.x, pos1.y, p1.diameter * aa, color);
				view.draw_disk_multi(pos2.x, pos2.y, p2.diameter * aa, color);
			}
		}
	}
};

const draw_cells = (gravitle, world, memory, view) => {
	const cells_ptr = world.cells();
	const cell_size = gravitle.Cell.size();
	const cells_view = new DataView(
		memory.buffer,
		cells_ptr,
		world.cells_count() * cell_size,
	);
	const links_ptr = world.links();
	const link_size = gravitle.Link.size();
	const links_view = new DataView(
		memory.buffer,
		links_ptr,
		world.links_count() * link_size,
	);
	const mult = 1.0;
	for (let i = 0; i < world.cells_count(); i++) {
		const cell = get_cell(cells_view, cell_size, i);
		if (cell.kind == 2) {
			view.center.x = cell.ap.x + cell.dp.x * 20;
			view.center.y = cell.ap.y + cell.dp.y * 20;
		}
	}
	for (let i = 0; i < world.cells_count(); i++) {
		const cell = get_cell(cells_view, cell_size, i);
		if (cell.activated && cell.kind == 1) {
			const p = add_vectors(
				cell.ap,
				multiply_vector(cell.direction, 0.007 + Math.random() * 0.003),
			);
			const d = cell.diameter * 0.7 * mult;
			const c = "#f00";
			view.draw_disk_multi(p.x, p.y, d, c);
			const p2 = add_vectors(
				cell.ap,
				multiply_vector(cell.direction, 0.005 + Math.random() * 0.001),
			);
			const d2 = cell.diameter * 0.9 * mult;
			const c2 = "#f80";
			view.draw_disk_multi(p2.x, p2.y, d2, c2);
			const p3 = cell.ap;
			const d3 = cell.diameter * mult;
			const c3 = "#fa0";
			view.draw_disk_multi(p3.x, p3.y, d3, c3);
		} else if (cell.kind == 1) {
			const p3 = cell.ap;
			const d3 = cell.diameter * mult;
			const c3 = "#fa0";
			view.draw_disk_multi(p3.x, p3.y, d3, c3);
		} else {
		}
	}
	for (let i = 0; i < world.cells_count(); i++) {
		const cell = get_cell(cells_view, cell_size, i);
		if (cell.kind == 5) {
			const p3 = cell.ap;
			const d3 = cell.diameter * mult;
			const c3 = "#ff93";
			view.draw_disk_multi(p3.x, p3.y, d3, c3);
		} else if (cell.kind == 6) {
			const p3 = cell.ap;
			const d3 = cell.diameter * mult;
			const c3 = "#ff9d";
			view.draw_disk_multi(p3.x, p3.y, d3, c3);
		} else {
		}
	}
	for (let i = 0; i < world.cells_count(); i++) {
		const cell = get_cell(cells_view, cell_size, i);
		if (cell.activated && cell.kind == 1) {
		} else if (cell.kind == 1) {
		} else if (cell.kind == 2) {
			const p3 = cell.ap;
			const d3 = cell.diameter * mult;
			const c3 = "#ffa";
			view.draw_disk_multi(p3.x, p3.y, d3, c3);
		} else if (cell.kind == 4) {
		} else if (cell.kind == 0) {
			const p3 = cell.ap;
			const d3 = cell.diameter * mult;
			const c3 = "#aaf";
			view.draw_disk_multi(p3.x, p3.y, d3, c3);
		} else if (cell.kind == 5) {
		} else if (cell.kind == 6) {
		} else {
			const p3 = cell.ap;
			const d3 = cell.diameter * mult;
			const c3 = "#f0f";
			view.draw_disk_multi(p3.x, p3.y, d3, c3);
		}
	}
	for (let color_id_1 of [1, 2, 0]) {
		for (let i = 0; i < world.links_count(); i++) {
			const l = get_link(links_view, link_size, i);
			const p1 = get_cell(cells_view, cell_size, l.a);
			const p2 = get_cell(cells_view, cell_size, l.b);
			const color_id =
				colors[p1.kind].score > colors[p2.kind].score ? p1.kind : p2.kind;
			if (color_id_1 == color_id) {
				const wa = wrap_around(p1.ap, p2.ap);
				const delt = multiply_vector(delta(wa.a, wa.b), 0.5);
				const color = colors[color_id].value[0];
				const aa = 0.75;
				const pos1 = add_vectors(p1.p, delt);
				const pos2 = del_vector(p2.p, delt);
				view.draw_disk_multi(pos1.x, pos1.y, p1.diameter * aa, color);
				view.draw_disk_multi(pos2.x, pos2.y, p2.diameter * aa, color);
			}
		}
	}
	for (let i = 0; i < world.cells_count(); i++) {
		const cell = get_cell(cells_view, cell_size, i);
		if (cell.activated && cell.kind == 1) {
		} else if (cell.kind == 1) {
		} else if (cell.kind == 2) {
		} else if (cell.kind == 4) {
			const p3 = cell.ap;
			const d3 = cell.diameter * mult;
			const c3 = "rgba(203, 110, 63, 1)";
			view.draw_disk_multi(p3.x, p3.y, d3, c3);
		} else {
		}
	}
};
export { draw_cells, draw_ship_only };
