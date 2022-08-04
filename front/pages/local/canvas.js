const context_coordinates = (context, p) => {
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
  const cc = context_coordinates(context, p)
  context.font = `${size}px monospace`;
  context.fillStyle = "#fff"
  context.fillText(
    text,
    cc.x - size*text.length*0.3,
    cc.y + size * 0.33  );
}


const fill_circle = (context, p, diameter, color) => {
  const cc = context_coordinates(context, p)
  const radius = diameter * min_dim(context) * 0.25;
  context.beginPath();
  context.arc(cc.x, cc.y, radius, 0, 2 * Math.PI, false);
  context.fillStyle = color;
  context.fill();
}


const stroke_circle = (context, p, diameter, color, lineWidth) => {
  const cc = context_coordinates(context, p)
  const radius = diameter * min_dim(context) * 0.25;
  context.beginPath();
  context.arc(cc.x, cc.y, radius, 0, 2 * Math.PI, false);
  context.strokeStyle = color;
  context.lineWidth = lineWidth?lineWidth:2;
  context.stroke();
}


const clear = (context) => {
  context.clearRect(0,0,context.canvas.width, context.canvas.height)
}


const resize = (canvas) => {
  canvas.width = window.innerWidth
  canvas.height = window.innerHeight
}


export {
  clear,
  fill_circle,
  fill_text,
  stroke_circle,
  resize,
}
