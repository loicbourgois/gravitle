use core::part::bloop_dna_to_kind;
use core::part::kind_to_bloop_dna;
use core::part::Kind;
use rand::Rng;
use std::collections::HashMap;

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
#[derive(Clone, Debug)]
pub struct Plan {
    pub kinds: [Kind; 2],
    pub colors: [u8; 6],
    pub part_plans: Vec<PartPlan>,
}

type LinkID = usize;

#[derive(Copy, Clone, Debug)]
pub struct PartLinkPlan {
    pub l: LinkID,
    pub kind: Kind,
    pub cr: u8,
    pub cg: u8,
    pub cb: u8,
}
#[derive(Clone, Debug)]
pub struct LinkPlan {
    pub kinds: [Kind; 2],
    pub colors: [u8; 6],
    pub part_plans: Vec<PartLinkPlan>,
}

pub fn ab_plan_to_link_plan(plan: &Plan) -> LinkPlan {
    let mut link_plan = LinkPlan {
        kinds: plan.kinds,
        colors: plan.colors,
        part_plans: Vec::new(),
    };
    let mut link_id_to_part_ids = vec![(0, 1), (1, 0)];
    let mut part_count = 2;
    for part_plan in &plan.part_plans {
        let mut part_ids_to_link_id: HashMap<(usize, usize), usize> = HashMap::new();
        for (i, link) in link_id_to_part_ids.iter().enumerate() {
            part_ids_to_link_id.insert(*link, i);
        }
        let a = part_plan.a;
        let b = part_plan.b;
        let link_id: usize = *part_ids_to_link_id.get(&(a, b)).unwrap();
        link_id_to_part_ids.remove(link_id);
        link_plan.part_plans.push(PartLinkPlan {
            l: link_id,
            kind: part_plan.k,
            cr: part_plan.cr,
            cg: part_plan.cg,
            cb: part_plan.cb,
        });
        link_id_to_part_ids.push((a, part_count));
        link_id_to_part_ids.push((part_count, b));
        part_count += 1;
    }
    link_plan
}

pub fn link_plan_to_ab_plan(link_plan: &LinkPlan) -> Plan {
    let mut ab_plan = Plan {
        kinds: link_plan.kinds,
        colors: link_plan.colors,
        part_plans: Vec::new(),
    };
    let mut link_id_to_part_ids = vec![(0, 1), (1, 0)];
    let mut part_count = 2;
    for part_plan in &link_plan.part_plans {
        let link_id = part_plan.l;
        let (a, b) = link_id_to_part_ids[link_id];
        link_id_to_part_ids.remove(link_id);
        link_id_to_part_ids.push((a, part_count));
        link_id_to_part_ids.push((part_count, b));
        ab_plan.part_plans.push(PartPlan {
            a,
            b,
            k: part_plan.kind,
            cr: part_plan.cr,
            cg: part_plan.cg,
            cb: part_plan.cb,
        });
        part_count += 1;
    }
    ab_plan
}

// pub fn get_plan_3() -> Plan {
//     Plan {
//         kinds: [Kind::Mouth, Kind::Mouth],
//         colors: [0, 255, 255, 0, 255, 255],
//         part_plans: vec![
//             PartPlan {
//                 a: 1,
//                 b: 0,
//                 k: Kind::Core,
//                 cr: 255,
//                 cg: 155,
//                 cb: 0,
//             },
//         ]
//     }
// }
//
// pub fn get_plan_4() -> Plan {
//     Plan {
//         kinds: [Kind::Mouth, Kind::Mouth],
//         colors: [0, 255, 255, 0, 255, 255],
//         part_plans: vec![
//             PartPlan {
//                 a: 1,
//                 b: 0,
//                 k: Kind::Core,
//                 cr: 255,
//                 cg: 155,
//                 cb: 0,
//             },
//             PartPlan {
//                 a: 0,
//                 b: 1,
//                 k: Kind::Core,
//                 cr: 255,
//                 cg: 155,
//                 cb: 0,
//             },
//         ]
//     }
// }
//
//
// pub fn get_plan_6() -> Plan {
//     Plan {
//         kinds: [Kind::Mouth, Kind::Mouth],
//         colors: [0, 255, 255, 0, 255, 255],
//         part_plans: vec![
//             PartPlan {
//                 a: 1,
//                 b: 0,
//                 k: Kind::Core,
//                 cr: 255,
//                 cg: 155,
//                 cb: 0,
//             },
//             PartPlan {
//                 a: 1,
//                 b: 2,
//                 k: Kind::Core,
//                 cr: 255,
//                 cg: 155,
//                 cb: 0,
//             },
//             PartPlan {
//                 a: 3,
//                 b: 2,
//                 k: Kind::Core,
//                 cr: 255,
//                 cg: 155,
//                 cb: 0,
//             },
//             PartPlan {
//                 a: 4,
//                 b: 2,
//                 k: Kind::Core,
//                 cr: 255,
//                 cg: 155,
//                 cb: 0,
//             },
//         ]
//     }
// }

const DNA_SIZE: usize = 2 * 4 + 5 * 24;

pub type Dna = [u8; DNA_SIZE];

pub fn random_dna() -> Dna {
    let mut rng = rand::thread_rng();
    let mut dna =  [0; DNA_SIZE];
    for gene in dna.iter_mut().take(DNA_SIZE) {
        *gene = rng.gen_range(0..=255);
    }
    dna
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

pub fn dna_to_link_plan(dna: &Dna) -> LinkPlan {
    let mut plan = LinkPlan {
        kinds: [bloop_dna_to_kind(dna[0]), bloop_dna_to_kind(dna[1])],
        colors: [dna[2], dna[3], dna[4], dna[5], dna[6], dna[7]],
        part_plans: Vec::new(),
    };
    let i_start = 2 * 4;
    let ss_link_plan = 5;
    //let mut link_id_to_part_ids = vec![(0,1),(1,0)];
    let mut link_count = 2;
    for i in 0..24 {
        plan.part_plans.push(PartLinkPlan {
            kind: bloop_dna_to_kind(dna[i_start + i * ss_link_plan]),
            l: (dna[i_start + i * ss_link_plan + 1] as usize) % link_count,
            cr: dna[i_start + i * ss_link_plan + 2],
            cg: dna[i_start + i * ss_link_plan + 3],
            cb: dna[i_start + i * ss_link_plan + 4],
        });
        link_count += 1;
    }
    plan
}

pub fn link_plan_to_dna(plan: &LinkPlan) -> Dna {
    let mut dna_vec: Vec<u8> = vec![
        kind_to_bloop_dna(plan.kinds[0]),
        kind_to_bloop_dna(plan.kinds[1]),
        plan.colors[0],
        plan.colors[1],
        plan.colors[2],
        plan.colors[3],
        plan.colors[4],
        plan.colors[5],
    ];
    for part_plan in &plan.part_plans {
        dna_vec.push(kind_to_bloop_dna(part_plan.kind));
        dna_vec.push(part_plan.l as u8);
        dna_vec.push(part_plan.cr);
        dna_vec.push(part_plan.cg);
        dna_vec.push(part_plan.cb);
    }
    dna_vec.resize(DNA_SIZE, 0);
    vec_to_array(dna_vec)
}

pub fn dna_to_str(dna: &Dna) -> String {
    let mut str_: String = "".to_string();
    let i_start = 2 * 4;
    for gene in dna.iter().take(i_start) {
        str_ += &format!("{:02X?}", gene)
    }
    let ss_link_plan = 5;
    for i in 0..24 {
        let kind = dna[i_start + i * ss_link_plan];
        let l = dna[i_start + i * ss_link_plan + 1];
        let cr = dna[i_start + i * ss_link_plan + 2];
        let cg = dna[i_start + i * ss_link_plan + 3];
        let cb = dna[i_start + i * ss_link_plan + 4];
        str_ += &format!("-{:02X?}{:02X?}{:02X?}{:02X?}{:02X?}", kind, l, cr, cg, cb);
    }
    str_.to_string()
}

fn vec_to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

pub fn get_plan_pusher() -> Plan {
    Plan {
        kinds: [Kind::Eye, Kind::Eye],
        colors: [255, 0, 255, 255, 0, 255],
        part_plans: vec![
            PartPlan { // 2
                a: 1,
                b: 0,
                k: Kind::Core,
                cr: 255,
                cg: 0,
                cb: 255,
            },
            PartPlan {
                a: 1,
                b: 2,
                k: Kind::Muscle,
                cr: 255,
                cg: 0,
                cb: 255,
            },
            PartPlan { // 4
                a: 1,
                b: 3,
                k: Kind::Mouth,
                cr: 255,
                cg: 0,
                cb: 255,
            },
            PartPlan {
                a: 4,
                b: 3,
                k: Kind::Metal,
                cr: 255,
                cg: 0,
                cb: 255,
            },
            PartPlan {
                a: 2,
                b: 0,
                k: Kind::Muscle,
                cr: 255,
                cg: 0,
                cb: 255,
            },
            PartPlan { // 7
                a: 6,
                b: 0,
                k: Kind::Mouth,
                cr: 255,
                cg: 0,
                cb: 255,
            },
            PartPlan {
                a: 6,
                b: 7,
                k: Kind::Metal,
                cr: 255,
                cg: 0,
                cb: 255,
            },
            PartPlan {
                a: 5,
                b: 3,
                k: Kind::Grip,
                cr: 255,
                cg: 0,
                cb: 255,
            },
            PartPlan {
                a: 6,
                b: 8,
                k: Kind::Grip,
                cr: 255,
                cg: 0,
                cb: 255,
            },
        ],
    }
}

pub fn get_plan_planner() -> Plan {
    Plan {
        kinds: [Kind::Metal, Kind::Metal],
        colors: [0, 255, 255, 0, 255, 255],
        part_plans: vec![
            PartPlan { // 2
                a: 0,
                b: 1,
                k: Kind::Eye,
                cr: 0,
                cg: 255,
                cb: 255,
            },
            PartPlan { // 3
                a: 0,
                b: 2,
                k: Kind::Mouth,
                cr: 0,
                cg: 255,
                cb: 255,
            },
            PartPlan { // 4
                a: 2,
                b: 1,
                k: Kind::Mouth,
                cr: 0,
                cg: 255,
                cb: 255,
            },
            PartPlan { // 5
                a: 0,
                b: 3,
                k: Kind::Metal,
                cr: 0,
                cg: 255,
                cb: 255,
            },
            PartPlan { // 6
                a: 4,
                b: 1,
                k: Kind::Metal,
                cr: 0,
                cg: 255,
                cb: 255,
            },
            PartPlan { // 7
                a: 1,
                b: 0,
                k: Kind::Core,
                cr: 0,
                cg: 255,
                cb: 255,
            },
            PartPlan { // 8
                a: 0,
                b: 5,
                k: Kind::Metal,
                cr: 0,
                cg: 255,
                cb: 255,
            },
            PartPlan { // 9
                a: 6,
                b: 1,
                k: Kind::Metal,
                cr: 0,
                cg: 255,
                cb: 255,
            },
            PartPlan { // 10
                a: 8,
                b: 5,
                k: Kind::Muscle,
                cr: 0,
                cg: 255,
                cb: 255,
            },
            PartPlan { // 11
                a: 6,
                b: 9,
                k: Kind::Muscle,
                cr: 0,
                cg: 255,
                cb: 255,
            },
            PartPlan { // 12
                a: 10,
                b: 5,
                k: Kind::Metal,
                cr: 0,
                cg: 255,
                cb: 255,
            },
            PartPlan { // 13
                a: 6,
                b: 11,
                k: Kind::Metal,
                cr: 0,
                cg: 255,
                cb: 255,
            },
            PartPlan { // 14
                a: 10,
                b: 12,
                k: Kind::Metal,
                cr: 0,
                cg: 255,
                cb: 255,
            },
            PartPlan { // 15
                a: 13,
                b: 11,
                k: Kind::Metal,
                cr: 0,
                cg: 255,
                cb: 255,
            },
            PartPlan { // 16
                a: 10,
                b: 14,
                k: Kind::Grip,
                cr: 0,
                cg: 255,
                cb: 255,
            },
            PartPlan { // 17
                a: 15,
                b: 11,
                k: Kind::Grip,
                cr: 0,
                cg: 255,
                cb: 255,
            },
        ],
    }
}


pub fn get_plan_pyra() -> Plan {
    Plan {
        kinds: [Kind::Metal, Kind::Metal],
        colors: [255, 155, 0, 255, 155, 0],
        part_plans: vec![
            PartPlan { // 2
                a: 0,
                b: 1,
                k: Kind::Eye,
                cr: 255,
                cg: 155,
                cb: 0,
            },
            PartPlan { // 3
                a: 2,
                b: 1,
                k: Kind::Mouth,
                cr: 255,
                cg: 155,
                cb: 0,
            },
            PartPlan { // 4
                a: 0,
                b: 2,
                k: Kind::Mouth,
                cr: 255,
                cg: 155,
                cb: 0,
            },
            PartPlan { // 5
                a: 0,
                b: 4,
                k: Kind::Metal,
                cr: 255,
                cg: 155,
                cb: 0,
            },
            PartPlan { // 6
                a: 3,
                b: 1,
                k: Kind::Metal,
                cr: 255,
                cg: 155,
                cb: 0,
            },
            PartPlan { // 7
                a: 0,
                b: 5,
                k: Kind::Metal,
                cr: 255,
                cg: 155,
                cb: 0,
            },
            PartPlan { // 8
                a: 1,
                b: 0,
                k: Kind::Core,
                cr: 255,
                cg: 155,
                cb: 0,
            },
            PartPlan { // 9
                a: 6,
                b: 1,
                k: Kind::Metal,
                cr: 255,
                cg: 155,
                cb: 0,
            },
            PartPlan { // 10
                a: 6,
                b: 9,
                k: Kind::Metal,
                cr: 255,
                cg: 155,
                cb: 0,
            },
            PartPlan { // 11
                a: 7,
                b: 5,
                k: Kind::Metal,
                cr: 255,
                cg: 155,
                cb: 0,
            },
            PartPlan { // 12
                a: 7,
                b: 11,
                k: Kind::Metal,
                cr: 255,
                cg: 155,
                cb: 0,
            },
            PartPlan { // 13
                a: 12,
                b: 11,
                k: Kind::Turbo,
                cr: 255,
                cg: 155,
                cb: 0,
            },
            PartPlan { // 14
                a: 10,
                b: 9,
                k: Kind::Metal,
                cr: 255,
                cg: 155,
                cb: 0,
            },
            PartPlan { // 15
                a: 10,
                b: 14,
                k: Kind::Turbo,
                cr: 255,
                cg: 155,
                cb: 0,
            },
        ],
    }
}

pub fn get_plan_aie() -> Plan {
    Plan {
        kinds: [Kind::Mouth, Kind::Eye],
        colors: [0, 155, 255, 0, 155, 255],
        part_plans: vec![
            PartPlan { // 2
                a: 0,
                b: 1,
                k: Kind::Metal,
                cr: 0,
                cg: 155,
                cb: 255,
            },
            PartPlan { // 3
                a: 0,
                b: 2,
                k: Kind::Eye,
                cr: 0,
                cg: 155,
                cb: 255,
            },
            PartPlan { // 4
                a: 2,
                b: 1,
                k: Kind::Turbo,
                cr: 0,
                cg: 155,
                cb: 255,
            },
            PartPlan { // 5
                a: 3,
                b: 2,
                k: Kind::Turbo,
                cr: 0,
                cg: 155,
                cb: 255,
            },
            PartPlan { // 6
                a: 3,
                b: 5,
                k: Kind::Metal,
                cr: 0,
                cg: 155,
                cb: 255,
            },
            PartPlan { // 7
                a: 4,
                b: 1,
                k: Kind::Metal,
                cr: 0,
                cg: 155,
                cb: 255,
            },
            PartPlan { // 8
                a: 4,
                b: 7,
                k: Kind::Metal,
                cr: 0,
                cg: 155,
                cb: 255,
            },
            PartPlan { // 9
                a: 5,
                b: 2,
                k: Kind::Metal,
                cr: 0,
                cg: 155,
                cb: 255,
            },
            PartPlan { // 10
                a: 6,
                b: 5,
                k: Kind::Metal,
                cr: 0,
                cg: 155,
                cb: 255,
            },
        ]
    }
}
