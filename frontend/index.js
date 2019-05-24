import {
    Universe
} from 'gravitle';
import { memory } from "gravitle/gravitle_bg";

import * as Examples from './examples.js';

const fps_infos = document.getElementById('fps-infos');
const infos = document.getElementById('infos');
const reloadButton = document.getElementById('button-reload');
const stepButton = document.getElementById('button-step');
const startButton = document.getElementById('button-start');
const stopButton = document.getElementById('button-stop');
const heartButton = document.getElementById('button-heart');
const diamondButton = document.getElementById('button-diamond');
const randomizeButton = document.getElementById('button-randomize');
const symetryButton = document.getElementById('button-symetry');
const spaceCroquetButton = document.getElementById('button-space-croquet');
const clubButton = document.getElementById('button-club');
const spadeButton = document.getElementById('button-spade');
const buttonExample5 = document.getElementById('button-example-5');
const buttonExample6 = document.getElementById('button-example-6');
const buttonExample7 = document.getElementById('button-example-7');
const buttonExample8 = document.getElementById('button-example-8');
const jsonTextarea = document.getElementById('json');
const inputCount = document.getElementById('input-count');
const inputWidth = document.getElementById('input-width');
const inputHeight = document.getElementById('input-height');
const inputDeltaTime = document.getElementById('input-delta-time');
const inputG = document.getElementById('input-g');
const buttonGenerateSpaceship = document.getElementById('button-spaceship');
const selectAlgorithm = document.getElementById('select-algorithm');
const selectCollisionBehavior = document.getElementById('select-collision-behavior');
const selectIntersectionBehavior = document.getElementById('select-intersection-behavior');
const selectLinkIntersectionBehavior = document.getElementById('select-link-intersection-behavior');
const selectWrapAround = document.getElementById('select-wrap-around');
const selectFixedCloneCount = document.getElementById('select-fixed-clone-count');
const selectStabilisePositionsEnabled = document.getElementById('select-stabilise-positions-enabled');
const inputMinimalDistanceForGravity = document.getElementById('input-minimal-distance-for-gravity');
const inputDefaultLinkLength = document.getElementById('input-default-link-length');
const inputDefaultLinkStrengh = document.getElementById('input-default-link-strengh');
const inputDragCoefficient = document.getElementById('input-drag-coefficient');
const inputStabilisePower = document.getElementById('input-stabiliser-power');

const inputTrajectoriesPeriod = document.getElementById('input-trajectories-period');
const buttonTrajectoriesOn = document.getElementById('button-trajectories-on');
const buttonTrajectoriesOff = document.getElementById('button-trajectories-off');

const inputGravitationalFieldResolution = document.getElementById('input-gravitational-field-resolution');
const buttonGravitationalFieldOn = document.getElementById('button-gravitational-field-on');
const buttonGravitationalFieldOff = document.getElementById('button-gravitational-field-off');

const canvas = document.getElementById('canvas');
canvas.height = 1000;
canvas.width = 1000;
const context = canvas.getContext("2d");

let MODE = null;
let SHOW_TRAJECTORIES = null;
let SHOW_GRAVITATIONAL_FIELD = null;

const BASE_CONF = Object.freeze({
    algorithm: 'verlet',
    collision_behavior: 'do-nothing',
    intersection_behavior: 'do-nothing',
    link_intersection_behavior: 'do-nothing',
    wrap_around: false,
    fixed_clone_count: true,
    width: 200,
    height: 200,
    delta_time: 0.01,
    gravitational_constant: 66.74,
    minimal_distance_for_gravity: 1.0,
    default_link_length: 10,
    default_link_strengh: 100,
    drag_coefficient: 0.0,
    stabilise_positions_enabled: false,
    stabiliser_power: 10,
    particles: [],
    links: []
});

let space_croquet_links = null;
const SPACE_CROQUET_LINK_COUNT = 4;

const universe = Universe.new();
universe.load_from_json(JSON.stringify(BASE_CONF));
const launchers = [];

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

buttonExample6.addEventListener('click', () => {
    loadExample6();
});

buttonExample7.addEventListener('click', () => {
    loadExample7();
});

buttonExample8.addEventListener('click', () => {
    loadExample8();
});

spaceCroquetButton.addEventListener('click', () => {
    spaceCroquet();
});

buttonTrajectoriesOn.addEventListener('click', () => {
    trajectoriesOn();
});

buttonTrajectoriesOff.addEventListener('click', () => {
    trajectoriesOff();
});

buttonGravitationalFieldOn.addEventListener('click', () => {
    gravitationalFieldOn();
});

buttonGravitationalFieldOff.addEventListener('click', () => {
    gravitationalFieldOff();
});

buttonGenerateSpaceship.addEventListener('click', () => {
    generateSpaceship();
});

selectAlgorithm.addEventListener('change', () => {
    updateConf();
});

selectCollisionBehavior.addEventListener('change', () => {
    updateConf();
});

selectIntersectionBehavior.addEventListener('change', () => {
    updateConf();
});

selectLinkIntersectionBehavior.addEventListener('change', () => {
    updateConf();
});

selectWrapAround.addEventListener('change', () => {
    updateConf();
});

selectFixedCloneCount.addEventListener('change', () => {
    updateConf();
});

inputG.addEventListener('change', () => {
    updateConf();
});

inputWidth.addEventListener('change', () => {
    updateConf();
});

inputHeight.addEventListener('change', () => {
    updateConf();
});

inputDeltaTime.addEventListener('change', () => {
    updateConf();
});

selectStabilisePositionsEnabled.addEventListener('change', () => {
    updateConf();
});

inputMinimalDistanceForGravity.addEventListener('change', () => {
    updateConf();
});

inputDefaultLinkLength.addEventListener('change', () => {
    updateConf();
});

inputDefaultLinkStrengh.addEventListener('change', () => {
    updateConf();
});

inputDragCoefficient.addEventListener('change', () => {
    updateConf();
});

inputStabilisePower.addEventListener('change', () => {
    updateConf();
});

canvas.addEventListener('mousedown', (event) => {
    mouse_positions = {};
    mouse_positions.down = getMousePos(canvas, event);
    mouse_positions.up = mouse_positions.down;
});

canvas.addEventListener('mousemove', (event) => {
    if (mouse_positions) {
        mouse_positions.up = getMousePos(canvas, event);
    } else {
        // Do nothing
    }
});

canvas.addEventListener('mouseup', (event) => {
    mouse_positions.up = getMousePos(canvas, event);
    launchParticle(mouse_positions);
    mouse_positions = null;
});

const updateConf = () => {
    const conf = getParameterizedConf(JSON.parse(jsonTextarea.value));
    universe.set_algorithm_from_string(conf.algorithm);
    universe.set_gravitational_constant(conf.gravitational_constant);
    universe.set_width(conf.width);
    universe.set_height(conf.height);
    universe.set_delta_time(conf.delta_time);
    universe.set_collision_behavior_from_string(conf.collision_behavior);
    universe.set_intersection_behavior_from_string(conf.intersection_behavior);
    universe.set_link_intersection_behavior_from_string(conf.link_intersection_behavior);
    universe.set_wrap_around(conf.wrap_around);
    universe.set_fixed_clone_count(conf.fixed_clone_count);
    universe.set_stabilise_positions_enabled(conf.stabilise_positions_enabled);
    universe.set_minimal_distance_for_gravity(conf.minimal_distance_for_gravity);
    universe.set_default_link_length(conf.default_link_length);
    universe.set_default_link_strengh(conf.default_link_strengh);
    universe.set_drag_coefficient(conf.drag_coefficient);
    universe.set_stabiliser_power(conf.stabiliser_power);
    jsonTextarea.value = JSON.stringify(conf, null, 4);
}

const getParameterizedConf = (conf) => {
    conf.algorithm = selectAlgorithm.options[selectAlgorithm.selectedIndex].value;
    conf.gravitational_constant = parseFloat(inputG.value);
    conf.width = parseFloat(inputWidth.value);
    conf.height = parseFloat(inputHeight.value);
    conf.delta_time = parseFloat(inputDeltaTime.value);
    conf.collision_behavior = selectCollisionBehavior.options[selectCollisionBehavior.selectedIndex].value;
    conf.intersection_behavior = selectIntersectionBehavior.options[selectIntersectionBehavior.selectedIndex].value;
    conf.link_intersection_behavior = selectLinkIntersectionBehavior.options[selectLinkIntersectionBehavior.selectedIndex].value;
    conf.wrap_around = selectWrapAround.options[selectWrapAround.selectedIndex].value === 'true';
    conf.fixed_clone_count = selectFixedCloneCount.options[selectFixedCloneCount.selectedIndex].value === 'true';
    conf.stabilise_positions_enabled =
        selectStabilisePositionsEnabled.options[selectStabilisePositionsEnabled.selectedIndex].value === 'true';
    conf.minimal_distance_for_gravity = parseFloat(inputMinimalDistanceForGravity.value);
    conf.default_link_length = parseFloat(inputDefaultLinkLength.value);
    conf.default_link_strengh = parseFloat(inputDefaultLinkStrengh.value);
    conf.drag_coefficient = parseFloat(inputDragCoefficient.value);
    conf.stabiliser_power = parseInt(inputStabilisePower.value);
    return conf;
}

const reload = () => {
    // Reload parameters values fro json
    const parsedJson = JSON.parse(jsonTextarea.value);
    selectAlgorithm.value = parsedJson.algorithm;
    inputWidth.value = parsedJson.width;
    inputHeight.value = parsedJson.height;
    inputDeltaTime.value = parsedJson.delta_time;
    inputG.value = parsedJson.gravitational_constant;
    selectCollisionBehavior.value = parsedJson.collision_behavior;
    selectIntersectionBehavior.value = parsedJson.intersection_behavior;
    selectLinkIntersectionBehavior.value = parsedJson.link_intersection_behavior;
    selectWrapAround.value = parsedJson.wrap_around;
    selectFixedCloneCount.value = parsedJson.fixed_clone_count;
    selectStabilisePositionsEnabled.value = parsedJson.stabilise_positions_enabled;
    inputMinimalDistanceForGravity.value = parsedJson.minimal_distance_for_gravity;
    inputDefaultLinkLength.value = parsedJson.default_link_length;
    inputDefaultLinkStrengh.value = parsedJson.default_link_strengh;
    inputDragCoefficient.value = parsedJson.drag_coefficient;
    inputStabilisePower.value = parsedJson.stabiliser_power;
    // Reload universe
    stop();
    universe.reset();
    launchers.length = 0;
    interval = null;
    time = null;
    delta = null;
    universe.load_from_json(jsonTextarea.value);
    start();
};

const renderLoop = () => {
    // Setup analytics
    const start = Date.now();
    // Render
    fps_infos.textContent = `FPS : ${average_fps.toFixed(0)}\n`
        + `Frame : ${average_frame_length.toFixed(2)} ms`;
    infos.textContent = universe.get_infos();
    draw();
    requestAnimationFrame(renderLoop);
    // Update analytics
    updateFps();
    updateFrameLength(start);
}

const updateFrameLength = (start) => {
    const frame_length = Date.now() - start;
    frame_lengths.push(frame_length);
    while(frame_lengths.length > 100) {
        frame_lengths.shift();
    }
    let frame_lengths_sum = 0.0;
    const count = frame_lengths.length;
    for (let i = 0 ; i < count ; i += 1) {
        frame_lengths_sum += frame_lengths[i];
    }
    average_frame_length = frame_lengths_sum / count;
};

const updateFps = () => {
    const gap = Date.now() - last_now;
    last_now = Date.now();
    frame_gaps.push(gap);
    while(frame_gaps.length > 100) {
        frame_gaps.shift();
    }
    let gaps_sum = 0.0;
    const count = frame_gaps.length;
    for (let i = 0 ; i < count ; i+=1) {
        gaps_sum += frame_gaps[i];
    }
    average_fps = 1.0 / (gaps_sum / count / 1000.0);
};

const draw = () => {
    context.clearRect(0, 0, canvas.width, canvas.height);
    const gravitational_field_resolution = parseInt(inputGravitationalFieldResolution.value);
    if (SHOW_GRAVITATIONAL_FIELD === true && gravitational_field_resolution > 0) {
        drawGravitationalGrid(gravitational_field_resolution);
    } else {
        // Do nothing
    }
    if (MODE === 'SPACE-CROQUET') {
        drawLaunchers();
    } else {
        // Do nothing
    }
    const period = parseInt(inputTrajectoriesPeriod.value);
    if (SHOW_TRAJECTORIES === true && period > 0) {
        drawTrajectories(period);
    } else {
        // Do nothing
    }
    drawSegments();
    drawParticles();
    drawMouseInteraction();
};

const trajectoriesOn = () => {
    buttonTrajectoriesOn.disabled = true;
    buttonTrajectoriesOff.disabled = false;
    SHOW_TRAJECTORIES = true;
}

const trajectoriesOff = () => {
    buttonTrajectoriesOn.disabled = false;
    buttonTrajectoriesOff.disabled = true;
    SHOW_TRAJECTORIES = false;
}

const gravitationalFieldOn = () => {
    buttonGravitationalFieldOn.disabled = true;
    buttonGravitationalFieldOff.disabled = false;
    SHOW_GRAVITATIONAL_FIELD = true;
}

const gravitationalFieldOff = () => {
    buttonGravitationalFieldOn.disabled = false;
    buttonGravitationalFieldOff.disabled = true;
    SHOW_GRAVITATIONAL_FIELD = false;
}

const drawGravitationalGrid = (resolution) => {
    const width = resolution;
    const height = resolution;
    const grid = universe.get_gravitational_grid(width, height);
    let max = -Infinity;
    let min = Infinity;
    for (let i = 0, l = grid.length ; i < l ; i++) {
        grid[i] = Math.sqrt(grid[i]);
        if (grid[i] < min) {
            min = grid[i];
        } else {
            // Do nothing
        }
        if (grid[i] > max) {
            max = grid[i];
        } else {
            // Do nothing
        }
    }
    for (let i = 0 ; i < width ; i += 1) {
        for (let j = 0 ; j < height ; j += 1) {
            const value = (grid[i * width + j] - min) / (max-min) * 255;
            context.fillStyle = `rgba(${value*.9}, ${value*.9}, ${value}, 1)`;
            context.fillRect(i * canvas.width / width,
                (height-1-j) * canvas.height / height,
                canvas.width / width,
                canvas.height / height
            );
        }
    }
};

const drawLaunchers = () => {
    context.strokeStyle = "#888";
    context.lineWidth = 2;
    for (let i = 0 ; i < launchers.length ; i += 1 ) {
        context.beginPath();
        context.moveTo(launchers[i].up.x, launchers[i].up.y);
        context.lineTo(launchers[i].down.x, launchers[i].down.y);
        context.stroke();
    }
};

const drawTrajectories = (period) => {
    const trajectories = universe.get_trajectories_position_at_period(period);
    context.strokeStyle = "#888";
    context.lineWidth = 1;
    const diameter = 1;
    for (let i = 0 ; i < trajectories.length ; i += 2) {
        const p = getPositionFromUniverseToCanvas({
            x: trajectories[i + 0],
            y: trajectories[i + 1]
        });
        context.beginPath();
        context.arc(
            p.x,
            p.y,
            diameter,
            0,
            2 * Math.PI
        );
        context.stroke();
    }
}

const drawSegments = () => {
    const linksPointer = universe.get_links();
    const linksCount = universe.get_links_count();
    const LINK_SIZE = 7;
    const links = new Float64Array(memory.buffer, linksPointer, linksCount * LINK_SIZE);
    const universeWidth = universe.get_width();
    const universeHeight = universe.get_height();
    const unitX = canvas.width / universeWidth;
    const unitY = canvas.height / universeHeight;
    context.strokeStyle = "#eee";
    context.lineWidth = 2;
    for (let id = 0 ; id < linksCount ; id += 1 ) {
        let i = id * LINK_SIZE;
        const p1 = getPositionFromUniverseToCanvas({
            x: links[i + 0],
            y: links[i + 1]
        });
        const p2 = getPositionFromUniverseToCanvas({
            x: links[i + 2],
            y: links[i + 3]
        });
        context.beginPath();
        context.moveTo(p1.x, p1.y);
        context.lineTo(p2.x, p2.y);
        context.stroke();
    }
};

const drawParticles = () => {
    const particlesPointer = universe.get_particles();
    const particlesCount = universe.get_particles_count();
    const PARTICLE_SIZE = 13;
    const particles = new Float64Array(memory.buffer, particlesPointer, particlesCount * PARTICLE_SIZE);
    const unitX = canvas.width / universe.get_width();
    const unitY = canvas.height / universe.get_height();
    context.strokeStyle = "#FFF";
    context.lineWidth = 2;
    for (let i = 0 ; i < particles.length ; i+= PARTICLE_SIZE ) {
        const position = getPositionFromUniverseToCanvas({
            x: particles[i + 0],
            y: particles[i + 1]
        });
        const diameter = (unitX / 2) * particles[i + 2];
        context.beginPath();
        context.arc(
            position.x,
            position.y,
            diameter,
            0,
            2 * Math.PI
        );
        context.stroke();
    }
};

const drawMouseInteraction = () => {
    if (mouse_positions) {
        // Position
        const unitX = canvas.width / universe.get_width();
        const diameter = (unitX / 2);
        context.strokeStyle = "#eef";
        context.lineWidth = 2;
        context.beginPath();
        context.arc(
            mouse_positions.down.x,
            mouse_positions.down.y,
            diameter,
            0,
            2 * Math.PI
        );
        context.stroke();
        // Line
        context.strokeStyle = "#ddf";
        context.beginPath();
        context.moveTo(mouse_positions.down.x, mouse_positions.down.y);
        context.lineTo(mouse_positions.up.x, mouse_positions.up.y);
        context.stroke();
    } else {
        // Do nothing
    }
}

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
    if (universe.get_particles_to_disable_indexes_length() && MODE === 'SPACE-CROQUET') {
        universe.set_links_json(JSON.stringify(space_croquet_links));
    } else {
        // Do nothing
    }
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
    MODE = null;
    const conf = Examples.get_example_1_conf(jsonCopy(BASE_CONF));
    jsonTextarea.value = JSON.stringify(conf, null, 4);
    reload();
};

const diamond = () => {
    MODE = null;
    const conf = Examples.get_example_2_conf(jsonCopy(BASE_CONF));
    jsonTextarea.value = JSON.stringify(conf, null, 4);
    reload();
};

const club = () => {
    MODE = null;
    const conf = Examples.get_example_3_conf(jsonCopy(BASE_CONF));
    jsonTextarea.value = JSON.stringify(conf, null, 4);
    reload();
}

const spade = () => {
    MODE = null;
    const conf = Examples.get_example_4_conf(jsonCopy(BASE_CONF));
    jsonTextarea.value = JSON.stringify(conf, null, 4);
    reload();
}

const loadExample5 = () => {
    MODE = null;
    const conf = Examples.get_example_5_conf(jsonCopy(BASE_CONF));
    jsonTextarea.value = JSON.stringify(conf, null, 4);
    reload();
}

const loadExample6 = () => {
    MODE = null;
    const conf = Examples.get_example_6_conf(jsonCopy(BASE_CONF));
    jsonTextarea.value = JSON.stringify(conf, null, 4);
    reload();
}

const loadExample7 = () => {
    MODE = null;
    const conf = Examples.get_example_7_conf(jsonCopy(BASE_CONF));
    jsonTextarea.value = JSON.stringify(conf, null, 4);
    reload();
}

const loadExample8 = () => {
    MODE = null;
    const conf = Examples.get_example_8_conf(jsonCopy(BASE_CONF));
    jsonTextarea.value = JSON.stringify(conf, null, 4);
    reload();
}

const randomize = () => {
    MODE = null;
    const conf = getParameterizedConf(JSON.parse(jsonTextarea.value));
    const particles = [];
    for (let i = 0 ; i < parseFloat(inputCount.value) ; i += 1) {
        const x = getRandomNumber(- conf.width / 5, conf.width / 5);
        const y = getRandomNumber(- conf.height / 5, conf.height / 5);
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
    conf.links = [];
    jsonTextarea.value = JSON.stringify(conf, null, 4);
    reload();
}

const symetry = () => {
    MODE = 'SYMETRY';
    const conf = getParameterizedConf(JSON.parse(jsonTextarea.value));
    const particles = [];
    for (let i = 0 ; i < parseFloat(inputCount.value) ; i += 2) {
        const x = getRandomNumber(- conf.width / 5, conf.width / 5);
        const y = getRandomNumber(- conf.height / 5, conf.height / 5);
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
    conf.links = [];
    jsonTextarea.value = JSON.stringify(conf, null, 4);
    reload();
}

const spaceCroquet = () => {
    MODE = 'SPACE-CROQUET';
    const conf = jsonCopy(BASE_CONF);
    conf.intersection_behavior = 'destroy-link';
    conf.wrap_around = true;
    const particles = [];
    const links = [];
    const zones = [];
    const maxDiameter = 5.0;
    const checkpointLength = conf.width / 8;
    const innerRadius = checkpointLength / 2;
    const zoneRadius = innerRadius + maxDiameter / 2;
    for (let i = 0 ; i < SPACE_CROQUET_LINK_COUNT ; i += 1) {
        let x = getRandomIntInclusive(- conf.width / 4, conf.width / 4);
        let y = getRandomIntInclusive(- conf.height / 4, conf.height / 4);
        let i = 1000;
        while (isInZones(x, y, zones, zoneRadius) && i > 0) {
            x = getRandomIntInclusive(- conf.width / 4, conf.width / 4);
            y = getRandomIntInclusive(- conf.height / 4, conf.height / 4);
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
        const mass = getRandomNumber(maxDiameter, maxDiameter);
        const fixed = true;
        const diameter = mass;
        const angle = getRandomIntInclusive(0, 359);
        const p1 = getCoordinateRotatedAround(
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
        const p2 = getCoordinateRotatedAround(
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
    conf.particles = particles;
    conf.links = links;
    space_croquet_links = links;
    jsonTextarea.value = JSON.stringify(conf, null, 4);
    reload();
}

const generateSpaceship = () => {
    MODE = 'SPACE-SHIP';
    const conf = jsonCopy(BASE_CONF);
    conf.collision_behavior = 'create-link';
    conf.intersection_behavior = 'do-nothing';
    conf.link_intersection_behavior = 'do-nothing';
    conf.gravitational_constant = 10;
    conf.default_link_length = 10;
    conf.default_link_strengh = 1000;
    conf.drag_coefficient = 1;
    conf.stabilise_positions_enabled = false;
    conf.stabiliser_power = 10;
    conf.minimal_distance_for_gravity = 1.0;
    const COUNT = 16;
    const DIVISOR = 30;
    const particles = [];
    const minDiameter = 4.0;
    const maxDiameter = 5.0;
    const MASS = 1.0;
    const diameter = getRandomNumber(minDiameter, maxDiameter);
    particles.push({
        x: 0,
        y: 0,
        mass: MASS,
        diameter: diameter
    });
    particles.push({
        x: 0,
        y: 0,
        mass: MASS,
        diameter: diameter
    });
    for (let i = 2 ; i < COUNT ; i += 2) {
        const x = getRandomNumber(- conf.width / DIVISOR, conf.width / DIVISOR);
        const y = getRandomNumber(- conf.height / DIVISOR, conf.height / DIVISOR);
        const diameter = getRandomNumber(minDiameter, maxDiameter);
        particles.push({
            x: x,
            y: y,
            mass: MASS,
            diameter: diameter
        });
        particles.push({
            x: -x,
            y: y,
            mass: MASS,
            diameter: diameter
        });
    }
    conf.particles = particles;
    jsonTextarea.value = JSON.stringify(conf, null, 4);
    reload();
}

const getCoordinateRotatedAround = (center, point, angle) => {
    const angleRad = (angle) * (Math.PI / 180);
    return {
        x: Math.cos(angleRad) * (point.x - center.x) - Math.sin(angleRad) * (point.y - center.y) + center.x,
        y: Math.sin(angleRad) * (point.x - center.x) + Math.cos(angleRad) * (point.y - center.y) + center.y
    };
}

const isInZones = (x, y, zones, zoneRadius) => {
    let r = false;
    for (const index in zones) {
        const zone = zones[index];
        if (circlesCollide(x, y, zone.x, zone.y, zone.radius, zoneRadius)) {
            r = true;
        } else {
            // Do nothing
        }
    }
    return r;
}

const circlesCollide = (x1, y1, x2, y2, zoneRadius1, zoneRadius2) => {
    const distance_squared_centers = get_distance_squared(x1, y1, x2, y2);
    const diameters_squared = (zoneRadius1 + zoneRadius2) * (zoneRadius1 + zoneRadius2);
    return distance_squared_centers < diameters_squared;
}

const get_distance_squared = (x1, y1, x2, y2) => {
    const delta_x = x1 - x2;
    const delta_y = y1 - y2;
    return delta_x * delta_x + delta_y * delta_y;
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

const getIndex = (row, column) => {
    return row * width + column;
};

const getMousePos = (canvas, event) => {
    const rect = canvas.getBoundingClientRect(); // abs. size of element
    const scaleX = canvas.width / rect.width;   // relationship bitmap vs. element for X
    const scaleY = canvas.height / rect.height;  // relationship bitmap vs. element for Y
    return {
        x: (event.clientX - rect.left) * scaleX,   // scale mouse coordinates after they have
        y: (event.clientY - rect.top) * scaleY     // been adjusted to be relative to element
    }
}

const getPositionFromCanvasToUniverse = (position_in_canvas) => {
    const universeWidth = universe.get_width();
    const universeHeight = universe.get_height();
    return {
        x: position_in_canvas.x / canvas.width * universeWidth - universeWidth * 0.5,
        y: - (position_in_canvas.y / canvas.height * universeHeight - universeHeight * 0.5)
    }
}

const getPositionFromUniverseToCanvas = (position_in_universe) => {
    const universeWidth = universe.get_width();
    const universeHeight = universe.get_height();
    const unitX = canvas.width / universeWidth;
    const unitY = canvas.height / universeHeight;
    return {
        x: (universeWidth / 2) * unitX + position_in_universe.x * unitX,
        y: (universeHeight / 2) * unitY - position_in_universe.y * unitY
    }
}

const launchParticle = (mouse_position) => {
    const position_in_universe = getPositionFromCanvasToUniverse(mouse_positions.down);
    const position_in_universe_old = getPositionFromCanvasToUniverse(mouse_positions.up);
    const dx = position_in_universe_old.x - position_in_universe.x;
    const dy = position_in_universe_old.y - position_in_universe.y;
    const sensibility = 0.01;
    let collision_behavior = 'do-nothing';
    if (MODE === 'SPACE-CROQUET') {
        collision_behavior = 'disable-self';
    } else {
        // Do nothing
    }
    universe.add_particle_json(JSON.stringify(
        {
            x: position_in_universe.x,
            y: position_in_universe.y,
            old_x: position_in_universe.x + dx * sensibility,
            old_y: position_in_universe.y + dy * sensibility,
            collision_behavior: collision_behavior
        }
    ));
    launchers.push(mouse_position);
    if (MODE === 'SPACE-CROQUET') {
        universe.set_links_json(JSON.stringify(space_croquet_links));
    } else {
        // Do nothing
    }
}

trajectoriesOff();
gravitationalFieldOff();
heart();
last_now = Date.now();
requestAnimationFrame(renderLoop);

