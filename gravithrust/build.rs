use anyhow::Result;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Write;
fn main() -> Result<(), std::io::Error> {
    let kinds = [
        "Default",
        "Armor",
        "Core",
        "Booster",
        "Sun",
        "Light",
        "Plant",
        "Metal",
        "Depot",
        "Target",
        "Ray",
        "Cargo",
        "Plasma",
        "Field",
        "Anchor",
        "SunCore",
        "ElectroField",
        "PlasmaElectroField",
        "PlasmaCargo",
        "PlasmaCollector",
        "PlasmaDepot",
        "PlasmaRefinery",
        "Static",
    ];
    let envs = env::vars().collect::<HashMap<String, String>>();
    let path = format!(
        "{}/github.com/loicbourgois/gravitle/gravithrust/src/kind_generated.rs",
        envs["HOME"]
    );
    let path_2 = format!(
        "{}/github.com/loicbourgois/gravitle/front/kind_generated.js",
        envs["HOME"]
    );
    let mut output = File::create(path)?;
    let mut output_2 = File::create(path_2)?;
    for l in &[
        "// generated by build.rs",
        "use serde::Deserialize;",
        "use serde::Serialize;",
        "use wasm_bindgen::prelude::wasm_bindgen;",
        "#[wasm_bindgen]",
        "#[derive(Serialize, Deserialize, Hash, Copy, Clone, Debug, PartialEq, Eq)]",
        "#[repr(u32)]",
        "pub enum Kind {",
    ] {
        writeln!(output, "{}", l)?;
    }
    writeln!(output_2, "// generated by build.rs")?;
    writeln!(output_2, "const Kind = {{")?;
    for (i, kind) in kinds.iter().enumerate() {
        writeln!(output, "    {} = {},", kind, i + 1)?;
        writeln!(output_2, "    {}: {},", kind, i + 1)?;
    }
    writeln!(output, "}}")?;
    for l in &["}", "export {", "    Kind,", "}"] {
        writeln!(output_2, "{}", l)?;
    }
    Ok(())
}
