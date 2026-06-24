use super::*;

pub(crate) fn emit_syscall_builtin(bytecode: &mut Vec<u8>, name: &str, arg_count: usize) {
    if name == "System.Storage.Find" && arg_count == 2 {
        // Neo N3 Storage.Find(context, prefix, options)
        bytecode.push(0x10); // FindOptions.None
    }
    emit_syscall(bytecode, name);
    if is_void_syscall(name) {
        bytecode.push(0x11); // PUSH1 for void syscalls
    }
}
