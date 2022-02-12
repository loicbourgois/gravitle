use core::part::Kind;
use rand::Rng;

type PlanID = usize;

#[derive(Copy, Clone, Debug)]
pub struct PartPlan {
    pub a: PlanID, // parent A id
    pub b: PlanID, // parent B id
    pub k: Kind,
    pub cr: u8,
    pub cg: u8,
    pub cb: u8,
}
pub struct Plan {
    pub kinds: [Kind; 2],
    pub colors: [u8; 6],
    pub part_plans: Vec<PartPlan>,
}

const DNA_SIZE: usize = 2*4 + 6 * 20;

pub type Dna = [u8; DNA_SIZE];

pub fn plan_to_dna(plan: &Plan) -> Dna {
    let mut dna_vec: Vec<u8> = vec![
        plan.kinds[0] as u8,
        plan.kinds[1] as u8,
        plan.colors[0],
        plan.colors[1],
        plan.colors[2],
        plan.colors[3],
        plan.colors[4],
        plan.colors[5],
    ];
    for part_plan in &plan.part_plans {
        dna_vec.push(part_plan.k as u8);
        dna_vec.push(part_plan.a as u8);
        dna_vec.push(part_plan.b as u8);
        dna_vec.push(part_plan.cr);
        dna_vec.push(part_plan.cg);
        dna_vec.push(part_plan.cb);
    }
    dna_vec.resize(DNA_SIZE, 0);
    vec_to_array(dna_vec)
}

pub fn mutate_dna_inplace(dna: &mut Dna) {
    let mut rng = rand::thread_rng();
    let toggler_id = rng.gen_range(0..8);
    let byte_id = rng.gen_range(0..DNA_SIZE);
    let togglers = [
        0b0000_0001,
        0b0000_0010,
        0b0000_0100,
        0b0000_1000,
        0b0001_0000,
        0b0010_0000,
        0b0100_0000,
        0b1000_0000,
    ];
    dna[byte_id] ^= togglers[toggler_id];
}

// pub fn mutate_dna(dna: & Dna) -> Dna {
//     let mut dna_ = *dna;
//     mutate_dna_inplace(&mut dna_);
//     dna_
// }

pub fn dna_to_plan(dna: &Dna) -> Plan {
    let mut plan = Plan {
        kinds: [Kind::from(dna[0]), Kind::from(dna[1])],
        colors: [
            dna[2],
            dna[3],
            dna[4],
            dna[5],
            dna[6],
            dna[7],
        ],
        part_plans: Vec::new(),
    };
    let i_start = 2*4;
    let ss = 6;
    for i in 0..20 {
        plan.part_plans.push(PartPlan {
            k: Kind::from(dna[i_start + i * ss]),
            a: dna[i_start + i * ss + 1] as usize,
            b: dna[i_start + i * ss + 2] as usize,
            cr: dna[i_start + i * ss + 3] ,
            cg: dna[i_start + i * ss + 4] ,
            cb: dna[i_start + i * ss + 5] ,
        })
    }
    plan
}

fn vec_to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

pub fn get_plan() -> Plan {
    Plan {
        kinds: [Kind::Metal, Kind::Metal],
        colors: [0, 255, 255, 0, 255, 255],
        part_plans: vec![
            PartPlan {
                a: 0,
                b: 1,
                k: Kind::Mouth,
                cr: 0,
                cg: 255,
                cb: 255,
            },
            PartPlan {
                a: 1,
                b: 0,
                k: Kind::Core,
                cr: 255,
                cg: 255,
                cb: 255,
            },
            PartPlan {
                a: 1,
                b: 3,
                k: Kind::Metal,
                cr: 0,
                cg: 255,
                cb: 255,
            },
            PartPlan {
                a: 3,
                b: 0,
                k: Kind::Metal,
                cr: 0,
                cg: 255,
                cb: 255,
            },
            PartPlan {
                a: 1,
                b: 4,
                k: Kind::Metal,
                cr: 0,
                cg: 255,
                cb: 255,
            },
            PartPlan {
                a: 5,
                b: 0,
                k: Kind::Metal,
                cr: 0,
                cg: 255,
                cb: 255,
            },
            PartPlan {
                a: 1,
                b: 6,
                k: Kind::Mouth,
                cr: 0,
                cg: 255,
                cb: 255,
            },
            PartPlan {
                a: 7,
                b: 0,
                k: Kind::Mouth,
                cr: 0,
                cg: 255,
                cb: 255,
            },
            PartPlan {
                a: 6,
                b: 4,
                k: Kind::Turbo,
                cr: 0,
                cg: 255,
                cb: 255,
            },
            PartPlan {
                a: 5,
                b: 7,
                k: Kind::Turbo,
                cr: 0,
                cg: 255,
                cb: 255,
            },
            //
            //
            PartPlan {
                a: 6,
                b: 10,
                k: Kind::Metal,
                cr: 0,
                cg: 255,
                cb: 255,
            },
            PartPlan {
                a: 12,
                b: 10,
                k: Kind::Metal,
                cr: 0,
                cg: 255,
                cb: 255,
            },
            PartPlan {
                a: 11,
                b: 7,
                k: Kind::Metal,
                cr: 0,
                cg: 255,
                cb: 255,
            },
            PartPlan {
                a: 11,
                b: 14,
                k: Kind::Metal,
                cr: 0,
                cg: 255,
                cb: 255,
            },
        ],
    }
}
