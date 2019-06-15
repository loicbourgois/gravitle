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

const get_tests = () => {
    let list = [];
    list.push(get_test_1());
    list.push(get_test_2());
    list.push(get_test_3());
    list.push(get_test_4());
    list.push(get_test_5());
    list.push(get_test_6());
    list.push(get_test_7());
    list.push(get_test_8());
    list.push(get_test_9());
    list.push(get_test_10());
    list.push(get_test_11());
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
