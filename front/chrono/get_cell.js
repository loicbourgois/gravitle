const get_cell = (cells_view, cell_size, idx) => {
	const cell = {
		idx: cells_view.getUint32(idx * cell_size + 0, true),
		diameter: cells_view.getFloat32(idx * cell_size + 4, true),
		p: {
			x: cells_view.getFloat32(idx * cell_size + 8, true),
			y: cells_view.getFloat32(idx * cell_size + 12, true),
		},
		pp: {
			x: cells_view.getFloat32(idx * cell_size + 16, true),
			y: cells_view.getFloat32(idx * cell_size + 20, true),
		},
		np: {
			x: cells_view.getFloat32(idx * cell_size + 24, true),
			y: cells_view.getFloat32(idx * cell_size + 28, true),
		},
		dp: {
			x: cells_view.getFloat32(idx * cell_size + 32, true),
			y: cells_view.getFloat32(idx * cell_size + 36, true),
		},
		direction: {
			x: cells_view.getFloat32(idx * cell_size + 40, true),
			y: cells_view.getFloat32(idx * cell_size + 44, true),
		},
		dv: {
			x: cells_view.getFloat32(idx * cell_size + 48, true),
			y: cells_view.getFloat32(idx * cell_size + 52, true),
		},
		link_response: {
			x: cells_view.getFloat32(idx * cell_size + 56, true),
			y: cells_view.getFloat32(idx * cell_size + 60, true),
		},
		collision_response: {
			x: cells_view.getFloat32(idx * cell_size + 64, true),
			y: cells_view.getFloat32(idx * cell_size + 68, true),
		},
		collision_response_count: cells_view.getUint32(idx * cell_size + 72, true),
		activated: cells_view.getUint32(idx * cell_size + 76, true),
		activated_previous: cells_view.getUint32(idx * cell_size + 80, true),
		kind: cells_view.getUint32(idx * cell_size + 84, true),
		user_kind: cells_view.getUint32(idx * cell_size + 88, true),
	};
	cell.ap = {
		x: (cell.p.x + cell.pp.x) * 0.5,
		y: (cell.p.y + cell.pp.y) * 0.5,
	};
	return cell;
};
export { get_cell };
