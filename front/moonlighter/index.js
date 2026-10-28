import init, * as engine from "./engine.js";
import { draw } from "./draw.js";
import { setup } from "./setup.js";
import { debug } from "./debug.js";
import { 
    TIME_MODE,
} from "./consts.js";


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


const resizeCanvas = (canvas) => {
    canvas.width = canvas.clientWidth*2;
    canvas.height = canvas.clientHeight*2;
};


const setup_rendering = () => {
    const canvas = document.createElement('canvas');
    canvas.id = 'canvas';
    document.body.appendChild(canvas);
    resizeCanvas(canvas);
    window.addEventListener('resize', () => {resizeCanvas(canvas)});
    return canvas.getContext('2d');
}


const main = async () => {
    console.log("start")
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
