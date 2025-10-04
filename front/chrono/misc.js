const sep1 = "."
const sep2 = "l"

const json_to_b64 = (j) => {
	let b = "";
	for (const k in j) {
		const e = j[k];
		b += `${k}`;
		for (const e2 of e) {
			b += `:${e2.c}:${e2.a}`;
		}
		b += `|`;
	}
	return btoa(b);
};

const json_to_short = (j) => {
	let b = "";
	for (const k in j) {
		const e = j[k];
		b += `${k}`;
		for (const e2 of e) {
			b += `${sep2}${e2.c}`;
		}
		b += sep1;
	}
	return b;
};

const short_to_json = (s) => {
	const j = {};
	for (const e of s.split(sep1)) {
		let i = "step";
		let step = null;
		for (const e2 of e.split(sep2)) {
			if (i == "step") {
				step = e2;
				i = "c";
				if (step.length) {
					j[step] = [];
				}
			} else if (i == "c") {
				j[step].push({
					c: parseInt(e2),
				});
			}
		}
	}
	return j;
};

const b64_to_json = (bt) => {
	console.log(bt);
	const b = atob(bt);
	console.log(b);
	const j = {};
	for (const e of b.split("|")) {
		let i = "step";
		let step = null;
		let c = null;
		let a = null;
		for (const e2 of e.split(":")) {
			if (i == "step") {
				step = e2;
				i = "c";
				if (step.length) {
					j[step] = [];
				}
			} else if (i == "c") {
				c = e2;
				i = "a";
			} else if (i == "a") {
				a = e2;
				i = "c";
				j[step].push({
					c: parseInt(c),
					a: parseInt(a),
				});
				c = null;
				a = null;
			}
		}
	}
	return j;
};

export {
	//json_to_b64, b64_to_json,
	short_to_json,
	json_to_short,
};
