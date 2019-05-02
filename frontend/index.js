import { Universe, Algorithm } from 'gravitle';
import { memory } from "gravitle/gravitle_bg";

const infos = document.getElementById('infos');
const reloadButton = document.getElementById('button-reload');
const stepButton = document.getElementById('button-step');
const startButton = document.getElementById('button-start');
const stopButton = document.getElementById('button-stop');
const heartButton = document.getElementById('button-heart');
const diamondButton = document.getElementById('button-diamond');
const jsonTextarea = document.getElementById('json');

const canvas = document.getElementById('canvas');
canvas.height = 1000;
canvas.width = 1000;
const context = canvas.getContext("2d");

const universe = Universe.new();
universe.load_from_json(`{
    "width": 200,
    "height": 200,
    "delta_time": 0.01,
    "gravitational_constant": 667.4,
    "minimal_distance_for_gravity": 0.1,
    "algorithm": ${Algorithm.Verlet}
}`);

let interval = null;
let time = null;
let delta = null;

let last = null;

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

const renderLoop = () => {
    infos.textContent = universe.get_infos();
    draw();
    requestAnimationFrame(renderLoop);
};

const draw = () => {
    context.clearRect(0, 0, canvas.width, canvas.height);
    const particlesPointer = universe.get_particles();
    const particlesCount = universe.get_particles_count();
    const PARTICLE_SIZE = 12;
    const particles = new Float64Array(memory.buffer, particlesPointer, particlesCount * PARTICLE_SIZE);
    const universeWidth = universe.get_width();
    const universeHeight = universe.get_height();
    const unitX = canvas.width / universeWidth;
    const unitY = canvas.height / universeHeight;
    context.strokeStyle = "#FFF";
    context.lineWidth = 4;
    for (let i = 0 ; i < particles.length ; i+= PARTICLE_SIZE ) {
        context.beginPath();
        context.arc(
            (universeWidth / 2) * unitX + particles[i + 0] * unitX,
            (universeHeight / 2) * unitY - particles[i + 1] * unitY,
            unitX / 2,
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
        universe.tick();
    }
    time = now - delta;
};

const heart = () => {
    jsonTextarea.value = `{
    "width": 200,
    "height": 200,
    "delta_time": 0.01,
    "gravitational_constant": 667.4,
    "minimal_distance_for_gravity": 0.1,
    "algorithm": ${Algorithm.Verlet},
    "particles": [
        {
            "x": 0,
            "y": 20,
            "fixed": false
        }, {
            "x": 10,
            "y": 30,
            "fixed": false
        }, {
            "x": 20,
            "y": 30,
            "fixed": false
        }, {
            "x": 30,
            "y": 20,
            "fixed": false
        }, {
            "x": 20,
            "y": 5,
            "fixed": false
        }, {
            "x": 10,
            "y": -10,
            "fixed": false
        }, {
            "x": 0,
            "y": -20,
            "fixed": false
        }, {
            "x": -10,
            "y": 30,
            "fixed": false
        }, {
            "x": -20,
            "y": 30,
            "fixed": false
        }, {
            "x": -30,
            "y": 20,
            "fixed": false
        }, {
            "x": -20,
            "y": 5,
            "fixed": false
        }, {
            "x": -10,
            "y": -10,
            "fixed": false
        }, {
            "x": 0,
            "y": -35,
            "fixed": false
        }
    ]
}`;
    reload();
};

const diamond = () => {
    jsonTextarea.value = `{
    "width": 200,
    "height": 200,
    "delta_time": 0.01,
    "gravitational_constant": 667.4,
    "minimal_distance_for_gravity": 0.1,
    "algorithm": ${Algorithm.Verlet},
    "particles": [
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
    ]
}`;
    reload();
};

const reload = () => {
    last = diamond;
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

diamond();
requestAnimationFrame(renderLoop);

