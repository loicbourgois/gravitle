import { get_particle } from "./particle.js";


const setup = (engine, world) => {
    world.add_particle(0, 0, engine.Kind.Moon)
    world.add_particle(1, 0, engine.Kind.Moon)
    world.add_particle(-1, 0, engine.Kind.Moon);
    world.add_particle(0, 1, engine.Kind.Moon);
    world.add_particle(0, -1, engine.Kind.Moon);
    const v1 = world.add_particle(5, 0.1, engine.Kind.Valve);
    world.add_particle(4, 0, engine.Kind.Pipe);
    world.add_particle(3, 0.1, engine.Kind.Pipe);
    const p = world.add_particle(2, -0.1, engine.Kind.Pipe);
    const f1 = world.add_particle(6, 0.1, engine.Kind.Filter);
    const p1 = world.add_particle(8, -1.0, engine.Kind.Pump);
    const b1 = world.add_particle(7.1, 1.2, engine.Kind.Battery);
    const b2 = world.add_particle(9.1, 1., engine.Kind.Battery);
    const sp1 = world.add_particle(7, 2.2, engine.Kind.SolarPanel);
    const p2 = world.add_particle(8.1, 1.1, engine.Kind.Pump);
    const sp2 = world.add_particle(9, 2.2, engine.Kind.SolarPanel);
    // const sp3 = world.add_particle(10, 2, engine.Kind.SolarPanel);
    const w1 = world.add_particle(7.6, 1.8, engine.Kind.Wire);
    const w2 = world.add_particle(8, 2, engine.Kind.Wire);
    const w3 = world.add_particle(8.4, 1.8, engine.Kind.Wire);
    const w4 = world.add_particle(9, 0, engine.Kind.Wire);
    const w5 = world.add_particle(9, -1, engine.Kind.Wire);
    const l1 = world.add_particle(9, -2, engine.Kind.Led);
    const w6 = world.add_particle(8, -2, engine.Kind.Wire);
    const s1 = world.add_particle(8, -3, engine.Kind.Switch);
    const l2 = world.add_particle(7, -3, engine.Kind.Led);
    const bl1 = world.add_particle(6, -3, engine.Kind.Blower);
    const p5 = world.add_particle(6, -2, engine.Kind.Pipe);
    const p6 = world.add_particle(6, -1, engine.Kind.Pipe);
    const v2 = world.add_particle(7, 0.1, engine.Kind.Valve);
    const v3 = world.add_particle(6, 1, engine.Kind.Valve);
    const p8 = world.add_particle(6, 2, engine.Kind.Pipe);
    const sp4 = world.add_particle(7, -2, engine.Kind.SolarPanel);
    const t1 = world.add_particle(4, 2, engine.Kind.Tank);
    const p9 = world.add_particle(5, 2, engine.Kind.Pipe);
    world.add_link(5, 6)
    world.add_link(6, 7)
    world.add_link(7, 8)
    world.add_link(8, 1)
    world.add_link(5, f1)
    world.add_link(v2, f1)
    world.add_link(p1, v2)
    world.add_link(b2, p1)
    world.add_link(b1, sp1)
    world.add_link(b1, p2)
    world.add_link(b2, p2)
    world.add_link(b2, sp2)
    // world.add_link(b2, sp3)
    world.add_link(b1, w1)
    world.add_link(w1, w2)
    world.add_link(w2, w3)
    world.add_link(w3, b2)
    world.add_link(b2, w4)
    world.add_link(w4, w5)
    world.add_link(l1, w5)
    world.add_link(l1, w6)
    world.add_link(s1, w6)
    world.add_link(s1, l2)
    world.add_link(bl1, l2)
    world.add_link(bl1, p5)
    world.add_link(p5, p6)
    world.add_link(p6, f1)
    world.add_link(v3, f1)
    world.add_link(v3, p8)
    world.add_link(sp4, w6)
    world.add_link(p8, p9)
    world.add_link(p9, t1)
    world.set_activation(v1, 1);
    world.set_activation(v2, 1);
    let switched = false;
    const on_keydown = (event) => {
        if (event.key.toLowerCase() !== "s" || event.repeat) return;
        switched = !switched;
        world.set_activation(s1, switched ? 1 : 0);
        world.set_activation(v1, switched ? 0 : 1);
        world.set_activation(v2, switched ? 0 : 1);
        world.set_activation(v3, switched ? 1 : 0);
    };
    window.addEventListener("keydown", on_keydown);
    return (world, engine, memory) => {
        const particles_ptr = world.particles();
        const particle_size = engine.Particle.size();
        const particles_view = new DataView(
            memory.buffer,
            particles_ptr,
            world.particles_count() * particle_size,
        );
        const f1p = get_particle(particles_view, particle_size, f1);
        if (f1p.moon_q >= 3) {
            world.set_activation(s1, 1);
            world.set_activation(v1, 0);
            world.set_activation(v2, 0);
            world.set_activation(v3, 1);
        } else {
            world.set_activation(s1,  0);
            world.set_activation(v1, 1);
            world.set_activation(v2, 1);
            world.set_activation(v3, 0);
        }
    }
}

export {
    setup,
}
