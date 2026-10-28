# General Context

You are improving the codebase.
Your role is to address any TODO left in the code.
Only write down the actual changes, with some context around.
Do not format as a git diff.
Answer as a markdown document:


## Output
````
# Reasoning
<thorough reasoning>


# Updates


## <path #1>


### <change #1>
```<language>
<code>
```


### <change #2>
```<language>
<code>
```

````


# Specific Rules
- keep code tidy


# Files
## file://./index.js

```javascript
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
    // Each particle visits all links that contain its index, so every
    // undirected link is considered from both endpoints: once as (a, b)
    // and once as (b, a). A pair is therefore processed twice, not once.
    // The pressure comparison permits transfer only from the lower-
    // pressure endpoint to the higher-pressure endpoint, so the reverse
    // visit cannot transfer in the opposite direction during this step.

    // TODO: update code to 
    //  1 - list all pairs 
    //  2 - sort pairs, with a < b always
    //  3 - process each pair only once 

    for (let ia = 0; ia < data.particles.length; ia++) {
        const pa = data.particles[ia];
        const linkedIndices = data.links
            .filter((link) => link.a === ia || link.b === ia)
            .map((link) => link.a === ia ? link.b : link.a);
        for (const ib of linkedIndices) {
            const pb = data.particles[ib];
            let r = 1.0
            if (
                pa.pressure < pb.pressure
                && pb.q.MOON  > 0
                && pa.q.MOON  < pa.c.MOON
                && r > Math.random()
            ) {
                pa.q_delta.MOON += 1;
                pb.q_delta.MOON -= 1;
            }
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
            q: {
                MOON: q_m,
            },
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
```

