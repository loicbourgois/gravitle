const get_cell = (cells_view, cell_size, idx) => {
	const cell = {
		idx: cells_view.getUint32(idx * cell_size + 12, true),
		diameter: cells_view.getFloat32(idx * cell_size + 16, true),
		p: {
			x: cells_view.getFloat32(idx * cell_size + 20, true),
			y: cells_view.getFloat32(idx * cell_size + 24, true),
		},
		pp: {
			x: cells_view.getFloat32(idx * cell_size + 28, true),
			y: cells_view.getFloat32(idx * cell_size + 32, true),
		},
		np: {
			x: cells_view.getFloat32(idx * cell_size + 36, true),
			y: cells_view.getFloat32(idx * cell_size + 40, true),
		},
		dp: {
			x: cells_view.getFloat32(idx * cell_size + 44, true),
			y: cells_view.getFloat32(idx * cell_size + 48, true),
		},
		direction: {
			x: cells_view.getFloat32(idx * cell_size + 52, true),
			y: cells_view.getFloat32(idx * cell_size + 56, true),
		},
		activated: cells_view.getUint8(idx * cell_size + 89, true),
		kind: cells_view.getUint8(idx * cell_size + 88, true),
	};
	cell.ap = {
		x: (cell.p.x + cell.pp.x) * 0.5,
		y: (cell.p.y + cell.pp.y) * 0.5,
	};
	return cell;
};
export { get_cell };
