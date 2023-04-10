use crate::blueprint::load_raw_blueprint;
use crate::blueprint::RawBlueprint;
use std::collections::HashMap;
use std::env;
use std::fs;

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
