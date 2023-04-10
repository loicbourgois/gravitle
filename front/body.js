const body = `
<div id="left">
<canvas id="canvas_trace"></canvas>
  <canvas id="canvas"></canvas>
</div>
<div id="right">
  <div>
    <button id="go_fullscreen" onclick="go_fullscreen()">Fullscreen</button>
    <button id="exit_fullscreen" onclick="exit_fullscreen()" style="display:none">Exit Fullscreen</button>
    <button id="zen_mode" onclick="zen_mode()">Zen</button>
  </div>
  <canvas id="canvas_2"></canvas>
  <div>
    <label>particles:           </label>
    <label id="particles_count">...</label>
  </div>
  <div>
    <label>update duration:     </label>
    <label id="update_duration">...</label>
    <label >ms</label>
  </div>
  <div>
    <label>draw duration:       </label>
    <label id="frame">...</label>
    <label >ms</label>
  </div>
  <div>
    <label>audio duration:      </label>
    <label id="audio_duration">...</label>
    <label >ms</label>
  </div>
  <div>
    <label>full duration:       </label>
    <label id="iter_duration">...</label>
    <label >ms</label>
  </div>
  <div>
    <label>ups:                 </label>
    <label id="ups">...</label>
  </div>
  <div>
    <label>fps:                 </label>
    <label id="fps">...</label>
  </div>
  <div>
    <label>points:              </label>
    <label id="points">...</label>
  </div>
  <div>
    <label>points per megastep: </label>
    <label id="ppms">...</label>
  </div>
  <div>
    <label>duration:            </label>
    <label id="duration">...</label>
  </div>
  <div>
    <label>steps:               </label>
    <label id="step">...</label>
  </div>
</div>
`
export {
    body
}