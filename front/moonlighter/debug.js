const debug = (engine, world, memory) => {
    const particles_ptr = world.particles();
	const particle_size = engine.Particle.size();
	const particles_view = new DataView(
		memory.buffer,
		particles_ptr,
		world.particles_count() * particle_size,
	);
    for (
        let i = 0;
        i < world.particles_count();
        i++
    ) {
        console.log("-------")
        for (
            let j = 0;
            j <  engine.Particle.size();
            j++
        ) {
            try {
                const idx = i*particle_size + j;
                console.log(
                    idx,
                    j,
                    `@${particles_view.getUint32(idx, true)}`,
                    `@${particles_view.getFloat32(idx, true)}`,
                    `@${particles_view.getUint8(idx, true)}`,
                    `@${particles_view.getUint8(idx, false)}`,
                );
            } catch (error) {}
        }
    }
}

export {
    debug,
}
