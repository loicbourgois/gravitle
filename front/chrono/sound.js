const get_gain = (audio_context, value) => {
    const n = audio_context.createGain();
    n.gain.setValueAtTime(value, audio_context.currentTime);
    return n
}

const get_osc = (audio_context, value) => {
    const n = audio_context.createOscillator();
    n.type = "sine";
    n.frequency.setValueAtTime(value, audio_context.currentTime);
    n.start()
    return n
}

const get_filter = (audio_context, type, frequency) => {
    const n = audio_context.createBiquadFilter();
    n.type = type
    n.frequency.setValueAtTime(frequency, audio_context.currentTime)
    return n
}

const get_booster = (audio_context) => {
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

function Audio() {
    const audio_context = new AudioContext();
    const compressor = audio_context.createDynamicsCompressor();
    compressor.threshold.setValueAtTime(-54, audio_context.currentTime);    // dB value where compression starts
    compressor.knee.setValueAtTime(30, audio_context.currentTime);          // How smoothly the curve transitions
    compressor.ratio.setValueAtTime(12, audio_context.currentTime);         // Amount of compression
    compressor.attack.setValueAtTime(0.05, audio_context.currentTime);      // Time (s) to start compressing
    compressor.release.setValueAtTime(0.05, audio_context.currentTime);     // Time (s) to release compression
    const main = get_gain(audio_context, 0.2)
    const left = get_booster(audio_context)
    const right = get_booster(audio_context)
    left.connect(compressor);
    right.connect(compressor);
    compressor.connect(main);
    main.connect(audio_context.destination);
    this.audio_context = audio_context
    this.main = main
    this.left = left
    this.right = right
    this.keys = {}
    this.cc = 100
    this.transition = 0.2
    this.cells = {
        13: {
            activated: false,
            node: left,
        },
        14: {
            activated: false,
            node: right,
        }
    }
}
Audio.prototype.activate = function (idx) {
    const c = this.cells[idx]
    if (!c.activated) {
    	c.activated = true
    	const now = this.audio_context.currentTime + 0.001
    	c.node.gain.cancelScheduledValues(now)
    	for (let i = 1; i < this.cc; i++) {
    		const value = Math.max(c.node.gain.value, i/this.cc)
    		const time = i/this.cc * 1.0
    		c.node.gain.exponentialRampToValueAtTime(value, now+time)
    	}
    }
}
Audio.prototype.deactivate = function (idx) {
    const c = this.cells[idx]
    c.node.gain.cancelScheduledValues(0)
    const now = this.audio_context.currentTime
    for (let i = 0; i < this.cc; i++) {
        const value = Math.min(c.node.gain.value, 1-i/this.cc)
        const time = i/this.cc * this.transition
        c.node.gain.exponentialRampToValueAtTime(value, now+time)
    }
    c.node.gain.setValueAtTime(0, now+this.transition)
    c.activated = false
}
export {
    Audio,
}
