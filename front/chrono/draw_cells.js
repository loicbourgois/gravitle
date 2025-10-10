import { get_cell } from "./get_cell.js";
import { get_link } from "./get_link.js";
import {
	wrap_around,
	delta,
	multiply_vector,
	del_vector,
	add_vectors,
} from "./math.js";
import { link_color_priority, get_color } from "./colors.js";
import { kind } from "./kind.js";

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
	for (let i = 0; i < world.cells_count(); i++) {
		const cell = get_cell(cells_view, cell_size, i);
		if (cell.kind == 6) {
			const p3 = cell.ap;
			const d3 = cell.diameter;
			view.draw_disk_multi(
				p3.x,
				p3.y,
				d3,
				get_color(cell.kind, cell.activated, 0, player_kind),
			);
		}
	}
	for (let i = 0; i < world.cells_count(); i++) {
		const cell = get_cell(cells_view, cell_size, i);
		if (cell.kind == 1) {
			if (cell.activated) {
				const p = add_vectors(
					cell.ap,
					multiply_vector(cell.direction, 0.007 + Math.random() * 0.003),
				);
				const d = cell.diameter * 0.7;
				const p2 = add_vectors(
					cell.ap,
					multiply_vector(cell.direction, 0.005 + Math.random() * 0.001),
				);
				const d2 = cell.diameter * 0.9;
				view.draw_disk_multi(
					p.x,
					p.y,
					d,
					get_color(cell.kind, cell.activated, 2, player_kind),
				);
				view.draw_disk_multi(
					p2.x,
					p2.y,
					d2,
					get_color(cell.kind, cell.activated, 1, player_kind),
				);
			}
			const p3 = cell.ap;
			const d3 = cell.diameter;
			view.draw_disk_multi(
				p3.x,
				p3.y,
				d3,
				get_color(cell.kind, cell.activated, 0, player_kind),
			);
		}
	}
	for (let i = 0; i < world.cells_count(); i++) {
		const cell = get_cell(cells_view, cell_size, i);
		if ([kind.ARMOR, kind.CORE].includes(cell.kind)) {
			const p3 = cell.ap;
			const d3 = cell.diameter;
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
				link_color_priority[p1.kind] < link_color_priority[p2.kind]
					? p1.kind
					: p2.kind;
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
	const player_kind = "m";
	const cells_ptr = world.cells();
	const cell_size = gravitle.Cell.size();
	const cells_view = new DataView(
		memory.buffer,
		cells_ptr,
		world.cells_count() * cell_size,
	);
	for (let i = 0; i < world.cells_count(); i++) {
		const cell = get_cell(cells_view, cell_size, i);
		if (cell.kind == 2) {
			view.center.x = cell.ap.x + cell.dp.x * 20;
			view.center.y = cell.ap.y + cell.dp.y * 20;
		} else if (cell.kind == 5) {
			const p3 = cell.ap;
			const d3 = cell.diameter;
			view.draw_disk_multi(
				p3.x,
				p3.y,
				d3,
				get_color(cell.kind, cell.activated, 0, player_kind),
			);
		}
	}
	draw_ship_only(gravitle, world, memory, view, "m");
	for (let i = 0; i < world.cells_count(); i++) {
		const cell = get_cell(cells_view, cell_size, i);
		if (cell.kind == 4) {
			const c3 = get_color(cell.kind, cell.activated, 0, player_kind);
			view.draw_disk_multi(cell.ap.x, cell.ap.y, cell.diameter, c3);
		}
	}
	// const p3 = cell.ap;
	// 		const d3 = cell.diameter;
	// 		const c3 = "#f0f";
	// 		view.draw_disk_multi(p3.x, p3.y, d3, c3);
};
export { draw_cells, draw_ship_only };
