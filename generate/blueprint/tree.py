from .thing import Thing


def generate_blueprint_tree():
    tree = Thing("wood")
    tree.add_right(0, "wood")
    tree.add_right(1, "wood")
    tree.add_left(0, "green")
    tree.add(0, 1, "green")
    tree.add(1, 2, "green")
    tree.add(5, 2, "green")
    tree.add(2, 1, "green")
    tree.add(2, 6, "green")
    tree.add(4, 0, "green")
    tree.add(1, 0, "green")
    tree.add(10, 0, "green")
    tree.add(7, 2, "green")
    tree.add_link(12, 8, "/")
    tree.add_link(3, 9, "/")
    tree.add_link(3, 11, "\\")
    print(tree.to_txt(mode="kind"))
    print(tree.to_txt(mode="idx"))
