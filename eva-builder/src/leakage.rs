use super::generic::Clause;

pub struct Leakage {
    leakage_type: LeakageType,
    clauses: Vec<Clause>,
}

pub enum LeakageType {
    Template,
    HammingWeight,
    PartialBit,
    Collision,
    Fault,
}

impl Leakage {
    pub fn new(leakage_type: LeakageType) -> Self {
        Leakage {
            leakage_type,
            clauses: vec![],
        }
    }
}
