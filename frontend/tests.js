import * as utils from './utils.js';

//
// Contains a list of Universe configurations used to test the different
// features of Gravitle.
//

const get_test_1 = (conf) => {
    const test = {
        id: 'heart',
        title: 'Heart',
        description: `
            .
        `,
        conf: utils.get_base_conf_copy()
    };
    test.conf.stabilise_positions_enabled = true;
    test.conf.particles = [
        {
            "x": 0,
            "y": 20
        }, {
            "x": 10,
            "y": 30
        }, {
            "x": 20,
            "y": 30
        }, {
            "x": 30,
            "y": 20
        }, {
            "x": 20,
            "y": 5
        }, {
            "x": 10,
            "y": -10
        }, {
            "x": 0,
            "y": -20
        }, {
            "x": -10,
            "y": 30
        }, {
            "x": -20,
            "y": 30
        }, {
            "x": -30,
            "y": 20
        }, {
            "x": -20,
            "y": 5
        }, {
            "x": -10,
            "y": -10
        }, {
            "x": 0,
            "y": -35
        }
    ];
    return test;
}

const get_test_2 = (conf) => {
    const test = {
        id: 'diamond',
        title: 'Diamond',
        description: `
            .
        `,
        conf: utils.get_base_conf_copy()
    };
    test.conf.particles = [
        {
            "x": -30,
            "y": -40,
            "fixed": false
        }, {
            "x": -30,
            "y": -41,
            "fixed": false
        }, {
            "x": 0,
            "y": 41,
            "fixed": false
        }, {
            "x": 20,
            "y": 20,
            "fixed": true
        }, {
            "x": 20,
            "y": -20,
            "fixed": true
        }, {
            "x": -20,
            "y": 20,
            "fixed": true
        }, {
            "x": -20,
            "y": -20,
            "fixed": true
        }, {
            "x": 0,
            "y": 40,
            "fixed": true
        }, {
            "x": 0,
            "y": -40,
            "fixed": true
        }, {
            "x": 40,
            "y": 0,
            "fixed": true
        }, {
            "x": -40,
            "y": 0,
            "fixed": true
        }
    ];
    return test;
}

const get_test_3 = (conf) => {
    const test = {
        id: 'club',
        title: 'Club',
        description: `
            .
        `,
        conf: utils.get_base_conf_copy()
    };
    test.conf.particles = [
        {
            "x": 0.01,
            "y": -25,
            "fixed": false
        },
        {
            "x": 10,
            "y": -35,
            "fixed": false
        },
        {
            "x": -10,
            "y": -35,
            "fixed": false
        },
        {
            "x": 0,
            "y": 40,
            "fixed": true,
            "diameter": 5,
            "mass": 5
        },
        {
            "x": 40,
            "y": -10,
            "fixed": true,
            "diameter": 5,
            "mass": 5
        },
        {
            "x": -40,
            "y": -10,
            "fixed": true,
            "diameter": 5,
            "mass": 5
        }
    ];
    return test;
};

const get_test_4 = (conf) => {
    const test = {
        id: 'spade',
        title: 'Spade',
        description: `
            .
        `,
        conf: utils.get_base_conf_copy()
    };
    test.conf.intersection_behavior = 'destroy-link';
    test.conf.particles = [
        {
            "x": 0,
            "y": 35
        },
        {
            "x": 10,
            "y": 25,
            "fixed": true
        },
        {
            "x": -10,
            "y": 25,
            "fixed": true
        },{
            "x": 15,
            "y": 15,
            "fixed": true
        },
        {
            "x": -15,
            "y": 15,
            "fixed": true
        },{
            "x": 20,
            "y": 5,
            "fixed": true
        },
        {
            "x": -20,
            "y": 5,
            "fixed": true
        },{
            "x": 15,
            "y": 0,
            "fixed": true
        },
        {
            "x": -15,
            "y": 0,
            "fixed": true
        },{
            "x": 5,
            "y": -5,
            "fixed": true
        },
        {
            "x": -5,
            "y": -5,
            "fixed": true
        },{
            "x": 10,
            "y": -10,
            "fixed": true
        },
        {
            "x": -10,
            "y": -10,
            "fixed": true
        }
    ];
    test.conf.links = [
        {
            "p1_index": 1,
            "p2_index": 2
        }, {
            "p1_index": 3,
            "p2_index": 4
        }, {
            "p1_index": 5,
            "p2_index": 6
        }, {
            "p1_index": 7,
            "p2_index": 8
        }, {
            "p1_index": 9,
            "p2_index": 10
        }, {
            "p1_index": 11,
            "p2_index": 12
        }
    ];
    return test;
}

const get_test_5 = (conf) => {
    const test = {
        id: 'v',
        title: 'Test V',
        description: `
            .
        `,
        conf: utils.get_base_conf_copy()
    };
    test.conf.intersection_behavior = 'destroy-particle';
    test.conf.default_link_strengh = 0.0;
    test.conf.particles = [
        {
            "x": 0,
            "y": 0
        },
        {
            "x": -10,
            "y": 30,
            "fixed": true
        },
        {
            "x": 10,
            "y": 30,
            "fixed": true
        },
        {
            "x": -10,
            "y": -10,
            "fixed": true
        },
        {
            "x": 10,
            "y": -10,
            "fixed": true
        },
        {
            "x": -40,
            "y": 35,
            "fixed": false
        },
        {
            "x": 40,
            "y": 35,
            "fixed": false
        },
        {
            "x": -50,
            "y": 30,
            "fixed": true
        },
        {
            "x": 50,
            "y": 30,
            "fixed": true
        }
    ];
    test.conf.links = [
        {
            "p1_index": 0,
            "p2_index": 1
        },
        {
            "p1_index": 0,
            "p2_index": 2
        },
        {
            "p1_index": 3,
            "p2_index": 4
        },
        {
            "p1_index": 5,
            "p2_index": 6
        }
    ];
    return test;
}

const get_test_6 = (conf) => {
    const test = {
        id: 'test_6',
        title: 'Test 6',
        description: `
            .
        `,
        conf: utils.get_base_conf_copy()
    };
    test.conf.collision_behavior = 'create-link';
    test.conf.intersection_behavior = 'destroy-link';
    test.conf.gravitational_constant = 100;
    test.conf.default_link_length = 10;
    test.conf.default_link_strengh = 1000;
    test.conf.drag_coefficient = 1;
    test.conf.stabilise_positions_enabled = false;
    test.conf.particles = [
        {
            "x": 0,
            "y": 0,
            "fixed": true
        },
        {
            "x": 30,
            "y": 0,
            "fixed": true
        },
        {
            "x": -30,
            "y": 0,
            "fixed": true
        },
        {
            "x": 0,
            "y": 30,
            "fixed": true
        },
        {
            "x": 0,
            "y": -30,
            "fixed": true
        },
        {
            "x": 20,
            "y": 20
        },
        {
            "x": 20,
            "y": -20
        },
        {
            "x": -20,
            "y": 20
        },
        {
            "x": -20,
            "y": -20
        },
        {
            "x": 0,
            "y": 20
        },
        {
            "x": 0,
            "y": -20
        },
        {
            "x": 20,
            "y": 0
        },
        {
            "x": -20,
            "y": 0
        }
    ];
    return test;
}

const get_test_7 = (conf) => {
    const test = {
        id: 'test_7',
        title: 'Test 7',
        description: `
            .
        `,
        conf: utils.get_base_conf_copy()
    };
    test.conf.collision_behavior = 'create-link';
    test.conf.intersection_behavior = 'destroy-link';
    test.conf.gravitational_constant = 100;
    test.conf.default_link_length = 10;
    test.conf.default_link_strengh = 1000;
    test.conf.drag_coefficient = 1;
    test.conf.stabilise_positions_enabled = false;
    test.conf.particles = [
        {
            "x": 0,
            "y": 0
        },
        {
            "x": 0,
            "y": 0
        }
    ];
    return test;
}

const get_test_8 = (conf) => {
    const test = {
        id: 'test_8',
        title: 'Test 8',
        description: `
            .
        `,
        conf: utils.get_base_conf_copy()
    };
    test.conf.collision_behavior = 'create-link';
    test.conf.link_intersection_behavior = 'destroy-links';
    test.conf.drag_coefficient = 1;
    test.conf.gravitational_constant = 100;
    test.conf.particles = [
        {
            x: 0.1,
            y: 0
        },
        {
            x: 0.2,
            y: 0
        },
        {
            x: 0.3,
            y: 0.1
        },
        {
            x: 4,
            y: 0
        }
    ];
    return test;
}

const get_test_9 = () => {
    const test = {
        id: 'test_9',
        title: 'Triangles wrapping around',
        description: `
            Two triangles should go upward, wrap around to appear at the bottom
            and then collide with the bottom particle and create links between
            itself and the particle.
            Links should then be destroyed when they intersect and be created
            again when two particle collides.
        `,
        conf: utils.get_base_conf_copy()
    };
    test.conf.wrap_around = true;
    test.conf.drag_coefficient = 0.001;
    test.conf.collision_behavior = 'create-link';
    test.conf.link_intersection_behavior = 'destroy-links';
    test.conf.particles = [
        {
            x: 1,
            y: 80
        },
        {
            x: 2,
            y: 90
        },
        {
            x: -5,
            y: 85
        },
        {
            x: 5,
            y: 85
        },
        {
            x: 0,
            y: -50,
            fixed: true,
            mass: 10,
            diameter: 10
        }
    ];
    test.conf.links = [
        {
            p1_index: 0,
            p2_index: 1
        },{
            p1_index: 1,
            p2_index: 2
        },{
            p1_index: 2,
            p2_index: 0
        },{
            p1_index: 1,
            p2_index: 3
        },{
            p1_index: 3,
            p2_index: 0
        }
    ];
    return test;
};

const get_test_10 = () => {
    const test = {
        id: 'test_10',
        title: 'Small spaceship',
        description: `
            Use [A, Z, E] or [Q, W, E] or [1, 2, 3] or [4, 5, 6] or [Left, Up, Right] to move.
            Don't get hit.
        `,
        conf: utils.get_base_conf_copy(),
        bindings: {
            'ArrowLeft' : {
                link_indexes : [0]
            },
            'ArrowUp' : {
                link_indexes : [1]
            },
            'ArrowRight' : {
                link_indexes : [2]
            },
            'a' : {
                link_indexes : [0]
            },
            'z' : {
                link_indexes : [1]
            },
            'e' : {
                link_indexes : [2]
            },
            'q' : {
                link_indexes : [0]
            },
            'w' : {
                link_indexes : [1]
            },
            'e' : {
                link_indexes : [2]
            },
            '1' : {
                link_indexes : [0]
            },
            '2' : {
                link_indexes : [1]
            },
            '3' : {
                link_indexes : [2]
            },
            '4' : {
                link_indexes : [2]
            },
            '5' : {
                link_indexes : [1]
            },
            '6' : {
                link_indexes : [0]
            }
        }
    };
    test.conf.wrap_around = true;
    test.conf.width = 300;
    test.conf.height = 300;
    test.conf.drag_coefficient = 0.025;
    test.conf.intersection_behavior = 'destroy-link';
    test.conf.link_intersection_behavior = 'destroy-links';
    test.conf.default_link_strengh = 1000.0;
    test.conf.default_link_thrust_force = 1000.0;
    test.conf.particles = [
        {x: -15, y: 5},
        {x: -5, y: 5},
        {x: 5, y: 5},
        {x: 15, y: 5},
        {x: -20, y: 0},
        {x: -10, y: 0},
        {x: 0, y: 0},
        {x: 10, y: 0},
        {x: 20, y: 0},
        {x: -5, y: -5},
        {x: 5, y: -5},
        {x: 0, y: -100, diameter: 5, mass:5 }
    ];
    const THRUST = 100.0;
    test.conf.links = [
        {
            p1_index: 4,
            p2_index: 5
        }, {
            p1_index: 9,
            p2_index: 10
        }, {
            p1_index: 7,
            p2_index: 8
        }, {
            p1_index: 4,
            p2_index: 0
        }, {
            p1_index: 0,
            p2_index: 5
        }, {
            p1_index: 0,
            p2_index: 1
        }, {
            p1_index: 5,
            p2_index: 9
        }, {
            p1_index: 1,
            p2_index: 6
        }, {
            p1_index: 6,
            p2_index: 2
        }, {
            p1_index: 6,
            p2_index: 7
        }, {
            p1_index: 6,
            p2_index: 10
        }, {
            p1_index: 2,
            p2_index: 3
        }, {
            p1_index: 2,
            p2_index: 7
        }, {
            p1_index: 10,
            p2_index: 7
        }, {
            p1_index: 3,
            p2_index: 7
        }, {
            p1_index: 3,
            p2_index: 8
        }, {
            p1_index: 5,
            p2_index: 1
        }, {
            p1_index: 9,
            p2_index: 6
        }, {
            p1_index: 5,
            p2_index: 6
        }, {
            p1_index: 5,
            p2_index: 6
        }, {
            p1_index: 1,
            p2_index: 2
        }
    ];
    return test;
};

const get_test_11 = () => {
    const test = {
        id: 'test_11',
        title: 'Particle link intersection',
        description: `
            A particle is fixed at the center. A link comes from the right.
            When the moving link intersect with the fixed particle, the link
            is destroyed.
        `,
        conf: utils.get_base_conf_copy()
    };
    test.conf.intersection_behavior = 'destroy-link';
    test.conf.particles = [
        {
            x: 1,
            y: 0,
            fixed: true
        },
        {
            x: 20,
            y: -5
        },
        {
            x: 20,
            y: 5
        }
    ];
    test.conf.links = [
        {
            p1_index: 1,
            p2_index: 2
        }
    ];
    return test;
};

const get_test_12 = () => {
    const test = {
        id: 'test_12',
        mode: 'SPACE-SHIP',
        title: 'Spaceship generator',
        description: `
            After the 200th tick, try to move using [A, Z] or [Q, W].
        `,
        conf: utils.get_base_conf_copy()
    };
    test.conf.particles = [];
    test.conf.links = [];
    test.conf.collision_behavior = 'create-link';
    test.conf.intersection_behavior = 'do-nothing';
    test.conf.link_intersection_behavior = 'do-nothing';
    test.conf.gravitational_constant = 1;
    test.conf.default_link_length = 10;
    test.conf.default_link_strengh = 1000;
    test.conf.drag_coefficient = 1;
    test.conf.stabilise_positions_enabled = false;
    test.conf.minimal_distance_for_gravity = 1.0;
    test.conf.wrap_around = true;
    test.conf.default_link_thrust_force = 1000.0;
    const COUNT = 20;
    const DIVISOR = 20;
    const particles = [];
    const minDiameter = 4.0;
    const maxDiameter = 5.0;
    const MASS = 1.0;
    const diameter = utils.get_random_number(minDiameter, maxDiameter);
    particles.push({
        x: 0,
        y: 0,
        mass: MASS,
        diameter: diameter
    });
    for (let i = 2 ; i < COUNT ; i += 2) {
        const x = utils.get_random_number(
            - test.conf.width / DIVISOR,
            test.conf.width / DIVISOR);
        const y = utils.get_random_number(
            - test.conf.height / DIVISOR,
            test.conf.height / DIVISOR);
        const diameter = utils.get_random_number(minDiameter, maxDiameter);
        particles.push({
            x: x,
            y: y,
            mass: MASS,
            diameter: diameter
        });
    }
    const l = particles.length;
    for (let i = l-1 ; i >= 0 ; i -= 1) {
        const particle = particles[i];
        particles.push({
            x: -particle.x,
            y: particle.y,
            mass: particle.mass,
            diameter: particle.diameter
        });
    }
    test.conf.particles = particles;
    return test;
};

const get_test_13 = () => {
    const isInZones = (x, y, zones, zoneRadius) => {
        let r = false;
        for (const index in zones) {
            const zone = zones[index];
            if (utils.circles_collide(x, y, zone.x, zone.y, zone.radius, zoneRadius)) {
                r = true;
            } else {
                // Do nothing
            }
        }
        return r;
    };

    const test = {
        id: 'test_13',
        mode: 'SPACE-CROQUET',
        title: 'Space croquet',
        description: `
            Click and drag on canvas, Launching particles, Destroy all the links.
        `,
        conf: utils.get_base_conf_copy()
    };
    test.conf.intersection_behavior = 'destroy-link';
    test.conf.wrap_around = true;
    const particles = [];
    const links = [];
    const zones = [];
    const maxDiameter = 5.0;
    const checkpointLength = test.conf.width / 8;
    const innerRadius = checkpointLength / 2;
    const zoneRadius = innerRadius + maxDiameter / 2;
    const SPACE_CROQUET_LINK_COUNT = 4;
    for (let i = 0 ; i < SPACE_CROQUET_LINK_COUNT ; i += 1) {
        let x = utils.get_random_int_inclusive(- test.conf.width / 4, test.conf.width / 4);
        let y = utils.get_random_int_inclusive(- test.conf.height / 4, test.conf.height / 4);
        let i = 1000;
        while (isInZones(x, y, zones, zoneRadius) && i > 0) {
            x = utils.get_random_int_inclusive(- test.conf.width / 4, test.conf.width / 4);
            y = utils.get_random_int_inclusive(- test.conf.height / 4, test.conf.height / 4);
            i -= 1;
        }
        if (i) {
            zones.push({
                x: x,
                y: y,
                diameter: zoneRadius * 2,
                radius: zoneRadius,
                fixed: true
            });
        }
    }
    for (let i = 0 ; i < zones.length ; i += 1) {
        const mass = utils.get_random_number(maxDiameter, maxDiameter);
        const fixed = true;
        const diameter = mass;
        const angle = utils.get_random_int_inclusive(0, 359);
        const p1 = utils.get_coordinate_rotated_around(
            {
                x: zones[i].x,
                y: zones[i].y
            },
            {
                x: zones[i].x + innerRadius,
                y: zones[i].y
            },
            angle
        );
        const p2 = utils.get_coordinate_rotated_around(
            {
                x: zones[i].x,
                y: zones[i].y
            },
            {
                x: zones[i].x - innerRadius,
                y: zones[i].y
            },
            angle
        );
        particles.push({
            x: p1.x,
            y: p1.y,
            mass: mass,
            fixed: fixed,
            diameter: diameter
        });
        particles.push({
            x: p2.x,
            y: p2.y,
            mass: mass,
            fixed: fixed,
            diameter: diameter
        });
        links.push({
            "p1_index": i*2,
            "p2_index": i*2+1
        });
    }
    test.conf.particles = particles;
    test.conf.links = links;
    return test;
};

const get_test_14 = () => {
    const test = {
        id: 'test_14',
        mode: 'test_14',
        title: 'Particle-Particle collision response',
        description: `
            Two particles move toward each other.
            They bounce when colliding.
        `,
        conf: utils.get_base_conf_copy()
    };
    const diameter = 20;
    test.conf.collision_behavior = 'push-particles';
    test.conf.gravitational_constant = 20.0;
    test.conf.default_push_force = 100.0;
    test.conf.drag_coefficient = 1.0;
    test.conf.particles = [
        {
            x: -5,
            y: 3,
            diameter: diameter,
            mass: diameter
        },
        {
            x: 25,
            y: 9,
            diameter: diameter,
            mass: diameter
        }
    ];
    return test;
};

const get_tests = () => {
    let list = [];
    list.push(get_test_10());
    list.push(get_test_12());
    list.push(get_test_13());
    list.push(get_test_1());
    list.push(get_test_2());
    list.push(get_test_3());
    list.push(get_test_4());
    list.push(get_test_5());
    list.push(get_test_6());
    list.push(get_test_7());
    list.push(get_test_8());
    list.push(get_test_9());
    list.push(get_test_11());
    list.push(get_test_14());
    return list;
};

const get_test_by_id = (id) => {
    let tests = get_tests();
    let return_test = null;
    tests.forEach(test => {
        if (test.id === id) {
            return_test = test;
        } else {
            // Do nothing
        }
    });
    return return_test;
};

export {
    get_test_9,
    get_test_by_id,
    get_tests
};
