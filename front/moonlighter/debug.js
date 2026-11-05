const debug = (engine, world, memory) => {
    // const particles_ptr = world.particles();
	// const particle_size = engine.Particle.size();
	// const particles_view = new DataView(
	// 	memory.buffer,
	// 	particles_ptr,
	// 	world.particles_count() * particle_size,
	// );
    // for (
    //     let i = 0;
    //     i < world.particles_count();
    //     i++
    // ) {
    //     console.log("-------")
    //     for (
    //         let j = 0;
    //         j <  engine.Particle.size();
    //         j++
    //     ) {
    //         try {
    //             const idx = i*particle_size + j;
    //             console.log(
    //                 idx,
    //                 j,
    //                 `@${particles_view.getUint32(idx, true)}`,
    //                 `@${particles_view.getFloat32(idx, true)}`,
    //                 `@${particles_view.getUint8(idx, true)}`,
    //                 `@${particles_view.getUint8(idx, false)}`,
    //             );
    //         } catch (error) {}
    //     }
    // }

    const links_ptr = world.links();
	const link_size = engine.Link.size();
	const links_view = new DataView(
		memory.buffer,
		links_ptr,
		world.links_count() * link_size,
	);

    for (
        let i = 0;
        i < world.links_count();
        i++
    ) {
        console.log("-------")
        for (
            let j = 0;
            j <  engine.Link.size();
            j++
        ) {
            try {
                const idx = i*link_size + j;
                console.log(
                    idx,
                    j,
                    `@${links_view.getUint32(idx, true)}`,
                    `@${links_view.getFloat32(idx, true)}`,
                    `@${links_view.getUint8(idx, true)}`,
                    `@${links_view.getUint8(idx, false)}`,
                );
            } catch (error) {}
        }
    }
}

export {
    debug,
}
