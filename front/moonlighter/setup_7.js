const setup = (engine, world) => {
    let X = -5
    const m2 = world.add_particle(X+1, 0, engine.Kind.Moon)
    const v1 = world.add_particle(X+5, 0, engine.Kind.Valve);
    const t0 =world.add_particle(X+4, 0, engine.Kind.Tank);
    const p0 = world.add_particle(X+3, 0, engine.Kind.Pipe);
    const p = world.add_particle(X+2, -1, engine.Kind.Pipe);
    const f1 = world.add_particle(X+6, 0, engine.Kind.Filter);
    const p1 = world.add_particle(X+9, 0, engine.Kind.Pump);
    const w6 = world.add_particle(X+8, -2, engine.Kind.Wire);
    const s1 = world.add_particle(X+8, -3, engine.Kind.Switch);
    const l2 = world.add_particle(X+7, -3, engine.Kind.Led);
    const bl1 = world.add_particle(X+6, -3, engine.Kind.Blower);
    const p5 = world.add_particle(X+6, -2, engine.Kind.Pipe);
    const v4 = world.add_particle(X+6, -1, engine.Kind.Valve);
    const f2 = world.add_particle(X+7, 0, engine.Kind.Filter);
    const v3 = world.add_particle(X+6, 1, engine.Kind.Valve);
    const p8 = world.add_particle(X+6, 2, engine.Kind.Pipe);
    const sp4 = world.add_particle(X+9, -2, engine.Kind.SolarPanel);
    const t1 = world.add_particle(X+4, 2, engine.Kind.Tank);
    const p9 = world.add_particle(X+5, 2, engine.Kind.Pipe);
    const v2 = world.add_particle(X+8, 0, engine.Kind.Valve);
    const sens1 = world.add_particle(X+5, -1, engine.Kind.SensorMoonFull);
    const trit1 = world.add_particle(X+5, -2, engine.Kind.Trit);
    const sens2 = world.add_particle(X+4, -1, engine.Kind.SensorMoonEmpty);
    const inv1 = world.add_particle(X+4, -2, engine.Kind.SignalInverter);
    const s2 = world.add_particle(X+9, -1, engine.Kind.Switch);
    world.add_link(v1, t0)
    world.add_link(t0, p0)
    world.add_link(p, p0)
    world.add_link(p, m2)
    world.add_link(v1, f1)
    world.add_link(f2, f1)
    world.add_link(s1, w6)
    world.add_link(s2, w6)
    world.add_link(s2, p1)
    world.add_link(s1, l2)
    world.add_link(bl1, l2)
    world.add_link(bl1, p5)
    world.add_link(p5, v4)
    world.add_link(v4, f1)
    world.add_link(v3, f1)
    world.add_link(v3, p8)
    world.add_link(sp4, w6)
    world.add_link(p8, p9)
    world.add_link(p9, t1)
    world.add_link(v2, f2)
    world.add_link(v2, p1)
    world.add_link(v2, p1)
    world.add_link(f1, sens1)
    world.add_link(f1, sens2)
    world.add_link(trit1, sens1)
    world.add_link(trit1, sens2)
    world.add_link(trit1, inv1)

    world.add_link(trit1, s1)
    world.add_link(trit1, v3)
    world.add_link(trit1, v4)
    world.add_link(inv1, v1)
    world.add_link(inv1, v2)
    world.add_link(inv1, s2)


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
        world.set_activation(v4, switched ? 1 : 0);
    };
    window.addEventListener("keydown", on_keydown);
    return (world, engine, memory) => {
    }
}

export {
    setup,
}
