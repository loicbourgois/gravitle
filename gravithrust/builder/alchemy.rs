use convert_case::Case;
use convert_case::Casing;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
pub fn build_alchemy_mermaid() -> Result<(), std::io::Error> {
    let envs = env::vars().collect::<HashMap<String, String>>();
    let in_ = fs::read_to_string(format!(
        "{}/github.com/loicbourgois/gravitle/gravithrust/alchemy.txt",
        envs["HOME"]
    ))
    .expect("Should have been able to read the file");
    writeln!(
        File::create(format!(
            "{}/github.com/loicbourgois/gravitle/gravithrust/alchemy.mmd",
            envs["HOME"]
        ))?,
        "flowchart LR\n  {}",
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
                    [
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
pub fn alchemy_transfer(in_: &str, qks_by_k: &mut HashMap<String, HashSet<String>>) -> String {
    in_.split('\n')
        .collect::<Vec<_>>()
        .iter()
        .filter(|k| k.split(' ').collect::<Vec<_>>().len() >= 4)
        .filter(|k| k.split(' ').collect::<Vec<_>>()[0] == "transfer")
        .map(|k| {
            let words = k.split(' ').collect::<Vec<_>>();
            let function_name = match words[0] {
                "transfer" => "transfer_from_to".to_owned(),
                _ => format!("/* {} */ todo", words[0]),
            };
            let k1 = words[2].from_case(Case::Snake).to_case(Case::UpperCamel);
            let k2 = words[3].from_case(Case::Snake).to_case(Case::UpperCamel);
            let qk = words[1].from_case(Case::Snake).to_case(Case::UpperCamel);
            qks_by_k.entry(k1.clone()).or_default().insert(qk.clone());
            qks_by_k.entry(k2.clone()).or_default().insert(qk.clone());
            [
                format!("// {k}"),
                format!("(Kind::{k1}, Kind::{k2}, QuantityKind::{qk}) => {{",),
                format!("  {function_name}(p1, p2, pi1, pi2, QuantityKind::{qk});"),
                format!("}}"),
            ]
            .join("\n")
        })
        .collect::<Vec<_>>()
        .join("\n  ")
}
pub fn alchemy_transform(in_: &str) -> String {
    in_.split('\n')
        .collect::<Vec<_>>()
        .iter()
        .filter(|line| line.split(' ').collect::<Vec<_>>().len() >= 2)
        .filter(|line| line.split(' ').collect::<Vec<_>>()[0] == "transform")
        .map(|line| {
            let words = line.split(' ').collect::<Vec<_>>();
            let words2 = words[1].split('=').collect::<Vec<_>>();
            let k1 = words2[0].from_case(Case::Snake).to_case(Case::UpperCamel);
            let words3 = words2[1].split("->").collect::<Vec<_>>();
            let inputs = words3[0].split('+').collect::<Vec<_>>();
            let outputs = words3[1].split('+').collect::<Vec<_>>();
            let mut conditions: Vec<_> = inputs
                .iter()
                .filter(|x| !x.is_empty())
                .map(|x| {
                    let words_ = x.split('*').collect::<Vec<_>>();
                    let qk = words_[1].from_case(Case::Snake).to_case(Case::UpperCamel);
                    let quantity = words_[0];
                    format!("p1.quantity(QuantityKind::{qk}) >= {quantity}")
                })
                .collect();
            let mut conditions_2: Vec<_> = outputs
                .iter()
                .filter(|x| !x.is_empty())
                .map(|x| {
                    let words_ = x.split('*').collect::<Vec<_>>();
                    let qk = words_[1].from_case(Case::Snake).to_case(Case::UpperCamel);
                    let quantity = words_[0];
                    format!("p1.remaining_capacity(QuantityKind::{qk}) >= {quantity}")
                })
                .collect();
            conditions.append(&mut conditions_2);
            let mut actions: Vec<_> = inputs
                .iter()
                .filter(|x| !x.is_empty())
                .map(|x| {
                    let words_ = x.split('*').collect::<Vec<_>>();
                    let qk = words_[1].from_case(Case::Snake).to_case(Case::UpperCamel);
                    let quantity = words_[0];
                    format!("p1.remove_quantity(QuantityKind::{qk}, {quantity});")
                })
                .collect();
            let mut actions_2: Vec<_> = outputs
                .iter()
                .filter(|x| !x.is_empty())
                .map(|x| {
                    let words_ = x.split('*').collect::<Vec<_>>();
                    let qk = words_[1].from_case(Case::Snake).to_case(Case::UpperCamel);
                    let quantity = words_[0];
                    format!("p1.add_quantity(QuantityKind::{qk}, {quantity});")
                })
                .collect();
            actions.append(&mut actions_2);
            let condition_str = conditions.join("&&");
            let action_str = actions.join("\n");
            [
                format!("// {line}"),
                format!("Kind::{k1} => {{"),
                format!("  if {condition_str} {{"),
                format!("    {action_str}"),
                format!("    pi1.new_state = Some(State {{live: p1.live}});"),
                format!("  }}"),
                format!("}},"),
            ]
            .join("\n")
        })
        .collect::<Vec<_>>()
        .join("\n  ")
}
pub fn build_alchemy_rs(kinds: &[&String]) -> Result<(), std::io::Error> {
    let envs = env::vars().collect::<HashMap<String, String>>();
    let in_ = fs::read_to_string(format!(
        "{}/github.com/loicbourgois/gravitle/gravithrust/alchemy.txt",
        envs["HOME"]
    ))?;
    let mut qks_by_k: HashMap<String, HashSet<String>> = HashMap::new();
    writeln!(
        File::create(format!(
            "{}/github.com/loicbourgois/gravitle/gravithrust/src/alchemy_generated.rs",
            envs["HOME"]
        ))?,
        "{}",
        fs::read_to_string(format!(
            "{}/github.com/loicbourgois/gravitle/resources/template/alchemy_generated.rs",
            envs["HOME"]
        ))
        .expect("Should have been able to read the file")
        .replace(
            "//__ALCHEMY_TRANSFER__//",
            &alchemy_transfer(&in_, &mut qks_by_k)
        )
        .replace("//__ALCHEMY_TRANSFORM__//", &alchemy_transform(&in_))
        .replace(
            "//__QKS__//",
            &kinds
                .iter()
                .map(|k| {
                    let kind = k.from_case(Case::Snake).to_case(Case::UpperCamel);
                    let qks = qks_by_k
                        .entry(kind.clone())
                        .or_default()
                        .iter()
                        .map(|x| {
                            let aa = x;
                            format!("QuantityKind::{aa}")
                        })
                        .collect::<Vec<_>>()
                        .join(",");
                    format!("Kind::{kind} => &[{qks}],")
                })
                .collect::<Vec<_>>()
                .join("\n")
        )
    )
}
