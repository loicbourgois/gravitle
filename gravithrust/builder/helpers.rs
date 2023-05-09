use crate::src::math_small::rotate;
use crate::src::math_small::Vector;
use anyhow::Result;
use convert_case::Case;
use convert_case::Casing;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
#[derive(Serialize, Deserialize)]
pub struct KindDefinition {
    pub kinds: Vec<String>,
    pub static_kinds: Vec<String>,
    pub capacities: HashMap<String, Capacities>,
}
#[derive(Serialize, Deserialize)]
pub struct Capacities {
    pub hard: u32,
    pub soft: u32,
}
pub fn kind_generated_js(kd: &KindDefinition) -> Result<(), std::io::Error> {
    let envs = env::vars().collect::<HashMap<String, String>>();
    writeln!(
        File::create(format!(
            "{}/github.com/loicbourgois/gravitle/front/kind_generated.js",
            envs["HOME"]
        ))?,
        "{}",
        fs::read_to_string(format!(
            "{}/github.com/loicbourgois/gravitle/front/kind_generated.js.template",
            envs["HOME"]
        ))
        .expect("Should have been able to read the file")
        .replace(
            "__KIND__",
            &kd.kinds
                .iter()
                .enumerate()
                .map(|(i, k)| {
                    format!(
                        "{}: {i},",
                        k.from_case(Case::Snake).to_case(Case::UpperCamel),
                    )
                })
                .collect::<Vec<_>>()
                .join("\n  "),
        )
        .replace(
            "__KINDS__",
            &kd.kinds
                .iter()
                .map(|k| format!("{{label:'{k}'}},",))
                .collect::<Vec<_>>()
                .join("\n  "),
        )
    )
}
pub fn kind_generated_wgsl(kd: &KindDefinition) -> Result<(), std::io::Error> {
    let envs = env::vars().collect::<HashMap<String, String>>();
    writeln!(
        File::create(format!(
            "{}/github.com/loicbourgois/gravitle/front/webgpu/kind_generated.wgsl",
            envs["HOME"]
        ))?,
        "{}",
        fs::read_to_string(format!(
            "{}/github.com/loicbourgois/gravitle/front/webgpu/kind_generated.wgsl.template",
            envs["HOME"]
        ))
        .expect("Should have been able to read the file")
        .replace(
            "__KINDS__",
            &kd.kinds
                .iter()
                .enumerate()
                .map(|(i, k)| format!("const KIND_{k} = {i};"))
                .collect::<Vec<_>>()
                .join("\n"),
        )
    )
}
pub fn disk_generated() -> Result<(), std::io::Error> {
    let envs = env::vars().collect::<HashMap<String, String>>();
    let triangles = 16;
    let triangles_f: f32 = triangles as f32;
    let mut vectors = vec![];
    let a = Vector {
        x: 0.0,
        y: 0.0,
    };
    let b = Vector {
        x: 1.0,
        y: 0.0,
    };
    for x in 0..triangles {
        let x_f: f32 = x as f32;
        let x2_f: f32 = (x + 1) as f32;
        vectors.push(a);
        vectors.push(rotate(a, b, 1.0 * x_f / triangles_f));
        vectors.push(rotate(a, b, 1.0 * x2_f / triangles_f));
    }
    writeln!(
        File::create(format!(
            "{}/github.com/loicbourgois/gravitle/front/webgpu/disk_generated.wgsl",
            envs["HOME"]
        ))?,
        "{}",
        fs::read_to_string(format!(
            "{}/github.com/loicbourgois/gravitle/front/webgpu/disk_generated.wgsl.template",
            envs["HOME"]
        ))
        .expect("Should have been able to read the file")
        .replace("__POSITIONS_COUNT__", &format!("{}", vectors.len()),)
        .replace(
            "__POSITIONS__",
            &vectors
                .iter()
                .map(|v| format!("vec2f( {},  {}),", v.x, v.y))
                .collect::<Vec<_>>()
                .join("\n    "),
        )
    )
}
// #[derive(Copy, Clone)]
// pub struct Vector {
//     x: f32,
//     y: f32,
// }
// pub fn rotate(p1: Vector, p2: Vector, angle: f32) -> Vector {
//     // Rotates p2 around p1
//     // angle should be in [0 ; 1.0]
//     let angle = std::f32::consts::PI * 2.0 * angle;
//     let dx = p2.x - p1.x;
//     let dy = p2.y - p1.y;
//     let cos_ = angle.cos();
//     let sin_ = angle.sin();
//     Vector {
//         x: p1.x + dx * cos_ - dy * sin_,
//         y: p1.y + dy * cos_ + dx * sin_,
//     }
// }
