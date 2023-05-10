use convert_case::Case;
use convert_case::Casing;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
pub fn build_alchemy_mermaid() -> Result<(), std::io::Error> {
    let envs = env::vars().collect::<HashMap<String, String>>();
    let in_ = fs::read_to_string(format!(
        "{}/github.com/loicbourgois/gravitle/alchemy.txt",
        envs["HOME"]
    ))
    .expect("Should have been able to read the file");
    writeln!(
        File::create(format!(
            "{}/github.com/loicbourgois/gravitle/alchemy.mmd",
            envs["HOME"]
        ))?,
        "flowchart TB\n  {}",
        in_.split('\n')
            .collect::<Vec<_>>()
            .iter()
            .filter(|k| k.split(' ').collect::<Vec<_>>().len() >= 4)
            .map(|k| {
                let words = k.split(' ').collect::<Vec<_>>();
                let function = words[0];
                let qk = words[1];
                let k1 = words[2];
                let k2 = words[3];
                if words[0] == "harvest" {
                    let k3 = words[4];
                    vec![
                        format!("{k1} -.-> {function}{k1}{k2}({function} {qk})"),
                        format!("{k2} -.-> {function}{k1}{k2}"),
                        format!("  {function}{k1}{k2} -.-> {k3}"),
                    ]
                    .join("\n")
                } else if words[0] == "transfer" {
                    format!("{k1} -.->|{qk}| {k2}")
                } else {
                    format!("{k1} -.->|{function} {qk}| {k2}")
                }
            })
            .collect::<Vec<_>>()
            .join("\n  "),
    )
}
pub fn build_alchemy_rs() -> Result<(), std::io::Error> {
    let envs = env::vars().collect::<HashMap<String, String>>();
    let in_ = fs::read_to_string(format!(
        "{}/github.com/loicbourgois/gravitle/alchemy.txt",
        envs["HOME"]
    ))?;
    writeln!(
        File::create(format!(
            "{}/github.com/loicbourgois/gravitle/gravithrust/src/alchemy_generated.rs",
            envs["HOME"]
        ))?,
        "{}",
        fs::read_to_string(format!(
            "{}/github.com/loicbourgois/gravitle/gravithrust/template/alchemy_generated.rs",
            envs["HOME"]
        ))
        .expect("Should have been able to read the file")
        .replace(
            "//__ALCHEMY__//",
            &in_.split('\n')
                .collect::<Vec<_>>()
                .iter()
                .filter(|k| k.split(' ').collect::<Vec<_>>().len() >= 4)
                .map(|k| {
                    let words = k.split(' ').collect::<Vec<_>>();
                    let function_name = match words[0] {
                        "transfer" => "transfer_from_to".to_owned(),
                        "drop" => "transfer_from_to".to_owned(),
                        "collect" => "transfer_from_to".to_owned(),
                        "transfer_and_delete" => "transfer_and_delete".to_owned(),
                        "harvest" => "harvest".to_owned(),
                        _ => format!("/* {} */ todo", words[0]),
                    };
                    let new_kind = match words[0] {
                        "harvest" => format!(
                            ", Kind::{}",
                            words[4].from_case(Case::Snake).to_case(Case::UpperCamel)
                        ),
                        _ => String::new(),
                    };
                    match words[0] {
                        "create" => String::new(),
                        _ => vec![
                            format!("// {k}"),
                            format!(
                                "(Kind::{}, _, Kind::{}, _) => {{",
                                words[2].from_case(Case::Snake).to_case(Case::UpperCamel),
                                words[3].from_case(Case::Snake).to_case(Case::UpperCamel)
                            ),
                            format!("  {function_name}(p1, p2, pi1, pi2 {new_kind});"),
                            format!("}}"),
                        ]
                        .join("\n"),
                    }
                })
                .collect::<Vec<_>>()
                .join("\n  ")
        )
    )
}
