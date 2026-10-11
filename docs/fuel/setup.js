import { fetch_as_text, fetch_as_json_string } from "../slingshot/fetch.js";
const setup = async (
	world,
	r1,
	r2,
) => {
	world.set_gravity_2(0.00028)
	world.set_crdp(0.00001)
	world.set_crdv(0.00001)
	world.set_c2c_gravity(true)
	world.set_c2c_colision(true)
	for (const url of [
		"/slingshot/material/steel.json",
		"/slingshot/material/light_gold.json",
		"/slingshot/material/osmium.json",
	]) {
		world.add_material(url, await fetch_as_json_string(url));
	}
	world.add_cell(
		"/slingshot/material/steel.json",
		0.0, 0.0, 0.01
	)
	world.add_cell(
		"/slingshot/material/steel.json",
		0.01, 0.01, 0.01
	)
	world.add_cell(
		"/slingshot/material/steel.json",
		-0.01, 0.01, 0.01
	)
	world.add_link(
		0, 1
	)
	world.add_link(
		0, 2
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
