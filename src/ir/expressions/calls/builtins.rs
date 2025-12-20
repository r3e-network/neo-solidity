include!("builtins/helpers.rs");
include!("builtins/member_access.rs");
include!("builtins/member_runtime.rs");
include!("builtins/member_syscalls.rs");
include!("builtins/member_storage.rs");
include!("builtins/member_neo.rs");
include!("builtins/member_nativecalls.rs");
include!("builtins/resolved.rs");

fn try_lower_builtin_call(
    func: &Expression,
    args: &[Expression],
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> Option<bool> {
    if let Some(result) = try_lower_member_builtin(func, args, ctx, instructions) {
        return Some(result);
    }

    try_lower_resolved_builtin_call(func, args, ctx, instructions)
}
