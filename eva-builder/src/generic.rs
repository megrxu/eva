pub type Literal = (u32, bool);
pub type Clause = Vec<Literal>;
pub struct Instance {
    pub equations: Vec<Clause>,
    pub var_count: u32,
}

impl Instance {
    fn new() -> Self {
        Instance {
            equations: vec![],
            var_count: 0,
        }
    }

    fn add_clause(mut self, clause: Clause) {
        self.equations.push(clause);
    }
}
