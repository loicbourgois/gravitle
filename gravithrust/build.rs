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
struct KindDefinition {
    pub kinds: Vec<String>,
    pub static_kinds: Vec<String>,
    pub capacities: HashMap<String, Capacities>,
}
#[derive(Serialize, Deserialize)]
struct Capacities {
    hard: u32,
    soft: u32,
}
fn kind_generated_js(kd: &KindDefinition) -> Result<(), std::io::Error> {
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
fn main() -> Result<(), std::io::Error> {
    let envs = env::vars().collect::<HashMap<String, String>>();
    let kd: KindDefinition = serde_json::from_str(
        &fs::read_to_string(format!(
            "{}/github.com/loicbourgois/gravitle/gravithrust/src/kind.json",
            envs["HOME"]
        ))
        .expect("Should have been able to read the file"),
    )
    .unwrap();
    let mut kind_generated_rs = fs::read_to_string(format!(
        "{}/github.com/loicbourgois/gravitle/gravithrust/src/kind_generated.rs.template",
        envs["HOME"]
    ))
    .expect("Should have been able to read the file");
    let mut aa = String::new();
    let mut bb = String::new();
    for (i, kind_snake) in kd.kinds.iter().enumerate() {
        let kind = kind_snake.from_case(Case::Snake).to_case(Case::UpperCamel);
        aa.push_str(&format!("    {kind} = {i},"));
        bb.push_str(&format!("    \"{kind_snake}\" => Kind::{kind},"));
    }
    kind_generated_rs = kind_generated_rs.replace("__Kind__", &aa);
    kind_generated_rs = kind_generated_rs.replace("__kindstr_to_kind__", &bb);
    kind_generated_rs = kind_generated_rs.replace(
        "__is_static__",
        &kd.static_kinds
            .iter()
            .map(|x| {
                format!(
                    "Kind::{}",
                    x.from_case(Case::Snake).to_case(Case::UpperCamel)
                )
            })
            .collect::<Vec<String>>()
            .join("|"),
    );
    kind_generated_rs = kind_generated_rs.replace(
        "__soft_capacity__",
        &kd.capacities
            .iter()
            .filter(|(_k, v)| v.soft != 0)
            .map(|(k, v)| {
                format!(
                    "Kind::{} => {},",
                    k.from_case(Case::Snake).to_case(Case::UpperCamel),
                    v.soft,
                )
            })
            .collect::<String>(),
    );
    kind_generated_rs = kind_generated_rs.replace(
        "__hard_capacity__",
        &kd.capacities
            .iter()
            .filter(|(_k, v)| v.hard != 0)
            .map(|(k, v)| {
                format!(
                    "Kind::{} => {},",
                    k.from_case(Case::Snake).to_case(Case::UpperCamel),
                    v.hard,
                )
            })
            .collect::<String>(),
    );
    writeln!(
        File::create(format!(
            "{}/github.com/loicbourgois/gravitle/gravithrust/src/kind_generated.rs",
            envs["HOME"]
        ))?,
        "{kind_generated_rs}"
    )?;
    kind_generated_js(&kd)?;
    Ok(())
}
