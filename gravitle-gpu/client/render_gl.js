'use strict';
const canvas = document.querySelector('#canvas_main', {preserveDrawingBuffer: false})
const gl = canvas.getContext('webgl2')
gl.imageSmoothingEnabled = false
const program = create_program_from_strs(
  gl,
  {
    vertex: vertex_shader,
    fragment: fragment_shader
  },
  [
    {
      placeholder: '{RED_AMOUNT}',
      value: conf.red_amount
    },
  ]
)
const position = {
  attrib_location: gl.getAttribLocation(program, 'position'),
  vao: gl.createVertexArray(),
  buffer: gl.createBuffer(),
  data: new Float32Array(triangles_positions),
  parameters: {
    size: 2,
    type: gl.FLOAT,
    normalize: false,
    stride: 0,
    offset: 0
  },
  draw_parameters: {
    offset: 0,
    primitiveType: gl.TRIANGLES,
  },
  get draw_parameters_count() {
    return this.data.length / this.parameters.size
  }
}
gl.bindBuffer(gl.ARRAY_BUFFER, position.buffer)
gl.bufferData(gl.ARRAY_BUFFER, position.data, gl.STATIC_DRAW)
gl.bindVertexArray(position.vao)
gl.enableVertexAttribArray(position.attrib_location)
gl.vertexAttribPointer(
  position.attrib_location,
  position.parameters.size,
  position.parameters.type,
  position.parameters.normalize,
  position.parameters.stride,
  position.parameters.offset)
resize_canvas(gl.canvas)
const uniforms = {
  time_ms: {
    value: 0,
    uniform_location: gl.getUniformLocation(program, 'time_ms'),
    setter: set_uniform1f,
  },
  circle_diameter: {
    value: 1,
    uniform_location: gl.getUniformLocation(program, 'circle_diameter'),
    setter: set_uniform1f,
  },
  buffer_dimensions: {
    values: [gl.canvas.width, gl.canvas.height],
    uniform_location: gl.getUniformLocation(program, 'buffer_dimensions'),
    setter: set_uniform2f,
  },
  seeds: {
    values: [Math.random(), Math.random(), Math.random(), Math.random()],
    uniform_location: gl.getUniformLocation(program, 'seeds'),
    setter: set_uniform4f,
  },
  slider_0: {
    value: 1,
    uniform_location: gl.getUniformLocation(program, 'slider_0'),
    setter: set_uniform1f,
  },
  slider_1: {
    value: 1,
    uniform_location: gl.getUniformLocation(program, 'slider_1'),
    setter: set_uniform1f,
  },
  slider_2: {
    value: 1,
    uniform_location: gl.getUniformLocation(program, 'slider_2'),
    setter: set_uniform1f,
  },
  slider_3: {
    value: 1,
    uniform_location: gl.getUniformLocation(program, 'slider_3'),
    setter: set_uniform1f,
  },
  particles: {
    values: [1.0, 2.0, 3.0, 4.0],
    uniform_location: gl.getUniformLocation(program, 'u_particles'),
    setter: set_uniform4fv,
  },
}
const textures = {
  previous_image: {
    buffer: new Uint8Array(gl.canvas.width * gl.canvas.height * 4),
    uniform_location: gl.getUniformLocation(program, "previous_image"),
    gl_texture: gl.createTexture(),
    texture_id: 0,
    parameters: {
      mip_evel: 0,
      internal_format: gl.RGBA,
      source_format: gl.RGBA,
      source_type: gl.UNSIGNED_BYTE,
      width: gl.canvas.width,
      height: gl.canvas.height,
      border: 0
    }
  }
}
gl.activeTexture(gl.TEXTURE0 + textures.previous_image.texture_id);
gl.bindTexture(gl.TEXTURE_2D, textures.previous_image.gl_texture);
gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.NEAREST);
gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.NEAREST);
gl.useProgram(program)
const render_gl = () => {
  const start_time_ms = Date.now()
  uniforms.seeds.values = [Math.random(), Math.random(), Math.random(), Math.random()]
  set_uniforms(gl, program, uniforms)
  gl.uniform1i(textures.previous_image.uniform_location, textures.previous_image.texture_id)
  load_texture(gl, textures.previous_image)
  gl.bindVertexArray(position.vao)
  // gl.bindFramebuffer(gl.FRAMEBUFFER, null)
  gl.viewport(0, 0, gl.canvas.width, gl.canvas.height)
  gl.vertexAttribPointer(
    position.attrib_location,
    position.parameters.size,
    position.parameters.type,
    position.parameters.normalize,
    position.parameters.stride,
    position.parameters.offset)
  // gl.bindBuffer(gl.ARRAY_BUFFER, position.buffer)
  clear_buffer(gl)
  gl.drawArrays(
    position.draw_parameters.primitiveType,
    position.draw_parameters.offset,
    position.draw_parameters_count)
  gl.readPixels(0, 0, gl.canvas.width, gl.canvas.height, gl.RGBA, gl.UNSIGNED_BYTE, textures.previous_image.buffer);
  if (conf.log_render_duration) {
    console.log(`a: ${Date.now() - start_time_ms}`)
    console.log(`b: ${Date.now() - conf.render.last_time}`)
  }
  conf.render.last_time = Date.now()
}
const handle_controls = () => {
  for (let i = 0 ; i < 4 ; i+= 1) {
    const str = `slider_${i}`
    const slider = document.querySelector(`#${str}`)
    uniforms[str].value = slider.value / (slider.max - slider.min)
    document.querySelector(`#${str}_value_label`).innerHTML = uniforms[str].value.toFixed(3)
  }
}
/*const start_render_loop_gl = () => {
  conf.start_render_loop_ms = Date.now()
  conf.interval_render_id = setInterval(render_gl, conf.interval_render_ms);
}*/
const controls_loop = () => {
  conf.interval_controls_id = setInterval(handle_controls, conf.interval_controls_ms);
}
const step = () => {
  uniforms.time_ms.value = Date.now() - conf.start_ms
}
const main_loop = () => {
  conf.start_ms = Date.now()
  conf.interval_main_id = setInterval(step, conf.interval_main_ms);
}
//render_loop_gl()
//controls_loop()
//main_loop()
//render_gl()
