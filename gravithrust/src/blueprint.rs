use crate::kind::kindstr_to_kind;
use crate::kind::Kind;
use crate::link::Link;
use crate::math::rotate;
use crate::math::Vector;
use crate::ship::OrientationMode;
use serde::Deserialize;
use serde::Serialize;
use wasm_bindgen::prelude::wasm_bindgen;
#[derive(Debug, Serialize, Deserialize)]
pub struct RawBlueprint {
    name: String,
    ascii: String,
    pub orientation_mode: Option<OrientationMode>,
    base_particles: [String; 2],
    particles: Vec<[String; 4]>,
    links: Vec<[usize; 2]>,
    forward: Vec<usize>,
    slow: Vec<usize>,
    left: Vec<usize>,
    right: Vec<usize>,
    translate_right: Vec<usize>,
    translate_left: Vec<usize>,
}
#[wasm_bindgen]
pub struct Blueprint {
    #[wasm_bindgen(skip)]
    pub blueprint_id: usize,
    #[wasm_bindgen(skip)]
    pub particles: Vec<ParticleBlueprint>,
    #[wasm_bindgen(skip)]
    pub links: Vec<Link>,
    #[wasm_bindgen(skip)]
    pub forward: Vec<usize>,
    #[wasm_bindgen(skip)]
    pub slow: Vec<usize>,
    #[wasm_bindgen(skip)]
    pub left: Vec<usize>,
    #[wasm_bindgen(skip)]
    pub right: Vec<usize>,
    #[wasm_bindgen(skip)]
    pub translate_left: Vec<usize>,
    #[wasm_bindgen(skip)]
    pub translate_right: Vec<usize>,
    #[wasm_bindgen(skip)]
    pub orientation_mode: OrientationMode,
}
#[wasm_bindgen]
pub struct ParticleBlueprint {
    #[wasm_bindgen(skip)]
    pub p: Vector,
    #[wasm_bindgen(skip)]
    pub k: Kind,
}
pub fn load_raw_blueprint(r_blueprint: &RawBlueprint, diameter: f32) -> Blueprint {
    let mut particles = vec![];
    let mut links = vec![];
    particles.push(ParticleBlueprint {
        p: Vector {
            x: 0.0,
            y: 0.0,
        },
        k: kindstr_to_kind(&r_blueprint.base_particles[0]),
    });
    particles.push(ParticleBlueprint {
        p: rotate(
            particles[0].p,
            Vector {
                x: diameter * 1.0,
                y: 0.0,
            },
            4.0 / 6.0,
        ),
        k: kindstr_to_kind(&r_blueprint.base_particles[1]),
    });
    for x in &r_blueprint.particles {
        let new_particle_id = x[0].parse::<usize>().expect("invalid particle_id");
        let p1_id = x[1].parse::<usize>().expect("invalid p1_id");
        let p2_id = x[2].parse::<usize>().expect("invalid p2_id");
        let kind = kindstr_to_kind(&x[3]);
        assert!(new_particle_id == particles.len(), "bad length");
        particles.push(ParticleBlueprint {
            p: rotate(particles[p1_id].p, particles[p2_id].p, 1.0 / 6.0),
            k: kind,
        });
    }
    for pids in &r_blueprint.links {
        links.push(Link {
            a: pids[0],
            b: pids[1],
        });
    }
    let blueprint_id = 2;
    let orientation_mode = match r_blueprint.orientation_mode {
        Some(x) => x,
        None => OrientationMode::Triangle,
    };
    Blueprint {
        blueprint_id,
        particles,
        links,
        orientation_mode,
        left: r_blueprint.left.clone(),
        right: r_blueprint.right.clone(),
        forward: r_blueprint.forward.clone(),
        slow: r_blueprint.slow.clone(),
        translate_left: r_blueprint.translate_left.clone(),
        translate_right: r_blueprint.translate_right.clone(),
    }
}
