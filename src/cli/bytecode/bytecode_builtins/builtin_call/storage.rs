use super::*;

pub(crate) fn emit_storage_find(bytecode: &mut Vec<u8>, arg_count: usize) {
    // Stack input: [prefix] or [prefix, options]
    // Neo N3 Find signature: Find(context, prefix, options)
    // Stack order for syscall: [options, prefix, context]
    if arg_count == 1 {
        bytecode.push(0x10); // PUSH0 (FindOptions.None)
        bytecode.push(0x50); // SWAP -> [options, prefix]
    } else {
        // Assume the last arg is `options`.
        bytecode.push(0x50); // SWAP -> [options, prefix]
    }
    emit_syscall(bytecode, "System.Storage.GetContext");
    emit_syscall(bytecode, "System.Storage.Find");
}

pub(crate) fn emit_storage_put(bytecode: &mut Vec<u8>) {
    // Stack input: [key, value]
    // Stack order for System.Storage.Put: [value, key, context]
    bytecode.push(0x50); // SWAP -> [value, key]
    emit_syscall(bytecode, "System.Storage.GetContext");
    emit_syscall(bytecode, "System.Storage.Put");
    bytecode.push(0x11); // PUSH1 to avoid stack underflow on statement calls
}

pub(crate) fn emit_storage_get(bytecode: &mut Vec<u8>) {
    emit_syscall(bytecode, "System.Storage.GetReadOnlyContext");
    emit_syscall(bytecode, "System.Storage.Get");
}

pub(crate) fn emit_storage_delete(bytecode: &mut Vec<u8>) {
    emit_syscall(bytecode, "System.Storage.GetContext");
    emit_syscall(bytecode, "System.Storage.Delete");
    bytecode.push(0x11); // PUSH1
}
