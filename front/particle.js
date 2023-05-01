const randf = (min, max) => {
    return Math.random() * (max - min) + min
}


const particle = (view, view_pid, kind) => {
    const direction = {
        x: view.getFloat32(view_pid + 4*6, true),
        y: view.getFloat32(view_pid + 4*7, true),
    }
    const p = {
        x: view.getFloat32(view_pid, true),
        y: view.getFloat32(view_pid + 4, true),
    }
    const pout = {
        x: p.x + direction.x * 0.002 * randf(0.15, 0.45),
        y: p.y + direction.y * 0.002 * randf(0.15, 0.45),
    }
    const pout2 = {
        x: p.x + direction.x * 0.002 * randf(0.45, 0.65),
        y: p.y + direction.y * 0.002 * randf(0.45, 0.65),
    }
    const particle_ = {
        x:  p.x,
        y:  p.y,
        p: p,
        dx: direction.x,
        dy: direction.y,
        k:  view.getUint32(  view_pid + 4*9, true),
        k2: kind,
        a:  view.getInt32(  view_pid + 4*10, true),
        quantity:  view.getUint32(  view_pid + 4*11, true),
        live:  view.getUint32(  view_pid + 4*12, true),
        direction: direction,
        pout: pout,
        pout2: pout2,
    }
    if (particle_.k2 != 0 && particle_.k2 != 1) {
        // console.log(`kind properly set: ${particle_.k2}`)
        // throw `kind properly set: ${particle_.k2}`
    }
    // console.log(particle_)
    // for (let index = 0; index < 13; index++) {
    //     console.log(index, 
    //         view.getInt32(  view_pid + 4*index, true), 
    //         view.getUint32(view_pid + 4*index, true),
    //         view.getFloat32(view_pid + 4*index, true)
    //         )
    // }
    return particle_
}

export {
    particle,
}