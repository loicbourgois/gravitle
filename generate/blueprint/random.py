from .thing import Thing
from ..misc import write_force
from ..env import HOME
import random


def random_kind():
    return random.choice(["armor", "armor", "booster_s", "booster_d"])


def generate_blueprint_random(c=3):
    t = Thing("core")
    t.add_right(0, random_kind())
    t.add(0, 1, random_kind())
    for _ in range(c - 3):
        t.add_random(random_kind())
    write_force(
        f"{HOME}/github.com/loicbourgois/gravitle/blueprint/random.txt",
        t.to_txt(mode="kind"),
    )
    # print(t.to_txt(mode="kind"))
    # print(t.to_txt(mode="idx"))
