const fragment_shader = `#version 300 es
precision highp float;
uniform float time_ms;
uniform float circle_diameter;
uniform vec2 buffer_dimensions;
uniform sampler2D previous_image;
uniform vec4 seeds;
uniform float slider_0;
uniform float slider_1;
uniform float slider_2;
uniform float slider_3;

uniform vec2 canvas_dimensions;

uniform vec4 u_particles[1000];

in vec4 position_varying;
out vec4 color;
float decimal_part(float f) {
  return f - trunc(f);
}
float random2 (vec2 co) {
  highp float a = 12.9898;
  highp float b = 78.233;
  highp float c = 43758.5453;
  highp float dt= dot(co.xy ,vec2(a,b));
  highp float sn= mod(dt,3.14);
  return fract(sin(sn) * c);
}
float random3 (vec3 v) {
    return random2(vec2(v.z, random2(v.xy)));
}
float random4 (vec4 v) {
    return  random2(vec2(random2(vec2(v.rg)), random2(vec2(v.ba))));
}
void main() {
float t = time_ms * 0.001;
  // coordinates of the current fragment
  // a fragment is the pixel being rendered
  // from [0, 0] to [canvas.width, canvas.height]
  vec4 f = gl_FragCoord;
  vec4 previous_f_color = texelFetch(previous_image, ivec2(f.xy), 0);
    //f.x += time;
  f /= 4.0;
  // coordinates in clipspace
  // from [0, 0] to [1, 1]
  vec4 pix = (position_varying + vec4(1.0, 1.0, 0.0, 0.0))*0.5;
  //
  // vec2 dimensions = vec2(100.0, 5.7);
  pix = gl_FragCoord;
  int p_count = 100;

  int particle_materials[100];


  vec4 materials_definition[4];
  materials_definition[0] = vec4(0.5, 0.5, 0.5, 0.0);
  materials_definition[1] = vec4(0.0, 0.5, 0.0, 0.0);
  materials_definition[2] = vec4(0.0, 0.5, 1.0, 0.0);
  materials_definition[3] = vec4(1.0, 1.0, 0.0, 1.0);

  particle_materials[0] = 0;
  particle_materials[1] = 1;
  particle_materials[2] = 2;
  particle_materials[3] = 0;
  particle_materials[4] = 3;
  particle_materials[5] = 1;
  particle_materials[6] = 1;
  particle_materials[7] = 2;
  particle_materials[8] = 0;

  float pix_materials[5];
  for (int i = 0 ; i < 5 ; i +=1 ) {
    pix_materials[i] = 0.0;
  }

  vec3 c = vec3(0.0, 0.0, 0.0);
  float divider = 0.0;
  vec3 white = vec3(1.0, 1.0, 1.0);
  vec3 red = vec3(1.0, 0.0, 0.0);
  vec3 green = vec3(0.0, 1.0, 0.0);
  float v = 0.0;
  float w = 0.0;
  float intensity = 0.0;
  float particle_area = 0.0;

  for (int i = 0 ; i < p_count ; i+=1) {
    vec4 p = u_particles[i];
    float dx = p.x - pix.x;
    float dy = p.y - pix.y;
    float d = (distance(p.xy, pix.xy));
    float r = p.z * 0.5;
    float wn = r / (dx*dx + dy*dy);
    pix_materials[particle_materials[i]] += wn;
    intensity += wn;
    particle_area += max(p.z*0.5 - (dx*dx + dy*dy), 0.0);
  }

  if (particle_area * 1000.0 > 0.1) {
    particle_area = 1.0;
  }

  float strength = slider_0;
  float size = 0.8;

  float material_divider = 0.0;

  c.r = 0.0;
  c.g = 0.0;
  c.b = 0.0;

  float glow = 0.0;
  for (int i = 0 ; i < p_count ; i+=1) {
    // int mid = particle_materials[i];
    int mid = 2;
    c.r += materials_definition[mid].r * pix_materials[mid];
    c.g += materials_definition[mid].g * pix_materials[mid];
    c.b += materials_definition[mid].b * pix_materials[mid];
    glow += materials_definition[mid].w * pix_materials[mid];
    material_divider += pix_materials[mid];
  }

  glow /= material_divider;
  float glow_inverted = (1.0 - glow)*50.0;
  glow_inverted = 50.00 - glow * 100.00 ;

  glow_inverted = 50.0 - glow*50.0;

  float field_limit = pow(min(floor(intensity * strength) + (intensity - floor(intensity * strength) )*size, 1.0), glow_inverted);

  c.rgb /= material_divider;
  c.rgb *= field_limit;
  c.r = field_limit;
  //c.g = 0.0;
  //c.b = particle_area;

  //c.r = 0.0;
  //c.g = particle_area * 0.5;
  //c.b = field_limit;
  float a = 0.0;
  if (c.r + c.g + c.b > 0.1) {
    a = 1.0;
  }
  color = vec4(c, 0.5);
}`
