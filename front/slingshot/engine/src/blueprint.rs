use crate::point::Point;

impl Blueprint {
    #[must_use]
    pub fn new(material_str: &str) -> Blueprint {
        let kinds: HashMap<String, String> = material_str
            .lines()
            .filter(|line| line.trim_start().starts_with('#'))
            .filter_map(|line| {
                line.split_once(':').map(|(key, value)| {
                    (
                        key.replace('#', "").trim().to_string(), // sanitize key
                        value.trim().to_string(),                // sanitize value
                    )
                })
            })
            .collect();
        let filtered: String = material_str
            .lines()
            .filter(|line| !line.trim_start().starts_with('#'))
            .collect::<Vec<_>>()
            .join("\n");
        txt_to_blueprint(&(filtered + "    "), &kinds)
    }
    #[must_use]
    pub fn to_js(&self) -> String {
        let ship_json = serde_json::to_string_pretty(self).unwrap();
        format!(
            "// Generated from front/slingshot/engine/src/blueprint.rs
const ship = {ship_json};
export {{ ship }};
"
        )
    }
}

use serde::Serialize;
use std::collections::HashMap;

pub struct Cell {
    kind: String,
    xy: [usize; 2],
    idx: usize,
    x: f32,
    y: f32,
}

pub struct Link {
    kind: String,
    xy: [usize; 2],
}

#[derive(Serialize)]
pub struct LinkDetailed {
    a: usize,
    b: usize,
}

#[derive(Serialize)]
// #[wasm_bindgen]
pub struct Part {
    pub p: Point,
    pub d: f32,
    kind: String,
    pub material_url: String,
}

#[derive(Serialize)]
// #[wasm_bindgen]
pub struct Blueprint {
    diam: f32,
    center: Point,
    pub parts: Vec<Part>,
    links: Vec<LinkDetailed>,
}

fn txt_to_blueprint(material_str: &str, kinds: &HashMap<String, String>) -> Blueprint {
    let s = material_str;
    let mut col = 0;
    let mut row = 0;
    let mut cells: Vec<Cell> = Vec::new();
    let mut links: Vec<Link> = Vec::new();
    let mut items: HashMap<usize, HashMap<usize, usize>> = HashMap::new();
    let mut links_detailed: Vec<LinkDetailed> = Vec::new();
    let y_ratio = (3.0_f32).sqrt() / 2.0;
    let diam = 0.01;
    for i in 0..s.len() - 4 {
        let c = &s[i..=i];
        let c2 = &s[i..i + 2];
        let c4 = &s[i..i + 4];
        if kinds.contains_key(&c2.to_string()) {
            cells.push(Cell {
                kind: c2.to_string(),
                xy: [col, row],
                idx: cells.len(),
                x: (col as f32) / 6.0 * diam,
                y: -(row as f32) * diam * y_ratio / 2.0,
            });
        } else if c == "\\" {
            links.push(Link {
                kind: "\\".to_string(),
                xy: [col, row],
            });
        } else if c == "/" {
            links.push(Link {
                kind: "/".to_string(),
                xy: [col, row],
            });
        } else if c4 == "----" {
            links.push(Link {
                kind: "----".to_string(),
                xy: [col, row],
            });
        }
        if c == "\n" {
            col = 0;
            row += 1;
        } else {
            col += 1;
        }
    }
    for cell in &cells {
        let x = cell.xy[0];
        let y = cell.xy[1];
        if items.get(&x).is_none() {
            items.insert(x, HashMap::new());
        }
        items.get_mut(&x).unwrap().insert(y, cell.idx);
    }
    for link in links {
        let x = link.xy[0];
        let y = link.xy[1];
        let k = link.kind;
        let a;
        let b;
        if k == "/" {
            a = [x + 1, y - 1];
            b = [x - 2, y + 1];
        } else if k == "\\" {
            a = [x - 2, y - 1];
            b = [x + 1, y + 1];
        } else if k == "----" {
            a = [x - 2, y];
            b = [x + 4, y];
        } else {
            panic!("invalid k")
        }
        let aci = items[&a[0]][&a[1]];
        let bci = items[&b[0]][&b[1]];
        links_detailed.push(LinkDetailed { a: aci, b: bci });
    }
    let mut center = Point { x: 0.0, y: 0.0 };
    for c in &cells {
        center.x += c.x;
        center.y += c.y;
    }
    center.x /= cells.len() as f32;
    center.y /= cells.len() as f32;

    for c in &mut cells {
        c.x -= center.x;
        c.y -= center.y;
    }
    let mut center = Point { x: 0.0, y: 0.0 };
    for c in &cells {
        center.x += c.x;
        center.y += c.y;
    }
    center.x /= cells.len() as f32;
    center.y /= cells.len() as f32;
    Blueprint {
        center,
        links: links_detailed,
        diam,
        parts: cells
            .iter()
            .map(|c| Part {
                p: Point { x: c.x, y: c.y },
                d: diam,
                kind: c.kind.clone(),
                material_url: kinds[&c.kind.clone()].clone(),
            })
            .collect(),
    }
}
