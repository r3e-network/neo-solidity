impl CodeGenerator {
    fn generate_assembly_representation(&self, bytecode: &[u8]) -> String {
        let mut assembly = String::new();
        let mut i = 0;

        while i < bytecode.len() {
            match bytecode[i] {
                0x50..=0x60 => {
                    assembly.push_str(&format!("PUSH{}\n", bytecode[i] - 0x50));
                    i += 1;
                }
                0x0C => {
                    assembly.push_str("PUSHDATA1 ");
                    i += 1;
                    if i < bytecode.len() {
                        let len = bytecode[i] as usize;
                        i += 1;
                        if i + len <= bytecode.len() {
                            let data = &bytecode[i..i + len];
                            assembly.push_str(&format!("{:02X?}\n", data));
                            i += len;
                        }
                    }
                }
                0x9E => {
                    assembly.push_str("ADD\n");
                    i += 1;
                }
                0x9F => {
                    assembly.push_str("SUB\n");
                    i += 1;
                }
                0xA0 => {
                    assembly.push_str("MUL\n");
                    i += 1;
                }
                0xA1 => {
                    assembly.push_str("DIV\n");
                    i += 1;
                }
                0x22 => {
                    assembly.push_str("JMP\n");
                    i += 1;
                }
                0x23 => {
                    assembly.push_str("JMPIFNOT\n");
                    i += 1;
                }
                0x40 => {
                    assembly.push_str("RET\n");
                    i += 1;
                }
                0x41 => {
                    assembly.push_str("SYSCALL\n");
                    i += 1;
                }
                _ => {
                    assembly.push_str(&format!("OP_{:02X}\n", bytecode[i]));
                    i += 1;
                }
            }
        }

        assembly
    }
}

