import { 
    TIME_MODE,
} from "./consts.js";
import { draw } from "./draw.js";


const step = (
    world, 
    context,
    engine, 
    memory,
) => {
    world.tick()
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
            )
        });
    } else if (TIME_MODE == "slow") {
        window.setTimeout(()=>{
            step(
                world, 
                context,
                engine, 
                memory,
            )
        }, 500);
    } else {
        // pass
    }
};


export {
    step,
}
