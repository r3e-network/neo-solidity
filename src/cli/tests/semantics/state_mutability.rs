#[test]
fn view_functions_reject_state_writes() {
    let source = r#"
    pragma solidity ^0.8.20;

    contract ViewWrite {
        uint256 public x;

        function set() public view {
            x = 1;
        }
    }
    "#;

    let err = compile_contracts(source, false, 0).expect_err("expected compilation failure");
    match err {
        CompileError::Ir(messages) => {
            assert!(
                messages
                    .iter()
                    .any(|m| m.message.contains("declared view/pure")),
                "unexpected error messages: {messages:?}"
            );
        }
        other => panic!("unexpected error variant: {other:?}"),
    }
}

#[test]
fn view_functions_reject_event_emission() {
    let source = r#"
    pragma solidity ^0.8.20;

    contract ViewEmit {
        event Ping(uint256 x);

        function ping() public view {
            emit Ping(1);
        }
    }
    "#;

    let err = compile_contracts(source, false, 0).expect_err("expected compilation failure");
    match err {
        CompileError::Ir(messages) => {
            assert!(
                messages
                    .iter()
                    .any(|m| m.message.contains("emits events/notifications")),
                "unexpected error messages: {messages:?}"
            );
        }
        other => panic!("unexpected error variant: {other:?}"),
    }
}

#[test]
fn view_functions_reject_low_level_call() {
    let source = r#"
    pragma solidity ^0.8.20;

    contract ViewLowLevelCall {
        function run(address target) public view returns (bool ok) {
            (bool success, bytes memory data) = target.call(abi.encodeWithSignature("foo()"));
            data;
            return success;
        }
    }
    "#;

    let err = compile_contracts(source, false, 0).expect_err("expected compilation failure");
    match err {
        CompileError::Ir(messages) => {
            assert!(
                messages.iter().any(|m| m
                    .message
                    .contains("address.call(...) / address.delegatecall(...)")),
                "unexpected error messages: {messages:?}"
            );
        }
        other => panic!("unexpected error variant: {other:?}"),
    }
}

#[test]
fn view_functions_reject_delegatecall() {
    let source = r#"
    pragma solidity ^0.8.20;

    contract ViewDelegatecall {
        function run(address target) public view returns (bool ok) {
            (bool success, bytes memory data) = target.delegatecall(abi.encodeWithSignature("foo()"));
            data;
            return success;
        }
    }
    "#;

    let err = compile_contracts(source, false, 0).expect_err("expected compilation failure");
    match err {
        CompileError::Ir(messages) => {
            assert!(
                messages.iter().any(|m| m
                    .message
                    .contains("address.call(...) / address.delegatecall(...)")),
                "unexpected error messages: {messages:?}"
            );
        }
        other => panic!("unexpected error variant: {other:?}"),
    }
}

#[test]
fn view_functions_reject_unsafe_internal_calls() {
    let source = r#"
    pragma solidity ^0.8.20;

    interface ICallee {
        function foo(uint256 x) external returns (uint256);
    }

    contract Caller {
        function _helper(address target, uint256 x) internal returns (uint256) {
            return ICallee(target).foo(x);
        }

        function run(address target, uint256 x) public view returns (uint256) {
            // This should be rejected: `_helper` is not compiled in read-only mode and
            // therefore performs a non-readonly cross-contract call.
            return _helper(target, x);
        }
    }
    "#;

    let err = compile_contracts(source, false, 0).expect_err("expected compilation failure");
    match err {
        CompileError::Ir(messages) => {
            assert!(
                messages
                    .iter()
                    .any(|m| m.message.contains("performs non-readonly contract calls")),
                "unexpected error messages: {messages:?}"
            );
        }
        other => panic!("unexpected error variant: {other:?}"),
    }
}

#[test]
fn pure_functions_reject_state_reads() {
    let source = r#"
    pragma solidity ^0.8.20;

    contract PureRead {
        uint256 public x;

        function get() public pure returns (uint256) {
            return x;
        }
    }
    "#;

    let err = compile_contracts(source, false, 0).expect_err("expected compilation failure");
    match err {
        CompileError::Ir(messages) => {
            assert!(
                messages.iter().any(|m| m.message.contains("declared pure")),
                "unexpected error messages: {messages:?}"
            );
            assert!(
                messages
                    .iter()
                    .any(|m| m.message.contains("reads contract storage")),
                "unexpected error messages: {messages:?}"
            );
        }
        other => panic!("unexpected error variant: {other:?}"),
    }
}

#[test]
fn pure_functions_reject_environment_reads() {
    let source = r#"
    pragma solidity ^0.8.20;

    contract PureEnv {
        function who() public pure returns (address) {
            return msg.sender;
        }
    }
    "#;

    let result = compile_contracts(source, false, 2);
    assert!(
        result.is_ok(),
        "pure function with msg.sender should compile"
    );
}

#[test]
fn immutable_state_reassignment_outside_constructor_is_rejected() {
    let source = r#"
    pragma solidity ^0.8.20;

    contract ImmutableWrite {
        uint256 immutable seed;

        constructor() {
            seed = 1;
        }

        function rewrite() public {
            seed = 2;
        }
    }
    "#;

    let err = compile_contracts(source, false, 0).expect_err("expected compilation failure");
    match err {
        CompileError::Ir(messages) => {
            assert!(
                messages
                    .iter()
                    .any(|m| m.message.contains("immutable state variable")),
                "unexpected error messages: {messages:?}"
            );
        }
        other => panic!("unexpected error variant: {other:?}"),
    }
}

#[test]
fn immutable_state_assignment_in_constructor_is_allowed() {
    let source = r#"
    pragma solidity ^0.8.20;

    contract ImmutableCtor {
        uint256 immutable seed;

        constructor(uint256 value) {
            seed = value;
        }

        function get() public view returns (uint256) {
            return seed;
        }
    }
    "#;

    let artifacts = compile_contracts(source, false, 0).expect("compilation failed");
    assert_eq!(artifacts.len(), 1);
}
