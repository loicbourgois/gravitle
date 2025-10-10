const get_link = (links_view, cell_size, idx) => {
	const link = {
		// kind: cells_view.getUint8(idx * cell_size + 77, true),
		// activated: cells_view.getUint8(idx * cell_size + 76, true),
		idx: links_view.getUint32(idx * cell_size + 0, true),
		a: links_view.getUint32(idx * cell_size + 4, true),
		b: links_view.getUint32(idx * cell_size + 8, true),
		// p: {
		// 	x: cells_view.getFloat32(idx * cell_size + 0, true),
		// 	y: cells_view.getFloat32(idx * cell_size + 4, true),
		// },
		// pp: {
		// 	x: cells_view.getFloat32(idx * cell_size + 8, true),
		// 	y: cells_view.getFloat32(idx * cell_size + 12, true),
		// },
		// np: {
		// 	x: cells_view.getFloat32(idx * cell_size + 16, true),
		// 	y: cells_view.getFloat32(idx * cell_size + 20, true),
		// },
		// diameter: cells_view.getFloat32(idx * cell_size + 68, true),
		// direction: {
		// 	x: cells_view.getFloat32(idx * cell_size + 32, true),
		// 	y: cells_view.getFloat32(idx * cell_size + 36, true),
		// },
	};
	// cell.ap = {
	// 	x: (cell.p.x + cell.pp.x) * 0.5,
	// 	y: (cell.p.y + cell.pp.y) * 0.5,
	// };
	return link;
};
export { get_link };
