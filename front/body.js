const body = `
<div id="left">
  <canvas id="canvas"></canvas>
</div>
<div id="right">
  <button id="go_fullscreen" onclick="go_fullscreen()">Fullscreen</button>
  <button id="exit_fullscreen" onclick="exit_fullscreen()" style="display:none">Exit Fullscreen</button>
  <button id="zen_mode" onclick="zen_mode()">Zen</button>
  <div id="texts"></div>
  <div>
    <label>collide color:</label>
    <input id="color_0" value="#ff4" />
  </div>
  <div>
    <label>base color:   </label>
    <input id="color_1" value="#fc0" />
  </div>
  <div>
    <label>edge color:   </label>
    <input id="color_2" value="#e80" />
  </div>
  <div>
    <label>particles:    </label>
    <label id="particles_count">...</label>
  </div>
  <div>
    <label>ups:          </label>
    <label id="ups">...</label>
  </div>
  <div>
    <label>points:       </label>
    <label id="points">...</label>
  </div>
  <div>
    <label>ppMs:         </label>
    <label id="mpps">...</label>
  </div>
  <div>
    <label>duration:     </label>
    <label id="duration">...</label>
  </div>
  <div>
    <label>step:         </label>
    <label id="step">...</label>
  </div>
</div>
`
export {
    body
}