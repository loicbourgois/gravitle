import os
import math
from .misc import read, write_force
import json

HOME = os.environ['HOME']
DIAM = 0.0125
y_ratio = math.sqrt(3) / 2

def blueprint():
    print("blueprint")
    name = "ship"
    path = "{HOME}/github.com/loicbourgois/gravitle/blueprint/{name}.{extension}"
    s = read(path.format(HOME=HOME, name=name, extension="txt"))
    print(s)
    col = 0
    row = 0
    cells = []
    links = []
    items = {}
    links_detailed = []
    for i, c in enumerate(s):
        c2 = s[i:i+2]
        c4 = s[i:i+4]
        if c2 == "aa":
            cells.append({
                "kind": "armor",
                "xy": [col, row],
                "idx": len(cells),
                "x": col/6*DIAM,
                "y": -row*DIAM*y_ratio/2,
            })
        elif c == "b":
            cells.append({
                "kind": "booster",
                "xy": [col, row],
                "idx": len(cells),
                "x": col/6*DIAM,
                "y": -row*DIAM*y_ratio/2,
                "binding": s[i+1],
            })
        elif c2 == "cc":
            cells.append({
                "kind": "core",
                "xy": [col, row],
                "idx": len(cells),
                "x": col/6*DIAM,
                "y": -row*DIAM*y_ratio/2,
            })
        elif c == "\\":
            links.append({
                "kind": "\\",
                "xy": [col, row]
            })
        elif c == "/":
            links.append({
                "kind": "/",
                "xy": [col, row]
            })
        elif c4 == "----":
            links.append({
                "kind": "----",
                "xy": [col, row]
            })
        if c == "\n":
            col = 0
            row +=1
        else:
            col += 1
    for cell in cells:
        x = cell['xy'][0]
        y = cell['xy'][1]
        if not items.get(x):
            items[x] = {}
        items[x][y] = cell['idx']
    for link in links:
        x = link['xy'][0]
        y = link['xy'][1]
        k = link['kind']
        if k == "/":
            a = x+1,y-1
            b = x-2,y+1
        elif k == "\\":
            a = x-2,y-1
            b = x+1,y+1
        elif k == "----":
            a = x-2,y
            b = x+4,y
        else:
            raise Exception('invalid k')
        aci = items[a[0]][a[1]]
        bci = items[b[0]][b[1]]
        links_detailed.append({
            "a": aci, 
            "b": bci,
        })
    print(cells)
    print(links_detailed)
    center = {
        "x": 0,
        "y": 0,
    }
    for c in cells:
        center["x"] += c['x']
        center["y"] += c['y']
    center["x"] /= len(cells)
    center["y"] /= len(cells)

    for c in cells:
        c['x'] += 0.5 - center["x"]
        c['y'] += 0.5 - center["y"]
    
    center = {
        "x": 0,
        "y": 0,
    }
    for c in cells:
        center["x"] += c['x']
        center["y"] += c['y']
    center["x"] /= len(cells)
    center["y"] /= len(cells)
    

    str_ = "const ship = " + json.dumps({
            "DIAM": DIAM,
            "center": center,
            "parts": [
                {
                    "p": {
                        "x": c['x'],
                        "y": c['y'],
                    },
                    "d": DIAM,
                    "player_id": "0",
                    "kind": c['kind'],
                    "binding": c.get('binding'),
                } for c in cells
            ],
            "links": links_detailed,
        }, indent=2) + """;
export { ship };
"""
    write_force(path.format(HOME=HOME, name=name, extension="js"),str_)
    write_force(f"{HOME}/github.com/loicbourgois/gravitle/front/chrono/ship.js",str_)
