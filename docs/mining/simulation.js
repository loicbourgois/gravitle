import {
    fill_circle_2,
    clear,
    clear_trans,
    resize_square,
    // resize,
} from "./canvas.js"
import {
    particle,
} from "./particle.js"
import {
    link_js,
} from "./link.js"
import {
    Ship,
} from "./ship.js"
import {
    Kind,
} from "./kind_generated.js"
import {
    draw_particle,
} from "./particle_draw.js"
import {
    setup_webgpu,
    draw_gpu,
} from "./simulation_gpu.js"


const RESOLUTION = 2

const Simulation = async (
    gravithrust,
    wasm,
    canvas,
    canvas_2,
    canvas_trace,
) => {
    let context = undefined
    let context_2 = undefined
    let webgpu = undefined
    const adapter = await navigator.gpu?.requestAdapter();
    const device = await adapter?.requestDevice();
    if (device) {
        webgpu = await setup_webgpu(
            device,
            gravithrust,
            canvas,
            canvas_2,
        );
        canvas_2.width = 1000;
        canvas_2.height = 1000;
    } else {
        console.error('need a browser that supports WebGPU');
        context = canvas.getContext('2d')
        context_2 = canvas_2.getContext('2d')
    }
    const context_trace = canvas_trace.getContext('2d')
    resize_square(canvas, RESOLUTION * 0.9)
    resize_square(canvas_trace, RESOLUTION * 0.9 )
    const dimension = Math.min(window.innerWidth, window.innerHeight)
    canvas.style.width = `${dimension*0.9}px`
    canvas.style.height = `${dimension*0.9}px`
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
        draw_gpu: () => {draw_gpu(self)},
        draw_not_gpu: () => {draw_not_gpu(self)},
        update_audio: () => {update_audio(self)},
        audio_starts: [],
        audio_durations: [],
        do_physics: true,
        stop_physics: () => {self.do_physics = false},
        context_trace: context_trace,
        iter_durations: [],
        webgpu: webgpu,
    }
    return self
}


const start = async (self) => {
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
    // if (self.audioCtx) {
    //     self.update_audio()
    // }
    // requestAnimationFrame(()=>{
        requestAnimationFrame(self.iter)
    // })
    self.iter_durations.push(performance.now() - iter_start)
    while (self.iter_durations.length > 20) {
        self.iter_durations.shift()
    }
}


const ppms_count = 400
const update = (self) => {
    const update_start = performance.now()
    self.update_starts.push(update_start)
    const state = JSON.parse(self.gravithrust.ticks())
    // console.log(state)
    const lines = []
    for (const k of Object.keys(state.count).sort()) {
        lines.push(`${k}: ${state.count[k]}`)
        for ( const k2 of Object.keys(state.capacity[k]).sort() ) {
            const capacity = state.capacity[k][k2]
            const quantity = state.quantity[k][k2]
            lines.push(`    ${k2}: ${quantity} / ${capacity}`)
        }
    }

    // const state = JSON.stringify(JSON.parse(self.gravithrust.ticks()), null, 4)
    document.getElementById("state").innerHTML = lines.join("\n")
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


const draw_not_gpu = (self) => {
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
    const pre_link_particles = {
        [Kind.Booster]: true,
        [Kind.Ray]: true,
        [Kind.Sun]: true,
    }
    for (let pid = 0; pid < self.gravithrust.particles_count(); pid++) {
        const p = particle(particles_view, pid*particle_size, self.gravithrust.get_particle_kind(pid));
        if (p.live == 0 || !pre_link_particles[p.k]) {
            continue
        }
        draw_particle(self.context, self.context_trace, self.gravithrust.diameter, p)
    }
    // console.log("ee")

    for (let lid = 0; lid < self.gravithrust.links_count(); lid++) {
        const l = link_js(links_js_view, lid*link_js_size);
        if (l.ak != Kind.Sun && l.bk != Kind.Sun) {
            fill_circle_2(self.context, l, self.gravithrust.diameter , "#da4")
        } else {
            fill_circle_2(self.context_trace, l, self.gravithrust.diameter , "#ca28")
        }
    }
    // console.log("ee - 1")

    // self.gravithrust.print_particle()

    for (let pid = 0; pid < self.gravithrust.particles_count(); pid++) {
        // console.log("hey",pid)
        // self.gravithrust.print_particle(pid)
        // console.log()
        const p = particle(particles_view, pid*particle_size, self.gravithrust.get_particle_kind(pid));
        if (p.live == 0 || pre_link_particles[p.k] ) {
            continue
        }
        draw_particle(self.context, self.context_trace, self.gravithrust.diameter, p)
    }
    // console.log("ee - 2")
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
}


const draw = (self) => {
    // self.context_trace.globalCompositeOperation = "hard-light"
    const frame_start = performance.now()
    self.frame_starts.push(frame_start)
    if (self.webgpu) {
        self.draw_gpu()
    } else {
        self.draw_not_gpu()
    }
    document.querySelector("#points").innerHTML = self.gravithrust.points
    const duration = (( performance.now() - self.start_time) / 1000 )
    document.querySelector("#ppms").innerHTML = (self.gravithrust.points * 1000000 / self.gravithrust.step).toFixed(1)
    document.querySelector("#duration").innerHTML = parseInt(duration)
    document.querySelector("#step").innerHTML = self.gravithrust.step
    document.querySelector("#particles_count").innerHTML = self.gravithrust.particles_count()
    document.querySelector("#ships_count").innerHTML = self.gravithrust.ships_count()
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

    document.querySelector("#duration_a").innerHTML = (self.gravithrust.avg_duration.a * 1000).toFixed(5)
    document.querySelector("#duration_b").innerHTML = (self.gravithrust.avg_duration.b * 1000).toFixed(5)
    document.querySelector("#duration_c").innerHTML = (self.gravithrust.avg_duration.c * 1000).toFixed(5)
    document.querySelector("#duration_d").innerHTML = (self.gravithrust.avg_duration.d * 1000).toFixed(5)
    document.querySelector("#duration_e").innerHTML = (self.gravithrust.avg_duration.e * 1000).toFixed(5)
    document.querySelector("#duration_f").innerHTML = (self.gravithrust.avg_duration.f * 1000).toFixed(5)
    // console.log(self.gravithrust.avg_duration.a)
}


export {
    Simulation,
}
