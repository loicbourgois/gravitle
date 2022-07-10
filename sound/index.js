import {
  cos_,
  fract,
  sin_,
} from './math.js'
import {
  clear,
  fill_circle,
  resize_canvas,
} from './canvas.js'


const MAX = 10000
const sliders = {}
const funcs = {}


const elements = () => {
  console.log(document.body)
  let ih = "<canvas id='canvas_0'></canvas>"
  for (var i = 0; i < 10; i++) {
    const value = localStorage.getItem(`slider_${i}.value`) ? localStorage.getItem(`slider_${i}.value`) : 100;
    ih += `
      <input type="range" min="0" max="${MAX}" value="${value}" id="slider_${i}" oninput="sliders['slider_${i}'].f()"></input>
    `
  }
  document.body.innerHTML = ih
  for (var i = 0; i < 10; i++) {
    const slider_id = `slider_${i}`
    sliders[slider_id] = {}
    sliders[slider_id].slider = document.getElementById(slider_id)
    sliders[slider_id].ff = (sv) => {
      // console.log(`todo: ${slider_id}, ${sv}`)
    }
    sliders[slider_id].f = () => {
      const s = sliders[slider_id].slider
      localStorage.setItem(`${slider_id}.value`, s.value)
      const sv = s.value / (s.max - s.min)
      sliders[slider_id].ff(sv)
      return sv
    }
  }
}


const audio_x = (time) => {
  return fract(time * sliders['slider_0'].f()*10)
}


const render = (context, audio_context, _) => {
  resize_canvas(context, window.innerWidth, 300)
  clear(context)
  const resolution = 500
  for (var i = 0; i < resolution; i++) {
    const x = i / resolution
    fill_circle(context, {x:x, y:funcs[0](x)}, 0.02, "#f0f")
    fill_circle(context, {x:x, y:funcs[1](x)}, 0.02, "#ff0")
    fill_circle(context, {x:x, y:funcs[3](x)}, 0.02, "#0f0")
  }
  const x = audio_x(audio_context.currentTime)
  fill_circle(context, {x:x , y: funcs[0](x)}, 0.1, "#f0f")
  fill_circle(context, {x:x , y: funcs[1](x)}, 0.1, "#ff0")
  fill_circle(context, {x:x , y: funcs[3](x)}, 0.1, "#0f0")
  window.requestAnimationFrame(()=>{
    render(context, audio_context, _)
  })
}


const sound = (_) => {
  for (var i = 0; i < 200; i++) {
    const time = _.audio_context.currentTime + i * 0.001
    const x = audio_x(time)
    _.gain.gain.linearRampToValueAtTime(funcs[0](x), time)
    _.osc.frequency.linearRampToValueAtTime(funcs[2](x), time)

    _.gain_2.gain.linearRampToValueAtTime(funcs[1](x), time)
  }
  setTimeout(()=>{
    sound(_)
  }, 1)
}


window.onload = async () => {
  window.sliders = sliders
  console.log("yo")
  elements()
  const context = new AudioContext()

  let osc = context.createOscillator()
  let gain = context.createGain();
  osc.frequency.value = 10
  osc.connect(gain);
  osc.start()
  gain.connect(context.destination)

  let osc_2 = context.createOscillator()
  let gain_2 = context.createGain();
  osc_2.connect(gain_2);
  osc_2.start()
  osc_2.frequency.value = 5
  gain_2.connect(context.destination)

  sliders['slider_0'].ff = (sv) => {
    gain.gain.value = sv;
  }
  for (var i = 0; i < 10; i++) {
    sliders[`slider_${i}`].f()
  }


  funcs[0] = (x) => {
    return cos_(x*200)  * (1.0- fract(x*10     )) * sliders['slider_1'].f()
  }
  funcs[1] = (x) => {
    return cos_(x*6)  * (1.0- fract(x*10)) * sliders['slider_2'].f()
  }
  funcs[2] = (x) => {
    return funcs[0](x)*1000*sliders['slider_2'].f()+funcs[3](x)*10000
  }
  funcs[3] = (x) => {
    return sin_(x*9)*sin_(x*1000)*1 * sin_(x*0.1)*sin_(x*10)*sliders['slider_3'].f()*2.0
  }


  const _ = {
    audio_context: context,
    osc:osc,
    gain: gain,
    gain_2: gain_2,
  }
  render(
    document.getElementById('canvas_0').getContext('2d'),
    context,
    _
  )
  sound(_)
}
