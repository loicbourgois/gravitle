use gravitle_slingshot::world::World;
use std::fs;

#[test]
fn world_new() {
    let home: String = std::env::var("HOME").expect("HOME environment variable not set");
    let mut world = World::new();
    for x in ["granite", "steel", "launcher"] {
        let path = format!("/slingshot/material/{x}.json");
        world.add_material(
            path.clone(),
            &fs::read_to_string(format!(
                "{home}/github.com/loicbourgois/gravitle/front/{path}"
            ))
            .unwrap(),
        );
    }
    for name in ["home", "slingshot"] {
        let slingshot_blueprint = fs::read_to_string(format!(
            "{home}/github.com/loicbourgois/gravitle/front/slingshot/blueprint/{name}/material.txt"
        ))
        .unwrap();
        world.add_from_blueprint(&slingshot_blueprint, 0.0, 0.0);
        println!("✅ {name}");
    }
}
