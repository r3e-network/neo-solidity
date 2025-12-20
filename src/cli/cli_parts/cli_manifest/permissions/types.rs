#[derive(Clone, Debug, PartialEq, Eq)]
enum AbstractValue {
    Literal(ir::LiteralValue),
    ExecutingScriptHash,
    Unknown,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct AbstractState {
    stack: Vec<AbstractValue>,
    locals: Vec<AbstractValue>,
}

impl AbstractState {
    fn new(local_count: u16) -> Self {
        Self {
            stack: Vec::new(),
            locals: vec![AbstractValue::Unknown; local_count as usize],
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum PermissionMethods {
    All,
    Some(std::collections::BTreeSet<String>),
}

impl PermissionMethods {
    fn merge_in(&mut self, other: PermissionMethods) {
        match (self, other) {
            (PermissionMethods::All, _) => {}
            (this, PermissionMethods::All) => {
                *this = PermissionMethods::All;
            }
            (PermissionMethods::Some(set), PermissionMethods::Some(other_set)) => {
                set.extend(other_set);
            }
        }
    }
}

#[derive(Clone, Debug)]
struct ContractCallRequirement {
    contract: Option<String>,
    method: Option<String>,
}
