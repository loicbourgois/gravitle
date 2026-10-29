const get_particle = (particles_view, particle_size, idx) => {
	const particle = {
		index: particles_view.getUint32(idx * particle_size + 0, true),
		diameter: particles_view.getFloat32(idx * particle_size + 4, true),
		p: {
			x: particles_view.getFloat32(idx * particle_size + 8, true),
			y: particles_view.getFloat32(idx * particle_size + 12, true),
		},
		pp: {
			x: particles_view.getFloat32(idx * particle_size + 16, true),
			y: particles_view.getFloat32(idx * particle_size + 20, true),
		},
		np: {
			x: particles_view.getFloat32(idx * particle_size + 24, true),
			y: particles_view.getFloat32(idx * particle_size + 28, true),
		},
		dp: {
			x: particles_view.getFloat32(idx * particle_size + 32, true),
			y: particles_view.getFloat32(idx * particle_size + 36, true),
		},
		direction: {
			x: particles_view.getFloat32(idx * particle_size + 40, true),
			y: particles_view.getFloat32(idx * particle_size + 44, true),
		},
		dv: {
			x: particles_view.getFloat32(idx * particle_size + 48, true),
			y: particles_view.getFloat32(idx * particle_size + 52, true),
		},
		link_response: {
			x: particles_view.getFloat32(idx * particle_size + 56, true),
			y: particles_view.getFloat32(idx * particle_size + 60, true),
		},
		collision_response: {
			x: particles_view.getFloat32(idx * particle_size + 64, true),
			y: particles_view.getFloat32(idx * particle_size + 68, true),
		},
		collision_response_count: particles_view.getUint32(
			idx * particle_size + 72,
			true,
		),
		kind: particles_view.getUint32(idx * particle_size + 76, true),
		pressure_c: particles_view.getInt32(idx * particle_size + 80, true),
		pressure_q: particles_view.getInt32(idx * particle_size + 84, true),
		pressure_qd: particles_view.getInt32(idx * particle_size + 88, true),
		moon_c: particles_view.getInt32(idx * particle_size + 92, true),
		moon_q: particles_view.getInt32(idx * particle_size + 96, true),
		moon_qd: particles_view.getInt32(idx * particle_size + 100, true),
		electricity_c: particles_view.getInt32(idx * particle_size + 104, true),
		electricity_q: particles_view.getInt32(idx * particle_size + 108, true),
		electricity_qd: particles_view.getInt32(idx * particle_size + 112, true),
	};
	particle.ap = {
		x: (particle.p.x + particle.pp.x) * 0.5,
		y: (particle.p.y + particle.pp.y) * 0.5,
	};
	return particle;
};
export { get_particle };
