import { Universe } from 'gravitle';
import WebGLRenderer from './webgl-renderer.js';
import * as utils from './utils.js';

//
//
//
const get_conf = () => {
    const conf = utils.get_base_conf_copy();
    conf.width = canvas.width * 0.5;
    conf.height = canvas.height * 0.5;
    conf.wrap_around = true;
    conf.drag_coefficient = 0.01;
    conf.intersection_behavior = 'destroy-link';
    conf.link_intersection_behavior = 'destroy-links';
    conf.default_link_strengh = 1000.0;
    conf.default_link_length = 16.0;
    conf.default_link_thrust_force = 1000.0;
    const diameter = 2.0;
    const big_dimameter = 20.0;
    const big_mass = 20.0;
    conf.particles = [
        {x: -30, y: 10, diameter: diameter},
        {x: -10, y: 10, diameter: diameter},
        {x: 10, y: 10, diameter: diameter},
        {x: 30, y: 10, diameter: diameter},
        {x: -40, y: 0, diameter: diameter},
        {x: -20, y: 0, diameter: diameter},
        {x: 0, y: 0, diameter: diameter},
        {x: 20, y: 0, diameter: diameter},
        {x: 40, y: 0, diameter: diameter},
        {x: -10, y: -10, diameter: diameter},
        {x: 10, y: -10, diameter: diameter},
        {x: conf.width * 0.2, y: conf.height * 0.2, diameter: big_dimameter, mass: big_mass },
        {x: conf.width * 0.2, y: -conf.height * 0.2, diameter: big_dimameter, mass: big_mass },
        {x: -conf.width * 0.2, y: conf.height * 0.2, diameter: big_dimameter, mass: big_mass },
        {x: -conf.width * 0.2, y: -conf.height * 0.2, diameter: big_dimameter, mass: big_mass }
    ];
    conf.links = [
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
            p1_index: 2,
            p2_index: 1
        }
    ];
    return conf;
};

const render_loop = () => {
    const resolution = 0;
    const period = 0;
    const link_states_period = 1;
    const link_states_history_length = 32;
    webgl_renderer.render(
        universe.get_links_coordinates_to_draw(),
        universe.get_particles_data_to_draw(),
        [],
        resolution,
        universe.get_width(),
        universe.get_height(),
        false,
        false,
        [],
        [],
        false,
        [],
        universe.get_links_states(link_states_history_length, link_states_period)
    );
    requestAnimationFrame(render_loop);
}

const tick = () => {
    universe.tick();
};

const tickMultiple = () => {
    const now = Date.now();
    delta = now - time;
    const resolution = universe.get_delta_time_milli();
    while (delta > resolution) {
        delta -= resolution;
        tick();
    }
    time = now - delta;
};

const start = () => {
    time = Date.now();
    interval = setInterval(tickMultiple, 1);
    requestAnimationFrame(render_loop);
    canvas.focus();
};

const keyup = (e) => {
    if (bindings && bindings[e.key]) {
        universe.deactivate_thrust_for_links(bindings[e.key].link_indexes);
        document.getElementById('popup').classList.add('faded');
    } else {
        // Do nothing
    }
};

const keydown = (e) => {
    if (bindings && bindings[e.key]) {
        universe.activate_thrust_for_links(bindings[e.key].link_indexes);
    } else {
        // Do nothing
    }
};

const get_bindings = () => {
    return {
        'ArrowLeft' : {
            link_indexes : [0]
        },
        'ArrowUp' : {
            link_indexes : [1]
        },
        'ArrowRight' : {
            link_indexes : [2]
        },
        'ArrowDown' : {
            link_indexes : [20]
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
    };
};

document.addEventListener('keyup', keyup);
document.addEventListener('keydown', keydown);

//
// Setup
//
const canvas = document.getElementById('canvas');
canvas.height = canvas.clientHeight;
canvas.width = canvas.clientWidth;
let context;
const webgl_context = canvas.getContext('webgl2');
let webgl_renderer;
if (webgl_context) {
    webgl_renderer = new WebGLRenderer(webgl_context);
} else {
    context = canvas.getContext('2d');
}
const universe = Universe.new();
universe.load_from_json(JSON.stringify(get_conf()));
const bindings = get_bindings();
let interval = null;
let time = null;
let delta = null;
let last = null;
let mouse_positions = null;
const frame_gaps = [];
const frame_lengths = [];
let last_now = null;
let average_fps = 0.0;
let average_frame_length = 0.0;

export {
    start
};
