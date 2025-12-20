fn modifier_placeholder_stmt(stmt: &Statement) -> bool {
    matches!(
        stmt,
        Statement::Expression(_, Expression::Variable(Identifier { name, .. }))
            if name == "_"
    )
}
