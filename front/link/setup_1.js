import { fetch_as_json_string } from "../slingshot/fetch.js";

const DIAM = 0.002
const G = "/slingshot/material/gold.json"

const setup_1 = async (world) => {
	world.set_gravity(0.0000001)
	world.set_link_strength_dp(0.1);
	world.set_link_strength_dv(0.2);
	world.set_crdp(0.0001)
	world.set_crdv(0.0001)
	// world.set_rdp(0.1)
	// world.set_rdv(0.01)
	world.set_c2c_colision(true)
	world.set_c2c_gravity(true)
	world.set_gravity_2(-0.001)
	world.set_zonesize(DIAM*2)
	const add_1 = (x,y) => {
		return world.add_cell(
			G,
			x, y, DIAM
		)
	}
	const add_up = (idx) => {
		return world.add_cell_down(
			G,
			idx, DIAM
		)
	}
	for (const url of [
		G,
	]) {
		world.add_material(url, await fetch_as_json_string(url));
	}
	add_1(0.0, 0.1)
	for (let index = 0; index < 100; index++) {
		add_up(index)
		world.add_link(index, index+1);
	}
	const i2 = add_1(-0.07, 0.07)
	for (let index = 0; index < 100; index++) {
		add_up(i2+index)
		world.add_link(i2+index, i2+index+1);
	}
	const i3 = add_1(0.07, 0.07)
	for (let index = 0; index < 100; index++) {
		add_up(i3+index)
		world.add_link(i3+index, i3+index+1);
	}
};
export {
	setup_1,
}