const get_example_1_conf = (conf) => {
    conf.stabilise_positions_enabled = true;
    conf.particles = [
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
    return conf;
}

const get_example_2_conf = (conf) => {
    conf.particles = [
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
    return conf;
}

const get_example_3_conf = (conf) => {
    conf.particles = [
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
    return conf;
};

const get_example_4_conf = (conf) => {
    conf.intersection_behavior = 'destroy-link';
    conf.particles = [
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
    conf.links = [
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
    return conf;
}

const get_example_5_conf = (conf) => {
    conf.intersection_behavior = 'destroy-particle';
    conf.default_link_strengh = 0.0;
    conf.particles = [
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
    conf.links = [
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
    return conf;
}

const get_example_6_conf = (conf) => {
    conf.collision_behavior = 'create-link';
    conf.intersection_behavior = 'destroy-link';
    conf.gravitational_constant = 100;
    conf.default_link_length = 10;
    conf.default_link_strengh = 1000;
    conf.drag_coefficient = 1;
    conf.stabilise_positions_enabled = false;
    conf.particles = [
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
    return conf;
}

const get_example_7_conf = (conf) => {
    conf.collision_behavior = 'create-link';
    conf.intersection_behavior = 'destroy-link';
    conf.gravitational_constant = 100;
    conf.default_link_length = 10;
    conf.default_link_strengh = 1000;
    conf.drag_coefficient = 1;
    conf.stabilise_positions_enabled = false;
    conf.particles = [
        {
            "x": 0,
            "y": 0
        },
        {
            "x": 0,
            "y": 0
        }
    ];
    return conf;
}

const get_example_8_conf = (conf) => {
    conf.collision_behavior = 'create-link';
    conf.link_intersection_behavior = 'destroy-links';
    conf.drag_coefficient = 1;
    conf.gravitational_constant = 100;
    conf.particles = [
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
    return conf;
}

export {
    get_example_1_conf,
    get_example_2_conf,
    get_example_3_conf,
    get_example_4_conf,
    get_example_5_conf,
    get_example_6_conf,
    get_example_7_conf,
    get_example_8_conf
};
