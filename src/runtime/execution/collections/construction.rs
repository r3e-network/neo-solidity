impl ExecutionContext {
    fn new_array0(&mut self) -> Result<(), RuntimeError> {
        self.push_stack(StackItem::array(Vec::new()))
    }

    fn new_struct0(&mut self) -> Result<(), RuntimeError> {
        self.push_stack(StackItem::array(Vec::new()))
    }

    fn new_struct(&mut self) -> Result<(), RuntimeError> {
        self.new_array()
    }

    fn new_array(&mut self) -> Result<(), RuntimeError> {
        let count = self.pop_usize("NEWARRAY")?;
        let mut items = Vec::with_capacity(count);
        for _ in 0..count {
            items.push(StackItem::Null);
        }
        self.push_stack(StackItem::array(items))
    }

    fn new_map(&mut self) -> Result<(), RuntimeError> {
        self.push_stack(StackItem::map(std::collections::HashMap::new()))
    }
}
