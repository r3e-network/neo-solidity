#[test]
fn receive_function_is_lowered_to_onnep17payment() {
    let source = r#"
    pragma solidity ^0.8.19;

    contract Receiver {
        event Deposit(address from, uint256 amount);

        receive() external payable {
            if (msg.value > 0) {
                emit Deposit(msg.sender, msg.value);
            }
        }
    }
    "#;

    let artifacts = compile_contracts(source, false, 2).expect("compilation failed");
    assert_eq!(artifacts.len(), 1);

    let methods = artifacts[0].manifest["abi"]["methods"]
        .as_array()
        .expect("methods array");
    let on_payment = methods
        .iter()
        .find(|m| m.get("name").and_then(Value::as_str) == Some("onNEP17Payment"))
        .expect("onNEP17Payment method should be present");

    let params = on_payment
        .get("parameters")
        .and_then(Value::as_array)
        .expect("parameters array");
    assert_eq!(params.len(), 3, "onNEP17Payment should have 3 parameters");

    assert!(
        artifacts[0]
            .manifest
            .get("features")
            .and_then(Value::as_object)
            .is_some_and(|features| features.is_empty()),
        "expected `manifest.features` to be an empty object for Neo N3 compatibility"
    );
}

#[test]
fn contract_can_define_onnep17payment_and_receive() {
    let source = r#"
    pragma solidity ^0.8.19;

    contract BothPaymentHooks {
        event Deposit(address from, uint256 amount);

        function onNEP17Payment(address from, uint256 amount, bytes memory /*data*/) external {
            if (amount > 0) {
                emit Deposit(from, amount);
            }
        }

        // EVM-style `receive()` kept for Solidity source compatibility.
        receive() external payable {
            // On Neo, `msg.value` is only mapped for `onNEP17Payment`.
            if (msg.value > 0) {
                emit Deposit(msg.sender, msg.value);
            }
        }
    }
    "#;

    let artifacts = compile_contracts(source, false, 2).expect("compilation failed");
    assert_eq!(artifacts.len(), 1);

    let methods = artifacts[0].manifest["abi"]["methods"]
        .as_array()
        .expect("methods array");

    assert!(
        methods
            .iter()
            .any(|m| m.get("name").and_then(Value::as_str) == Some("onNEP17Payment")),
        "expected explicit onNEP17Payment method"
    );
    assert!(
        methods
            .iter()
            .any(|m| m.get("name").and_then(Value::as_str) == Some("receive")),
        "expected receive() to remain a callable Neo method when onNEP17Payment is already defined"
    );

    assert!(
        artifacts[0]
            .manifest
            .get("features")
            .and_then(Value::as_object)
            .is_some_and(|features| features.is_empty()),
        "expected `manifest.features` to be an empty object for Neo N3 compatibility"
    );
}
