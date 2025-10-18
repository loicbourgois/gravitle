const USER_KIND_USER = 1;
const USER_KIND_GHOST = 2;
const USER_KIND_OTHER = 3;
const KIND_ARMOR = 0;
const KIND_BOOSTER = 1;
const KIND_CORE = 2;
const KIND_ASTEROID = 4;
const KIND_UNLIGHTED = 5;
const KIND_LIGHTED = 6;
struct Cell {
    idx: u32,
    diameter: f32,
    p: vec2f,
    pp: vec2f,
    np: vec2f,
    dp: vec2f,
    direction: vec2f,
    dv: vec2f,
    link_response: vec2f,
    collision_response: vec2f,
    collision_response_count: u32,
    activated: u32,
    activated_previous: u32,
    kind: u32,
    user_kind: u32,
    padding: u32,
}

struct VSOutput {
  @builtin(position) position: vec4f,
  @location(0) color: vec4f,
}
const disk_positions = array<vec2f, 48>(
    vec2f( 0,  0),
    vec2f( 1,  0),
    vec2f( 0.9238795,  0.38268346),
    vec2f( 0,  0),
    vec2f( 0.9238795,  0.38268346),
    vec2f( 0.70710677,  0.70710677),
    vec2f( 0,  0),
    vec2f( 0.70710677,  0.70710677),
    vec2f( 0.38268343,  0.9238795),
    vec2f( 0,  0),
    vec2f( 0.38268343,  0.9238795),
    vec2f( -0.00000004371139,  1),
    vec2f( 0,  0),
    vec2f( -0.00000004371139,  1),
    vec2f( -0.38268352,  0.9238795),
    vec2f( 0,  0),
    vec2f( -0.38268352,  0.9238795),
    vec2f( -0.70710677,  0.70710677),
    vec2f( 0,  0),
    vec2f( -0.70710677,  0.70710677),
    vec2f( -0.9238796,  0.38268328),
    vec2f( 0,  0),
    vec2f( -0.9238796,  0.38268328),
    vec2f( -1,  -0.00000008742278),
    vec2f( 0,  0),
    vec2f( -1,  -0.00000008742278),
    vec2f( -0.9238795,  -0.38268346),
    vec2f( 0,  0),
    vec2f( -0.9238795,  -0.38268346),
    vec2f( -0.70710665,  -0.7071069),
    vec2f( 0,  0),
    vec2f( -0.70710665,  -0.7071069),
    vec2f( -0.38268313,  -0.9238797),
    vec2f( 0,  0),
    vec2f( -0.38268313,  -0.9238797),
    vec2f( 0.000000011924881,  -1),
    vec2f( 0,  0),
    vec2f( 0.000000011924881,  -1),
    vec2f( 0.3826836,  -0.92387944),
    vec2f( 0,  0),
    vec2f( 0.3826836,  -0.92387944),
    vec2f( 0.707107,  -0.70710653),
    vec2f( 0,  0),
    vec2f( 0.707107,  -0.70710653),
    vec2f( 0.92387956,  -0.38268343),
    vec2f( 0,  0),
    vec2f( 0.92387956,  -0.38268343),
    vec2f( 1,  0.00000017484555),
);

const ZOOM = 1.5;
@group(0) @binding(0) var<storage, read> particles: array<Cell>;
@vertex fn vs_0(
  @builtin(vertex_index) vertexIndex : u32,
  @builtin(instance_index) instanceIndex: u32,
) -> VSOutput {
  let particle = particles[instanceIndex];
  let center = vec2f( 0.5,  0.5);
  var vsOut: VSOutput;
  var aa = 1.0;
  vsOut.position = vec4f(
    (disk_positions[vertexIndex]*particle.diameter*0.55*aa + particle.p*1.1 - center )* ZOOM,
    0.0, 1.0
  );
  // Generated from gravitle/generate/main.py
switch particle.user_kind {
  case USER_KIND_USER: {
    switch particle.kind {
      case KIND_ARMOR: {
        vsOut.color = vec4f(0.6666666666666666, 0.6666666666666666, 1.0, 1.0);
      }
      case KIND_BOOSTER: {
        vsOut.color = vec4f(1.0, 0.6666666666666666, 0.0, 1.0);
      }
      case KIND_CORE: {
        vsOut.color = vec4f(1.0, 1.0, 0.6666666666666666, 1.0);
      }
      case KIND_ASTEROID: {
        vsOut.color = vec4f(0.7333333333333333, 0.4, 0.0, 1.0);
      }
      case KIND_UNLIGHTED: {
        vsOut.color = vec4f(0.2, 0.2, 0.12, 0.2);
      }
      case KIND_LIGHTED: {
        vsOut.color = vec4f(0.8666666666666667, 0.8666666666666667, 0.5777777777777777, 0.8666666666666667);
      }
  default:{}
    }
  }
  case USER_KIND_GHOST: {
    switch particle.kind {
      case KIND_ARMOR: {
        vsOut.color = vec4f(0.3333333333333333, 0.3333333333333333, 0.5333333333333333, 1.0);
      }
      case KIND_BOOSTER: {
        vsOut.color = vec4f(0.5333333333333333, 0.3333333333333333, 0.0, 1.0);
      }
      case KIND_CORE: {
        vsOut.color = vec4f(0.5333333333333333, 0.5333333333333333, 0.3333333333333333, 1.0);
      }
      case KIND_LIGHTED: {
        vsOut.color = vec4f(0.0, 0.0, 0.0, 0.0);
      }
  default:{}
    }
  }
  case USER_KIND_OTHER: {
    switch particle.kind {
      case KIND_ARMOR: {
        vsOut.color = vec4f(0.3333333333333333, 0.5333333333333333, 0.3333333333333333, 1.0);
      }
      case KIND_BOOSTER: {
        vsOut.color = vec4f(0.5333333333333333, 0.3333333333333333, 0.0, 1.0);
      }
      case KIND_CORE: {
        vsOut.color = vec4f(0.5333333333333333, 0.5333333333333333, 0.3333333333333333, 1.0);
      }
      case KIND_LIGHTED: {
        vsOut.color = vec4f(0.07111111111111111, 0.26666666666666666, 0.07111111111111111, 0.26666666666666666);
      }
  default:{}
    }
  }
  default:{}
}
  if (particle.user_kind == USER_KIND_USER) {
    if (particle.kind == KIND_ASTEROID) {
      vsOut.position.z = 0.0;
    } else {
      vsOut.position.z = 0.1;
    }
  } else {
    vsOut.position.z = 0.2;
  }
  return vsOut;
}
@fragment fn fs_0(vsOut: VSOutput) -> @location(0) vec4f {
  return vsOut.color;
}
