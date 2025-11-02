from .misc import read, write_force
from .kind import kind
from .env import HOME


def code_wgsl():
    content = (
        read(f"{HOME}/github.com/loicbourgois/gravitle/generate/wgsl/code.wgsl")
        .replace(
            "{colors}",
            read(f"{HOME}/github.com/loicbourgois/gravitle/generate/wgsl/colors.wgsl"),
        )
        .replace(
            "{disk}",
            read(f"{HOME}/github.com/loicbourgois/gravitle/generate/wgsl/disk.wgsl"),
        )
        .replace(
            "{cell}",
            read(f"{HOME}/github.com/loicbourgois/gravitle/generate/wgsl/cell.wgsl"),
        )
        .replace(
            "{kind}",
            read(f"{HOME}/github.com/loicbourgois/gravitle/generate/wgsl/kind.wgsl"),
        )
    )
    write_force(
        f"{HOME}/github.com/loicbourgois/gravitle/front/chrono/webgpu/code.wgsl",
        content,
    )


def kind_wgsl():
    lines = []
    for x in kind:
        lines.append(f"const KIND_{x.name} = {x.value};")
    write_force(
        f"{HOME}/github.com/loicbourgois/gravitle/generate/wgsl/kind.wgsl",
        "\n".join(lines),
    )
