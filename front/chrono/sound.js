const get_gain = (audio_context, value) => {
	const n = audio_context.createGain();
	n.gain.setValueAtTime(value, audio_context.currentTime);
	return n;
};

const get_osc = (audio_context, value) => {
	const n = audio_context.createOscillator();
	n.type = "sine";
	n.frequency.setValueAtTime(value, audio_context.currentTime);
	n.start();
	return n;
};

const get_filter = (audio_context, type, frequency) => {
	const n = audio_context.createBiquadFilter();
	n.type = type;
	n.frequency.setValueAtTime(frequency, audio_context.currentTime);
	return n;
};

const get_booster_v1 = (audio_context) => {
    const m2 = get_osc(audio_context, 6000)
    const o2 = get_osc(audio_context, 194)
    const g3 = get_gain(audio_context, 5000)
    const g2 = get_gain(audio_context, 5000)
    const g4 = get_gain(audio_context, 1)
    const g5 = get_gain(audio_context, 30)
    const l1 = get_filter(audio_context, 'lowpass', 100)
    const l2 = get_filter(audio_context, 'lowpass', 50)
    const h1 = get_filter(audio_context, 'highpass', 100)
    const g = get_gain(audio_context, 0)
    m2.connect(g2)
    g2.connect(o2.detune)
    o2.connect(g3)
    g3.connect(m2.detune)
    o2.connect(h1)
    l1.connect(l2)
    l1.connect(g4)
    l2.connect(g5)
    h1.connect(l1)
    g4.connect(g)
    g5.connect(g)
    return g
}

const get_booster_v2 = (audio_context) => {
	const noise = new AudioWorkletNode(audio_context, "noise-generator");
	const g = get_gain(audio_context, 0.0);
	const h1 = get_filter(audio_context, "highpass", 51.69);
	const l1 = get_filter(audio_context, "lowpass", 198.93);
	const l2 = get_filter(audio_context, "lowpass", 218.77);
	const l3 = get_filter(audio_context, "lowpass", 209.5);
	const g5 = get_gain(audio_context, 4.76);
	const g3 = get_gain(audio_context, 10.53);
	const h2 = get_filter(audio_context, "highpass", 130.72);
	noise.connect(h1);
	l1.connect(l2);
	l3.connect(g3);
	l2.connect(l3);
	l2.connect(g5);
	h1.connect(l1);
	g5.connect(h2);
	g3.connect(h2);
	h2.connect(g);
	return g;
};

const get_booster_v3 = (audio_context) => {
	const noise = new AudioWorkletNode(audio_context, "noise-generator");
	const g = get_gain(audio_context, 0.0);
	const gu = get_gain(audio_context, 0.5);
	const h1 = get_filter(audio_context, "highpass", 478);
	const l1 = get_filter(audio_context, "lowpass", 369);
	const l2 = get_filter(audio_context, "lowpass", 481);
	const l3 = get_filter(audio_context, "lowpass", 1736);
	const g5 = get_gain(audio_context, 0.76);
	const g3 = get_gain(audio_context, 11.35);
	const h2 = get_filter(audio_context, "highpass", 130.72);
	noise.connect(h1);
	h1.connect(l1);
	l1.connect(l2);
	l3.connect(g3);
	l2.connect(l3);
	l2.connect(g5);
	g5.connect(h2);
	g3.connect(h2);
	h2.connect(gu);
	gu.connect(g);
	return g;
};

function Audio() {}
Audio.prototype.setup = async function () {
	const audio_context = new AudioContext();
	await audio_context.audioWorklet.addModule("chrono/noise-processor.js");
	const compressor = audio_context.createDynamicsCompressor();
	compressor.threshold.setValueAtTime(-10, audio_context.currentTime); // dB value where compression starts
	compressor.knee.setValueAtTime(10, audio_context.currentTime); // How smoothly the curve transitions
	compressor.ratio.setValueAtTime(12, audio_context.currentTime); // Amount of compression
	compressor.attack.setValueAtTime(0.05, audio_context.currentTime); // Time (s) to start compressing
	compressor.release.setValueAtTime(0.05, audio_context.currentTime); // Time (s) to release compression
	const main = get_gain(audio_context, 0.1);
	const left = get_booster_v3(audio_context);
	const right = get_booster_v3(audio_context);
	left.connect(compressor);
	right.connect(compressor);
	compressor.connect(main);
	main.connect(audio_context.destination);
	this.audio_context = audio_context;
	this.main = main;
	this.left = left;
	this.right = right;
	this.keys = {};
	this.cc = 1000;
	this.transition = 0.5;
	this.release = 0.125;
	this.cells = {
		13: {
			activated: false,
			node: left,
		},
		14: {
			activated: false,
			node: right,
		},
	};
};
Audio.prototype.activate = function (idx) {
	try {
		const c = this.cells[idx];
		if (!c.activated) {
			c.activated = true;
			const now = this.audio_context.currentTime + 0.001;
			c.node.gain.cancelScheduledValues(now);
			for (let i = 1; i < this.cc; i++) {
				const value = Math.max(c.node.gain.value, i / this.cc);
				const time = (i / this.cc) * 1.0;
				c.node.gain.exponentialRampToValueAtTime(value, now + time);
			}
		}
	} catch (error) {
		console.error(error)
	}
};
Audio.prototype.deactivate = function (idx) {
	try {
		const c = this.cells[idx];
		c.node.gain.cancelScheduledValues(0);
		const now = this.audio_context.currentTime;
		for (let i = 0; i < this.cc; i++) {
			const value = Math.min(c.node.gain.value, 1 - i / this.cc);
			const time = (i / this.cc) * this.release;
			c.node.gain.exponentialRampToValueAtTime(value, now + time);
		}
		c.node.gain.setValueAtTime(0, now + this.release);
		c.activated = false;
	} catch (error) {
		console.error(error)
	}
};
export { Audio };
