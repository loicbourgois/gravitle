const setup = (engine, world) => {
    let c = {x:-3, y:-1}
    const moon_1 = world.add_particle(c.x+0, c.y+0, engine.Kind.Moon)
    const pipe_1 = world.add_particle(c.x+1, c.y+0, engine.Kind.Pipe)
    const press_1 = world.add_particle(c.x+2, c.y+0, engine.Kind.Press)
    const conveyor_1 = world.add_particle(c.x+2, c.y+1, engine.Kind.Conveyor)
    const launcher_1 = world.add_particle(c.x+2, c.y+2, engine.Kind.Launcher)
    const filter_1 = world.add_particle(c.x+3, c.y+0, engine.Kind.Filter)
    const filter_2 = world.add_particle(c.x+4, c.y+0, engine.Kind.Filter)
    const pump_1 = world.add_particle(c.x+5, c.y+1, engine.Kind.Pump)
    const switch_1 = world.add_particle(c.x+6, c.y+0, engine.Kind.Switch)
    const solar_panel_1 = world.add_particle(c.x+7, c.y+0, engine.Kind.SolarPanel)
    const earth_1 = world.add_particle(c.x-1, c.y+5, engine.Kind.Earth)
    const blower_1 = world.add_particle(c.x+3, c.y-1, engine.Kind.Blower)
    const switch_2 = world.add_particle(c.x+7, c.y-1, engine.Kind.Switch)
    const sens1 = world.add_particle(c.x+4, c.y-2, engine.Kind.SensorMoonEmpty);
    const sens2 = world.add_particle(c.x+4, c.y-3, engine.Kind.SensorMoonFull);
    const trit1 = world.add_particle(c.x+5, c.y-2, engine.Kind.Trit);
    const inv1 = world.add_particle(c.x+6, c.y-2, engine.Kind.SignalInverter);
    world.add_link(pump_1, switch_1)
    world.add_link(switch_1, solar_panel_1)
    world.add_link(pump_1, filter_2)
    world.add_link(filter_1, filter_2)
    world.add_link(filter_1, press_1)
    world.add_link(pipe_1, press_1)
    world.add_link(pipe_1, moon_1)
    world.add_link(conveyor_1, press_1)
    world.add_link(conveyor_1, launcher_1)
    world.add_link(blower_1, filter_1)
    world.add_link(blower_1, switch_2)
    world.add_link(solar_panel_1, switch_2)
    world.add_link(sens1, filter_1)
    world.add_link(sens2, filter_1)
    world.add_link(sens1, trit1)
    world.add_link(sens2, trit1)
    world.add_link(inv1, trit1)
    world.add_link(inv1, switch_1)
    world.add_link(trit1, switch_2)
    return (world, engine, memory) => {
    }
}

export {
    setup,
}
