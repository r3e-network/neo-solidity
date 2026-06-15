use super::*;

impl ExecutionContext {
    pub(crate) fn new_array0(&mut self) -> Result<(), RuntimeError> {
        self.push_stack(StackItem::array(Vec::new()))
    }

    pub(crate) fn new_struct0(&mut self) -> Result<(), RuntimeError> {
        self.push_stack(StackItem::array(Vec::new()))
    }

    pub(crate) fn new_struct(&mut self) -> Result<(), RuntimeError> {
        self.new_array()
    }

    pub(crate) fn new_array(&mut self) -> Result<(), RuntimeError> {
        let count = self.pop_usize("NEWARRAY")?;
        // NeoVM's ExecutionEngineLimits.MaxStackSize (2048) counts EVERY item,
        // including elements inside arrays/maps/structs. A single collection that
        // exceeds 2048 therefore always blows the global limit and FAULTs on a
        // real node — model that here so the simulator does not hide an on-chain
        // fault by reporting success. (Conservative: a valid contract can never
        // hold a 2048+-element collection, so this never false-faults.)
        if count > 2048 {
            return Err(RuntimeError::ExecutionError {
                message: format!("NEWARRAY: {count} elements exceeds NeoVM MaxStackSize (2048)"),
            });
        }
        // Bound user-supplied count against memory_limit. Without this check,
        // a malicious PUSHINT + NEWARRAY/NEWSTRUCT/NEWARRAY_T sequence can
        // request ~2^31 elements and OOM-abort the host before gas accounting
        // fires. Mirrors the PUSHDATA4 length check in instruction/push.rs.
        let item_size = std::mem::size_of::<StackItem>();
        let required = count.saturating_mul(item_size);
        if required > self.memory_limit {
            return Err(RuntimeError::ExecutionError {
                message: format!(
                    "NEWARRAY: requested {} items ({} bytes) exceeds memory limit {}",
                    count, required, self.memory_limit
                ),
            });
        }
        let items = vec![StackItem::Null; count];
        self.push_stack(StackItem::array(items))
    }

    pub(crate) fn new_map(&mut self) -> Result<(), RuntimeError> {
        self.push_stack(StackItem::map(std::collections::HashMap::new()))
    }
}
