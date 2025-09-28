const delta = (a, b) => {
	return {
		x: b.x - a.x,
		y: b.y - a.y,
	};
};

const distance_sqrd = (a, b) => {
	if (b === undefined) {
		// b = { x: 0, y: 0 };
		throw "invalid";
	}
	const dp = delta(a, b);
	return dp.x * dp.x + dp.y * dp.y;
};

const test_wrap_around = () => {
	for (let index = 0; index < 2000; index++) {
		const p1 = {
			x: Math.random(),
			y: Math.random(),
		};
		const p2 = {
			x: Math.random(),
			y: Math.random(),
		};
		const r1 = Math.random();
		const r2 = Math.random();
		const p3 = {
			x: p1.x + r1,
			y: p1.y + r2,
		};
		const p4 = {
			x: p2.x + r1,
			y: p2.y + r2,
		};
		const wa1 = wrap_around(p1, p2);
		const wa2 = wrap_around(p3, p4);
		if (Math.abs(wa1.d_sqrd - wa2.d_sqrd) > 0.00000001) {
			console.log("uoih", index);
			console.log(p1);
			console.log(p2);
			console.log(r1);
			console.log(r2);
			console.log(wa1);
			console.log(wa2);
			throw "error";
		}
	}
};

const wrap_around = (a, b) => {
	let ar = a;
	let br = b;
	let m = 1.0;
	let dr = distance_sqrd(a, b);
	for (const ox of [1 - a.x, 1 - a.y, 1 - b.x, 1 - b.y]) {
		const mx = ox + m;
		let ax = {
			x: (a.x + mx) % m,
			y: (a.y + mx) % m,
		};
		let bx = {
			x: (b.x + mx) % m,
			y: (b.y + mx) % m,
		};
		let dx = distance_sqrd(ax, bx);
		if (dx < dr) {
			dr = dx;
			ar = ax;
			br = bx;
		}
	}
	return {
		a: ar,
		b: br,
		d_sqrd: dr,
	};
};

export { wrap_around, delta, test_wrap_around };
