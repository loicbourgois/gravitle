const vertex_shader = `#version 300 es
in vec4 position;
out vec4 position_varying;
void main() {
  gl_Position = position;
  position_varying = gl_Position;
}`
