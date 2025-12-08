import { fetch_as_text, fetch_as_json_string } from "./fetch.js";
const setup = async (world) => {
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
	world.add_from_blueprint(
		await fetch_as_text("/slingshot/blueprint/slingshot/material.txt"),
		0.0,
		0.0,
	);
	world.add_cell(
		"/slingshot/material/light_gold.json",
		0.006, 0.011, 0.01
	)
	world.add_cell(
		"/slingshot/material/osmium.json",
		0.0, 0.1, 0.02,
	)
	world.add_cell(
		"/slingshot/material/osmium.json",
		-0.01, 0.1, 0.02,
	)
	world.set_cell_fixed(7);
	world.set_cell_fixed(8);
	world.set_cell_fixed(0);
	world.set_cell_fixed(1);
	world.set_cell_fixed(2);
	world.set_cell_fixed(3);
	world.set_cell_fixed(4);
	world.set_cell_fixed(5);
	world.add_event(620, "set_cell_diameter", 0, 0.01143501)
	world.add_event(620, "set_cell_diameter", 2, 0.0113010001)
};
export {
	setup,
}