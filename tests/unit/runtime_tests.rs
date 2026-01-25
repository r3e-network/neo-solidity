//! Runtime execution unit tests.
//!
//! Tests memory management, calldata, and returndata operations.

use neo_solidity::runtime::execution::ExecutionContext;
use neo_solidity::runtime::{NeoRuntime, RuntimeConfig};
use sha3::{Digest, Keccak256};

#[cfg(test)]
mod runtime_tests {
    #[test]
    fn linear_memory_zero_extends_and_hashes() {
        let mut ctx = ExecutionContext::new(&RuntimeConfig::default()).expect("execution context");

        ctx.write_memory(4, &[0xAA, 0xBB]).expect("memory write");
        let snapshot = ctx.read_memory(0, 8).expect("memory read").to_vec();
        assert_eq!(snapshot[4], 0xAA);
        assert_eq!(snapshot[5], 0xBB);
        assert_eq!(ctx.memory_size(), 8);

        let hash = ctx.keccak_memory_slice(0, 8).expect("keccak hash");
        let mut reference = Keccak256::new();
        reference.update(&snapshot);
        assert_eq!(hash.as_slice(), reference.finalize().as_slice());
    }

    #[test]
    fn calldata_helpers_load_and_copy() {
        let mut ctx = ExecutionContext::new(&RuntimeConfig::default()).expect("execution context");
        ctx.initialize(&[], &[1, 2, 3, 4, 5, 6])
            .expect("context init");

        let word = ctx.calldataload_word(2);
        assert_eq!(&word[..4], &[3, 4, 5, 6]);

        ctx.calldatacopy(0, 1, 3).expect("calldata copy");
        assert_eq!(ctx.read_memory(0, 3).expect("memory slice"), &[2, 3, 4]);
    }

    #[test]
    fn returndatacopy_moves_bytes_into_memory() {
        let mut ctx = ExecutionContext::new(&RuntimeConfig::default()).expect("execution context");
        ctx.set_return_data(vec![10, 20, 30, 40]);
        ctx.returndatacopy(5, 1, 2).expect("returndatacopy");
        assert_eq!(ctx.returndatasize(), 4);
        assert_eq!(ctx.read_memory(5, 2).expect("memory slice"), &[20, 30]);
    }

    #[test]
    fn runtime_captures_return_payload() {
        let mut runtime = NeoRuntime::new(RuntimeConfig::default()).expect("runtime");
        let script = vec![0x0C, 0x03, b'f', b'o', b'o', 0x40];
        let result = runtime.execute(&script, &[]).expect("execution");
        assert!(result.is_success());
        assert_eq!(result.return_data, b"foo");
    }
}
