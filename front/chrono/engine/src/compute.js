const compute = () => {
	//   update_grid()
	//   let dp = 0
	// for (let p of parts) {
	// if (p.deleted) {
	//   continue
	// }
	// p.direction = { x: 0, y: 0 };
	// for (let p2_idx of p.links) {
	// 	const p2 = parts[p2_idx];
	// 	const wa = wrap_around(p.p, p2.p);
	// 	p.direction = add(p.direction, delta(wa.b, wa.a));
	// }
	// p.direction = normalize(p.direction);
	// p.dp.x = p.p.x - p.pp.x;
	// p.dp.y = p.p.y - p.pp.y;
	// if (p.kind == "booster" && p.activated) {
	// 	p.dp.x -= p.direction.x * 0.0001;
	// 	p.dp.y -= p.direction.y * 0.0001;
	// }
	// p.np.x = p.p.x + p.dp.x;
	// p.np.y = p.p.y + p.dp.y;
	// p.link_response.x = 0;
	// p.link_response.y = 0;
	// p.collision_response.x = 0;
	// p.collision_response.y = 0;
	// p.collision_response.count = 0;
	// dp += distance_sqrd(p.dp);
	// }
	// for (let p1 of parts) {
	// 	// if (p1.deleted) {
	// 	// 	continue;
	// 	// }
	// 	for (let idx2 of neighbours(p1.p)) {
	// 		// const p2 = parts[idx2];
	// 		// if (p2.deleted) {
	// 		// 	continue;
	// 		// }
	// 		// if (p1.idx < p2.idx) {
	// 		// 	const wa = wrap_around(p1.np, p2.np);
	// 		// 	wa.a.np = {
	// 		// 		x: wa.a.x,
	// 		// 		y: wa.a.y,
	// 		// 	};
	// 		// 	wa.b.np = {
	// 		// 		x: wa.b.x,
	// 		// 		y: wa.b.y,
	// 		// 	};
	// 		// 	wa.a.dp = p1.dp;
	// 		// 	wa.b.dp = p2.dp;
	// 		// 	const d = wa.d_sqrd;
	// 		// 	const diams = (p1.d + p2.d) * 0.5;
	// 		// 	const diams_sqrd = diams * diams;
	// 		// 	if (d < diams_sqrd) {
	// 		// 		let emerald_idx = null;
	// 		// 		let player_id = null;
	// 		// 		if (p1.player_id !== undefined && p2.kind == "emerald") {
	// 		// 			emerald_idx = p2.idx;
	// 		// 			player_id = p1.player_id;
	// 		// 		} else if (p2.player_id !== undefined && p1.kind == "emerald") {
	// 		// 			emerald_idx = p1.idx;
	// 		// 			player_id = p2.player_id;
	// 		// 		}
	// 		// 		if (emerald_idx) {
	// 		// 			parts[emerald_idx].deleted = true;
	// 		// 			parts_deleted.add(emerald_idx);
	// 		// 			scores[player_id] += 1;
	// 		// 		}
	// 		// 		let cr = collision_response(wa.a, wa.b);
	// 		// 		if (links_set.has(`${p1.idx}|${p2.idx}`)) {
	// 		// 			cr.x *= 0.5;
	// 		// 			cr.y *= 0.5;
	// 		// 		}
	// 		// 		p1.collision_response.x -= cr.x;
	// 		// 		p1.collision_response.y -= cr.y;
	// 		// 		p1.collision_response.count += 1;
	// 		// 		p2.collision_response.x += cr.x;
	// 		// 		p2.collision_response.y += cr.y;
	// 		// 		p2.collision_response.count += 1;
	// 		// 	}
	// 		// }
	// 	}
	// }
	for (let link of links) {
		const p1 = parts[link.a];
		const p2 = parts[link.b];
		if (p1.deleted && p2.deleted) {
			link.deleted = true;
		}
		if (p1.deleted || p2.deleted || link.deleted) {
			continue;
		}
		const wa = wrap_around(p1.np, p2.np);
		const d = Math.sqrt(wa.d_sqrd);
		const n = normalize(delta(wa.a, wa.b), d);
		const ds = (p1.d + p2.d) * 0.5;
		const factor = (ds - d) * LINK_STRENGH;
		p1.link_response.x -= n.x * factor * 0.5;
		p1.link_response.y -= n.y * factor * 0.5;
		p2.link_response.x += n.x * factor * 0.5;
		p2.link_response.y += n.y * factor * 0.5;
	}
	for (let p of parts) {
		if (p.deleted) {
			continue;
		}
		if (p.collision_response.count) {
			p.collision_response.x /= p.collision_response.count;
			p.collision_response.y /= p.collision_response.count;
			p.np.x += p.collision_response.x;
			p.np.y += p.collision_response.y;
			p.np.x += p.link_response.x;
			p.np.y += p.link_response.y;
		}
		p.p.x = (p.np.x + 1) % 1.0;
		p.p.y = (p.np.y + 1) % 1.0;
		p.pp.x = p.p.x - p.dp.x - p.collision_response.x - p.link_response.x;
		p.pp.y = p.p.y - p.dp.y - p.collision_response.y - p.link_response.y;
	}
	for (var i = 0; i < emeralds.length; i++) {
		const emerald = emeralds[i];
		let s = 0;
		for (var idx of emerald) {
			if (parts[idx].deleted) {
				s += 1;
			}
		}
		if (s === 4) {
			emeralds[i] = new_emerald();
		}
	}
	update_ups();
	winning_condition();
	window.setTimeout(() => {
		compute();
	}, 10 - get_ups_avg_delta());
};
