import {
    Universe
} from 'gravitle';
import { memory } from "gravitle/gravitle_bg";

const infos = document.getElementById('infos');
const reloadButton = document.getElementById('button-reload');
const stepButton = document.getElementById('button-step');
const startButton = document.getElementById('button-start');
const stopButton = document.getElementById('button-stop');
const heartButton = document.getElementById('button-heart');
const diamondButton = document.getElementById('button-diamond');
const randomizeButton = document.getElementById('button-randomize');
const symetryButton = document.getElementById('button-symetry');
const clubButton = document.getElementById('button-club');
const spadeButton = document.getElementById('button-spade');
const buttonExample5 = document.getElementById('button-example-5');
const jsonTextarea = document.getElementById('json');
const inputCount = document.getElementById('input-count');
const inputWidth = document.getElementById('input-width');
const inputHeight = document.getElementById('input-height');
const inputG = document.getElementById('input-g');

const canvas = document.getElementById('canvas');
canvas.height = 1000;
canvas.width = 1000;
const context = canvas.getContext("2d");

const BASE_CONF = Object.freeze({
    width: 200,
    height: 200,
    delta_time: 0.01,
    gravitational_constant: 66.74,
    minimal_distance_for_gravity: 0.1,
    algorithm: 'verlet',
    intersection_behavior: 'destroy-link',
    collision_behavior: 'create-link',
    particles: []
});

const universe = Universe.new();
universe.load_from_json(JSON.stringify(BASE_CONF));

let interval = null;
let time = null;
let delta = null;
let last = null;

randomizeButton.addEventListener('click', () => {
    randomize();
});

symetryButton.addEventListener('click', () => {
    symetry();
});

reloadButton.addEventListener('click', () => {
    reload();
});

stepButton.addEventListener('click', () => {
    tick();
});

startButton.addEventListener('click', () => {
    start();
});

stopButton.addEventListener('click', () => {
    stop();
});

heartButton.addEventListener('click', () => {
    heart();
});

diamondButton.addEventListener('click', () => {
    diamond();
});

clubButton.addEventListener('click', () => {
    club();
});

spadeButton.addEventListener('click', () => {
    spade();
});

buttonExample5.addEventListener('click', () => {
    loadExample5();
});

const renderLoop = () => {
    infos.textContent = universe.get_infos();
    draw();
    requestAnimationFrame(renderLoop);
};

const draw = () => {
    context.clearRect(0, 0, canvas.width, canvas.height);
    drawSegments();
    drawParticles();
};

const drawSegments = () => {
    const linksPointer = universe.get_links();
    const linksCount = universe.get_links_count();
    const LINK_SIZE = 5;
    const links = new Float64Array(memory.buffer, linksPointer, linksCount * LINK_SIZE);
    const universeWidth = universe.get_width();
    const universeHeight = universe.get_height();
    const unitX = canvas.width / universeWidth;
    const unitY = canvas.height / universeHeight;
    context.strokeStyle = "#eee";
    context.lineWidth = 4;
    for (let id = 0 ; id < linksCount ; id += 1 ) {
        let i = id * LINK_SIZE;
        const x1 = (universeWidth / 2) * unitX + links[i + 0] * unitX;
        const y1 = (universeHeight / 2) * unitY - links[i + 1] * unitY;
        const x2 = (universeWidth / 2) * unitX + links[i + 2] * unitX;
        const y2 = (universeHeight / 2) * unitY - links[i + 3] * unitY;
        context.beginPath();
        context.moveTo(x1, y1);
        context.lineTo(x2, y2);
        context.stroke();
    }
};

const drawParticles = () => {
    const particlesPointer = universe.get_particles();
    const particlesCount = universe.get_particles_count();
    const PARTICLE_SIZE = 13;
    const particles = new Float64Array(memory.buffer, particlesPointer, particlesCount * PARTICLE_SIZE);
    const universeWidth = universe.get_width();
    const universeHeight = universe.get_height();
    const unitX = canvas.width / universeWidth;
    const unitY = canvas.height / universeHeight;
    context.strokeStyle = "#FFF";
    context.lineWidth = 4;
    for (let i = 0 ; i < particles.length ; i+= PARTICLE_SIZE ) {
        const x = (universeWidth / 2) * unitX + particles[i + 0] * unitX;
        const y = (universeHeight / 2) * unitY - particles[i + 1] * unitY;
        const diameter = (unitX / 2) * particles[i + 2];
        context.beginPath();
        context.arc(
            x,
            y,
            diameter,
            0,
            2 * Math.PI
        );
        context.stroke();
    }
};


const start = () => {
    time = Date.now();
    interval = setInterval(tickMultiple, 1);
    startButton.disabled = true;
    stopButton.disabled = false;
};

const stop = () => {
    if(interval) {
        clearInterval(interval);
    } else {
        // NTD
    }
    startButton.disabled = false;
    stopButton.disabled = true;
};

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

const heart = () => {
    const conf = jsonCopy(BASE_CONF);
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
    jsonTextarea.value = JSON.stringify(conf, null, 4);
    reload();
};

const diamond = () => {
    const conf = jsonCopy(BASE_CONF);
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
    jsonTextarea.value = JSON.stringify(conf, null, 4);
    reload();
};

const club = () => {
    const conf = jsonCopy(BASE_CONF);
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
    jsonTextarea.value = JSON.stringify(conf, null, 4);
    reload();
}

const spade = () => {
    const conf = jsonCopy(BASE_CONF);
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
    jsonTextarea.value = JSON.stringify(conf, null, 4);
    reload();
}

const loadExample5 = () => {
    const conf = jsonCopy(BASE_CONF);
    conf.intersection_behavior = 'destroy-particle';
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
    jsonTextarea.value = JSON.stringify(conf, null, 4);
    reload();
}

const randomize = () => {
    const conf = getParameterizedConf();
    const particles = [];
    for (let i = 0 ; i < parseFloat(inputCount.value) ; i += 1) {
        const x = getRandomIntInclusive(- conf.width / 10, conf.width / 10);
        const y = getRandomIntInclusive(- conf.height / 5, conf.height / 5);
        const mass = getRandomNumber(0.5, 5.0);
        const fixed = false;
        const diameter = mass;
        particles.push({
            x: x,
            y: y,
            mass: mass,
            fixed: fixed,
            diameter: diameter
        });
    }
    conf.particles = particles;
    jsonTextarea.value = JSON.stringify(conf, null, 4);
    reload();
}

const symetry = () => {
    const conf = getParameterizedConf();
    conf.stabilise_positions_enabled = true;
    conf.stabiliser_power = 10;
    const particles = [];
    for (let i = 0 ; i < parseFloat(inputCount.value) / 2 ; i += 1) {
        const x = getRandomIntInclusive(- conf.width / 10, conf.width / 10);
        const y = getRandomIntInclusive(- conf.height / 5, conf.height / 5);
        const mass = getRandomNumber(0.5, 5.0);
        const fixed = false;
        const diameter = mass;
        particles.push({
            x: x,
            y: y,
            mass: mass,
            fixed: fixed,
            diameter: diameter
        });
        particles.push({
            x: -x,
            y: y,
            mass: mass,
            fixed: fixed,
            diameter: diameter
        });
    }
    conf.particles = particles;
    jsonTextarea.value = JSON.stringify(conf, null, 4);
    reload();
}

const getParameterizedConf = () => {
    const conf = jsonCopy(BASE_CONF);
    conf.width = parseFloat(inputWidth.value);
    conf.height = parseFloat(inputHeight.value);
    conf.gravitational_constant = parseFloat(inputG.value);
    return conf;
}

const jsonCopy = (object) => {
    return JSON.parse(JSON.stringify(object));
}

const getRandomBoolean = () => {
    return Math.random() > 0.5;
}

const getRandomNumber = (min, max) => {
    return Math.random() * (max - min) + min;
}

function getRandomIntInclusive(min, max) {
    min = Math.ceil(min);
    max = Math.floor(max);
    return Math.floor(Math.random() * (max - min + 1)) + min;
}

const reload = () => {
    stop();
    universe.reset();
    interval = null;
    time = null;
    delta = null;
    universe.load_from_json(jsonTextarea.value);
    start();
};

const getIndex = (row, column) => {
    return row * width + column;
};

heart();
requestAnimationFrame(renderLoop);

