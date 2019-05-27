import {
    Universe
} from 'gravitle';
import { memory } from "gravitle/gravitle_bg";

import * as Examples from './examples.js';
import * as render from './render.js';
import * as Tests from './tests.js';
import * as utils from './utils.js';

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
const selectWrapAroundBehavior = document.getElementById('select-wrap-around-behavior');
const inputMinimalDistanceForGravity = document.getElementById('input-minimal-distance-for-gravity');
const inputDefaultLinkLength = document.getElementById('input-default-link-length');
const inputDefaultLinkStrengh = document.getElementById('input-default-link-strengh');
const inputDragCoefficient = document.getElementById('input-drag-coefficient');
const inputStabilisePower = document.getElementById('input-stabiliser-power');
const selectTest = document.getElementById('select-test');
const buttonRunTest = document.getElementById('button-run-test');
const testDescription = document.getElementById('test-description');

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
let bindings = {};

let MODE = null;
let SHOW_TRAJECTORIES = null;
let SHOW_GRAVITATIONAL_FIELD = null;

const BASE_CONF = utils.get_base_conf_copy();
const tests = Tests.get_tests();
tests.forEach(test => {
    let option = document.createElement('option');
    option.appendChild(document.createTextNode(test.title));
    option.value = test.id;
    selectTest.appendChild(option);
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
    reloadFromJSON();
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

buttonRunTest.addEventListener('click', () => {
    runTest();
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

selectWrapAroundBehavior.addEventListener('change', () => {
    updateConf();
});

selectTest.addEventListener('change', () => {
    selectTestChange();
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

const keyup = (e) => {
    if (bindings && bindings[e.key]) {
        universe.deactivate_thrust_for_links(bindings[e.key].link_indexes);
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
    universe.set_wrap_around_behavior_from_string(conf.wrap_around_behavior);
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
    conf.wrap_around_behavior = selectWrapAroundBehavior.options[selectWrapAroundBehavior.selectedIndex].value;
    return conf;
}

const reloadFromJSON = () => {
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
    selectWrapAroundBehavior.value = parsedJson.wrap_around_behavior;
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
    render.draw(
        context,
        parseInt(inputGravitationalFieldResolution.value),
        SHOW_GRAVITATIONAL_FIELD,
        MODE,
        parseInt(inputTrajectoriesPeriod.value),
        SHOW_TRAJECTORIES,
        universe,
        memory,
        mouse_positions,
        launchers
    );
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
    reloadFromJSON();
};

const diamond = () => {
    MODE = null;
    const conf = Examples.get_example_2_conf(jsonCopy(BASE_CONF));
    jsonTextarea.value = JSON.stringify(conf, null, 4);
    reloadFromJSON();
};

const club = () => {
    MODE = null;
    const conf = Examples.get_example_3_conf(jsonCopy(BASE_CONF));
    jsonTextarea.value = JSON.stringify(conf, null, 4);
    reloadFromJSON();
}

const spade = () => {
    MODE = null;
    const conf = Examples.get_example_4_conf(jsonCopy(BASE_CONF));
    jsonTextarea.value = JSON.stringify(conf, null, 4);
    reloadFromJSON();
}

const loadExample5 = () => {
    MODE = null;
    const conf = Examples.get_example_5_conf(jsonCopy(BASE_CONF));
    jsonTextarea.value = JSON.stringify(conf, null, 4);
    reloadFromJSON();
}

const loadExample6 = () => {
    MODE = null;
    const conf = Examples.get_example_6_conf(jsonCopy(BASE_CONF));
    jsonTextarea.value = JSON.stringify(conf, null, 4);
    reloadFromJSON();
}

const loadExample7 = () => {
    MODE = null;
    const conf = Examples.get_example_7_conf(jsonCopy(BASE_CONF));
    jsonTextarea.value = JSON.stringify(conf, null, 4);
    reloadFromJSON();
}

const loadExample8 = () => {
    MODE = null;
    const conf = Examples.get_example_8_conf(jsonCopy(BASE_CONF));
    jsonTextarea.value = JSON.stringify(conf, null, 4);
    reloadFromJSON();
}

const selectTestChange = () => {
    const testId = selectTest.options[selectTest.selectedIndex].value;
    const test = Tests.get_test_by_id(testId);
    testDescription.innerHTML = test.description;
}

const runTest = (testId_) => {
    const testId = testId_ ? testId_ : selectTest.options[selectTest.selectedIndex].value;
    const test = Tests.get_test_by_id(testId);
    MODE = test.id;
    bindings = test.bindings;
    jsonTextarea.value = JSON.stringify(test.conf, null, 4);
    reloadFromJSON();
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
    reloadFromJSON();
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
    reloadFromJSON();
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
    reloadFromJSON();
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
    conf.wrap_around = true;
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
    reloadFromJSON();
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

heartButton.addEventListener('click', heart);
document.addEventListener('keyup', keyup);
document.addEventListener('keydown', keydown);

trajectoriesOff();
gravitationalFieldOff();
heart();
last_now = Date.now();
requestAnimationFrame(renderLoop);
selectTestChange();
runTest('test_10');
canvas.focus();
