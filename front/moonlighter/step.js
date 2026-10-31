import { 
    TIME_MODE,
} from "./consts.js";
import { draw } from "./draw.js";


const step = (
    world, 
    context,
    engine, 
    memory,
    custom_logic,
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
    if (TIME_MODE == "real time") {
        window.requestAnimationFrame(()=>{
            step(
                world, 
                context,
                engine, 
                memory,
                custom_logic,
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
            )
        }, 500);
    } else {
        // pass
    }
};


export {
    step,
}
