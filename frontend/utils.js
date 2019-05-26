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
    default_link_length: 10,
    default_link_strengh: 100,
    drag_coefficient: 0.0,
    stabilise_positions_enabled: false,
    stabiliser_power: 10,
    particles: [],
    links: []
});

const get_base_conf_copy = () => {
    return JSON.parse(JSON.stringify(BASE_CONF));
};

export {
    get_base_conf_copy
};
