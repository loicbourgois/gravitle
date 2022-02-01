use core::part::Kind;

type PlanID = usize;

#[derive(Copy, Clone, Debug)]
pub struct PartPlan {
    pub a: PlanID, // parent A id
    pub b: PlanID, // parent B id
    pub k: Kind,
}
pub struct Plan {
    pub kinds: [Kind; 2],
    pub part_plans: Vec<PartPlan>,
}

pub fn get_plan() -> Plan {
    Plan {
        kinds: [Kind::Metal, Kind::Metal],
        part_plans: vec![
            PartPlan {
                a: 0,
                b: 1,
                k: Kind::Mouth,
            },
            PartPlan {
                a: 1,
                b: 0,
                k: Kind::Core,
            },
            PartPlan {
                a: 1,
                b: 3,
                k: Kind::Metal,
            },
            PartPlan {
                a: 3,
                b: 0,
                k: Kind::Metal,
            },
            PartPlan {
                a: 1,
                b: 4,
                k: Kind::Metal,
            },
            PartPlan {
                a: 5,
                b: 0,
                k: Kind::Metal,
            },
            PartPlan {
                a: 1,
                b: 6,
                k: Kind::Mouth,
            },
            PartPlan {
                a: 7,
                b: 0,
                k: Kind::Mouth,
            },
            PartPlan {
                a: 6,
                b: 4,
                k: Kind::Turbo,
            },
            PartPlan {
                a: 5,
                b: 7,
                k: Kind::Turbo,
            },
            //
            //
            PartPlan {
                a: 6,
                b: 10,
                k: Kind::Metal,
            },
            PartPlan {
                a: 12,
                b: 10,
                k: Kind::Metal,
            },
            PartPlan {
                a: 11,
                b: 7,
                k: Kind::Metal,
            },
            PartPlan {
                a: 11,
                b: 14,
                k: Kind::Metal,
            },
        ],
    }
}
