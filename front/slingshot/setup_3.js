import { fetch_as_json_string } from "./fetch.js";
const setup_3 = async (world) => {
	for (const url of [
		"/slingshot/material/steel.json",
		"/slingshot/material/launcher.json",
		"/slingshot/material/ice.json",
		"/slingshot/material/gold.json",
		"/slingshot/material/osmium.json",
	]) {
		world.add_material(url, await fetch_as_json_string(url));
	}
	world.add_cell(
		"/slingshot/material/gold.json",
		0.0, 0.1, 0.01
	)
	world.add_cell(
		"/slingshot/material/gold.json",
		0.1, 0.1, 0.01,
	)
	world.add_cell(
		"/slingshot/material/gold.json",
		0.1, 0.0, 0.01,
	)
	world.add_cell(
		"/slingshot/material/gold.json",
		0.1, 0.03, 0.01,
	)
	// world.set_cell_fixed(1);
	world.gravity_2 = 0.0025;
	world.gravity = 0.0000001;
	world.crdp = 0.00001;
	world.crdv = 0.00001;
};
export {
	setup_3,
}