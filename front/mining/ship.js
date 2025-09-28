const Ship = (ships_view, view_sid) => {
    return {
      p: {
          x: ships_view.getFloat32(view_sid, true),
          y: ships_view.getFloat32(view_sid + 4, true),
      },
      v: {
        x: ships_view.getFloat32(view_sid + 4 * 4, true),
        y: ships_view.getFloat32(view_sid + 4 * 5, true),
      },
      t: {
        x: ships_view.getFloat32(view_sid + 4 * 6, true),
        y: ships_view.getFloat32(view_sid + 4 * 7, true),
      },
      td: {
        x: ships_view.getFloat32(view_sid + 4 * 8, true),
        y: ships_view.getFloat32(view_sid + 4 * 9, true),
      },
      orientation: {
        x: ships_view.getFloat32(view_sid + 4 * 10, true),
        y: ships_view.getFloat32(view_sid + 4 * 11, true),
      },
      vt: {
        x: ships_view.getFloat32(view_sid + 4 * 12, true),
        y: ships_view.getFloat32(view_sid + 4 * 13, true),
      },
      cross: {
        x: ships_view.getFloat32(view_sid + 4 * 14, true),
        y: ships_view.getFloat32(view_sid + 4 * 15, true),
      },
    }
  }
export {
    Ship,
}