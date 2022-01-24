#[derive(Copy, Clone, Debug)]
pub enum Kind {
    // Firefly = 1,
    Metal = 2,
    // Turbo = 3,
    // Diatom = 4,
    // Neuron = 5,
    // Mouth = 6,
    // Core = 7,
    // Egg = 8,
}

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
