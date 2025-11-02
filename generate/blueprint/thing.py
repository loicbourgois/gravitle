import random

kinds = {
    "wood": "ww",
    "green": "gg",
    "armor": "aa",
    "booster_s": "bs",
    "booster_d": "bd",
    "core": "cc",
}


class Cell:
    def __init__(self, kind, x, y):
        self.k = kind
        self.x = x
        self.y = y


class Link:
    def __init__(self, a, b, str_, idx):
        self.a = a
        self.b = b
        self.s = str_
        self.idx = idx


class Thing:
    def __init__(self, first_kind):
        self.cells = []
        self.cells_2 = {}
        self.links = []
        self.links_2 = {}
        self.rand_links = {}
        self.add_cell(first_kind, 0, 0)


    def add_link(self, ia, ib, s):
        idx = len(self.links)
        self.links.append(Link(ia, ib, s, idx))
        self.links_2[f"{ia}|{ib}"] = idx


    def add_cell(self, k, x, y):
        assert self.cells_2.get(f"{x}|{y}") is None, self.cells_2
        idx = len(self.cells)
        self.cells.append(Cell(kinds[k], x, y))
        self.cells_2[f"{x}|{y}"] = idx


    def to_txt(self, mode):
        min_x = 0
        max_x = 0
        min_y = 0
        max_y = 0
        for c in self.cells:
            min_x = min(min_x, c.x)
            max_x = max(max_x, c.x)
            min_y = min(min_y, c.y)
            max_y = max(max_y, c.y)
        self.w = max_x - min_x + 2
        self.h = max_y - min_y + 1 
        lines = [[" " for _ in range(self.w)] for _ in range(self.h)]
        for i, c in enumerate(self.cells):
            try:
                if mode == "kind":
                    lines[c.y - min_y][c.x - min_x] = c.k[0]
                    lines[c.y - min_y][c.x - min_x + 1] = c.k[1]
                elif  mode == "idx":
                    idx_str = f"{i:02d}"
                    lines[c.y - min_y][c.x - min_x] = idx_str[0]
                    lines[c.y - min_y][c.x - min_x + 1] = idx_str[1]
                else: 
                    raise Exception("no implemented")
            except:
                print(f"{c.x} | {min_x} | {self.w} | ")
                raise
        for l in self.links:
            try:
                c = self.cells[l.a]
            except:
                print(f"{l.a} | {len(self.cells)}")
                raise
            for i, char in enumerate(l.s):
                if l.s == "----":
                    lines[c.y - min_y][c.x - min_x +i+2] = char
                elif l.s == "/":
                    lines[c.y - min_y - 1][c.x - min_x +i+2] = char
                elif l.s == "\\":
                    lines[c.y - min_y + 1][c.x - min_x +i+2] = char
                else:
                    raise Exception("not implemented")
        return "\n".join(["", ""]+["    " +"".join(line) for line in lines] + ["", ""])


    def add_right(self, idx, k):
        c0 = self.cells[idx]
        x = c0.x + 6
        y = c0.y
        self.add_cell(k, x, y)
        self.add_link(idx, len(self.cells)-1, "----")
        
    
    def add_left(self, idx, k):
        c0 = self.cells[idx]
        x = c0.x - 6
        y = c0.y
        self.add_cell(k, x, y)
        self.add_link(len(self.cells)-1, idx, "----")


    def add(self, ia, ib, k):
        ca = self.cells[ia]
        cb = self.cells[ib]
        if self.links_2.get(f"{ia}|{ib}") is not None:
            l = self.links[self.links_2[f"{ia}|{ib}"]]
            reverse = False
        elif self.links_2.get(f"{ib}|{ia}") is not None:
            l = self.links[self.links_2[f"{ib}|{ia}"]]
            reverse = True
        else:
            raise Exception(f"not found: {ia}|{ib}")
        if reverse:
            if l.s == "----":
                x = cb.x + 3
                y = cb.y + 2
                ic = len(self.cells)
                self.add_cell(k, x, y)
                self.add_link(ic, ia, "/")
                self.add_link(ib, ic, "\\")
            elif l.s == "\\":
                x = ca.x - 6
                y = ca.y
                ic = len(self.cells)
                self.add_cell(k, x, y)
                self.add_link(ic, ia, "----")
                self.add_link(ic, ib, "/")
            elif l.s == "/":
                x = ca.x - 6
                y = ca.y
                ic = len(self.cells)
                self.add_cell(k, x, y)
                self.add_link(ic, ia, "----")
                self.add_link(ic, ib, "\\")
            else:
                raise Exception(f"not implemented: add({l.s})")
        else:
            if l.s == "----":
                x = ca.x + 3
                y = ca.y - 2
                ic = len(self.cells)
                self.add_cell(k, x, y)
                self.add_link(ia, ic, "/")
                self.add_link(ic, ib, "\\")
            elif l.s == "\\":
                x = ca.x + 6
                y = ca.y
                ic = len(self.cells)
                self.add_cell(k, x, y)
                self.add_link(ia, ic, "----")
                self.add_link(ib, ic, "/")
            elif l.s == "/":
                x = ca.x + 6
                y = ca.y
                ic = len(self.cells)
                self.add_cell(k, x, y)
                self.add_link(ia, ic, "----")
                self.add_link(ib, ic, "\\")
            else:
                raise Exception(f"not implemented: add({l.s})")

    def add_random(self, k):
        for _ in range(100):
            for _ in range(100):
                l = random.choice(self.links)
                ia, ib = random.choice([(l.a, l.b), (l.b, l.a)])
                if self.rand_links.get(f"{ia}|{ib}") is None:
                    break
            self.rand_links[f"{ia}|{ib}"] = True
            try:
                self.add(ia, ib, k)
                break
            except Exception:
                pass
