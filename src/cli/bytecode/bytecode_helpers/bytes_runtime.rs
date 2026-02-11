fn emit_new_buffer(bytecode: &mut Vec<u8>) {
    bytecode.push(0x88); // NEWBUFFER
}

fn stack_item_type_code(target: ir::ConvertTarget) -> u8 {
    match target {
        ir::ConvertTarget::Any => 0x00,
        ir::ConvertTarget::Boolean => 0x20,
        ir::ConvertTarget::Integer => 0x21,
        ir::ConvertTarget::ByteArray => 0x28,
        ir::ConvertTarget::Array => 0x40,
        ir::ConvertTarget::Map => 0x48,
    }
}

fn emit_convert(bytecode: &mut Vec<u8>, target: ir::ConvertTarget) {
    bytecode.push(0xDB); // CONVERT
    bytecode.push(stack_item_type_code(target));
}

fn emit_is_type(bytecode: &mut Vec<u8>, target: ir::ConvertTarget) {
    bytecode.push(0xD9); // ISTYPE
    bytecode.push(stack_item_type_code(target));
}
