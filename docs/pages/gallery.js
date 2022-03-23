import*as wasm from"../../wasm/pkg";import*as webgpu_server from"../webgpu_server";import*as webgpu_renderer from"../webgpu_renderer";import{memory}from"../../wasm/pkg/wasm_bg.wasm";import*as render_reset from"../shaders/render_reset";import*as render_trace from"../shaders/render_trace";import{update_fps}from"../renderer_util";import{random_int}from"../maths";const float_size=4,part_args=13,part_size=52,max_parts_sqrt=64,data_size=48,max_parts=4096,RESOLUTION=.5,kind={FIREFLY:1,METAL:2,TURBO:3,DIATOM:4,NEURON:5,MOUTH:6,CORE:7,EGG:8};function gallery(){let e=!1;const i=window.location.search.replace("?","").split("&");for(var t of i){const i=t.split("=")[0],r=t.split("=")[1];"webgpu"===i&&"true"===r&&(e=!0)}document.getElementById("content").innerHTML='<canvas id="canvas"></canvas>\n<div id="panel">\n<div id="menu">\n  <a href="/playground">Playground</a>\n  <a href="/gallery">Gallery</a>\n  <a href="/gallery?webgpu=true">Gallery (WebGPU)</a>\n</div>\n  <canvas id="minimap"></canvas>\n  <div>\n    Zoom: <input type="range" min="0" max="1000" value="0" id="zoom_slider">\n  </div>\n  <div>\n    x: <input type="range" min="0" max="1000" value="500" id="x_slider">\n  </div>\n  <div>\n    y: <input type="range" min="0" max="1000" value="500" id="y_slider">\n  </div>\n  <p id="p_step"></p>\n  <p id="p_fps"></p>\n  <p id="p_render_duration"></p>\n  <p id="p_cps"></p>\n  <p id="p_compute_duration"></p>\n  <p id="p_pids"></p>\n  <pre id="p_counter_global"></pre>\n  <pre id="p_counter_activity"></pre>\n  <pre id="p_counter_collision"></pre>\n  <pre id="p_counter_linked"></pre>\n  <p id="p_energy"></p>\n</div>';const r=wasm.Server.new(128,128),a=wasm.Plan.new(kind.NEURON,.25,kind.NEURON);a.add(0,1,kind.MOUTH),a.add(1,0,kind.CORE);let n=kind.TURBO;a.add(1,3,n),a.add(3,0,n),a.add(1,4,kind.NEURON),a.add(5,0,kind.NEURON),a.add(1,6,kind.MOUTH),a.add(7,0,kind.MOUTH),a.add(6,4,kind.NEURON),a.add(5,7,kind.NEURON),r.add_entity(a,wasm.Point.new(0,0));for(var d=0;d<10;d++)r.add_entity(a,wasm.Point.new(Math.random(),Math.random()));if(run(r),!0===e)if("gpu"in navigator)setup_webgpu(r);else{const e="Gravitle works best with WebGPU.\nInstructions on how to enable at https://web.dev/gpu/#use";alert(e),console.error(e),render_canvas(r)}else render_canvas(r)}async function render_webgpu(e,i,t){const r=performance.now();void 0===i.fps_counter&&(i.fps_counter=[]),void 0===i.fps_counter_length&&(i.fps_counter_length=100);const a=Math.max(canvas.width,canvas.height),n={x:document.getElementById("x_slider").value/1e3,y:document.getElementById("y_slider").value/1e3,zoom:1e3/(1e3-document.getElementById("zoom_slider").value)},d=render_minimap(n,a),o=(d.canvas,new Float32Array(new Float32Array(memory.buffer,e.parts_ptr(),13*e.parts_count()))),s=(canvas.width,canvas.height,n.x-.5/n.zoom*i.image_width/a),u=n.y-.5/n.zoom*i.image_height/a,f=s+1/n.zoom*i.image_width/a,p=u+1/n.zoom*i.image_height/a;for(let i=0;i<e.parts_count();i++){const e=o[13*i+0],t=o[13*i+1];render_p_minimap(e,t,o[13*i+4],d,!(s<=e&&e<=f&&u<=t&&t<=p))}let m=i.buffers.write.mapAsync(GPUMapMode.WRITE),c=i.buffers.write_data.mapAsync(GPUMapMode.WRITE);await m,await c,new Float32Array(i.buffers.write.getMappedRange()).set(o);const g=e.parts_count();new Float32Array(i.buffers.write_data.getMappedRange()).set([n.zoom,n.x,n.y,g,i.image_width,i.image_height,s,u,f,p,performance.now(),e.get_step()/1]),i.buffers.write.unmap(),i.buffers.write_data.unmap();{const e=i.device.createCommandEncoder(),t=e.beginComputePass();t.setPipeline(i.compute_pipelines.reset),t.setBindGroup(0,i.bind_groups.reset),t.dispatch(i.image_width,i.image_height),t.endPass();const r=e.finish();i.device.queue.submit([r])}{const e=i.device.createCommandEncoder();e.copyBufferToBuffer(i.buffers.write,0,i.buffers.in,0,i.data_buffer_size),e.copyBufferToBuffer(i.buffers.write_data,0,i.buffers.in_data,0,48);const t=e.beginComputePass();t.setPipeline(i.compute_pipelines.trace),t.setBindGroup(0,i.bind_groups.trace),t.dispatch(i.dispatch,i.dispatch),t.endPass(),e.copyBufferToBuffer(i.buffers.out,0,i.buffers.read,0,image_buffer_size(i)),e.copyBufferToBuffer(i.buffers.out,0,i.buffers.previous_img,0,image_buffer_size(i));const r=e.finish();i.device.queue.submit([r])}let _=i.buffers.read.mapAsync(GPUMapMode.READ);await _,t.putImageData(new ImageData(Uint8ClampedArray.from(new Uint32Array(i.buffers.read.getMappedRange())),i.image_width,i.image_height),0,0),i.buffers.read.unmap(),document.getElementById("p_counter_global").innerHTML=`Global:      ${e.get_counter_value(0).toFixed(2)}ms`,update_fps(i),window.requestAnimationFrame((function(){render_webgpu(e,i,t)}));const h=performance.now();i.fps_counter.push({start:r,end:h,duration:h-r})}async function setup_webgpu(e){const i=await navigator.gpu.requestAdapter();if(!i)return void console.error("No gpu adapter found");const t={buffer_size:data_buffer_size({})},r={data_buffer_size:t.buffer_size,image_width:Math.floor(.5*window.innerWidth),image_height:Math.floor(.5*window.innerHeight),max_parts_sqrt:64};r.device=await i.requestDevice(),r.buffers={write:r.device.createBuffer({size:t.buffer_size,usage:GPUBufferUsage.MAP_WRITE|GPUBufferUsage.COPY_SRC}),in:r.device.createBuffer({size:t.buffer_size,usage:GPUBufferUsage.STORAGE|GPUBufferUsage.COPY_DST|GPUBufferUsage.COPY_SRC}),out:r.device.createBuffer({size:image_buffer_size(r),usage:GPUBufferUsage.STORAGE|GPUBufferUsage.COPY_DST|GPUBufferUsage.COPY_SRC}),previous_img:r.device.createBuffer({size:image_buffer_size(r),usage:GPUBufferUsage.STORAGE|GPUBufferUsage.COPY_DST|GPUBufferUsage.COPY_SRC}),read:r.device.createBuffer({size:image_buffer_size(r),usage:GPUBufferUsage.MAP_READ|GPUBufferUsage.COPY_DST}),write_data:r.device.createBuffer({size:48,usage:GPUBufferUsage.MAP_WRITE|GPUBufferUsage.COPY_SRC}),in_data:r.device.createBuffer({size:48,usage:GPUBufferUsage.STORAGE|GPUBufferUsage.COPY_DST|GPUBufferUsage.COPY_SRC})},r.bind_group_layouts={reset:r.device.createBindGroupLayout({entries:[{binding:0,visibility:GPUShaderStage.COMPUTE,buffer:{type:"storage"}},{binding:1,visibility:GPUShaderStage.COMPUTE,buffer:{type:"storage"}}]}),trace:r.device.createBindGroupLayout({entries:[{binding:0,visibility:GPUShaderStage.COMPUTE,buffer:{type:"storage"}},{binding:1,visibility:GPUShaderStage.COMPUTE,buffer:{type:"storage"}},{binding:2,visibility:GPUShaderStage.COMPUTE,buffer:{type:"storage"}}]})},r.bind_groups={reset:r.device.createBindGroup({layout:r.bind_group_layouts.reset,entries:[{binding:0,resource:{buffer:r.buffers.previous_img}},{binding:1,resource:{buffer:r.buffers.out}}]}),trace:r.device.createBindGroup({layout:r.bind_group_layouts.trace,entries:[{binding:0,resource:{buffer:r.buffers.in}},{binding:1,resource:{buffer:r.buffers.out}},{binding:2,resource:{buffer:r.buffers.in_data}}]})},r.workgroup_size=16,r.dispatch=Math.ceil(64/r.workgroup_size),r.compute_pipelines={reset:r.device.createComputePipeline({layout:r.device.createPipelineLayout({bindGroupLayouts:[r.bind_group_layouts.reset]}),compute:{module:r.device.createShaderModule({code:render_reset.get(r)}),entryPoint:"main"}}),trace:r.device.createComputePipeline({layout:r.device.createPipelineLayout({bindGroupLayouts:[r.bind_group_layouts.trace]}),compute:{module:r.device.createShaderModule({code:render_trace.get(r)}),entryPoint:"main"}})};const a=document.getElementById("canvas");a.width=r.image_width,a.height=r.image_height,a.style.width=window.innerWidth+"px",a.style.height=window.innerHeight+"px",render_webgpu(e,r,a.getContext("2d"))}function image_buffer_size(e){return e.image_width*e.image_height*4*4}function data_buffer_size(e){return 212992}function render_minimap(e,i){const t=document.getElementById("minimap");t.width=256,t.height=256;const r=t.getContext("2d");return r.beginPath(),r.fillStyle="#ff000088",r.rect(e.x*t.width-.5*t.width/e.zoom,e.y*t.height-.5*t.height/e.zoom,t.width/e.zoom,t.height/e.zoom),r.fill(),r.beginPath(),r.fillStyle="#FFFF0088",r.rect(e.x*t.width-.5*t.width/e.zoom*canvas.width/i,e.y*t.height-.5*t.height/e.zoom*canvas.height/i,t.width/e.zoom*canvas.width/i,t.height/e.zoom*canvas.height/i),r.fill(),r.fillStyle="#000",r}function render_p_minimap(e,i,t,r,a){r.fillStyle=a?"#888":"#000",r.beginPath(),r.arc(e*minimap.width,i*minimap.height,t*minimap.width*.5,0,2*Math.PI),r.fill(),r.beginPath(),r.arc((e+1)*minimap.width,i*minimap.height,t*minimap.width*.5,0,2*Math.PI),r.fill(),r.beginPath(),r.arc((e-1)*minimap.width,i*minimap.height,t*minimap.width*.5,0,2*Math.PI),r.fill(),r.beginPath(),r.arc(e*minimap.width,(i+1)*minimap.height,t*minimap.width*.5,0,2*Math.PI),r.fill(),r.beginPath(),r.arc(e*minimap.width,(i-1)*minimap.height,t*minimap.width*.5,0,2*Math.PI),r.fill()}function render_canvas(e){const i=document.getElementById("canvas");i.width=window.innerWidth,i.height=window.innerHeight,i.style.width=window.innerWidth+"px",i.style.height=window.innerHeight+"px";const t=i.getContext("2d"),r=Math.max(i.width,i.height),a={x:document.getElementById("x_slider").value/1e3,y:document.getElementById("y_slider").value/1e3,zoom:1e3/(1e3-document.getElementById("zoom_slider").value)},n=render_minimap(a,r),d=(n.canvas,new Float32Array(memory.buffer,e.parts_ptr(),13*e.parts_count())),o=.5*(1-1/a.zoom),s=o*i.width/r,u=o*i.height/r,f=(r-i.width)/r*.5,p=(r-i.height)/r*.5;for(let i=0;i<e.parts_count();i++){const e=d[13*i+0],o=d[13*i+1],m=d[13*i+4];render_p_minimap(e,o,m,n),t.fillStyle="#aaa",t.beginPath(),t.arc((e+.5-a.x-s-f)*a.zoom*r,(o+.5-a.y-u-p)*a.zoom*r,m*r*.5*a.zoom,0,2*Math.PI),t.fill()}window.requestAnimationFrame((function(){render_canvas(e)}))}function run(e){e.tick(),window.setTimeout((async function(){await run(e)}),5)}export{gallery};