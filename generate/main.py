import os
from .color import color
from .kind import kind
from .user_kind import user_kind
from .blueprint import blueprint
from .misc import read, write_force


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
        "/root/github.com/loicbourgois/gravitle/front/chrono/colors_generated.js",
        "\n".join(lines),
    )


def colors_wgsl():
    lines = [
        "// Generated from gravitle/generate/main.py",
        "switch particle.user_kind {",
    ]
    for y in user_kind:
        lines.append(f"  case USER_KIND_{y.name}: {{")
        lines.append(f"    switch particle.kind {{")
        kl = {
            user_kind.USER:'m',
            user_kind.GHOST:'g',
            user_kind.OTHER:'o',
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
                lines.append(f"        vsOut.color = vec4f({r*a}, {g*a}, {b*a}, {a});")
                lines.append(f"      }}")
        lines.append(f"  default:{{}}")
        lines.append(f"    }}")
        lines.append(f"  }}")
    lines.append(f"  default:{{}}")
    lines.append(f"}}")

    write_force(
        "/root/github.com/loicbourgois/gravitle/generate/wgsl/colors.wgsl",
        "\n".join(lines),
    )


def code_wgsl():
    content = read("/root/github.com/loicbourgois/gravitle/generate/wgsl/code.wgsl").replace(
        "{colors}", read("/root/github.com/loicbourgois/gravitle/generate/wgsl/colors.wgsl")
    ).replace(
        "{disk}", read("/root/github.com/loicbourgois/gravitle/generate/wgsl/disk.wgsl"),
    ).replace(
        "{cell}", read("/root/github.com/loicbourgois/gravitle/generate/wgsl/cell.wgsl"),
    ).replace(
        "{kind}", read("/root/github.com/loicbourgois/gravitle/generate/wgsl/kind.wgsl"),
    )
    write_force(
        "/root/github.com/loicbourgois/gravitle/front/chrono/webgpu/code.wgsl",
        content,
    )


def kind_wgsl():
    lines = []
    for x in kind:
        lines.append(f"const KIND_{x.name} = {x.value};")
    write_force(
        "/root/github.com/loicbourgois/gravitle/generate/wgsl/kind.wgsl",
        "\n".join(lines),
    )


if __name__ == "__main__":
    print("# Generate - start")
    colors_generated_js()
    colors_wgsl()
    kind_wgsl()
    code_wgsl()
    blueprint()
    print("# Generate - end")
