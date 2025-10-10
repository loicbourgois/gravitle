import {
  distance_sqrd,
} from "./math.js"

const context_coordinates = (context, p) => {
  return {
    x: min_dim(context) * p.x,
    y: context.canvas.height -  min_dim(context) * p.y,
  }
}


const min_dim = (context) => {
  return Math.min(context.canvas.width, context.canvas.height)
}


const fill_text = (context, p, text, size=14, color="#fff") => {
  text = ""+`${text}`
  const cc = context_coordinates(context, p)
  context.font = `${size}px monospace`;
  context.fillStyle = color
  context.fillText(
    text,
    cc.x - size*text.length*0.3,
    cc.y + size * 0.33  );
}


const fill_circle = (context, p, diameter, color) => {
  const aa = 0.005
  if (
    p.x < aa
    || p.y < aa
    || p.x > 1-aa
    || p.y > 1-aa
  ) {
    return
  }


  const cc = context_coordinates(context, p)
  const radius = diameter * min_dim(context) * 0.5;
  context.beginPath();
  context.arc(cc.x, cc.y, radius, 0, 2 * Math.PI, false);
  context.fillStyle = color;
  context.fill();
}


const stroke_circle = (context, p, diameter, color, lineWidth) => {
  const cc = context_coordinates(context, p)
  const radius = diameter * min_dim(context) * 0.5;
  context.beginPath();
  context.arc(cc.x, cc.y, radius, 0, 2 * Math.PI, false);
  context.strokeStyle = color;
  context.lineWidth = lineWidth?lineWidth:2;
  context.stroke();
}


const stroke_circle_2 = (context, p, diameter, color, lineWidth) => {
  for (var xy of [[0,0],[1,0],[0,1],[0,-1],[-1,0]]) {
    const pp = {
      x: p.x + xy[0],
      y: p.y + xy[1],
    }
    stroke_circle(context, pp, diameter, color, lineWidth)
  }
}


const fill_circle_2 = (context, p, diameter, color) => {
  if ( distance_sqrd(p, {x:0.5,y:0.5}) < 0.45 * 0.45 ) {
    fill_circle(context, p, diameter, color)
  } else {
    for (var xy of [[0,0],[1,0],[0,1],[0,-1],[-1,0]]) {
      const pp = {
        x: p.x + xy[0],
        y: p.y + xy[1],
      }
      fill_circle(context, pp, diameter, color)
    }
  }
}


const clear = (context) => {
  context.clearRect(0,0,context.canvas.width, context.canvas.height)
}


const resize = (canvas) => {
  canvas.width = window.innerWidth
  canvas.height = window.innerHeight
}


const line = (context, p1, p2, color, line_width) => {
  const cc1 = context_coordinates(context, p1)
  const cc2 = context_coordinates(context, p2)
  context.beginPath();
  context.moveTo(cc1.x, cc1.y);
  context.lineTo(cc2.x, cc2.y);
  context.strokeStyle = color;
  context.lineWidth = line_width?line_width:2;
  context.stroke();
}


const resize_square = (canvas) => {
  const dim = Math.min(window.innerWidth, window.innerHeight)
  canvas.width = dim
  canvas.height = dim
}


export {
  clear,
  fill_circle,
  fill_circle_2,
  fill_text,
  stroke_circle,
  stroke_circle_2,
  resize,
  resize_square,
  line,
}
