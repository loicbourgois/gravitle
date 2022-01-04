mod utils;
use utils::{
    log
};
use rand;
// use rand::Rng;
use uuid::Uuid;
use std::ops;
// use getrandom;
use std::collections::{HashMap,HashSet};
use wasm_bindgen::prelude::wasm_bindgen;
const BASE_CAPACITY: usize = 2;
const DIAMETER:float = 0.01;
const MASS: float = 1.0;
type float = f32;
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
#[wasm_bindgen]
pub fn serve() {
    log("Hello, wasm!");
}
#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Point {
    x: float,
    y: float,
}
impl ops::Sub<&Point> for &Point {
    type Output = Point;
    fn sub(self, p2: &Point) -> Point {
        Point {
            x: self.x - p2.x,
            y: self.y - p2.y,
        }
    }
}
impl ops::Mul<&Point> for &Point {
    type Output = Point;
    fn mul(self, p2: &Point) -> Point {
        Point {
            x: self.x * p2.x,
            y: self.y * p2.y,
        }
    }
}
#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Part {
    p: Point,   // position
    pp: Point,  // previous position
    d: float,   // diameter
    m: float,   // mass
}
#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Blocks {
    w: usize,
    h: usize,
}
#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Block {
    x: usize,
    y: usize,
}
#[wasm_bindgen]
pub struct Server {
    blocks: Blocks,
    parts: HashMap<u128, Part>,
    parts_: Vec<Part>,
    pids: Vec<Vec<Vec<u128>>>,
    step: usize,
}
#[wasm_bindgen]
pub struct New {
    width: usize,
    height: usize,
}
#[wasm_bindgen]
pub struct AddPart {
    p: Point
}
#[wasm_bindgen]
impl Block {
    pub fn new(p: &Point, b: &Blocks) -> Block {
        Block {
            x: ((p.x * b.w as float) % b.w as float).floor() as usize,
            y: ((p.y * b.h as float) % b.h as float).floor() as usize,
        }
    }
}
#[wasm_bindgen]
impl Point {
    pub fn new(x: float, y: float) -> Point {
        Point {
            x: x,
            y: y
        }
    }
    // pub fn sub(&mut self, p2: &Point) {
    //     self.x -= p2.x;
    //     self.y -= p2.y;
    // }
}
#[wasm_bindgen]
impl Server {
    pub fn new(width: usize, height: usize) -> Server {
        Server {
            blocks: Blocks{
                w: width,
                h: height,
            },
            parts: HashMap::new(),
            parts_: Vec::new(),
            pids: vec![vec![Vec::with_capacity(BASE_CAPACITY); height]; width],
            step: 0,
        }
    }
    pub fn add_part(&mut self, p: &Point, v: &Point) {
        let block = Block::new(&p, &self.blocks);
        let pid: u128 = Uuid::new_v4().as_u128();
        self.parts.insert(
            pid,
            Part {
                p:  *p,
                pp: p - v,
                d:  DIAMETER,
                m:  MASS,
            },
        );
        self.pids[block.x][block.y].push(pid);
    }
    pub fn tick(&mut self) {
        let mut new_parts: HashMap<u128, Part>  = HashMap::new();
        let mut new_pids: Vec<Vec<Vec<u128>>> = vec![vec![Vec::with_capacity(BASE_CAPACITY); self.blocks.h]; self.blocks.w];
        for i in 0..self.blocks.w {
            for j in 0..self.blocks.h {
                for pid in self.pids[i][j].iter() {
                    let p1 = self.parts.get(pid).unwrap();
                    let v = Point {
                        x: p1.p.x - p1.pp.x,
                        y: p1.p.y - p1.pp.y,
                    };
                    let p = Point {
                        x: (p1.p.x + 1.0 + v.x).fract(),
                        y: (p1.p.y + 1.0 + v.y).fract(),
                    };
                    let block = Block::new(&p, &self.blocks);
                    let pp = &p - &v;
                    let delta = 0.0005;
                    let mx_speed = 0.001;
                    let mut pp_ = &pp * &Point {
                        x: (1.0 - delta*0.5 + rand::random::<float>() * delta),
                        y: (1.0 - delta*0.5 + rand::random::<float>() * delta),
                    };
                    pp_.x = pp_.x.max(p.x-mx_speed).min(p.x+mx_speed);
                    pp_.y = pp_.y.max(p.y-mx_speed).min(p.y+mx_speed);
                    new_parts.insert(*pid, Part{
                        p:  p,
                        pp: pp_,
                        d:  p1.d,
                        m:  p1.m,
                    });
                    new_pids[block.x][block.y].push(*pid);
                }
            }
        }
        self.pids = new_pids;
        self.parts = new_parts;
        self.parts_ = self.parts.values().cloned().collect::<Vec<Part>>();
        self.step += 1;
    }
    pub fn parts_count(&self) -> usize {
        self.parts_.len()
    }
    pub fn parts_ptr(&self) -> *const Part {
        self.parts_.as_ptr()
    }
    pub fn get_step(&self) -> usize{
        self.step
    }
}
