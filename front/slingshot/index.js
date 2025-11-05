import init, * as gravitle from "./gravitle_slingshot.js";

const main = async () => {
    await init();
	const world = gravitle.setup();
    for (const url of [
        "/slingshot/material/ice.js",
    ]) {
        const module = await import(url);
        world.add_material(url, JSON.stringify(module.default))
    }
    world.add_cell(
        "/slingshot/material/ice.js",
        0,
        0,
    )
}
main()
