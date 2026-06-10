use super::*;

impl ExecutionContext {
    pub(crate) fn execute_arithmetic_comparison(
        &mut self,
        opcode: u8,
    ) -> Result<bool, RuntimeError> {
        match opcode {
            0xB3 => {
                // NUMEQUAL
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.stack_items_equal(&a, &b)?;
                self.push_stack(StackItem::Boolean(result))?;
                self.instruction_pointer += 1;
                Ok(true)
            }
            0xB4 => {
                // NUMNOTEQUAL
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.stack_items_equal(&a, &b)?;
                self.push_stack(StackItem::Boolean(!result))?;
                self.instruction_pointer += 1;
                Ok(true)
            }
            0xB5 => {
                // LT
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.less_than(&a, &b)?;
                self.push_stack(StackItem::Boolean(result))?;
                self.instruction_pointer += 1;
                Ok(true)
            }
            0xB6 => {
                // LE
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let lt = self.less_than(&a, &b)?;
                let eq = self.stack_items_equal(&a, &b)?;
                self.push_stack(StackItem::Boolean(lt || eq))?;
                self.instruction_pointer += 1;
                Ok(true)
            }
            0xB7 => {
                // GT
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.greater_than(&a, &b)?;
                self.push_stack(StackItem::Boolean(result))?;
                self.instruction_pointer += 1;
                Ok(true)
            }
            0xB8 => {
                // GE
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let gt = self.greater_than(&a, &b)?;
                let eq = self.stack_items_equal(&a, &b)?;
                self.push_stack(StackItem::Boolean(gt || eq))?;
                self.instruction_pointer += 1;
                Ok(true)
            }
            0xB9 => {
                // MIN
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.min_stack_items(a, b)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
                Ok(true)
            }
            0xBA => {
                // MAX
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.max_stack_items(a, b)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
                Ok(true)
            }
            0xBB => {
                // WITHIN (left inclusive)
                let max_item = self.pop_stack()?;
                let min_item = self.pop_stack()?;
                let value = self.pop_stack()?;
                let result = self.within_stack_items(value, min_item, max_item)?;
                self.push_stack(StackItem::Boolean(result))?;
                self.instruction_pointer += 1;
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}
