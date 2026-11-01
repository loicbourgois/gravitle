const setup = (engine, world) => {
    let c = {x:0, y:0}
    const moon_1 = world.add_particle(c.x+0, c.y+0, engine.Kind.Moon)
    const pipe_1 = world.add_particle(c.x+1, c.y+0, engine.Kind.Pipe)
    const press_1 = world.add_particle(c.x+2, c.y+0, engine.Kind.Press)
    const conveyor_1 = world.add_particle(c.x+2, c.y+1, engine.Kind.Conveyor)
    const launcher_1 = world.add_particle(c.x+2, c.y+2, engine.Kind.Launcher)
    const filter_1 = world.add_particle(c.x+3, c.y+0, engine.Kind.Filter)
    const filter_2 = world.add_particle(c.x+4, c.y+0, engine.Kind.Filter)
    const pump_1 = world.add_particle(c.x+5, c.y+0, engine.Kind.Pump)
    const solar_panel_1 = world.add_particle(c.x+6, c.y+0, engine.Kind.SolarPanel)
    world.add_link(pump_1, solar_panel_1)
    world.add_link(pump_1, filter_2)
    world.add_link(filter_1, filter_2)
    world.add_link(filter_1, press_1)
    world.add_link(pipe_1, press_1)
    world.add_link(pipe_1, moon_1)
    world.add_link(conveyor_1, press_1)
    world.add_link(conveyor_1, launcher_1)
    return (world, engine, memory) => {
    }
}

export {
    setup,
}
