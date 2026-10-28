const setup = (engine, world) => {
    world.add_particle(0, 0, engine.Kind.Moon)
    world.add_particle(1, 0, engine.Kind.Moon)
    world.add_particle(-1, 0, engine.Kind.Moon);
    world.add_particle(0, 1, engine.Kind.Moon);
    world.add_particle(0, -1, engine.Kind.Moon);
    world.add_particle(5, 0.1, engine.Kind.Pipe);
    world.add_particle(4, 0, engine.Kind.Pipe);
    world.add_particle(3, 0.1, engine.Kind.Pipe);
    world.add_particle(2, -0.1, engine.Kind.Pipe);
    world.add_particle(6, 0.1, engine.Kind.Filter);
    world.add_particle(7, 0.2, engine.Kind.Pump);

    world.add_link(5, 6)
    world.add_link(6, 7)
    world.add_link(7, 8)
    world.add_link(8, 1)
    world.add_link(5, 9)
    world.add_link(10, 9)
}

export {
    setup,
}
