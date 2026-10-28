import { ZOOM } from "./consts.js";
const COLOR = '#eee'

const STYLE = {
    MOON: '#eee',
    EXTRACTOR: '#88d',
    PUMP: '#d88',
    PIPE: '#b88',
    LINK: '#ff8',
    FILTER: 'rgb(136, 164, 255)',
}


const draw_diam = (canvas) => {
    return Math.min(canvas.height, canvas.width) * ZOOM;
}


const draw = (ctx, centerX, centerY, data) => {
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


export {
    draw,
}
