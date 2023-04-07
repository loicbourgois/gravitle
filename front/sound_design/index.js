let started = false
// const stereoNode = new StereoPannerNode(audioContext, { pan: 0 });

const config = {
  stereo_1: {
    kind: 'stereo',
    pan: 0,
    destinations: ['audioCtx.destination'],
    top: 0,
    right: 0,
  },
  gain_1: {
    kind: 'gain',
    gain: 0.3,
    destinations: ['stereo_1'],
    top: 0,
    right: 1,
  },
  osc_1: {
    kind: 'osc',
    destinations: ['gain_1'],
    frequency: 90,
    detune: 0,
    top: 0,
    right: 2,
  },
  gain_2: {
    kind: 'gain',
    gain: 100,
    destinations: [['osc_1','frequency']],
    top: 0,
    right: 3,
  },
  osc_2: {
    kind: 'osc',
    destinations: ['gain_2'],
    frequency: 30.02,
    detune: 0,
    top: 0,
    right: 4,
  },
}


const show_config = () => {
  document.body.innerHTML = ""
  for (const k in config) {
    if (Object.hasOwnProperty.call(config, k)) {
      const v = config[k];
      let fields = ""
      switch (v.kind) {
        case 'stereo':
          fields += `<span>
            <label id="${k}_pan">pan:</label>
            <input id="${k}_pan_input" value="${v.pan}"></input>
          </span>`
          break;
        case 'gain':
          fields += `<span>
            <label id="${k}_gain">gain:</label>
            <input id="${k}_gain_input" value="${v.gain}"></input>
          </span>`
          break;
        case 'osc':
          fields += `<span>
            <label id="${k}_frequency">frequency:</label>
            <input id="${k}_frequency_input" value="${v.frequency}"></input>
          </span>`
          fields += `<span>
            <label id="${k}_detune">detune:</label>
            <input id="${k}_detune_input" value="${v.detune}"></input>
          </span>`
          break;
        default:
          throw "Missing case in show_config"
      }
      document.body.innerHTML += `
        <div style="right:${v.right*14}rem;" >
          <label id="${k}" >${k}</label></br>
          ${fields}
        </div>
      `
    }
  }
  for (const k in config) {
    if (Object.hasOwnProperty.call(config, k)) {
      const v = config[k];
      const a = get_elem_center(`${k}`)
      for (const k2 of v.destinations) {
        if (k2 != 'audioCtx.destination') {
          let b = null
          if (Array.isArray(k2)) {
            b = get_elem_center(`${k2[0]}_${k2[1]}`)
          } else {
            b = get_elem_center(`${k2}`)
          }
          document.body.innerHTML += `
            <svg>
              <line x1=${a.x} y1=${a.y} x2=${b.x} y2=${b.y} stroke="#ff08" stroke-width="2"/>
            </svg>
          `
        }
      }
    }
  }


  const lines = [
    "audioCtx = new (window.AudioContext || window.webkitAudioContext)()",
    "for (let sid = 0; sid < ship_count; sid++) {",
    "  ship_sounds.push({})",
    "  const ss = ship_sounds[sid]",
  ]

  for (const k in config) {
    if (Object.hasOwnProperty.call(config, k)) {
      const v = config[k];
      switch (v.kind) {
        case 'stereo':
          lines.push(`  ss.${k} = audioCtx.createStereoPanner()`)
          lines.push(`  ss.${k}.pan.setValueAtTime(0, audioCtx.currentTime)`)
          lines.push(`  ss.${k}.pan.linearRampToValueAtTime(${v.pan}, audioCtx.currentTime + 0.5)`)
          lines.push(`  ss.${k}_base_pan = ${v.pan}`)
          break;
        case 'gain':
          lines.push(`  ss.${k} = audioCtx.createGain()`)
          lines.push(`  ss.${k}.gain.setValueAtTime(0, audioCtx.currentTime)`)
          lines.push(`  ss.${k}.gain.linearRampToValueAtTime(${v.gain}, audioCtx.currentTime + 0.5)`)
          lines.push(`  ss.${k}_base_gain = ${v.gain}`)
          break;
        case 'osc':
          lines.push(`  ss.${k} = audioCtx.createOscillator()`)
          lines.push(`  ss.${k}.frequency.setValueAtTime(${v.frequency}, audioCtx.currentTime)`)
          lines.push(`  ss.${k}_base_frequency = ${v.frequency}`)
          lines.push(`  ss.${k}.detune.setValueAtTime(${v.detune}, audioCtx.currentTime)`)
          lines.push(`  ss.${k}_base_detune = ${v.detune}`)
          lines.push(`  ss.${k}.start()`)
          break;
        default:
          throw "Missing case in show_config_js"
      }
      for (const dest of v.destinations) {
        if (dest == 'audioCtx.destination') {
          lines.push(`  ss.${k}.connect(audioCtx.destination)`)
        } else if (Array.isArray(dest)) {
          lines.push(`  ss.${k}.connect(ss.${dest[0]}.${dest[1]})`)
        } else {
          lines.push(`  ss.${k}.connect(ss.${dest})`)
        }
      }
    }
  }

  lines.push('}')
  console.log(lines.join("\n"))

}


const add_events = () => {
  for (const k in config) {
    console.log(k)
    if (Object.hasOwnProperty.call(config, k)) {
      const v = config[k];
      switch (v.kind) {
        case 'stereo':
          document.getElementById(`${k}_pan_input`).addEventListener("change", (v) => {
            config[k].pan = parseFloat(v.target.value)
            restart()
          })
          break;
        case 'gain':
          document.getElementById(`${k}_gain_input`).addEventListener("change", (v) => {
            config[k].gain = parseFloat(v.target.value)
            restart()
          })
          break;
        case 'osc':
          document.getElementById(`${k}_frequency_input`).addEventListener("change", (v) => {
            config[k].frequency = parseFloat(v.target.value)
            restart()
          })
          document.getElementById(`${k}_detune_input`).addEventListener("change", (v) => {
            config[k].detune = parseFloat(v.target.value)
            restart()
          })
          break;
        default:
          throw "Missing case in add_events"
      }
    }
  }
}


const get_elem_center = (id) => {
  var element = document.getElementById(id);
  var elementRect = element.getBoundingClientRect();
  var centerX = elementRect.left + elementRect.width / 2;
  var centerY = elementRect.top + elementRect.height / 2;
  return {x:centerX,y:centerY}
} 

let audioCtx

const start = () => {
  try {
    if (audioCtx) {
      config.gain_1.node.gain.exponentialRampToValueAtTime(0.3, audioCtx.currentTime + 0.1);
      config.gain_1.node.gain.exponentialRampToValueAtTime(0.2, audioCtx.currentTime + 0.2);
      config.gain_1.node.gain.exponentialRampToValueAtTime(0.00001, audioCtx.currentTime + 0.45);
      const old_audio_context = audioCtx
      setTimeout(() => {
        old_audio_context.close()
      }, 500);
    }
  } catch (error) {
    
  }
  audioCtx = new (window.AudioContext || window.webkitAudioContext)();
  for (const k in config) {
    if (Object.hasOwnProperty.call(config, k)) {
      const v = config[k];
      switch (v.kind) {
        case 'stereo':
          v.node = audioCtx.createStereoPanner()
          v.node.pan.setValueAtTime(0, audioCtx.currentTime);
          v.node.pan.linearRampToValueAtTime(v.pan, audioCtx.currentTime + 0.5);
          break;
        case 'gain':
          v.node = audioCtx.createGain()
          v.node.gain.setValueAtTime(0, audioCtx.currentTime);
          v.node.gain.linearRampToValueAtTime(v.gain, audioCtx.currentTime + 0.5);
          break;
        case 'osc':
          v.node = audioCtx.createOscillator()
          v.node.frequency.setValueAtTime(v.frequency, audioCtx.currentTime);
          v.node.detune.setValueAtTime(v.detune, audioCtx.currentTime);
          v.node.start()
          break;
        default:
          throw "Missing case in build audio network"
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
}


document.addEventListener("click", (v) => {
  if (!started){
    started = true
    console.log("start")
    start()
  }
});


const restart = () => {
  show_config()
  add_events()
  if (started){
    start()
  }
}


restart()
