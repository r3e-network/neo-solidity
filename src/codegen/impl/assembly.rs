impl CodeGenerator {
    fn generate_assembly_representation(&self, bytecode: &[u8]) -> String {
        let mut assembly = String::new();
        let mut i = 0;

        while i < bytecode.len() {
            match bytecode[i] {
                0x10..=0x20 => {
                    assembly.push_str(&format!("PUSH{}\n", bytecode[i] - 0x10));
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
                            assembly.push_str(&format!("{data:02X?}\n"));
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
                    i += 1;
                    if i < bytecode.len() {
                        let offset = bytecode[i] as i8;
                        assembly.push_str(&format!("JMP {offset}\n"));
                        i += 1;
                    } else {
                        assembly.push_str("JMP ??\n");
                    }
                }
                0x23 => {
                    i += 1;
                    if i + 4 <= bytecode.len() {
                        let offset = i32::from_le_bytes([bytecode[i], bytecode[i+1], bytecode[i+2], bytecode[i+3]]);
                        assembly.push_str(&format!("JMP_L {offset}\n"));
                        i += 4;
                    } else {
                        assembly.push_str("JMP_L ??\n");
                    }
                }
                0x26 => {
                    i += 1;
                    if i < bytecode.len() {
                        let offset = bytecode[i] as i8;
                        assembly.push_str(&format!("JMPIFNOT {offset}\n"));
                        i += 1;
                    } else {
                        assembly.push_str("JMPIFNOT ??\n");
                    }
                }
                0x40 => {
                    assembly.push_str("RET\n");
                    i += 1;
                }
                0x41 => {
                    assembly.push_str("SYSCALL ");
                    i += 1;
                    if i + 4 <= bytecode.len() {
                        let hash = &bytecode[i..i + 4];
                        assembly.push_str(&format!("{:02X}{:02X}{:02X}{:02X}\n", hash[0], hash[1], hash[2], hash[3]));
                        i += 4;
                    } else {
                        assembly.push_str("??\n");
                    }
                }
                0x50 => { assembly.push_str("SWAP\n"); i += 1; }
                0x51 => { assembly.push_str("ROT\n"); i += 1; }
                0x52 => { assembly.push_str("ROLL\n"); i += 1; }
                0x53 => { assembly.push_str("REVERSE3\n"); i += 1; }
                0x54 => { assembly.push_str("REVERSE4\n"); i += 1; }
                0x55 => { assembly.push_str("REVERSEN\n"); i += 1; }
                _ => {
                    assembly.push_str(&format!("OP_{:02X}\n", bytecode[i]));
                    i += 1;
                }
            }
        }

        assembly
    }
}

