const resize_canvas = (canvas) => {
    canvas.width = canvas.clientWidth*2;
    canvas.height = canvas.clientHeight*2;
};

export {
    resize_canvas,
}
