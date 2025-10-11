use crate::blueprint::RawBlueprint;
use crate::blueprint::load_raw_blueprint;
use crate::gravithrust::Gravithrust;
use crate::particle::Particle;
use crate::ship::Ship;
use anyhow::Result;
use glob::glob;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::mem;
#[test]
fn test_parse_blueprint() {
    let envs = env::vars().collect::<HashMap<String, String>>();
    let path = format!(
        "{}/github.com/loicbourgois/gravitle/resources/blueprint/*.yml",
        envs["HOME"],
    );
    for e in glob(&path).expect("Failed to read glob pattern") {
        let pathbuf = e.unwrap();
        let path = pathbuf.to_str().unwrap();
        println!("testing {}", path);
        match test_parse_blueprint_by_path(path) {
            Ok(_bl) => {}
            Err(err) => {
                println!("error with {}", path);
                panic!("{}", err);
            }
        }
    }
}
fn test_parse_blueprint_by_path(path: &str) -> Result<()> {
    let yaml = fs::read_to_string(path)?;
    let raw_blueprint: RawBlueprint = serde_yaml::from_str(&yaml)?;
    let _blueprint = load_raw_blueprint(&raw_blueprint, 0.005);
    Ok(())
}
#[test]
fn size() {
    println!("size_of(Ship):      {}", mem::size_of::<Ship>());
    println!("ship_size_internal: {}", Gravithrust::ship_size_internal());
    assert!(mem::size_of::<Ship>() == Gravithrust::ship_size_internal());
    println!("size_of(Particle):      {}", mem::size_of::<Particle>());
    println!("particle_size_internal: {}", Gravithrust::particle_size_internal());
    assert!(mem::size_of::<Particle>() == Gravithrust::particle_size_internal() + 8);
}
