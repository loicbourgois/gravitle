import { 
    TIME_MODE,
} from "./consts.js";
import { draw } from "./draw.js";
import { draw_triangle_webgpu } from "./draw_webgpu.js";


const step = (
    world, 
    context,
    engine, 
    memory,
    custom_logic,
    gpu,
    gpu_pipeline,
) => {
    world.tick()
    custom_logic(world, engine, memory)
    draw(
        context,
        {
            x: canvas.width / 2,
            y: canvas.height / 2,
        },
        world,
        engine, 
        memory,
    );
    draw_triangle_webgpu(gpu.device, gpu.context, gpu_pipeline)
    if (TIME_MODE == "real time") {
        window.requestAnimationFrame(()=>{
            step(
                world, 
                context,
                engine, 
                memory,
                custom_logic,
                gpu,
                gpu_pipeline,
            )
        });
    } else if (TIME_MODE == "slow") {
        window.setTimeout(()=>{
            step(
                world, 
                context,
                engine, 
                memory,
                custom_logic,
                gpu,
                gpu_pipeline,
            )
        }, 500);
    } else {
        // pass
    }
};


export {
    step,
}
