const randf = (min, max) => {
    return Math.random() * (max - min) + min
}


const particle = (view, view_pid) => {
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
    return {
        x:  p.x,
        y:  p.y,
        p: p,
        dx: direction.x,
        dy: direction.y,
        k:  view.getInt32(  view_pid + 4*9, true),
        a:  view.getInt32(  view_pid + 4*10, true),
        direction: direction,
        pout: pout,
        pout2: pout2,
    }
}


export {
    particle,
}