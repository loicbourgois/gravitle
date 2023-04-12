


const link_js = (view, view_pid) => {
    return {
        ak:  view.getInt32(  view_pid + 4*0, true),
        bk:  view.getInt32(view_pid + 4*1, true),
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