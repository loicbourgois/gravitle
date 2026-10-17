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
        vsOut.color = vec4f(0.46222222222222226, 0.23111111111111113, 0.23111111111111113, 0.8666666666666667);
      }
      case KIND_UNLIGHTED: {
        vsOut.color = vec4f(0.4, 0.4, 0.24, 0.4);
      }
      case KIND_LIGHTED: {
        vsOut.color = vec4f(0.9333333333333333, 0.9333333333333333, 0.6222222222222222, 0.9333333333333333);
      }
      case KIND_HEADLIGHT: {
        vsOut.color = vec4f(0.9333333333333333, 0.9333333333333333, 0.6222222222222222, 0.9333333333333333);
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
        vsOut.color = vec4f(0.10666666666666667, 0.4, 0.10666666666666667, 0.4);
      }
  default:{}
    }
  }
  default:{}
}