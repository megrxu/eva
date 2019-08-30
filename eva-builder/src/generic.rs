use std::collections::HashMap;

pub struct Literal {
    name: String,
    value: bool,
}

pub struct Clause {
    value: Vec<Literal>,
    clause_xor: bool,
}

#[derive(Default)]
pub struct Instance {
    pub equations: Vec<Clause>,
    pub variables: HashMap<String, u32>,
}

impl From<(&str, bool)> for Literal {
    fn from(tuple: (&str, bool)) -> Self {
        Literal {
            name: tuple.0.to_string(),
            value: tuple.1,
        }
    }
}

impl Clause {
    pub fn new(clause_xor: bool, value: Vec<(&str, bool)>) -> Self {
        Clause {
            value: value.into_iter().map(Literal::from).collect(),
            clause_xor,
        }
    }
}

impl Instance {
    pub fn new() -> Self {
        Instance {
            equations: vec![],
            variables: HashMap::new(),
        }
    }

    pub fn clauses(self) -> Vec<(bool, Vec<(u32, bool)>)> {
        let eqns = self.equations;
        let vars = self.variables;
        eqns.into_iter()
            .map(|clause| {
                (
                    clause.clause_xor,
                    clause
                        .value
                        .into_iter()
                        .map(|lit| (*(vars.get(&lit.name).unwrap()), lit.value))
                        .collect(),
                )
            })
            .collect()
    }

    pub fn add_clause(&mut self, clause: Clause) {
        self.equations.push(clause);
    }

    pub fn add_variable(&mut self, name: &str) {
        if self.variables.get(name).is_none() {
            self.variables
                .insert(name.to_string(), (self.variables.len() + 1) as u32);
        }
    }

    pub fn add_variables(&mut self, names: Vec<&str>) {
        for name in names.iter() {
            self.add_variable(name);
        }
    }

    pub fn to_cnf(&self) -> String {
        let mut cnf_string = "".to_string();
        for clause in self.equations.iter() {
            if clause.clause_xor {
                cnf_string.push_str("x ");
            }
            for lit in clause.value.iter() {
                let var = self.variables.get(&lit.name).unwrap();
                if lit.value {
                    cnf_string.push_str(&format!("{} ", var))
                } else {
                    cnf_string.push_str(&format!("-{} ", var))
                }
            }
            cnf_string.push_str(&"0\n".to_string());
        }
        cnf_string
    }
}
