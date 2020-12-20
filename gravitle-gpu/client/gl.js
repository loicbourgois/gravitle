const triangles_positions = [
  -1, 1,
  -1, -1,
  1, 1,
  -1, -1,
  1, -1,
  1, 1,
]
const texture_positions = [
  0, 0,
  0, 1,
  1, 0,
  0, 1,
  1, 1,
  1, 0
]
const create_program = (gl, shaders) => {
  const program = gl.createProgram()
  shaders.forEach((shader, i) => {
    gl.attachShader(program, shader);
  });
  gl.linkProgram(program);
  const success = gl.getProgramParameter(program, gl.LINK_STATUS);
  if (success) {
    return program;
  }
  console.log(gl.getProgramInfoLog(program));
  gl.deleteProgram(program);
}
const create_shader = (gl, type, source) => {
  const shader = gl.createShader(type)
  gl.shaderSource(shader, source)
  gl.compileShader(shader)
  const success = gl.getShaderParameter(shader, gl.COMPILE_STATUS)
  if (success) {
    return shader
  }
  console.log(gl.getShaderInfoLog(shader))
  gl.deleteShader(shader)
}
const resize_canvas = (canvas) => {
  canvas.width = window.innerWidth
  canvas.width = window.innerHeight
  canvas.height = window.innerHeight
  resizeCanvasToDisplaySize(canvas)
}
const clear_buffer = (gl) => {
  gl.clearColor(0, 0, 0, 0.0);
  gl.clear(gl.COLOR_BUFFER_BIT);
}
function resizeCanvasToDisplaySize(canvas, multiplier) {
  multiplier = multiplier || 1;
  const width  = canvas.clientWidth  * multiplier | 0;
  const height = canvas.clientHeight * multiplier | 0;
  if (canvas.width !== width ||  canvas.height !== height) {
    canvas.width  = width;
    canvas.height = height;
    return true;
  }
  return false;
}
const create_program_from_html = (gl, shader_query_selectors, values) => {
  let vertex_shader_source = document.querySelector(shader_query_selectors.vertex).text
  let fragment_shader_source = document.querySelector(shader_query_selectors.fragment).text
  values.forEach((item, i) => {
    vertex_shader_source = vertex_shader_source.split(item.placeholder).join(item.value)
    fragment_shader_source = fragment_shader_source.split(item.placeholder).join(item.value)
  });
  const vertex_shader = create_shader(gl, gl.VERTEX_SHADER, vertex_shader_source)
  const fragment_shader = create_shader(gl, gl.FRAGMENT_SHADER, fragment_shader_source)
  return create_program(gl, [
    vertex_shader,
    fragment_shader
  ])
}
const create_program_from_strs = (gl, shaders, values) => {
  let vertex_shader_source = shaders.vertex
  let fragment_shader_source = shaders.fragment
  values.forEach((item, i) => {
    vertex_shader_source = vertex_shader_source.split(item.placeholder).join(item.value)
    fragment_shader_source = fragment_shader_source.split(item.placeholder).join(item.value)
  })
  const vertex_shader = create_shader(gl, gl.VERTEX_SHADER, vertex_shader_source)
  const fragment_shader = create_shader(gl, gl.FRAGMENT_SHADER, fragment_shader_source)
  return create_program(gl, [
    vertex_shader,
    fragment_shader
  ])
}
const set_uniform1f = (gl, uniform) => {
  gl.uniform1f(uniform.uniform_location, uniform.value);
}
const set_uniform2f = (gl, uniform) => {
  gl.uniform2f(uniform.uniform_location,
    uniform.values[0],
    uniform.values[1],
  );
}
const set_uniform3f = (gl, uniform) => {
  gl.uniform3f(uniform.uniform_location,
    uniform.values[0],
    uniform.values[1],
    uniform.values[2],
  );
}
const set_uniform4f = (gl, uniform) => {
  gl.uniform4f(uniform.uniform_location,
    uniform.values[0],
    uniform.values[1],
    uniform.values[2],
    uniform.values[3],
  );
}
const set_uniform4fv = (gl, uniform) => {
  gl.uniform4fv(uniform.uniform_location, uniform.values);
}
const set_uniforms = (gl, program, uniforms) => {
  for (const uniform_id in uniforms) {
    const uniform = uniforms[uniform_id]
    uniform.setter(gl, uniform)
  }
}
const load_texture = (gl, texture) => {
  gl.activeTexture(gl.TEXTURE0 + texture.texture_id)
  gl.bindTexture(gl.TEXTURE_2D, texture.gl_texture)
  gl.texImage2D(
    gl.TEXTURE_2D,
    texture.parameters.mip_evel,
    texture.parameters.internal_format,
    texture.parameters.width,
    texture.parameters.height,
    texture.parameters.border,
    texture.parameters.source_format,
    texture.parameters.source_type,
    texture.buffer)
}
