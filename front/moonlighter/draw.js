import { ZOOM } from "./consts.js";
import { get_particle } from "./particle.js";
import { get_link } from "./link.js";
// const COLOR = '#eee'




const draw_diam = (canvas) => {
    return Math.min(canvas.height, canvas.width) * ZOOM;
}


const draw = (ctx, center, world, engine, memory) => {
    ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height);
    const diameter = draw_diam(ctx.canvas);
    const radius = diameter / 2;
    const particles_ptr = world.particles();
	const particle_size = engine.Particle.size();
	const particles_view = new DataView(
		memory.buffer,
		particles_ptr,
		world.particles_count() * particle_size,
	);
	for (let i = 0; i < world.particles_count(); i++) {
		const particle = get_particle(particles_view, particle_size, i);
        const p = particle.ap;
        ctx.beginPath();
        ctx.arc(p.x*diameter+center.x, -p.y*diameter+center.y, radius, 0, Math.PI * 2);
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
        // console.log()
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
        if (Number.isFinite(p.pr)) {
            const x = p.ap.x * diameter + center.x;
            const y = -p.ap.y * diameter + center.y;
            const label = String(p.pr);
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
            ctx.fillStyle = "#eee";
            ctx.fillText(label, labelX, labelY);
        }
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
            ctx.fillStyle = "#eee";
            ctx.fillText(label, labelX, labelY);
    }
}


export {
    draw,
}
