use crate::src::math_small::rotate;
use crate::src::math_small::Vector;
use anyhow::Result;
use convert_case::Case;
use convert_case::Casing;
use glob::glob;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
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
            "{}/github.com/loicbourgois/gravitle/resources/template/kind_generated.js",
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
            "{}/github.com/loicbourgois/gravitle/resources/template/kind_generated.wgsl",
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
            "{}/github.com/loicbourgois/gravitle/resources/template/disk_generated.wgsl",
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
// pub fn code_wgsl() -> Result<(), std::io::Error> {
// let envs = env::vars().collect::<HashMap<String, String>>();
// writeln!(
//     File::create(format!(
//         "{}/github.com/loicbourgois/gravitle/front/webgpu/code_generated.wgsl",
//         envs["HOME"]
//     ))?,
//     "{}",
//     fs::read_to_string(format!(
//         "{}/github.com/loicbourgois/gravitle/resources/template/code_generated.wgsl",
//         envs["HOME"]
//     ))
//     .expect("Should have been able to read the file")
//     .replace(
//         "//__DISK_GENERATED__//",
//         &fs::read_to_string(format!(
//             "{}/github.com/loicbourgois/gravitle/resources/templated/disk_generated.wgsl",
//             envs["HOME"]
//         ))
//         .expect("Should have been able to read the file")
//     )
//     .replace(
//         "//__KIND_GENERATED__//",
//         &fs::read_to_string(format!(
//             "{}/github.com/loicbourgois/gravitle/resources/templated/kind_generated.wgsl",
//             envs["HOME"]
//         ))
//         .expect("Should have been able to read the file")
//     )
// )
// }
pub fn resources_generated() -> Result<(), std::io::Error> {
    let envs = env::vars().collect::<HashMap<String, String>>();
    let job_names = glob(&format!(
        "{}/github.com/loicbourgois/gravitle/resources/job/*.json",
        envs["HOME"],
    ))
    .expect("Failed to read glob pattern")
    .map(|x| {
        let pathbuf = x.unwrap();
        let path = pathbuf.to_str().unwrap();
        let name = Path::new(&path).file_stem().unwrap();
        format!("'{}'", name.to_str().unwrap().to_owned())
    })
    .collect::<Vec<String>>()
    .join(",\n  ");
    let blueprint_names = glob(&format!(
        "{}/github.com/loicbourgois/gravitle/resources/blueprint/*.yml",
        envs["HOME"],
    ))
    .expect("Failed to read glob pattern")
    .map(|x| {
        let pathbuf = x.unwrap();
        let path = pathbuf.to_str().unwrap();
        let name = Path::new(&path).file_stem().unwrap();
        format!("'{}'", name.to_str().unwrap().to_owned())
    })
    .collect::<Vec<String>>()
    .join(",\n  ");
    writeln!(
        File::create(format!(
            "{}/github.com/loicbourgois/gravitle/front/resources_generated.js",
            envs["HOME"]
        ))?,
        "{}",
        fs::read_to_string(format!(
            "{}/github.com/loicbourgois/gravitle/resources/template/resources_generated.js",
            envs["HOME"]
        ))
        .expect("Should have been able to read the file")
        .replace("//__job_names__//", &job_names)
        .replace("//__blueprint_names__//", &blueprint_names)
    )
}
