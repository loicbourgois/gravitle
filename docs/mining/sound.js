const setup_audio = (ship_count) => {
    let ship_sounds = []
    let audioCtx = new (window.AudioContext || window.webkitAudioContext)()
    let master = audioCtx.createGain()
    master.connect(audioCtx.destination)
    master.gain.setValueAtTime(0, audioCtx.currentTime)
    for (let sid = 0; sid < ship_count; sid++) {
      ship_sounds.push({})
      const ss = ship_sounds[sid]
      ss.stereo_1 = audioCtx.createStereoPanner()
      ss.stereo_1.pan.setValueAtTime(0, audioCtx.currentTime)
      ss.stereo_1.pan.linearRampToValueAtTime(0, audioCtx.currentTime + 0.5)
      ss.stereo_1_base_pan = 0
      ss.stereo_1.connect(master)
      ss.gain_1 = audioCtx.createGain()
      ss.gain_1.gain.setValueAtTime(0, audioCtx.currentTime)
      ss.gain_1.gain.linearRampToValueAtTime(0.3, audioCtx.currentTime + 0.5)
      ss.gain_1_base_gain = 0.3
      ss.gain_1.connect(ss.stereo_1)
      ss.osc_1 = audioCtx.createOscillator()
      ss.osc_1.frequency.setValueAtTime(90, audioCtx.currentTime)
      ss.osc_1_base_frequency = 90
      ss.osc_1.detune.setValueAtTime(0, audioCtx.currentTime)
      ss.osc_1_base_detune = 0
      ss.osc_1.start()
      ss.osc_1.connect(ss.gain_1)
      ss.gain_2 = audioCtx.createGain()
      ss.gain_2.gain.setValueAtTime(0, audioCtx.currentTime)
      ss.gain_2.gain.linearRampToValueAtTime(100, audioCtx.currentTime + 0.5)
      ss.gain_2_base_gain = 100
      ss.gain_2.connect(ss.osc_1.frequency)
      ss.osc_2 = audioCtx.createOscillator()
      ss.osc_2.frequency.setValueAtTime(30, audioCtx.currentTime)
      ss.osc_2_base_frequency = 30
      ss.osc_2.detune.setValueAtTime(0, audioCtx.currentTime)
      ss.osc_2_base_detune = 0
      ss.osc_2.start()
      ss.osc_2.connect(ss.gain_2)
      ss.gain_3 = audioCtx.createGain()
      ss.gain_3.gain.setValueAtTime(0, audioCtx.currentTime)
      ss.gain_3.gain.linearRampToValueAtTime(100, audioCtx.currentTime + 0.5)
      ss.gain_3_base_gain = 100
      ss.gain_3.connect(ss.osc_1.frequency)
      ss.osc_3 = audioCtx.createOscillator()
      ss.osc_3.frequency.setValueAtTime(30, audioCtx.currentTime)
      ss.osc_3_base_frequency = 30
      ss.osc_3.detune.setValueAtTime(0, audioCtx.currentTime)
      ss.osc_3_base_detune = 0
      ss.osc_3.start()
      ss.osc_3.connect(ss.gain_3)
      ss.gain_4 = audioCtx.createGain()
      ss.gain_4.gain.setValueAtTime(0, audioCtx.currentTime)
      ss.gain_4.gain.linearRampToValueAtTime(1000, audioCtx.currentTime + 0.5)
      ss.gain_4_base_gain = 1000
      ss.gain_4.connect(ss.osc_2.frequency)
      ss.osc_4 = audioCtx.createOscillator()
      ss.osc_4.frequency.setValueAtTime(300, audioCtx.currentTime)
      ss.osc_4_base_frequency = 300
      ss.osc_4.detune.setValueAtTime(0, audioCtx.currentTime)
      ss.osc_4_base_detune = 0
      ss.osc_4.start()
      ss.osc_4.connect(ss.gain_4)
    }
    return {master:master, audioCtx:audioCtx, ship_sounds:ship_sounds}
  }
  export { setup_audio }