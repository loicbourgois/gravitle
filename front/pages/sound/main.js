const slider = (idx) => {
  if (!localStorage.getItem(`slider_${idx}`)) {
    localStorage.setItem(`slider_${idx}`, 100)
  }
  const value = localStorage.getItem(`slider_${idx}`)
  return `
    <input oninput="change_slider('${idx}')" type="range" min="0" max="1000" id="slider_${idx}" value="${value}"></input>
  `
}


const html = () => {
  return `
    <div id="winner" class="hide">
      ${slider(0)}
    </div>
  `
}


const style = () => {
    return `
      input {
        width: 30rem;
      }
    `
}


const change_slider = (idx) => {
  localStorage.setItem(`slider_${idx}`, document.querySelector(`#slider_${idx}`).value)
}


const slider_value = (idx) => document.querySelector(`#slider_${idx}`).value / 1000


const sound_main = () => {
  window.change_slider = change_slider
  document.querySelector('#content').innerHTML = html()
  const style_element = document.createElement('style')
  document.head.appendChild(style_element)
  for (let x of style().split('}')) {
      try {
        style_element.sheet.insertRule(x+'}');
      } catch(e) {}
  }
  const AudioContext = window.AudioContext || window.webkitAudioContext;
  const audio_context = new AudioContext();
  const oscillator = audio_context.createOscillator();
  const gain = audio_context.createGain();
  gain.gain.value = 0.0;
  oscillator.type = 'square';
  oscillator.frequency.linearRampToValueAtTime(30, audio_context.currentTime);
  oscillator.connect(gain);
  gain.connect(audio_context.destination);
  oscillator.start();
  let released = true
  document.addEventListener("keydown", (e) => {
    if (e.key == "s" && released ) {
      const t = audio_context.currentTime
      gain.gain.linearRampToValueAtTime(0.25, t + 0.01);
      oscillator.frequency.linearRampToValueAtTime(100 * slider_value(0), t + 0.01);
    }
  });
  document.addEventListener("keyup", (e) => {
    if (e.key == "s") {
      gain.gain.linearRampToValueAtTime(0.0, audio_context.currentTime + 0.5);
    }
  });
}


export {
  sound_main,
}
