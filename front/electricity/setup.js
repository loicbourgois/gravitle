import { fetch_as_json_string } from "../slingshot/fetch.js";
const setup = async (
	world,
) => {
	world.set_gravity_2(0.00028)
	world.set_crdp(0.00001)
	world.set_crdv(0.00001)
	world.set_c2c_gravity(true)
	world.set_c2c_colision(true)
	for (const url of [
		"/slingshot/material/steel.json",
		"/slingshot/material/lfp_battery.json",
		"/slingshot/material/solar_panel.json",
		"/slingshot/material/ion_thruster.json",
	]) {
		world.add_material(url, await fetch_as_json_string(url));
	}
	world.add_cell(
		"/slingshot/material/solar_panel.json",
		0.0, 0.0, 0.01
	)
	world.add_cell(
		"/slingshot/material/steel.json",
		0.011, 0.0, 0.01
	)
	world.add_cell(
		"/slingshot/material/lfp_battery.json",
		0.022, 0.0, 0.01
	)
	world.add_cell(
		"/slingshot/material/steel.json",
		0.033, 0.0, 0.01
	)
	world.add_cell(
		"/slingshot/material/ion_thruster.json",
		0.044, 0.0, 0.01
	)
	world.add_link(
		0, 1
	)
	world.add_link(
		1, 2
	)
	world.add_link(
		2, 3
	)
	world.add_link(
		3, 4
	)
	// TODO: draw link
	// world.add_container(
	// 	cell_id=0,
	// 	content=wasm_engine.Content.Fuel,
	// )
	// world.add_container(
	// 	cell_id=0,
	// 	content=wasm_engine.Content.Fuel,
	// )
	// world.con
	world.set_cell_fixed(0);
	world.set_cell_fixed(1);
};
export {
	setup,
}
