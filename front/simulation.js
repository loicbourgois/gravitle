import {
    fill_circle_2,
    clear,
    clear_trans,
} from "../canvas.js"
import {
    particle,
} from "./particle.js"
import {
    link_js,
} from "../link.js"
import {
    Ship,
} from "./ship.js"


const Simulation = (gravithrust, wasm, context, context_2, context_trace) => {
    const self = {
        start: () => { start(self) },
        gravithrust: gravithrust,
        wasm: wasm,
        context: context,
        context_2: context_2,
        ppms: [],
        draw: () => { draw(self) },
        frame_starts: [],
        frame_durations: [],
        update: () => {update(self)},
        iter: () => {iter(self)},
        ppms: [],
        update_starts: [],
        update_durations: [],
        update_audio: () => {update_audio(self)},
        audio_starts: [],
        audio_durations: [],
        do_physics: true,
        stop_physics: () => {self.do_physics = false},
        context_trace: context_trace,
        iter_durations: [],
    }
    return self
}


const start = (self) => {
    self.start_time = performance.now()
    self.iter()
}


const update_audio = (self) => {
    const audio_start = performance.now()
    const ship_size = self.gravithrust.ship_size()
    const ships_view = new DataView(
        self.wasm.memory.buffer, 
        self.gravithrust.ships(), 
        self.gravithrust.ships_size(),
    );
    for (let sid = 0; sid < self.gravithrust.ships_count(); sid++) {
        const ss = self.ship_sounds[sid]
        const ship = Ship(ships_view, sid*ship_size);
        const speed = Math.sqrt(ship.v.x * ship.v.x + ship.v.y * ship.v.y)
        ss.osc_2.frequency.linearRampToValueAtTime(
            ss.osc_2_base_frequency * (0.9 + speed * 10),
            self.audioCtx.currentTime + 0.01
        )
        ss.gain_2.gain.linearRampToValueAtTime(
            ss.gain_2_base_gain * (0.9 + speed * 150000),
            self.audioCtx.currentTime + 0.01
        )
        ss.osc_3.frequency.linearRampToValueAtTime(
            ss.osc_3_base_frequency * (0.9 + speed * 10),
            self.audioCtx.currentTime + 0.01
        )
        ss.gain_3.gain.linearRampToValueAtTime(
            ss.gain_3_base_gain * (0.9 + speed * 150000),
            self.audioCtx.currentTime + 0.01
        )
        ss.gain_1.gain.linearRampToValueAtTime(
            ss.gain_1_base_gain * (0.0 + speed * 10000),
            self.audioCtx.currentTime + 0.01
        )
        ss.stereo_1.pan.linearRampToValueAtTime(
            (ship.p.x - 0.5) * 3,
            self.audioCtx.currentTime + 0.01
        )
        ss.osc_4.stop()
        ss.osc_3.stop()
    }
    self.audio_durations.push(performance.now() - audio_start)
    while (self.audio_durations.length > 20) {
        self.audio_durations.shift()
    }
}


const iter = (self) => {
    const iter_start = performance.now()
    if (self.do_physics) {
        self.update()
    }
    self.draw()
    if (self.audioCtx) {
        self.update_audio()
    }
    requestAnimationFrame(()=>{
        requestAnimationFrame(self.iter)
    })
    self.iter_durations.push(performance.now() - iter_start)
    while (self.iter_durations.length > 20) {
        self.iter_durations.shift()
    }
}


const ppms_count = 400
const update = (self) => {
    const update_start = performance.now()
    self.update_starts.push(update_start)
    self.gravithrust.ticks()
    self.ppms.push({
      high: self.gravithrust.points * 1000000 / self.gravithrust.step,
      low: self.gravithrust.points * 1000000 / self.gravithrust.step,
      step_high: self.gravithrust.step,
      step_low: self.gravithrust.step,
    })
    if (self.ppms.length >= ppms_count) {
      for (let i = 0; i < ppms_count/2; i++) {
        self.ppms[i] = {
          high: Math.max(self.ppms[i*2].high, self.ppms[i*2+1].high),
          low: Math.min(self.ppms[i*2].low,self.ppms[i*2+1].low),
          step_high: Math.max(self.ppms[i*2].step_high, self.ppms[i*2+1].step_high),
          step_low: Math.min(self.ppms[i*2].step_low, self.ppms[i*2+1].step_low),
        }
      }
      self.ppms.length = ppms_count / 2;
    }
    while (self.update_starts.length > 100) {
        self.update_starts.shift()
    }
    self.update_durations.push(performance.now() - update_start)
    while (self.update_durations.length > 20) {
        self.update_durations.shift()
    }
}


const average = array => array.reduce((a, b) => a + b) / array.length;


const draw = (self) => {
    self.context_trace.globalCompositeOperation = "hard-light"
    const frame_start = performance.now()
    self.frame_starts.push(frame_start)
    clear(self.context)
    clear_trans(self.context_trace)
    const particle_size = self.gravithrust.particle_size()
    const particles_view = new DataView(
        self.wasm.memory.buffer, 
        self.gravithrust.particles(), 
        self.gravithrust.particles_size(),
    );
    const link_js_size = self.gravithrust.link_js_size()
    const links_js_view = new DataView(
        self.wasm.memory.buffer, 
        self.gravithrust.links_js(), 
        self.gravithrust.links_js_size(),
    );
    for (let pid = 0; pid < self.gravithrust.particles_count(); pid++) {
        const p = particle(particles_view, pid*particle_size);
        if (p.k == 3 ) {
            if (p.a == 1) {
                fill_circle_2(self.context, p.pout2, self.gravithrust.diameter*0.7, "#c22")
                fill_circle_2(self.context, p.pout, self.gravithrust.diameter*0.9, "#c00")
                fill_circle_2(self.context, p.p, self.gravithrust.diameter*1, "#d20")
                fill_circle_2(self.context_trace, p.p, self.gravithrust.diameter*1, "#d20")
            } else {
                fill_circle_2(self.context, p.p, self.gravithrust.diameter*1, "#b40")
            }
        }
    }
    for (let lid = 0; lid < self.gravithrust.links_count(); lid++) {
        const l = link_js(links_js_view, lid*link_js_size);
        fill_circle_2(self.context, l, self.gravithrust.diameter , "#da4")
    }
    for (let pid = 0; pid < self.gravithrust.particles_count(); pid++) {
        const p = particle(particles_view, pid*particle_size);
        if (p.k != 3 && p.k != 2) {
            fill_circle_2(self.context, p.p, self.gravithrust.diameter*1, "#da4")
        }
    }
    for (let pid = 0; pid < self.gravithrust.particles_count(); pid++) {
        const p = particle(particles_view, pid*particle_size);
        if (p.k == 2 ) {
            fill_circle_2(self.context, p.p, self.gravithrust.diameter*1, "#c83")
        }
    }
    if (self.ppms.length) {
        self.context_2.canvas.width = 400
        clear(self.context_2)
        let ppms_max = self.ppms[0].high
        for (const x of self.ppms) {
            ppms_max = Math.max(x.high, ppms_max)
        }
        for (let i = 0; i < self.ppms.length; i++) { 
            const x = (i+1) / self.ppms.length * canvas_2.width
            const y_low = self.ppms[i].low   / ppms_max * self.context_2.canvas.height
            const y_high = self.ppms[i].high / ppms_max * self.context_2.canvas.height
            self.context_2.beginPath();
            self.context_2.fillStyle = "#cc8";
            self.context_2.strokeStyle = "#cc8";
            self.context_2.rect(x, canvas_2.height - y_high, 1, Math.max(y_high-y_low, 1));
            self.context_2.fill();
            self.context_2.stroke();
        }
    }
    document.querySelector("#points").innerHTML = self.gravithrust.points
    const duration = (( performance.now() - self.start_time) / 1000 )
    document.querySelector("#ppms").innerHTML = (self.gravithrust.points * 1000000 / self.gravithrust.step).toFixed(1)
    document.querySelector("#duration").innerHTML = parseInt(duration)
    document.querySelector("#step").innerHTML = self.gravithrust.step
    document.querySelector("#particles_count").innerHTML = self.gravithrust.particles_count()
    document.querySelector("#update_duration").innerHTML = average(self.update_durations).toFixed(1)
    self.frame_durations.push(performance.now() - frame_start)
    while (self.frame_durations.length > 20) {
        self.frame_durations.shift()
    }
    while (self.frame_starts.length > 50) {
        self.frame_starts.shift()
    }
    const ups = 1000 / (self.update_starts[self.update_starts.length-1] - self.update_starts[0]) * (self.update_starts.length-1)
    document.querySelector("#ups").innerHTML = ups.toFixed(0)
    const fps = 1000 / (self.frame_starts[self.frame_starts.length-1] - self.frame_starts[0]) * (self.frame_starts.length-1)
    document.querySelector("#fps").innerHTML = fps.toFixed(0)
    document.querySelector("#frame").innerHTML = average(self.frame_durations).toFixed(1)
    if (self.audio_durations.length) {
        document.querySelector("#audio_duration").innerHTML = average(self.audio_durations).toFixed(1)
    }
    if (self.iter_durations.length) {
        document.querySelector("#iter_duration").innerHTML = average(self.iter_durations).toFixed(1)
    }
}


export {
    Simulation,
}
