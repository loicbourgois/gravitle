const world_5 = {
    structures: [
        {
            blueprint: "coal_asteroid",
            x: 0.4,
            y: 0.4,
        }, {
            blueprint: "coal_depot",
            x: 0.4,
            y: 0.6,
        }, {
            blueprint: "iron_asteroid",
            x: 0.6,
            y: 0.4,
        }, {
            blueprint: "iron_ore_depot",
            x: 0.6,
            y: 0.6,
        }
    ],
    ships: [
        {
            blueprint: "luciole_2",
            // blueprint: "luciole_2",
            // blueprint: "luciole",
            // blueprint: "mothership",
            x: Math.random()*0.4+0.3,
            y: Math.random()*0.4+0.3,
            job: 'mothership',
        },
        {
            blueprint: "luciole_3",
            // blueprint: "luciole_2",
            // blueprint: "luciole",
            // blueprint: "mothership",
            x: Math.random()*0.4+0.3,
            y: Math.random()*0.4+0.3,
            job: 'mothership',
        },
        {
            blueprint: "luciole_4",
            // blueprint: "luciole_2",
            // blueprint: "luciole",
            // blueprint: "mothership",
            x: Math.random()*0.4+0.3,
            y: Math.random()*0.4+0.3,
            job: 'mothership',
        },
        {
            // blueprint: "luciole_3",
            blueprint: "luciole_5",
            // blueprint: "luciole",
            // blueprint: "mothership",
            x: Math.random()*0.4+0.3,
            y: Math.random()*0.4+0.3,
            job: 'mothership',
        },
        {
            blueprint: "coal_collector",
            x: 0.5,
            y: 0.6,
            job: 'coal_collector',
        },
        {
            blueprint: "iron_ore_collector_2",
            x: 0.5,
            y: 0.4,
            job: 'iron_ore_collector',
        },
        {
            blueprint: "coal_collector",
            x: 0.4,
            y: 0.5,
            job: 'coal_collector',
        },
        {
            blueprint: "iron_ore_collector_2",
            x: 0.6,
            y: 0.5,
            job: 'iron_ore_collector',
        },
    ]
}
export {
    world_5,
}
