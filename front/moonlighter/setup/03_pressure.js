const setup = (engine, world) => {
    let c = {x:-3, y:-1}
    const moon_1 = world.add_particle(c.x+0, c.y+0, engine.Kind.Moon)
    const pipe_1 = world.add_particle(c.x+1, c.y+0, engine.Kind.Pipe)
    const tank_1 = world.add_particle(c.x+2, c.y+0, engine.Kind.Tank)
    const filter_1 = world.add_particle(c.x+3, c.y+0, engine.Kind.Filter)
    const filter_2 = world.add_particle(c.x+4, c.y+0, engine.Kind.Filter)
    const pump_1 = world.add_particle(c.x+5, c.y+1, engine.Kind.Pump)
    const solar_panel_1 = world.add_particle(c.x+7, c.y+0, engine.Kind.SolarPanel)
    world.add_link(pump_1, solar_panel_1)
    world.add_link(pump_1, filter_2)
    world.add_link(filter_1, filter_2)
    world.add_link(filter_1, tank_1)
    world.add_link(tank_1, pipe_1)
    world.add_link(moon_1, pipe_1)
    return (world, engine, memory) => {
    }
}

export {
    setup,
}
