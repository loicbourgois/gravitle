import init, * as engine from "./engine.js";
// import { setup } from "./setup_1.js";
// import { setup } from "./setup_2.js";
// import { setup } from "./setup_3.js";
// import { setup } from "./setup_4.js";
// import { setup } from "./setup_5.js";
// import { setup } from "./setup_6.js";
// import { setup } from "./setup_7.js";
// import { setup } from "./setup_8.js";
import { setup } from "./setup/01_pump.js";
import { debug } from "./debug.js";
import { setup_rendering, setup_rendering_webgpu } from "./render.js";
import { make_triangle_pipeline } from "./draw_webgpu.js";
import { step } from "./step.js";


const main = async () => {
    console.log("Gravitle - Moonlighter")
    await init();
    engine.setup();
    const memory = engine.initSync().memory;
    const context = setup_rendering()
    const gpu = await setup_rendering_webgpu();
    let gpu_pipeline = null;
    if (gpu !== null) {
        gpu_pipeline = await make_triangle_pipeline(gpu.device, gpu.format);
    }
    const world = engine.World.new()
    const custom_logic = setup(engine, world)
    // debug(engine, world, memory)
    step(
        world,
        context,
        engine,
        memory,
        custom_logic,
        gpu,
        gpu_pipeline,
    )
};


window.onload = () => {
    main();
};