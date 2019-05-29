extern crate cryptominisat;
use cryptominisat::{Lbool, Lit, Solver};
use eva_builder::generic::Literal;

fn clause(l: &[Literal]) -> Vec<Lit> {
    let lit = |var: u32, neg: bool| Lit::new(var, neg).unwrap();
    l.iter().map(|(var, val)| lit(*var, *val)).collect()
}

fn main() {
    let mut solver = Solver::new();

    solver.set_num_threads(4);
    solver.new_vars(3);

    solver.add_clause(&clause(&[(0, false)]));
    solver.add_clause(&clause(&[(1, true)]));
    solver.add_xor_literal_clause(&clause(&[(0, true), (1, false), (2, false)]), true);

    let ret = solver.solve();

    assert!(ret == Lbool::True);
    assert!(solver.get_model()[0] == Lbool::True);
    assert!(solver.get_model()[1] == Lbool::False);
    assert!(solver.get_model()[2] == Lbool::True);
}
