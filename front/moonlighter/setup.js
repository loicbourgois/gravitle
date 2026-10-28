const setup = (data) => {
    data.particles.add(0, 0, 'MOON');
    data.particles.add(1, 0, 'MOON');
    data.particles.add(-1, 0, 'MOON');
    data.particles.add(0, 1, 'MOON');
    data.particles.add(0, -1, 'MOON');
    data.particles.add(5, 0.1, 'PIPE');
    data.particles.add(4, 0, 'PIPE');
    data.particles.add(3, 0.1, 'PIPE');
    data.particles.add(2, -0.1, 'PIPE');
    data.particles.add(6, 0.1, 'FILTER');
    data.particles.add(7, 0.2, 'PUMP');
    data.links.add(5, 6)
    data.links.add(6, 7)
    data.links.add(7, 8)
    data.links.add(8, 1)
    data.links.add(5, 9)
    data.links.add(10, 9)
}

export {
    setup,
}
