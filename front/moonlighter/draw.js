import { ZOOM } from "./consts.js";
import { get_particle } from "./particle.js";
import { get_link } from "./link.js";


const history = {
    esum: [],
    emax: [],
    psum: [],
    pmax: [],
}



const draw_diam = (canvas) => {
    return Math.min(canvas.height, canvas.width) * ZOOM;
}


const draw = (ctx, center, world, engine, memory) => {
    ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height);
    const diameter = draw_diam(ctx.canvas);
    const particles_ptr = world.particles();
	const particle_size = engine.Particle.size();
	const particles_view = new DataView(
		memory.buffer,
		particles_ptr,
		world.particles_count() * particle_size,
	);
    let aa = `${world.step}\t|`
    let esum = 0
    let psum = 0
    let emax = 0
    let pmax = 0
	for (let i = 0; i < world.particles_count(); i++) {
		const particle = get_particle(particles_view, particle_size, i);
        esum += particle.electricity_q
        emax += particle.electricity_c
        aa = `${aa}\t${particle.electricity_q}`
	}
    aa = `${aa}\t${esum}\t|`
    for (let i = 0; i < world.particles_count(); i++) {
		const particle = get_particle(particles_view, particle_size, i);
        aa = `${aa}\t${particle.pressure_q}`
        if (particle.kind === engine.Kind.Pump) {  // Only for pumps
            psum += particle.pressure_q
            pmax += particle.pressure_c
        }
	}
    history.esum.push(esum)
    history.psum.push(psum)
    history.emax.push(emax)
    history.pmax.push(pmax)
    if (world.step <= 24) {
        console.log(aa)
    }
    for (let i = 0; i < world.particles_count(); i++) {
		const particle = get_particle(particles_view, particle_size, i);
        const p = particle.ap;
        ctx.beginPath();
        ctx.arc(p.x*diameter+center.x, -p.y*diameter+center.y, diameter/2*particle.diameter, 0, Math.PI * 2);
        ctx.fillStyle = engine.base_color(particle.kind);
        ctx.fill();
	}
    const links_ptr = world.links();
	const link_size = engine.Link.size();
	const links_view = new DataView(
		memory.buffer,
		links_ptr,
		world.links_count() * link_size,
	);
    for (let i = 0; i < world.links_count(); i++) {
        const link = get_link(links_view, link_size, i);
        const a = get_particle(particles_view, particle_size, link.a);
        const b = get_particle(particles_view, particle_size, link.b);
        ctx.beginPath();
        ctx.moveTo(a.ap.x * diameter + center.x, -a.ap.y * diameter + center.y);
        ctx.lineTo(b.ap.x * diameter + center.x, -b.ap.y * diameter + center.y);
        ctx.strokeStyle = "#ff8";
        ctx.stroke();
    }
    for (let i = 0; i < world.particles_count(); i++) {
        const p = get_particle(particles_view, particle_size, i);
            const x = p.ap.x * diameter + center.x;
            const y = -p.ap.y * diameter + center.y;
            const label = `p:${p.pressure_q}`;
            ctx.font = '16px monospace';
            ctx.textAlign = 'left';
            ctx.textBaseline = 'middle';
            const labelX = x  ;
            const labelY = y + 20*3;
            const padding = 3;
            const labelWidth = ctx.measureText(label).width;
            ctx.fillStyle = '#111';
            ctx.fillRect(
                labelX - padding,
                labelY - 8 - padding,
                labelWidth + padding * 2,
                16 + padding * 2,
            );
            ctx.fillStyle = "#eee";
            ctx.fillText(label, labelX, labelY);
    }
    for (let i = 0; i < world.particles_count(); i++) {
        const p = get_particle(particles_view, particle_size, i);
            const x = p.ap.x * diameter + center.x;
            const y = -p.ap.y * diameter + center.y;
            const label = `m:${p.moon_q}`;
            ctx.font = '16px monospace';
            ctx.textAlign = 'left';
            ctx.textBaseline = 'middle';
            const labelX = x;
            const labelY = y + 20;
            const padding = 3;
            const labelWidth = ctx.measureText(label).width;
            ctx.fillStyle = '#111';
            ctx.fillRect(
                labelX - padding,
                labelY - 8 - padding,
                labelWidth + padding * 2,
                16 + padding * 2,
            );
            ctx.fillStyle = "#eee";
            ctx.fillText(label, labelX, labelY);
    }

    for (let i = 0; i < world.particles_count(); i++) {
        const p = get_particle(particles_view, particle_size, i);
            const x = p.ap.x * diameter + center.x;
            const y = -p.ap.y * diameter + center.y;
            const l = String(p.electricity_c).length
            const label = `e:${String(p.electricity_q).padStart(l, " ")}/${p.electricity_c}`;
            ctx.font = '16px monospace';
            ctx.textAlign = 'left';
            ctx.textBaseline = 'middle';
            const labelX = x;
            const labelY = y + 20*2;
            const padding = 3;
            const labelWidth = ctx.measureText(label).width;
            ctx.fillStyle = '#111';
            ctx.fillRect(
                labelX - padding,
                labelY - 8 - padding,
                labelWidth + padding * 2,
                16 + padding * 2,
            );
            ctx.fillStyle = "#eee";
            ctx.fillText(label, labelX, labelY);
    }
    
    // for elec and pressure, draw {n}max as well
    // we should always see {n}max squares
    // use the full color for esum, then the empty color for the rest
    const ec_empty = "#8808"
    const ec_full = "#ff08"
    const pc_empty = "#488"
    const pc_full = "#8f8"

    // Elect draw
    // Keep the first sample at the bottom-left; later samples extend right.
    const scrollHistory = true
    const squareSize = 6;
    const squareStep = squareSize + 1;
    const visibleColumns = Math.floor(
        (ctx.canvas.width - squareSize) / squareStep,
    ) + 1;
    const startColumn = scrollHistory
        ? Math.max(0, history.esum.length - Math.max(0, visibleColumns))
        : 0;
    ctx.fillStyle = ec_full;
    for (
        let column = 0;
        column < visibleColumns &&
        startColumn + column < history.esum.length;
        column++
    ) {
        const index = startColumn + column;
        const esum = history.esum[index];
        const emax = history.emax[index];
        const x = column * squareStep;

        for (
            let row = 0;
            row < emax &&
            row * squareStep + squareSize <= ctx.canvas.height;
            row++
        ) {
            const y = ctx.canvas.height - squareSize - row * squareStep;
            ctx.fillStyle = row < esum ? ec_full : ec_empty;
            ctx.fillRect(x, y, squareSize, squareSize);
        }
    }
    // Pressure draw
    for (
        let column = 0;
        column < visibleColumns &&
        startColumn + column < history.psum.length;
        column++
    ) {
        const psum = -history.psum[startColumn + column];
        const pmax = -history.pmax[startColumn + column];
        const x = column * squareStep;
        const yStart = 0;  // Start from top
        for (
            let row = 0;
            row < pmax &&
            row * squareStep + squareSize <= ctx.canvas.height;
            row++
        ) {
            const y = yStart + row * squareStep;
            ctx.fillStyle = row < psum ? pc_full : pc_empty;
            ctx.fillRect(x, y, squareSize, squareSize);
        }
    }
}


export {
    draw,
}
