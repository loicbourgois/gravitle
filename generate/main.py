from .color import (
    colors_generated_js,
    colors_wgsl,
)
from .blueprint import blueprint
from .wgsl import (
    code_wgsl,
    kind_wgsl,
)
from .blueprint.tree import generate_blueprint_tree
from .blueprint.random import generate_blueprint_random


if __name__ == "__main__":
    colors_generated_js()
    colors_wgsl()
    kind_wgsl()
    code_wgsl()
    generate_blueprint_random(200)
    blueprint()
    # generate_blueprint_tree()
