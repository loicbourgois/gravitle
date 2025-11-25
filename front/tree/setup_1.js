import { fetch_as_json_string } from "../slingshot/fetch.js";
const fix_cells = (world) => {
	for (let index = 0; index < world.cells_count(); index++) {
		world.set_cell_fixed(index);
	}
}
const DIAM = 0.001

const gr = (min, max) => {
	return (max - min) * Math.random() + min 
}

const setup_1 = async (world) => {
	// world.set_gravity(0.000001)
	const W = "/slingshot/material/wood.json"
	const F = "/slingshot/material/foliage.json"
	const add_1 = (x,y) => {
		return world.add_cell(
			W,
			x, y, DIAM
		)
	}
	const add_up = (idx) => {
		return world.add_cell_up(
			W,
			idx, DIAM
		)
	}
	const add = (ia, ib) => {
		return world.add_cell_2(
			W,
			ia, ib, DIAM
		)
	}
	const addf = (ia, ib) => {
		return world.add_cell_2(
			F,
			ia, ib, DIAM
		)
	}
	for (const url of [
		W,
		F,
	]) {
		world.add_material(url, await fetch_as_json_string(url));
	}
	world.c.gravity_2 = 0.00001;
	world.c.crdp = 0.00000001;
	world.c.crdv = 0.00000001;
	world.c.zonesize = DIAM * 2.0
	const i0 = add_1(0, -0.05)
	const i1 = add_up(0)
	const i2 = add(i0, i1)
	const i3 = add(i2, i1)
	const i4 = add(i1, i0)
	const i5 = add(i1, i4)
	const i6 = add(i0, i2)
	const i7 = add(i4, i0)
	const top_left = (count, r1, r2) => {
		let ia = i3
		let ib = i2
		let ic = i1
		for (let index = 0; index < count; index++) {
			r1 += r2
			if (Math.random() > r1) {
				ic = add(ia, ic) // top
				ib = ia
				ia = add(ia, ic)
			} else {
				ib = add(ib, ia) // top-left
				ic = ia
				ia = add(ib, ia)
			}
		}
	}
	const top_right = (count, r1, r2) => {
		let ia = i5
		let ib = i4
		let ic = i1
		for (let index = 0; index < count; index++) {
			r1 += r2
			if (Math.random() > r1) { // top
				ic = add(ic, ia) 
				ib = ia
				ia = add(ic, ia)
			} else { // top-right
				ib = add(ia, ib) 
				ic = ia
				ia = add(ia, ib)
			}
		}
	}
	const bot_left = (count, r) => {
		let ia = i6
		let ib = i0
		let ic = i2
		for (let index = 0; index < count; index++) {
			if (Math.random() > r) { // bot
				ib = add(ib, ia) 
				ic = ia
				ia = add(ib, ia)
			} else { // bot-left
				ic = add(ia, ic) 
				ib = ia
				ia = add(ia, ic)
			}
		}
	}
	const bot_right = (count, r) => {
		let ia = i7
		let ib = i0
		let ic = i4
		for (let index = 0; index < count; index++) {
			if (Math.random() > r) { // bot
				ib = add(ia, ib) 
				ic = ia
				ia = add(ia, ib)
			} else { // bot-left
				ic = add(ic, ia) 
				ib = ia
				ia = add(ic, ia)
			}
		}
	}
	const branch_size = 200
	const trunk_size = 100
	const branch_count = 30
	for (let index = 0; index < branch_count; index++) {
		const r1 = Math.random()
		const r2 = gr(0.4, 1.0)
		const r3 = gr(-0.01, 0)
		if (r1 > 0.5) {
			top_left(branch_size, r2, r3)
		} else {
			top_right(branch_size, r2, r3)
		}
		
	}
	for (let index = 0; index < 40; index++) {
		const r1 = Math.random()
		const r2 = gr(0, 0.03)
		if (r1 > 0.5) {
			bot_left(trunk_size, r2)
		} else {
			bot_right(trunk_size, r2)
		}
	}
	// bot_left(trunk_size, 0)
	// bot_left(trunk_size, 0.01)
	// bot_left(trunk_size, 0.01)
	// bot_right(trunk_size, 0)
	// bot_right(trunk_size, 0.01)
	// bot_right(trunk_size, 0.01)
	// bot_right(trunk_size, 0.01)
	// fix_cells(world)
};
export {
	setup_1,
}