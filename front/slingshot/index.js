import init, * as gravitle from "./gravitle_slingshot.js";





const main = async () => {
    await init();
	const world = gravitle.setup();
    console.log(world.cells_count())
    for (const url of [
        "/slingshot/definitions/ice.js",
    ]) {
        const module = await import(url);
        const definition = module.default;
        console.log(definition);
    }
}
main()
