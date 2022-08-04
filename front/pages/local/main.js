import {
  resize,
  stroke_circle,
} from "./canvas"


const local_main = () => {
  document.querySelector('#content').innerHTML = `
    <canvas id="canvas"></canvas>
  `
  const style = document.createElement('style')
  document.head.appendChild(style)
  style.sheet.insertRule(`
      #canvas {
          background: blue;
      }
  `);
  const canvas = document.querySelector('#canvas')
  resize(canvas)
  const context = canvas.getContext('2d')
  stroke_circle(context, {
    x: 0.0,
    y: 0.0,
  }, 0.1, '#fff')
}


export {
  local_main
}
