fn emit_load_parameter(bytecode: &mut Vec<u8>, _method: &FunctionMetadata, index: usize) {
    if index <= 6 {
        bytecode.push(0x78 + index as u8);
    } else {
        bytecode.push(0x7F); // LDARG
        bytecode.push(index as u8);
    }
}

fn emit_load_local(bytecode: &mut Vec<u8>, index: usize) {
    match index {
        0..=6 => bytecode.push(0x68 + index as u8),
        _ => {
            bytecode.push(0x6F); // LDLOC
            bytecode.push(index as u8);
        }
    }
}

fn emit_store_local(bytecode: &mut Vec<u8>, index: usize) {
    match index {
        0..=6 => bytecode.push(0x70 + index as u8),
        _ => {
            bytecode.push(0x77); // STLOC
            bytecode.push(index as u8);
        }
    }
}

