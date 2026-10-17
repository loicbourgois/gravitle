import { get_particle } from "./get_particle.js";
import { get_link } from "./get_link.js";
import {
	wrap_around,
	delta,
	multiply_vector,
	del_vector,
	add_vectors,
	rotate_around,
} from "./math.js";
import { link_color_priority, get_color } from "./colors.js";
import { kind } from "./kind.js";


const COUNT = 6
const SIZE = 1
const SPACE_1 = 0.2
const SPACE_2 = 0.3


const draw_star = (view, particle, player_kind, space) => {
	const p3 = particle.ap;
	const d3 = particle.diameter;
	view.draw_disk_multi(
		p3.x,
		p3.y,
		d3,
		get_color(particle.kind, particle.activated, 0, player_kind),
	);
	let m = SIZE
	let m2 = (m + 1.0)*space
	let p_dot = {
		x: p3.x + d3 * m2,
		y: p3.y,
	}
	for (let index = 0; index < COUNT; index++) {
		const angle_degree = index * 360/COUNT;
		const p_dot_ = rotate_around(p_dot, p3, angle_degree)
		view.draw_disk_multi(
			p_dot_.x,
			p_dot_.y,
			d3 * m,
			get_color(particle.kind, particle.activated, 0, player_kind),
		);
	}
}


const draw_ship_only = (gravitle, world, memory, view, player_kind = "g") => {
	const particles_ptr = world.particles();
	const particle_size = gravitle.Particle.size();
	const particles_view = new DataView(
		memory.buffer,
		particles_ptr,
		world.particles_count() * particle_size,
	);
	const links_ptr = world.links();
	const link_size = gravitle.Link.size();
	const links_view = new DataView(
		memory.buffer,
		links_ptr,
		world.links_count() * link_size,
	);
	for (let i = 0; i < world.particles_count(); i++) {
		const particle = get_particle(particles_view, particle_size, i);
		// Put in here so that other player ghost
		// lighted star show up in green.
		if (particle.kind == kind.LIGHTED) {
			draw_star(view, particle, player_kind, SPACE_2)
		}
	}
	for (let i = 0; i < world.particles_count(); i++) {
		const particle = get_particle(particles_view, particle_size, i);
		if (particle.kind == kind.BOOSTER) {
			if (particle.activated) {
				const p = add_vectors(
					particle.ap,
					multiply_vector(particle.direction, 0.007 + Math.random() * 0.003),
				);
				const d = particle.diameter * 0.7;
				const p2 = add_vectors(
					particle.ap,
					multiply_vector(particle.direction, 0.005 + Math.random() * 0.001),
				);
				const d2 = particle.diameter * 0.9;
				view.draw_disk_multi(
					p.x,
					p.y,
					d,
					get_color(particle.kind, particle.activated, 2, player_kind),
				);
				view.draw_disk_multi(
					p2.x,
					p2.y,
					d2,
					get_color(particle.kind, particle.activated, 1, player_kind),
				);
			}
			const p3 = particle.ap;
			const d3 = particle.diameter;
			view.draw_disk_multi(
				p3.x,
				p3.y,
				d3,
				get_color(particle.kind, particle.activated, 0, player_kind),
			);
		}
	}
	for (let i = 0; i < world.particles_count(); i++) {
		const particle = get_particle(particles_view, particle_size, i);
		if ([kind.ARMOR, kind.CORE].includes(particle.kind)) {
			const p3 = particle.ap;
			const d3 = particle.diameter;
			view.draw_disk_multi(
				p3.x,
				p3.y,
				d3,
				get_color(particle.kind, particle.activated, 0, player_kind),
			);
		}
	}
	for (let color_id_1 of [1, 2, 0]) {
		for (let i = 0; i < world.links_count(); i++) {
			const l = get_link(links_view, link_size, i);
			const p1 = get_particle(particles_view, particle_size, l.a);
			const p2 = get_particle(particles_view, particle_size, l.b);
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

const draw_particles = (gravitle, world, memory, view) => {
	const player_kind = "m";
	const particles_ptr = world.particles();
	const particle_size = gravitle.Particle.size();
	const particles_view = new DataView(
		memory.buffer,
		particles_ptr,
		world.particles_count() * particle_size,
	);
	for (let i = 0; i < world.particles_count(); i++) {
		const particle = get_particle(particles_view, particle_size, i);
		if (particle.kind == kind.CORE) {
			view.center.x = particle.ap.x + particle.dp.x * 20;
			view.center.y = particle.ap.y + particle.dp.y * 20;
		} else if (particle.kind == kind.UNLIGHTED) {
			draw_star(view, particle, player_kind, SPACE_1)
		} else {
			// pass
		}
	}
	draw_ship_only(gravitle, world, memory, view, "m");
	for (let i = 0; i < world.particles_count(); i++) {
		const particle = get_particle(particles_view, particle_size, i);
		if (particle.kind == kind.ASTEROID) {
			const c3 = get_color(particle.kind, particle.activated, 0, player_kind);
			view.draw_disk_multi(particle.ap.x, particle.ap.y, particle.diameter, c3);
		}
	}
};
export { draw_particles, draw_ship_only };
