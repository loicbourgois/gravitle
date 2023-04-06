let started = false
console.log("ready")
const start = () => {
  const audioCtx = new (window.AudioContext || window.webkitAudioContext)();
  const config = {
    gain_1: {
      kind: 'gain',
      gain: 0.3,
      destinations: ['audioCtx.destination'],
    },
    osc_1: {
      kind: 'osc',
      destinations: ['gain_1'],
      frequency: 50,
      detune: 0,
    },
    gain_2: {
      kind: 'gain',
      gain: 300,
      destinations: [['osc_1','frequency']],
    },
    osc_2: {
      kind: 'osc',
      destinations: ['gain_2'],
      frequency: 22,
      detune: 0,
    },
  }
  for (const k in config) {
    if (Object.hasOwnProperty.call(config, k)) {
      const v = config[k];
      switch (v.kind) {
        case 'gain':
          v.node = audioCtx.createGain()
          v.node.gain.setValueAtTime(v.gain, audioCtx.currentTime);
          break;
        case 'osc':
          v.node = audioCtx.createOscillator()
          v.node.frequency.setValueAtTime(v.frequency, audioCtx.currentTime);
          v.node.detune.setValueAtTime(v.detune, audioCtx.currentTime);
          v.node.start()
          break;
      }
      for (const dest of v.destinations) {
        if (dest == 'audioCtx.destination') {
          v.node.connect(audioCtx.destination)
        } else {
          try {
            v.node.connect(config[dest].node)
          } catch (error) {
            v.node.connect(config[dest[0]].node[dest[1]])
          }
        }
      }
    }
  }
  // const oscillator = audioCtx.createOscillator();
  // const gain = audioCtx.createGain();
  // gain.gain.setValueAtTime(0.1, audioCtx.currentTime); // value in hertz
  // const osc2 = audioCtx.createOscillator();
  // const gain2 = audioCtx.createGain();
  // gain2.gain.setValueAtTime(200, audioCtx.currentTime); // value in hertz
  // osc2.frequency.setValueAtTime(4, audioCtx.currentTime); // value in hertz
  // osc2.connect(gain2)
  // gain2.connect(oscillator.detune)
  // osc2.start();
  // oscillator.frequency.setValueAtTime(44*2, audioCtx.currentTime); // value in hertz
  // oscillator.connect(gain);
  // gain.connect(audioCtx.destination);
  // oscillator.start();
  // console.log(oscillator)
}

document.addEventListener("click", (v) => {
  if (!started){
    started = true
    console.log("start")
    start()
  }
});
