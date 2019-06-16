const BASE_CONF = Object.freeze({
    algorithm: 'verlet',
    collision_behavior: 'do-nothing',
    intersection_behavior: 'do-nothing',
    link_intersection_behavior: 'do-nothing',
    wrap_around: false,
    wrap_around_behavior: 'do-nothing',
    fixed_clone_count: true,
    width: 200,
    height: 200,
    delta_time: 0.01,
    gravitational_constant: 66.74,
    minimal_distance_for_gravity: 1.0,
    default_link_length: 10.0,
    default_link_strengh: 100.0,
    default_link_thrust_force: 100.0,
    drag_coefficient: 0.0,
    stabilise_positions_enabled: false,
    stabiliser_power: 10,
    particles: [],
    links: []
});

const get_base_conf_copy = () => {
    return JSON.parse(JSON.stringify(BASE_CONF));
};

const get_random_boolean = () => {
    return Math.random() > 0.5;
}

const get_random_number = (min, max) => {
    return Math.random() * (max - min) + min;
}

const get_random_int_inclusive = (min, max) => {
    min = Math.ceil(min);
    max = Math.floor(max);
    return Math.floor(Math.random() * (max - min + 1)) + min;
}

const circles_collide = (x1, y1, x2, y2, zoneRadius1, zoneRadius2) => {
    const distance_squared_centers = get_distance_squared(x1, y1, x2, y2);
    const diameters_squared = (zoneRadius1 + zoneRadius2) * (zoneRadius1 + zoneRadius2);
    return distance_squared_centers < diameters_squared;
}

const get_distance_squared = (x1, y1, x2, y2) => {
    const delta_x = x1 - x2;
    const delta_y = y1 - y2;
    return delta_x * delta_x + delta_y * delta_y;
}

const get_coordinate_rotated_around = (center, point, angle) => {
    const angleRad = (angle) * (Math.PI / 180);
    return {
        x: Math.cos(angleRad) * (point.x - center.x) - Math.sin(angleRad) * (point.y - center.y) + center.x,
        y: Math.sin(angleRad) * (point.x - center.x) + Math.cos(angleRad) * (point.y - center.y) + center.y
    };
}

export {
    get_base_conf_copy,
    get_random_number,
    get_random_int_inclusive,
    circles_collide,
    get_distance_squared,
    get_coordinate_rotated_around
};
