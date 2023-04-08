const particle = (view, view_pid) => {
    return {
        x:  view.getFloat32(view_pid, true),
        y:  view.getFloat32(view_pid + 4, true),
        dx: view.getFloat32(view_pid + 4*6, true),
        dy: view.getFloat32(view_pid + 4*7, true),
        k:  view.getInt32(  view_pid + 4*9, true),
        a:  view.getInt32(  view_pid + 4*10, true),
    }
}


export {
    particle,
}