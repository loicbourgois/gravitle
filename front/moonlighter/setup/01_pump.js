const setup = (engine, world) => {
    let c = {x:-0, y:-0}
    const moon_1 = world.add_particle(c.x-4, c.y+0, engine.Kind.Moon)
    const filter_1 = world.add_particle(c.x+2, c.y+1, engine.Kind.Filter)
    const filter_2 = world.add_particle(c.x+3, c.y+1, engine.Kind.Filter)
    const pipe_1 = world.add_particle(c.x+0, c.y+0, engine.Kind.Pipe)
    const pipe_2 = world.add_particle(c.x+1, c.y+0, engine.Kind.Tank)
    const pipe_3 = world.add_particle(c.x+1, c.y+1, engine.Kind.Pipe)
    const pipe_4 = world.add_particle(c.x+2, c.y+2, engine.Kind.Pipe)
    const pipe_5 = world.add_particle(c.x-1, c.y, engine.Kind.Pipe)

    const pump_1 = world.add_particle(c.x+4, c.y+2, engine.Kind.Pump)
    const solar_panel_1 = world.add_particle(c.x+5, c.y+2, engine.Kind.SolarPanel)
    world.add_link(pipe_5, moon_1)
    world.add_link(pipe_5, moon_1)
    world.add_link(pipe_1, pipe_5)
    world.add_link(filter_1, filter_2)
    world.add_link(pump_1, filter_2)
    world.add_link(pipe_1, pipe_2)
    world.add_link(pipe_2, pipe_3)
    world.add_link(pipe_3, pipe_4)
    world.add_link(pipe_4, filter_1)
    world.add_link(pump_1, solar_panel_1)
    return (world, engine, memory) => {
    }
}

export {
    setup,
}
