import { draw_cells, draw_ship_only } from "./draw_cells.js";

function View2d(id) {
	this.canvas = document.getElementById(id);
	this.context = canvas.getContext("2d");
	this.center = {
		x: 0.5,
		y: 0.5,
	};
	this.zoom = 1;
	this.mouse = null;
	this.resize();
}

View2d.prototype.set_backgound = function (color) {
	this.background_color = color;
	this.context.fillStyle = color;
	this.context.fillRect(0, 0, this.canvas.width, this.canvas.height);
};

View2d.prototype.render = function (worlds, ghosts, gravitle, memory) {
	this.set_backgound("#102");
	for (let i = 1; i < worlds.length; i++) {
		if (ghosts[i - 1].kind == "me") {
			draw_ship_only(gravitle, worlds[i], memory, this, "g");
		}
	}
	for (let i = 1; i < worlds.length; i++) {
		if (ghosts[i - 1].kind != "me") {
			draw_ship_only(gravitle, worlds[i], memory, this, "o");
		}
	}
	draw_cells(gravitle, worlds[0], memory, this);
};

View2d.prototype.set_mouse = function (x, y) {
	this.mouse = {
		html: {
			x: x,
			y: y,
		},
		canvas: {
			x: x * this.dpr,
			y: y * this.dpr,
		},
		world: this.pixel_to_world({
			x: x * this.dpr,
			y: y * this.dpr,
		}),
	};
};

View2d.prototype.resize = function () {
	this.dpr = window.devicePixelRatio || 1;
	const size = Math.min(window.innerWidth, window.innerHeight) * this.dpr;
	this.canvas.width = size;
	this.canvas.height = size;
};

View2d.prototype.min_dim = function () {
	return Math.min(this.canvas.width, this.canvas.height);
};

View2d.prototype.world_to_pixel_2 = function (x, y) {
	return {
		x:
			x * this.min_dim() * this.zoom +
			this.canvas.width / 2 -
			this.center.x * this.zoom * this.canvas.width,
		y:
			-y * this.min_dim() * this.zoom +
			this.canvas.height / 2 +
			this.center.y * this.zoom * this.canvas.height,
	};
};

View2d.prototype.world_to_pixel = function (p) {
	return this.world_to_pixel_2(p.x, p.y);
};

View2d.prototype.pixel_to_world = function (p) {
	return {
		x: (p.x - this.canvas.width / 2) / (this.min_dim() * this.zoom),
		y: -(p.y - this.canvas.height / 2) / (this.min_dim() * this.zoom),
	};
};

View2d.prototype.draw_disk = function (x, y, diameter, color) {
	const radius = diameter * this.min_dim() * 0.5 * this.zoom;
	this.context.beginPath();
	const vp = this.world_to_pixel_2(x, y);
	this.context.arc(vp.x, vp.y, radius, 0, 2 * Math.PI, false);
	this.context.fillStyle = color;
	this.context.fill();
};

View2d.prototype.draw_disk_multi = function (x, y, diameter, color) {
	for (const x2 of [-1, 0, 1]) {
		for (const y2 of [-1, 0, 1]) {
			this.draw_disk(x + x2, y + y2, diameter, color);
		}
	}
};

View2d.prototype.draw_square = function (x, y, size, color) {
	const vp = this.world_to_pixel_2(x, y);
	const size_ = this.world_to_pixel_2(size, size);
	this.context.fillStyle = color;
	this.context.fillRect(
		vp.x,
		vp.y,
		size_.x - this.canvas.width / 2,
		size_.x - this.canvas.width / 2,
	);
};

View2d.prototype.draw_line = function (a, b, color, line_width) {
	const avp = this.world_to_pixel(a);
	const bvp = this.world_to_pixel(b);
	this.context.beginPath();
	this.context.moveTo(avp.x, avp.y);
	this.context.lineTo(bvp.x, bvp.y);
	this.context.strokeStyle = color;
	this.context.lineWidth = line_width ? line_width : 2;
	this.context.stroke();
};

export { View2d };
