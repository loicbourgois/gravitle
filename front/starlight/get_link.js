const get_link = (links_view, particle_size, idx) => {
	const link = {
		// kind: particles_view.getUint8(idx * particle_size + 77, true),
		// activated: particles_view.getUint8(idx * particle_size + 76, true),
		idx: links_view.getUint32(idx * particle_size + 0, true),
		a: links_view.getUint32(idx * particle_size + 4, true),
		b: links_view.getUint32(idx * particle_size + 8, true),
		// p: {
		// 	x: particles_view.getFloat32(idx * particle_size + 0, true),
		// 	y: particles_view.getFloat32(idx * particle_size + 4, true),
		// },
		// pp: {
		// 	x: particles_view.getFloat32(idx * particle_size + 8, true),
		// 	y: particles_view.getFloat32(idx * particle_size + 12, true),
		// },
		// np: {
		// 	x: particles_view.getFloat32(idx * particle_size + 16, true),
		// 	y: particles_view.getFloat32(idx * particle_size + 20, true),
		// },
		// diameter: particles_view.getFloat32(idx * particle_size + 68, true),
		// direction: {
		// 	x: particles_view.getFloat32(idx * particle_size + 32, true),
		// 	y: particles_view.getFloat32(idx * particle_size + 36, true),
		// },
	};
	// particle.ap = {
	// 	x: (particle.p.x + particle.pp.x) * 0.5,
	// 	y: (particle.p.y + particle.pp.y) * 0.5,
	// };
	return link;
};
export { get_link };
