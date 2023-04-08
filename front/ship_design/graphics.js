import {
    fill_circle_2,
    clear,
    set_draw_zoom,
} from "../canvas.js"
import {
    colors,
    colors2,
} from "../colors.js"
import {
    particle,
} from "./particle.js"
import {
    link_js,
} from "../link.js"


const Graphics = (gravithrust, wasm, context) => {
    set_draw_zoom(30)
    const self = {
        start: () => { start(self) },
        gravithrust: gravithrust,
        wasm: wasm,
        context: context,
        draw: () => { draw(self) },
    }
    return self
}


const start = (self) => {
    console.log("plouf")
    self.draw()
}


const draw = (self) => {
    // self.gravithrust.ticks()
    clear(self.context)

    const particle_size = self.gravithrust.particle_size()
    const particles_view = new DataView(
        self.wasm.memory.buffer, 
        self.gravithrust.particles(), 
        self.gravithrust.particles_size(),
    );
    for (let pid = 0; pid < self.gravithrust.particles_count(); pid++) {
        const p = particle(particles_view, pid*particle_size);
        // fill_circle_2(self.context, p, self.gravithrust.diameter*1.1, colors[p.k].low)
        if (p.k == 3 ) {
            fill_circle_2(self.context, p, self.gravithrust.diameter*1, "#b40")
        }
    }

    const link_js_size = self.gravithrust.link_js_size()
    const links_js_view = new DataView(
        self.wasm.memory.buffer, 
        self.gravithrust.links_js(), 
        self.gravithrust.links_js_size(),
    );
    for (let lid = 0; lid < self.gravithrust.links_count(); lid++) {
        const l = link_js(links_js_view, lid*link_js_size);
        fill_circle_2(self.context, l, self.gravithrust.diameter , "#da4")
    }

    for (let pid = 0; pid < self.gravithrust.particles_count(); pid++) {
        const p = particle(particles_view, pid*particle_size);
        if (p.k != 3 && p.k != 2) {
            fill_circle_2(self.context, p, self.gravithrust.diameter*1, "#da4")
        }
    }

    for (let pid = 0; pid < self.gravithrust.particles_count(); pid++) {
        const p = particle(particles_view, pid*particle_size);
        if (p.k == 2 ) {
            fill_circle_2(self.context, p, self.gravithrust.diameter*1, "#c83")
        }
    }

    
    

    
    // const ships_data_ptr = gravithrust.ships();
    // const ships = new DataView(wasm.memory.buffer, ships_data_ptr, gravithrust.ships_size());
    // for (let i = 0; i < gravithrust.particles_count(); i++) {
    //   const p = P(i);
    //   fill_circle_2(context, p, gravithrust.diameter*1.1, colors[p.k].low)
    //   if (p.k == 3 && p.a == 1) {
    //     fill_circle_2(context, p, gravithrust.diameter*1.1, colors2['boost'])
    //   }
    // }
    
    requestAnimationFrame(self.draw)
}


export {
    Graphics,
}
