// Context Point
const cp = (context, p) => {
  return {
    x: p.x * context.canvas.width,
    y: context.canvas.height - p.y * context.canvas.height,
  }
}


// Context Point
const cp2 = (context, p) => {
  return {
    x: 0.5 * context.canvas.width + min_dim(context) * p.x*0.5,
    y: 0.5 * context.canvas.height -  min_dim(context) * p.y*0.5,
  }
}


const min_dim = (context) => {
  return Math.min(context.canvas.width, context.canvas.height)
}


const fill_text = (context, p, text) => {
  const size = 14
  const cp_ = cp(context, p)
  context.font = `${size}px monospace`;
  context.fillStyle = "#fff"
  context.fillText(
    text,
    cp_.x - size*text.length*0.3,
    cp_.y + size * 0.33  );
}


const fill_circle = (context, p, diameter, color) => {
  const cp_ = cp(context, p)
  const radius = diameter * min_dim(context) * 0.5;
  context.beginPath();
  context.arc(cp_.x, cp_.y, radius, 0, 2 * Math.PI, false);
  context.fillStyle = color;
  context.fill();
}


const stroke_circle = (context, p, diameter, color, lineWidth) => {
  const cp_ = cp(context, p)
  const radius = diameter * min_dim(context) * 0.25;
  context.beginPath();
  context.arc(cp_.x, cp_.y, radius, 0, 2 * Math.PI, false);
  context.strokeStyle = color;
  context.lineWidth = lineWidth?lineWidth:2;
  context.stroke();
}


const resize_canvas = (context, width, height) => {
  context.canvas.width = width
  context.canvas.height = height
}

// const stroke_circle = (context, p, diameter, color, lineWidth) => {
//   const cp_ = cp(context, p)
//   const radius = diameter * min_dim(context) * 0.25;
//   context.beginPath();
//   context.arc(cp_.x, cp_.y, radius, 0, 2 * Math.PI, false);
//   context.strokeStyle = color;
//   context.lineWidth = lineWidth?lineWidth:2;
//   context.stroke();
// }


const clear = (context) => {
  context.clearRect(0,0,context.canvas.width, context.canvas.height)
}


export {
  clear,
  fill_circle,
  fill_text,
  stroke_circle,
  resize_canvas,
}
