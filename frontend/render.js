const draw = (
        context,
        gravitational_field_resolution,
        SHOW_GRAVITATIONAL_FIELD,
        MODE,
        period,
        SHOW_TRAJECTORIES,
        universe,
        memory,
        mouse_positions,
        launchers
) => {
    context.clearRect(0, 0, canvas.width, canvas.height);
    if (SHOW_GRAVITATIONAL_FIELD === true && gravitational_field_resolution > 0) {
        drawGravitationalGrid(universe, context, gravitational_field_resolution);
    } else {
        // Do nothing
    }
    if (MODE === 'SPACE-CROQUET') {
        drawLaunchers(context, launchers);
    } else {
        // Do nothing
    }
    if (SHOW_TRAJECTORIES === true && period > 0) {
        drawTrajectories(universe, context, period);
    } else {
        // Do nothing
    }
    drawSegments(universe, context, memory);
    drawParticles(universe, context, memory);
    drawMouseInteraction(universe, context, mouse_positions);
};

const drawGravitationalGrid = (universe, context, resolution) => {
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

const drawLaunchers = (context, launchers) => {
    context.strokeStyle = "#888";
    context.lineWidth = 2;
    for (let i = 0 ; i < launchers.length ; i += 1 ) {
        context.beginPath();
        context.moveTo(launchers[i].up.x, launchers[i].up.y);
        context.lineTo(launchers[i].down.x, launchers[i].down.y);
        context.stroke();
    }
};

const drawTrajectories = (universe, context, period) => {
    const trajectories = universe.get_trajectories_position_at_period(period);
    context.strokeStyle = "#888";
    context.lineWidth = 1;
    const diameter = 1;
    for (let i = 0 ; i < trajectories.length ; i += 2) {
        const p = getPositionFromUniverseToCanvas(universe, {
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

const drawSegments = (universe, context, memory) => {
    const link_coordinates = universe.get_links_coordinates_to_draw();
    context.strokeStyle = "#eee";
    context.lineWidth = 2;
    for (let i = 0, l = link_coordinates.length ; i < l ; i += 4 ) {
        const p1 = getPositionFromUniverseToCanvas(universe, {
            x: link_coordinates[i + 0],
            y: link_coordinates[i + 1]
        });
        const p2 = getPositionFromUniverseToCanvas(universe, {
            x: link_coordinates[i + 2],
            y: link_coordinates[i + 3]
        });
        context.beginPath();
        context.moveTo(p1.x, p1.y);
        context.lineTo(p2.x, p2.y);
        context.stroke();
    }
};

const drawParticles = (universe, context, memory) => {
    const particlesPointer = universe.get_particles();
    const particlesCount = universe.get_particles_count();
    const PARTICLE_SIZE = 14;
    const particles = new Float64Array(memory.buffer, particlesPointer, particlesCount * PARTICLE_SIZE);
    const unitX = canvas.width / universe.get_width();
    const unitY = canvas.height / universe.get_height();
    context.strokeStyle = "#FFF";
    context.lineWidth = 2;
    for (let i = 0 ; i < particles.length ; i+= PARTICLE_SIZE ) {
        const position = getPositionFromUniverseToCanvas(universe, {
            x: particles[i + 0],
            y: particles[i + 1]
        });
        const diameter = (unitX * 0.5) * particles[i + 2];
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

const drawMouseInteraction = (universe, context, mouse_positions) => {
    if (mouse_positions) {
        // Position
        const unitX = canvas.width / universe.get_width();
        const diameter = (unitX * 0.5);
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

const getPositionFromUniverseToCanvas = (universe, position_in_universe) => {
    const universeWidth = universe.get_width();
    const universeHeight = universe.get_height();
    const unitX = canvas.width / universeWidth;
    const unitY = canvas.height / universeHeight;
    return {
        x: (universeWidth * 0.5) * unitX + position_in_universe.x * unitX,
        y: (universeHeight * 0.5) * unitY - position_in_universe.y * unitY
    }
}

export {
    draw
};
