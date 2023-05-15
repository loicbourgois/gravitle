use anyhow::Result;
use convert_case::Case;
use convert_case::Casing;
use std::collections::HashMap;
use std::env;
mod builder;
mod src;
use crate::builder::alchemy::build_alchemy_mermaid;
use crate::builder::alchemy::build_alchemy_rs;
// use crate::builder::helpers::code_wgsl;
use crate::builder::helpers::disk_generated;
use crate::builder::helpers::kind_generated_js;
use crate::builder::helpers::kind_generated_wgsl;
use crate::builder::helpers::resources_generated;
use crate::builder::helpers::KindDefinition;
use std::fs;
use std::fs::File;
use std::io::Write;
fn main() -> Result<(), std::io::Error> {
    let envs = env::vars().collect::<HashMap<String, String>>();
    let kd: KindDefinition = serde_json::from_str(
        &fs::read_to_string(format!(
            "{}/github.com/loicbourgois/gravitle/kind.json",
            envs["HOME"]
        ))
        .expect("Should have been able to read the file"),
    )
    .unwrap();
    let mut kind_generated_rs = fs::read_to_string(format!(
        "{}/github.com/loicbourgois/gravitle/resources/template/kind_generated.rs",
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
    disk_generated()?;
    kind_generated_wgsl(&kd)?;
    build_alchemy_mermaid()?;
    build_alchemy_rs(&kd)?;
    resources_generated()?;
    // code_wgsl()?;
    Ok(())
}
