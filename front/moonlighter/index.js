import init, * as engine from "./engine.js";
// import { setup } from "./setup_1.js";
// import { setup } from "./setup_2.js";
// import { setup } from "./setup_3.js";
// import { setup } from "./setup_4.js";
// import { setup } from "./setup_5.js";
import { setup } from "./setup_6.js";
// import { setup } from "./setup_7.js";
import { debug } from "./debug.js";
import { setup_rendering } from "./render.js";
import { step } from "./step.js";


const main = async () => {
    console.log("Gravitle - Moonlighter")
    await init();
	engine.setup();
	const memory = engine.initSync().memory;
    const context = setup_rendering()
    const world = engine.World.new()
    setup(engine, world)
    // debug(engine, world, memory)
    step(
        world, 
        context,
        engine, 
        memory,
    )
};


window.onload = () => {
	main();
};
