import { get_cell } from "./get_cell.js";
import { get_link } from "./get_link.js";
const test = (gravitle, memory) => {
	console.log("test - start");
	const world = gravitle.World.new();
	const input = [
		{
			item_kind: "cell",
			kind: 23,
			p: {
				x: 2.4,
				y: -3.6,
			},
			pp: {
				x: 154,
				y: 3.6,
			},
			np: {
				x: 1154,
				y: 3.6,
			},
			direction: {
				x: 114.8,
				y: 3.6,
			},
			diameter: 1.2,
			activated: 19,
			dp: {
				x: 3.1,
				y: 3.2,
			},
		},
		{
			kind: 12,
			item_kind: "cell",
			p: {
				x: 12.4,
				y: -3.6,
			},
			pp: {
				x: 154,
				y: 3.6,
			},
			np: {
				x: 114,
				y: 3.6,
			},
			diameter: 1.2,
			activated: 0,
			direction: {
				x: 1154,
				y: 3.6,
			},
			dp: {
				x: 3.4,
				y: 3.5,
			},
		},
		{
			kind: 19,
			item_kind: "cell",
			p: {
				x: 5.4,
				y: 2.6,
			},
			np: {
				x: 114,
				y: -3.6,
			},
			pp: {
				x: 15.4,
				y: 23.6,
			},
			diameter: 1.3,
			activated: 1,
			direction: {
				x: 1154,
				y: 3.6,
			},
			dp: {
				x: 3.6,
				y: 3.7,
			},
		},
		{
			kind: 13,
			item_kind: "cell",
			p: {
				x: 5.4,
				y: 2.4,
			},
			pp: {
				x: 15.4,
				y: 23.6,
			},

			np: {
				x: 114,
				y: 4.6,
			},
			diameter: 1.3,
			activated: 5,
			direction: {
				x: 1154,
				y: 3.6,
			},
			dp: {
				x: 3.6,
				y: 3.7,
			},
		},
		{
			idx: 0,
			item_kind: "link",
			a: 2,
			b: 3,
		},
	];
	for (const e of input) {
		if (e.item_kind == "cell") {
			const idx = world.add_cell();
			world.set_cell_position_x(idx, e.p.x);
			world.set_cell_position_y(idx, e.p.y);
			world.set_cell_pp_x(idx, e.pp.x);
			world.set_cell_pp_y(idx, e.pp.y);
			world.set_cell_np_x(idx, e.np.x);
			world.set_cell_np_y(idx, e.np.y);
			world.set_cell_diameter(idx, e.diameter);
			world.set_cell_kind(idx, e.kind);
			world.set_cell_activated(idx, e.activated);
			world.set_cell_direction_x(idx, e.direction.x);
			world.set_cell_direction_y(idx, e.direction.y);
			world.set_cell_dp_x(idx, e.dp.x);
			world.set_cell_dp_y(idx, e.dp.y);
			e.idx = idx;
		} else if (e.item_kind == "link") {
			const _idx = world.add_link(e.a, e.b);
		} else {
			throw new Error(`invalid item_kind: ${e.item_kind}`);
		}
	}
	const cellsPtr = world.cells();
	const cells_view = new DataView(
		memory.buffer,
		cellsPtr,
		world.cells_count() * gravitle.Cell.size(),
	);
	const cell_size = gravitle.Cell.size();
	const linksPtr = world.links();
	const links_view = new DataView(
		memory.buffer,
		linksPtr,
		world.links_count() * gravitle.Link.size(),
	);
	const link_size = gravitle.Link.size();
	const errors = [];
	for (const ei of input) {
		if (ei.item_kind == "cell") {
			const eo = get_cell(cells_view, cell_size, ei.idx);
			for (const field of [
				"idx",
				"kind",
				"p.x",
				"p.y",
				"diameter",
				"pp.x",
				"pp.y",
				"np.x",
				"np.y",
				"activated",
				"direction.x",
				"direction.y",
				"dp.x",
				"dp.y",
			]) {
				let fi = ei;
				let fo = eo;
				for (const part of field.split(".")) {
					fi = fi[part];
					fo = fo[part];
				}
				if (fi == fo || Math.abs(fi - fo) < 0.00001) {
					// pass
				} else {
					console.error(field, fi, fo);
					errors.push(field, fi, fo);
				}
			}
		} else if (ei.item_kind == "link") {
			const eo = get_link(links_view, link_size, ei.idx);
			for (const field of ["idx", "kind", "a", "b"]) {
				let fi = ei;
				let fo = eo;
				for (const part of field.split(".")) {
					fi = fi[part];
					fo = fo[part];
				}
				if (fi == fo || Math.abs(fi - fo) < 0.00001) {
					// pass
				} else {
					console.error(field, fi, fo);
					errors.push(field, fi, fo);
				}
			}
		} else {
			throw new Error(`invalid item_kind: ${ei.item_kind}`);
		}
		if (errors.length) {
			break;
		}
	}
	if (errors.length) {
		for (let i = 0; i < world.cells_count() * gravitle.Cell.size(); i++) {
			try {
				console.log(
					i,
					`@${cells_view.getUint32(i, true)}`,
					`@${cells_view.getFloat32(i, true)}`,
					`@${cells_view.getUint8(i, true)}`,
					`@${cells_view.getUint8(i, false)}`,
				);
			} catch (error) {}
		}
		for (let i = 0; i < world.links_count() * gravitle.Link.size(); i++) {
			try {
				console.log(
					i,
					`@${links_view.getUint32(i, true)}`,
					// `@${links_view.getUint32(i, false)}`,
					// `@${links_view.getFloat32(i, true)}`,
					// `@${links_view.getUint8(i, true)}`,
					// `@${links_view.getUint8(i, false)}`,
				);
			} catch (error) {}
		}
	}

	// console.log("world.links_count()", world.links_count());
	// world.step();
	// console.log("world.links_count()", world.links_count());
	// const links_ptr = world.links();
	// const link_size = miniciv.Link.size();
	// const links_view = new DataView(
	// 	memory.buffer,
	// 	links_ptr,
	// 	world.links_count() * link_size,
	// );
	// const errors_2 = [];
	// for (const ei of input) {
	// 	if (ei.kind == "link") {
	// 		const eo = get_link(links_view, link_size, ei.idx);
	// 		for (const field of ["kind", "caid", "cbid", "live"]) {
	// 			let fi = ei;
	// 			let fo = eo;
	// 			for (const part of field.split(".")) {
	// 				fi = fi[part];
	// 				fo = fo[part];
	// 			}
	// 			if (fi == fo || Math.abs(fi - fo) < 0.000001) {
	// 				// pass
	// 			} else {
	// 				console.error(field, fi, fo);
	// 				errors_2.push(field, fi, fo);
	// 			}
	// 		}
	// 	} else if (ei.kind == "cell") {
	// 		// pass
	// 	} else {
	// 		throw new Error(`invalid kind: ${ei.kind}`);
	// 	}
	// }
	// if (errors_2.length) {
	// 	for (let i = 0; i < world.links_count() * miniciv.Link.size(); i++) {
	// 		try {
	// 			console.log(
	// 				i,
	// 				links_view.getUint32(i, true),
	// 				links_view.getFloat32(i, true),
	// 			);
	// 		} catch (error) {}
	// 	}
	// }
	console.log("test - ok");
};
export { test };
