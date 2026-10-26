const COLOR = '#eee'
const ZOOM = 1/16


// const kinds = {
//     MOON: 0,
//     EXTRACTOR: 0,
// }


// const k = {
//     M: kinds.MOON,
//     E: kinds.EXTRACTOR,
// }


// todo: cleanup
const STYLE = {
    MOON: '#eee',
    EXTRACTOR: '#88d',
    PUMP: '#d88',
    PIPE: '#b88',
    LINK: '#ff8',
}


const data = {
    particles: [],
    links: [],
}


const draw_circle = (ctx, x, y, radius) => {
    ctx.beginPath();
    ctx.arc(x, y, radius, 0, Math.PI * 2);
    ctx.strokeStyle = COLOR;
    ctx.stroke();
}

const draw_particle = (ctx, p) => {
    ctx.beginPath();
    ctx.arc(p.x, p.y, 1, 0, Math.PI * 2);
    ctx.fillStyle = COLOR;
    ctx.fill();
}

const draw_diam = (canvas) => {
    return Math.min(canvas.height, canvas.width) * ZOOM;
}


const draw = (ctx, centerX, centerY) => {
    ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height);
    const diameter = draw_diam(ctx.canvas);
    const radius = diameter / 2;
    for (const p of data.particles) {
        ctx.beginPath();
        ctx.arc(p.x*diameter+centerX, -p.y*diameter+centerY, radius, 0, Math.PI * 2);
        ctx.fillStyle = STYLE[p.kind] || COLOR;
        ctx.fill();
    }
    for (const link of data.links) {
        const a = data.particles[link.a];
        const b = data.particles[link.b];
        if (a && b) {
            ctx.beginPath();
            ctx.moveTo(a.x * diameter + centerX, -a.y * diameter + centerY);
            ctx.lineTo(b.x * diameter + centerX, -b.y * diameter + centerY);
            ctx.strokeStyle = STYLE.LINK;
            ctx.stroke();
        }
    }
    for (const p of data.particles) {
        if (Number.isFinite(p.pressure)) {
            const x = p.x * diameter + centerX;
            const y = -p.y * diameter + centerY;
            const label = String(p.pressure);
            ctx.font = '16px monospace';
            ctx.textAlign = 'left';
            ctx.textBaseline = 'middle';
            const labelX = x  ;
            const labelY = y - 16;
            const padding = 3;
            const labelWidth = ctx.measureText(label).width;
            ctx.fillStyle = '#111';
            ctx.fillRect(
                labelX - padding,
                labelY - 8 - padding,
                labelWidth + padding * 2,
                16 + padding * 2,
            );
            ctx.fillStyle = COLOR;
            ctx.fillText(label, labelX, labelY);
        }
    }
    for (const p of data.particles) {
        if ( Number.isFinite(p.q?.MOON)) {
            const x = p.x * diameter + centerX;
            const y = -p.y * diameter + centerY;
            const label = `m:${p.q.MOON}`;
            ctx.font = '16px monospace';
            ctx.textAlign = 'left';
            ctx.textBaseline = 'middle';
            const labelX = x;
            const labelY = y + 16;
            const padding = 3;
            const labelWidth = ctx.measureText(label).width;
            ctx.fillStyle = '#111';
            ctx.fillRect(
                labelX - padding,
                labelY - 8 - padding,
                labelWidth + padding * 2,
                16 + padding * 2,
            );
            ctx.fillStyle = COLOR;
            ctx.fillText(label, labelX, labelY);
        }
    }
}


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
            q_m = 1000000
        }
        if (kind=='PIPE') {
            capacity_m = 10000
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
    data.particles.add(0, 0, 'MOON');
    data.particles.add(1, 0, 'MOON');
    data.particles.add(-1, 0, 'MOON');
    data.particles.add(0, 1, 'MOON');
    data.particles.add(0, -1, 'MOON');
    data.particles.add(5, 0.1, 'PUMP');
    data.particles.add(4, 0, 'PIPE');
    data.particles.add(3, 0.1, 'PIPE');
    data.particles.add(2, -0.1, 'PIPE');
    data.links.add(5, 6)
    data.links.add(6, 7)
    data.links.add(7, 8)
    data.links.add(8, 1)
    const step = () => {
        const pressures = data.particles.map((particle) => particle.pressure);
        for (let i = 0; i < data.particles.length; i++) {
            const particle = data.particles[i];
            if (particle.kind === 'PUMP') {
                pressures[i] = -5;
            } else {
                const linkedPressures = data.links
                    .filter((link) => link.a === i || link.b === i)
                    .map((link) => link.a === i ? link.b : link.a)
                    .map((index) => pressures[index])
                    .filter(Number.isFinite);
                if (linkedPressures.length > 0) {
                    pressures[i] = Math.min(Math.min(...linkedPressures) + 1, 0);
                }
            }
        }
        data.particles.forEach((particle, index) => {
            particle.pressure = pressures[index];
        });

        // TODO: 
        // if particle.pressure < neighoubr.pressure && neighoubr.q.MOON > 0 && particle.q.MOON == 0
        // then
        //  particle.q.MOON += 1
        //  neighoubr.q.MOON -= 1
        //
        for (let i = 0; i < data.particles.length; i++) {
            const particle = data.particles[i];
            if (particle.pressure === undefined || particle.q === undefined) continue;

            const linkedIndices = data.links
                .filter((link) => link.a === i || link.b === i)
                .map((link) => link.a === i ? link.b : link.a);

            for (const neighborIndex of linkedIndices) {
                const neighbor = data.particles[neighborIndex];
                if (neighbor.pressure === undefined || neighbor.q === undefined) continue;
                if (particle.pressure < neighbor.pressure &&
                    (neighbor.q.MOON ) > 0 &&
                    (particle.q.MOON ) < particle.c.MOON) {
                    // Initialize q if not present
                    if (!particle.q) particle.q = {};
                    if (!neighbor.q) neighbor.q = {};
                    particle.q.MOON = (particle.q.MOON || 0) + 1;
                    neighbor.q.MOON = (neighbor.q.MOON || 0) - 1;
                }
            }
        }

        const centerX = canvas.width / 2;
        const centerY = canvas.height / 2;
        draw(ctx, centerX, centerY);
            window.requestAnimationFrame(step);
        // window.setTimeout(step, 100);
    };
    step()
};


window.onload = () => {
	main();
};
