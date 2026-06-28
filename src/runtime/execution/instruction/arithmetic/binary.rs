use super::*;
use crate::opcode::OpCode;

impl ExecutionContext {
    pub(crate) fn execute_arithmetic_binary(&mut self, opcode: u8) -> Result<bool, RuntimeError> {
        const ADD: u8 = OpCode::ADD.byte();
        const SUB: u8 = OpCode::SUB.byte();
        const MUL: u8 = OpCode::MUL.byte();
        const DIV: u8 = OpCode::DIV.byte();
        const MOD: u8 = OpCode::MOD.byte();
        const POW: u8 = OpCode::POW.byte();
        const MODMUL: u8 = OpCode::MODMUL.byte();
        const MODPOW: u8 = OpCode::MODPOW.byte();

        match opcode {
            ADD => {
                // ADD
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.add_stack_items(a, b)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
                Ok(true)
            }
            SUB => {
                // SUB
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.sub_stack_items(a, b)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
                Ok(true)
            }
            MUL => {
                // MUL
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.mul_stack_items(a, b)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
                Ok(true)
            }
            DIV => {
                // DIV
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.div_stack_items(a, b)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
                Ok(true)
            }
            MOD => {
                // MOD
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.mod_stack_items(a, b)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
                Ok(true)
            }
            POW => {
                // POW
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.pow_stack_items(a, b)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
                Ok(true)
            }
            MODMUL => {
                // MODMUL
                let modulus = self.pop_stack()?;
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.modmul_stack_items(a, b, modulus)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
                Ok(true)
            }
            MODPOW => {
                // MODPOW
                let modulus = self.pop_stack()?;
                let exponent = self.pop_stack()?;
                let base = self.pop_stack()?;
                let result = self.modpow_stack_items(base, exponent, modulus)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}
