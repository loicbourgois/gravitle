use crate::blueprint::load_raw_blueprint;
use crate::blueprint::RawBlueprint;
use crate::gravithrust::Gravithrust;
use crate::particle::Particle;
use crate::ship::Ship;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::mem;
#[test]
fn test_parse_blueprint() {
    let envs = env::vars().collect::<HashMap<String, String>>();
    let path = format!(
        "{}/github.com/loicbourgois/gravitle/gravithrust/src/blueprint/blueprint_01.yml",
        envs["HOME"]
    );
    let yaml = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(_error) => panic!("Could not read file"),
    };
    let raw_blueprint: RawBlueprint = serde_yaml::from_str(&yaml).unwrap();
    let _blueprint = load_raw_blueprint(&raw_blueprint, 0.005);
}
#[test]
fn test_parse_blueprint_2() {
    let envs = env::vars().collect::<HashMap<String, String>>();
    let path = format!(
        "{}/github.com/loicbourgois/gravitle/gravithrust/src/blueprint/blueprint_02.yml",
        envs["HOME"]
    );
    let yaml = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(_error) => panic!("Could not read file"),
    };
    let raw_blueprint: RawBlueprint = serde_yaml::from_str(&yaml).unwrap();
    let _blueprint = load_raw_blueprint(&raw_blueprint, 0.005);
}
#[test]
fn test_parse_blueprint_3() {
    let envs = env::vars().collect::<HashMap<String, String>>();
    let path = format!(
        "{}/github.com/loicbourgois/gravitle/gravithrust/src/blueprint/blueprint_03.yml",
        envs["HOME"]
    );
    let yaml = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(_error) => panic!("Could not read file"),
    };
    let raw_blueprint: RawBlueprint = serde_yaml::from_str(&yaml).unwrap();
    let _blueprint = load_raw_blueprint(&raw_blueprint, 0.005);
}
#[test]
fn test_parse_blueprint_4() {
    let envs = env::vars().collect::<HashMap<String, String>>();
    let path = format!(
        "{}/github.com/loicbourgois/gravitle/gravithrust/src/blueprint/blueprint_04.yml",
        envs["HOME"]
    );
    let yaml = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(_error) => panic!("Could not read file"),
    };
    let raw_blueprint: RawBlueprint = serde_yaml::from_str(&yaml).unwrap();
    let _blueprint = load_raw_blueprint(&raw_blueprint, 0.005);
}
#[test]
fn size() {
    println!("{}", mem::size_of::<Ship>());
    println!("{}", Gravithrust::ship_size_internal());
    assert!(mem::size_of::<Ship>() == Gravithrust::ship_size_internal());
    println!("{}", mem::size_of::<Particle>());
    println!("{}", Gravithrust::particle_size_internal());
    assert!(mem::size_of::<Particle>() == Gravithrust::particle_size_internal() + 8);
}
