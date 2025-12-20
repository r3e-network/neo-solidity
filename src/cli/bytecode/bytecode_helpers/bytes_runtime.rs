fn emit_new_buffer(bytecode: &mut Vec<u8>) {
    bytecode.push(0x88); // NEWBUFFER
}

fn emit_convert(bytecode: &mut Vec<u8>, target: ir::ConvertTarget) {
    let code = match target {
        ir::ConvertTarget::Any => 0x00,
        ir::ConvertTarget::Boolean => 0x20,
        ir::ConvertTarget::Integer => 0x21,
        ir::ConvertTarget::ByteArray => 0x28,
        ir::ConvertTarget::Array => 0x40,
        ir::ConvertTarget::Map => 0x48,
    };

    bytecode.push(0xDB); // CONVERT
    bytecode.push(code);
}
