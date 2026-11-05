import { resize_canvas } from "./canvas.js";


const setup_rendering = () => {
    const canvas = document.createElement('canvas');
    canvas.id = 'canvas';
    canvas.style.zIndex = '1'
    canvas.style.background = 'transparent';
    document.body.appendChild(canvas);
    resize_canvas(canvas);
    window.addEventListener('resize', () => {resize_canvas(canvas)});
    return canvas.getContext('2d');
}


const setup_rendering_webgpu = async () => {
    if (!navigator.gpu) {
        console.warn("WebGPU not supported, skipping overlay canvas");
        return null;
    }
    const adapter = await navigator.gpu.requestAdapter();
    if (!adapter) {
        console.warn("No WebGPU adapter, skipping overlay canvas");
        return null;
    }
    const device = await adapter.requestDevice();

    const canvas = document.createElement('canvas');
    canvas.id = 'canvas_webgpu';
    canvas.style.position = 'fixed';
    canvas.style.inset = '0';
    canvas.style.pointerEvents = 'none';
    document.body.appendChild(canvas);
    resize_canvas(canvas);
    window.addEventListener('resize', () => {resize_canvas(canvas)});

    const context = canvas.getContext('webgpu');
    const format = navigator.gpu.getPreferredCanvasFormat();
    context.configure({ device, format, alphaMode: 'premultiplied' });

    return { device, context, format };
}


export {
    setup_rendering,
    setup_rendering_webgpu,
}