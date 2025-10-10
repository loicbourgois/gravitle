use anyhow::Result;
use convert_case::Case;
use convert_case::Casing;
use std::collections::HashMap;
use std::env;
mod builder;
mod src;
use crate::builder::alchemy::build_alchemy_mermaid;
use crate::builder::alchemy::build_alchemy_rs;
use crate::builder::helpers::disk_generated;
use crate::builder::helpers::kind_generated_js;
use crate::builder::helpers::kind_generated_wgsl;
use crate::builder::helpers::resources_generated;
use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::Write;
fn main() -> Result<(), std::io::Error> {
    let envs = env::vars().collect::<HashMap<String, String>>();
    let mut kind_generated_rs = fs::read_to_string(format!(
        "{}/github.com/loicbourgois/gravitle/resources/template/kind_generated.rs",
        envs["HOME"]
    ))
    .expect("Should have been able to read the file");
    let mut aa = String::new();
    let mut bb = String::new();
    let mut kinds_set: HashSet<String> = HashSet::new();
    let mut static_kind_set: HashSet<String> = HashSet::new();
    let in_ = fs::read_to_string(format!(
        "{}/github.com/loicbourgois/gravitle/gravithrust/alchemy.txt",
        envs["HOME"]
    ))
    .expect("Should have been able to read the file");
    in_.split('\n').collect::<Vec<_>>().iter().for_each(|k| {
        let words = k.split(' ').collect::<Vec<_>>();
        if words.len() == 4 {
            kinds_set.insert(words[2].to_owned());
            kinds_set.insert(words[3].to_owned());
        }
        if words.len() == 1 && words[0].len() > 1 {
            kinds_set.insert(words[0].to_owned());
        }
        if words.len() == 2 && words[0] == "static" {
            static_kind_set.insert(words[1].to_owned());
        }
    });
    let kinds: Vec<&String> = kinds_set.iter().collect::<Vec<_>>();
    let static_kinds: Vec<&String> = static_kind_set.iter().collect::<Vec<_>>();
    for (i, kind_snake) in kinds.iter().enumerate() {
        let kind = kind_snake.from_case(Case::Snake).to_case(Case::UpperCamel);
        aa.push_str(&format!("    {kind} = {i},"));
        bb.push_str(&format!("    \"{kind_snake}\" => Kind::{kind},"));
    }
    kind_generated_rs = kind_generated_rs.replace("__Kind__", &aa);
    kind_generated_rs = kind_generated_rs.replace("__kindstr_to_kind__", &bb);
    kind_generated_rs = kind_generated_rs.replace(
        "__is_static__",
        &static_kinds
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
    writeln!(
        File::create(format!(
            "{}/github.com/loicbourgois/gravitle/gravithrust/src/kind_generated.rs",
            envs["HOME"]
        ))?,
        "{kind_generated_rs}"
    )?;
    kind_generated_js(&kinds)?;
    disk_generated()?;
    kind_generated_wgsl(&kinds)?;
    build_alchemy_mermaid()?;
    build_alchemy_rs(&kinds)?;
    resources_generated()?;
    Ok(())
}
