import init, * as engine from "./engine.js";
import { draw } from "./draw.js";
import { setup } from "./setup.js";
import { debug } from "./debug.js";
import { 
    TIME_MODE,
    SOMETIMES,
    NEVER,
} from "./consts.js";


const data = {
    particles: [],
    links: [],
}


const step = (
    world, 
    context,
    engine, 
    memory,
) => {
    // for (const particle of data.particles) {
    //     particle.new_pressure = null
    //     particle.q_delta = {
    //         MOON: 0
    //     }
    // }
    // for (let i = 0; i < data.particles.length; i++) {
    //     const particle = data.particles[i];
    //     if (particle.kind === 'PUMP') {
    //         particle.new_pressure = -6;
    //     } else {
    //         const linkedPressures = data.links
    //             .filter((link) => link.a === i || link.b === i)
    //             .map((link) => link.a === i ? link.b : link.a)
    //             .map((index) => data.particles[index].pressure)
    //             .filter(Number.isFinite);
    //         if (linkedPressures.length > 0) {
    //             particle.new_pressure = Math.min(Math.min(...linkedPressures) + 1, 0);
    //         }
    //     }
    // }
    // // The pressure comparison permits transfer only from the lower-
    // // pressure endpoint to the higher-pressure endpoint, so the reverse
    // // visit cannot transfer in the opposite direction during this step.
    // const pairs = new Set();
    // for (const link of data.links) {
    //     const a = Math.min(link.a, link.b);
    //     const b = Math.max(link.a, link.b);
    //     pairs.add([a,b]);
    //     pairs.add([b,a]);
    // }
    // for (const pair of pairs) {
    //     const [ia, ib] = pair;
    //     const pa = data.particles[ia];
    //     const pb = data.particles[ib];
    //     let r = 1.0;
    //     if (pb.kind == 'FILTER') {
    //         // If the destination is a filter, 
    //         // things will stay stuck sometimes.
    //         r = SOMETIMES;
    //     }
    //     if (pa.kind == 'FILTER') {
    //         // If the source is a filter, 
    //         // nothing will ever pass through.
    //         r = NEVER;
    //     }
    //     if (
    //         pb.pressure < pa.pressure
    //         && pa.q.MOON > 0
    //         && pb.q.MOON < pb.c.MOON
    //         && r > Math.random()
    //     ) {
    //         pa.q_delta.MOON -= 1;
    //         pb.q_delta.MOON += 1;
    //     }
    // }
    // for (const particle of data.particles) {
    //     particle.pressure = particle.new_pressure
    //     particle.q.MOON += particle.q_delta.MOON
    // }
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
            step(world, 
            context,
            engine, 
            memory,)
        });
    } else if (TIME_MODE == "slow") {
        window.setTimeout(()=>{
            step(context, world, 
    context,
    engine, 
    memory,)
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
    console.log(world.step)
    // data.particles.add = (x, y, kind) => {
    //     let q_m = 0
    //     let capacity_m = 0
    //     if (kind=='MOON') {
    //         q_m = 10000
    //     }
    //     if (kind=='PIPE') {
    //         capacity_m = 10
    //     }
    //     if (kind=='TANK') {
    //         capacity_m = 100
    //     }
    //     if (kind=='PUMP') {
    //         capacity_m = 10
    //     }
    //     if (kind=='FILTER') {
    //         capacity_m = 10
    //     }
    //     data.particles.push({
    //         x: x,
    //         y: y,
    //         kind: kind,
    //         // Quantity: current quantity stored
    //         q: {
    //             MOON: q_m,
    //         },
    //         // Capacity: maximum quantity that can be stored
    //         c: {
    //             MOON: capacity_m,
    //         }
    //     });
    // };
    // data.links.add = (a, b) => {
    //     data.links.push({
    //         a: a,
    //         b: b,
    //     });
    // };
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
