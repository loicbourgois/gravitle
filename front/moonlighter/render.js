import { resize_canvas } from "./canvas.js";


const setup_rendering = () => {
    const canvas = document.createElement('canvas');
    canvas.id = 'canvas';
    document.body.appendChild(canvas);
    resize_canvas(canvas);
    window.addEventListener('resize', () => {resize_canvas(canvas)});
    return canvas.getContext('2d');
}


export {
    setup_rendering,
}
