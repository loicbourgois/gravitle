


const link_js = (view, view_pid) => {
    return {
        x:  view.getFloat32(view_pid + 4*2, true),
        y:  view.getFloat32(view_pid + 4*3, true),
        // dx: view.getFloat32(view_pid + 4*6, true),
        // dy: view.getFloat32(view_pid + 4*7, true),
        // k:  view.getInt32(  view_pid + 4*9, true),
        // a:  view.getInt32(  view_pid + 4*10, true),
    }
}

export {
    link_js
}