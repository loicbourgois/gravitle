use gravitle_slingshot::blueprint::Blueprint;
use std::fs;

#[test]
fn blueprint_new() {
    let home: String = std::env::var("HOME").expect("HOME environment variable not set");
    for path in [
        format!("{home}/github.com/loicbourgois/gravitle/front/slingshot/blueprint/home"),
        format!("{home}/github.com/loicbourgois/gravitle/front/slingshot/blueprint/slingshot"),
    ] {
        let full_path = format!("{path}/material.txt");
        fs::write(
            format!("{path}/instance.js"),
            Blueprint::new(&fs::read_to_string(&full_path).unwrap()).to_js(),
        )
        .unwrap();
        println!("✅ {full_path}");
    }
}
