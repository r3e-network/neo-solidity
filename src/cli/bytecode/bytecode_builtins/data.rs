pub(crate) fn push_data(bytecode: &mut Vec<u8>, data: &[u8]) {
    if data.len() <= u8::MAX as usize {
        bytecode.push(0x0C);
        bytecode.push(data.len() as u8);
    } else if data.len() <= u16::MAX as usize {
        bytecode.push(0x0D);
        bytecode.extend_from_slice(&(data.len() as u16).to_le_bytes());
    } else {
        bytecode.push(0x0E);
        bytecode.extend_from_slice(&(data.len() as u32).to_le_bytes());
    }
    bytecode.extend_from_slice(data);
}
