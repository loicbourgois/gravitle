const world_1 = {
    structures: [
    {
        blueprint: "sun",
        x: 0.5,
        y: 0.5,
    },{
    blueprint: "plasma_depot",
    x: 0.6,
    y: 0.6,
    },{
    blueprint: "plasma_refinery",
    x: 0.5,
    y: 0.6,
    }
],
particles: [
    {
    kind: "anchor",
    x: 0.525,
    y: 0.5,
    }
],
ships: [
    {
    blueprint: "harvester",
    x: 0.525,
    y: 0.5,
    anchor: {k:'particles', id:0},
    target: {k:'structures', id:0},
    job: 'electro_field_launcher',
    }, 
    {
    blueprint: "plasma_collector",
    x: 0.6,
    y: 0.4,
    job: 'plasma_collector',
    }, 
    {
    blueprint: "plasma_collector",
    x: 0.4,
    y: 0.4,
    job: 'plasma_collector',
    },
    {
    blueprint: "plasma_transporter",
    x: 0.55,
    y: 0.6,
    job: 'plasma_transporter',
    }
]
}
export {
    world_1,
}