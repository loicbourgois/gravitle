use crate::test::helpers::set_job_by_path;
use crate::test::helpers::setup_simulation;
use glob::glob;
use std::collections::HashMap;
use std::env;
#[test]
fn test_job() {
    let envs = env::vars().collect::<HashMap<String, String>>();
    let mut g = setup_simulation();
    let path = format!(
        "{}/github.com/loicbourgois/gravitle/gravithrust/src/job/*.json",
        envs["HOME"],
    );
    for e in glob(&path).expect("Failed to read glob pattern") {
        let pathbuf = e.unwrap();
        let path = pathbuf.to_str().unwrap();
        println!("testing {}", path);
        match set_job_by_path(&mut g, path) {
            Ok(_) => {}
            Err(err) => {
                println!("error with {}", path);
                panic!("{}", err);
            }
        }
    }
}
