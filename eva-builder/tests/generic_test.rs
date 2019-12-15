use eva_builder::generic::*;

#[cfg(test)]
#[test]
fn test_instance() {
    let mut ins = Instance::new();
    ins.add_variables(vec!["a", "b", "c", "d"]);
    ins.add_variable("e");
    ins.add_clause(Clause::new(true, vec![("a", true), ("b", false)]));
    ins.add_clause(Clause::new(true, vec![("b", true), ("a", false)]));
    ins.add_clause(Clause::new(false, vec![("c", true), ("b", false)]));
    ins.add_clause(Clause::new(true, vec![("d", true), ("c", false)]));
    ins.add_clause(Clause::new(true, vec![("e", true), ("d", false)]));
    assert_eq!(
        ins.to_cnf(),
        r#"x 1 -2 0
x 2 -1 0
3 -2 0
x 4 -3 0
x 5 -4 0
"#
    )
}
