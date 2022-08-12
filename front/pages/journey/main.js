import {
  resize_square,
  stroke_circle,
  stroke_circle_2,
  fill_circle_2,
  fill_circle,
  clear,
  line,
  fill_text,
} from "../canvas"


const level_count = 2
if (!localStorage.getItem("progress")) {
  localStorage.setItem("progress", 0)
}
const progress = localStorage.getItem("progress")


const levels = () => {
  const level_idxs = []
  const u = []
  for (var i = 0; i < level_count; i++) {
    const uu = []
    for (var j = 0; j < level_count; j++) {
      const idx = i * level_count + j
      uu.push({
        locked: idx > progress ? 'locked' : '',
        idx: idx,
        label: (i * level_count + j + 1).toLocaleString('en-US', {
          minimumIntegerDigits: 2,
          useGrouping: false
        })
      })
    }
    u.push(
      uu.map(x => {
        if (x.locked) {
          return `<p class="level_link locked">${x.label}</p>`
        } else {
          return `<a class="level_link" href="journey-${x.idx+1}">${x.label}</a>`
        }
      }).join("")
    )
  }
  return u.map(x => `<div class="line">${x}</div>`).join("")
}


const html = () => {
  return `
    <div class="bob">
      <p> <a href="/">Home</a> </p>

    </div>

    <div id="levels">
      ${levels()}
    </div>

    <div class="bob">
    <!--  <p> <a href="/leaderboard">Leaderboard</a> </p>-->
      <p> <a href="/journey-garage">Garage</a> </p>
    </div>
  `
}


const style = () => {
  return `
    #levels > * {
      flex-grow: 1;
    }
    * {
      color: #ffa;
      background: #113;
      font-size: 1.05rem;
    }
    select {
      border: none;
    }
    option {
    }
    #content {
      display: flex;
      width: 100%;
      height: 100%;
      align-content: center;
      align-items: center;
      flex-direction: row;
    }
    a, .level_link {
      color: #ffa;
      text-decoration: none;
      background-color: #fff0;
      padding: 0.8rem;
    }
    .level_link.locked {
      color: #aaa;
    }
    a:hover {
      background-color: #fff2;
    }
    #score_player_1, #score_player_2 {
      font-size: 2rem;
    }
    #content > div.bob {
      width: 0;
      flex-grow: 1;
      display: flex;
      justify-content: space-around;
      flex-direction: column;
      height: 100%;
    }
    p {
      text-align: center;
      color: #ffa;
      font-family: monospace;
    }
    p span {
      color: #ffa;
    }
    #canvas {
        background: #113;
        display:flex;
        position: unset;
    }
    body {
      background: #113;
    }
    a {
      padding: 2rem;
      border-radius: 10rem;
    }
    #levels {
      display: flex;
      flex-wrap: wrap;
      align-content: center;
      align-items: center;
      flex-direction: column;
      justify-content: space-around;
      justify-items: stretch;
      flex-flow: column wrap;
      place-content: center space-around;
      place-items: center;
      padding: 1rem;
      flex-direction: column;
      justify-content: space-around;
      padding: 1rem;
    }
    #levels > * {
      flex-grow: 1;
      display: flex;
      flex-direction: row;
      align-content: center;
      align-items: center;
      display: flex;
      flex-direction: row;
      align-content: center;
      align-items: center;
    }
  `
}


const journey_main = () => {
  document.querySelector('#content').innerHTML = html()
  const style_element = document.createElement('style')
  document.head.appendChild(style_element)
  for (let x of style().split('}')) {
      try {
        style_element.sheet.insertRule(x+'}');
      } catch(e) {}
  }
  // const canvas = document.querySelector('#canvas')
  // resize_square(canvas)
  // const context = canvas.getContext('2d')
}


export {
  journey_main,
}
