console.log("start")
const canvas = document.getElementById("favicon")
const context = canvas.getContext("2d");

const draw_disk = (context,x,y, color, r=4) => {
    context.beginPath();
    context.arc(x,y+1, r, 0, 2 * Math.PI, false);
    context.fillStyle = color;
    context.fill();
}

draw_disk(context,16,10, "#aaf", 4.5)
draw_disk(context,10,13, "#aaf", 4)
draw_disk(context,22,13, "#aaf", 4)
draw_disk(context,10,19, "#aaf", 4)
draw_disk(context,22,19, "#aaf", 4)
draw_disk(context,16,22, "#aaf", 4.5)
draw_disk(context,16,16, "#ff0", 8)
draw_disk(context,8,16, "#aaf")
draw_disk(context,24,16, "#aaf")
draw_disk(context,20,9, "#aaf")
draw_disk(context,12,9, "#aaf")
draw_disk(context,20,23, "#aaf")
draw_disk(context,12,23, "#aaf")
