const world_4 = {
    structures: [
        {
            blueprint: "coal_asteroid",
            x: 0.4,
            y: 0.4,
        }, {
            blueprint: "iron_ore_depot",
            x: 0.4,
            y: 0.6,
        }, {
            blueprint: "iron_asteroid",
            x: 0.6,
            y: 0.4,
        }, {
            blueprint: "coal_depot",
            x: 0.6,
            y: 0.6,
        }
    ],
    ships: []
}
for (let index = 0; index < 16; index++) {
    world_4.ships.push({
        blueprint: "coal_collector",
        x: Math.random(),
        y: Math.random(),
        job: 'coal_collector',
    })
    world_4.ships.push({
        blueprint: "iron_ore_collector_2",
        x: Math.random(),
        y: Math.random(),
        job: 'iron_ore_collector',
    })
}
export {
    world_4,
}
