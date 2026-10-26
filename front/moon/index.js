import { draw } from "./draw.js";
import { setup } from "./setup.js";
import { TIME_MODE } from "./consts.js";


const data = {
    particles: [],
    links: [],
}


const step = (ctx) => {
    for (const particle of data.particles) {
        particle.new_pressure = null
        particle.q_delta = {
            MOON: 0
        }
    }
    for (let i = 0; i < data.particles.length; i++) {
        const particle = data.particles[i];
        if (particle.kind === 'PUMP') {
            particle.new_pressure = -6;
        } else {
            const linkedPressures = data.links
                .filter((link) => link.a === i || link.b === i)
                .map((link) => link.a === i ? link.b : link.a)
                .map((index) => data.particles[index].pressure)
                .filter(Number.isFinite);
            if (linkedPressures.length > 0) {
                particle.new_pressure = Math.min(Math.min(...linkedPressures) + 1, 0);
            }
        }
    }
    // The pressure comparison permits transfer only from the lower-
    // pressure endpoint to the higher-pressure endpoint, so the reverse
    // visit cannot transfer in the opposite direction during this step.
    const pairs = new Set();
    for (const link of data.links) {
        const a = Math.min(link.a, link.b);
        const b = Math.max(link.a, link.b);
        pairs.add([a,b]);
        pairs.add([b,a]);
    }
    for (const pair of pairs) {
        const [ia, ib] = pair;
        const pa = data.particles[ia];
        const pb = data.particles[ib];
        let r = 1.0;
        if (
            pa.pressure < pb.pressure
            && pb.q.MOON > 0
            && pa.q.MOON < pa.c.MOON
            && r > Math.random()
        ) {
            pa.q_delta.MOON += 1;
            pb.q_delta.MOON -= 1;
        }
    }
    for (const particle of data.particles) {
        particle.pressure = particle.new_pressure
        particle.q.MOON += particle.q_delta.MOON
    }
    const centerX = canvas.width / 2;
    const centerY = canvas.height / 2;
    draw(ctx, centerX, centerY, data);
    if (TIME_MODE == "real time") {
        window.requestAnimationFrame(()=>{
            step(ctx)
        });
    } else if (TIME_MODE == "slow") {
        window.setTimeout(()=>{
            step(ctx)
        }, 500);
    } else {
        // pass
    }
};


const main = async () => {
    console.log("start")
    const canvas = document.createElement('canvas');
    canvas.id = 'canvas';
    document.body.appendChild(canvas);
    const ctx = canvas.getContext('2d');
    const resizeCanvas = () => {
        canvas.width = canvas.clientWidth*2;
        canvas.height = canvas.clientHeight*2;
    };
    resizeCanvas();
    window.addEventListener('resize', resizeCanvas);
    data.particles.add = (x, y, kind) => {
        let q_m = 0
        let capacity_m = 0
        if (kind=='MOON') {
            q_m = 10000
        }
        if (kind=='PIPE') {
            capacity_m = 10
        }
        if (kind=='TANK') {
            capacity_m = 100
        }
        if (kind=='PUMP') {
            capacity_m = 10
        }
        if (kind=='FILTER') {
            capacity_m = 10
        }
        data.particles.push({
            x: x,
            y: y,
            kind: kind,
            // Quantity: current quantity stored
            q: {
                MOON: q_m,
            },
            // Capacity: maximum quantity that can be stored
            c: {
                MOON: capacity_m,
            }
        });
    };
    data.links.add = (a, b) => {
        data.links.push({
            a: a,
            b: b,
        });
    };
    setup(data)
    step(ctx)
};


window.onload = () => {
	main();
};
