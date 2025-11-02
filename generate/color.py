from .kind import kind
from .user_kind import user_kind
from .misc import write_force
from .env import HOME

m = "m"
g = "g"
o = "o"

color = {
    kind.ARMOR: {
        0: {
            0: {
                m: "#aaf",
                g: "#558",
                o: "#585",
            },
        },
    },
    kind.BOOSTER: {
        0: {
            0: {
                m: "#fa0",
                g: "#850",
                o: "#850",
            },
        },
        1: {
            0: {
                m: "#fa0",
                g: "#850",
                o: "#850",
            },
            1: {
                m: "#f80",
                g: "#840",
                o: "#840",
            },
            2: {
                m: "#f00",
                g: "#800",
                o: "#800",
            },
        },
    },
    kind.CORE: {
        0: {
            0: {
                m: "#ffa",
                g: "#885",
                o: "#885",
            },
        },
    },
    kind.ASTEROID: {
        0: {
            0: {
                m: "#b60",
            },
        },
    },
    kind.UNLIGHTED: {
        0: {
            0: {
                m: "#ff93",
            },
        },
    },
    kind.LIGHTED: {
        0: {
            0: {
                m: "#ffad",
                g: "#0000",
                o: "#4f44",
            },
        },
    },
}


def colors_generated_js():
    lines = [
        "// Generated from gravitle/generate/main.py",
        'import { kind } from "./kind.js";',
        "const colors_generated = {",
    ]
    for x in kind:
        lines.append(f"[kind.{x.name}]: {color[x]},")
    lines.append("}")
    lines.append("export {colors_generated}")
    write_force(
        f"{HOME}/github.com/loicbourgois/gravitle/front/chrono/colors_generated.js",
        "\n".join(lines),
    )


def colors_wgsl():
    lines = [
        "// Generated from gravitle/generate/main.py",
        "switch particle.user_kind {",
    ]
    for y in user_kind:
        lines.append(f"  case USER_KIND_{y.name}: {{")
        lines.append("    switch particle.kind {")
        kl = {
            user_kind.USER: "m",
            user_kind.GHOST: "g",
            user_kind.OTHER: "o",
        }[y]
        for x in kind:
            if color[x][0][0].get(kl):
                r = int(color[x][0][0][kl][1], 16) * 17 / 255.0
                g = int(color[x][0][0][kl][2], 16) * 17 / 255.0
                b = int(color[x][0][0][kl][3], 16) * 17 / 255.0
                if len(color[x][0][0][kl]) == 5:
                    a = int(color[x][0][0][kl][4], 16) * 17 / 255.0
                else:
                    a = 1.0
                lines.append(f"      case KIND_{x.name}: {{")
                lines.append(
                    f"        vsOut.color = vec4f({r * a}, {g * a}, {b * a}, {a});"
                )
                lines.append("      }")
        lines.append("  default:{}")
        lines.append("    }")
        lines.append("  }")
    lines.append("  default:{}")
    lines.append("}")

    write_force(
        f"{HOME}/github.com/loicbourgois/gravitle/generate/wgsl/colors.wgsl",
        "\n".join(lines),
    )
