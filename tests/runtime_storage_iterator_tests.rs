use neo_solidity::runtime::execution::ExecutionContext;
use neo_solidity::runtime::storage::StorageManager;
use neo_solidity::runtime::RuntimeConfig;
use serde_json;

#[test]
fn storage_find_respects_prefix_and_overlay() {
    let mut storage = StorageManager::new(&RuntimeConfig::default()).expect("storage");
    let mut ctx = ExecutionContext::new(&RuntimeConfig::default()).expect("ctx");
    let account = "0x0000000000000000000000000000000000000000";
    storage.set(account, b"aa", b"1").expect("set base storage");
    ctx.bind_storage(account, &mut storage).expect("bind");

    // Overlay change: delete "aa", add "ab"
    ctx.initialize(
        &[
            // Delete key "aa"
            0x0C, 0x02, b'a', b'a', // key "aa"
            0x41, 155, 246, 103, 206, // Storage.GetContext
            0x41, 47, 88, 197, 237, // Storage.Delete
            // Put key "ab" = "2"
            0x0C, 0x02, b'a', b'b', // key "ab"
            0x0C, 0x01, b'2', // value "2"
            0x41, 155, 246, 103, 206, // Storage.GetContext
            0x50, // SWAP to reorder stack [key, context, value]
            0x41, 230, 63, 24, 132, // Storage.Put
            // Find with prefix "a"
            0x41, 155, 246, 103, 206, // Storage.GetContext
            0x0C, 0x01, b'a', // prefix "a"
            0x41, 223, 48, 184, 154, // Storage.Find
            // Advance iterator and read first value
            0x41, 156, 8, 237, 156, // Iterator.Next
            0x45,                   // DROP bool
            0x41, 243, 84, 191, 29, // Iterator.Value
            0x50,                   // SWAP to drop iterator token
            0x45,                   // DROP token
            0x40, // RET
        ],
        &[],
    )
    .expect("init");

    while !ctx.step().expect("step").halted {}
    let result = ctx.return_data();
    let entry: Vec<serde_json::Value> =
        serde_json::from_slice(result).expect("iterator value json");
    let key_bytes = entry
        .get(0)
        .and_then(|k| k.get("ByteArray"))
        .and_then(|b| b.as_array())
        .cloned()
        .unwrap_or_default();
    let key: Vec<u8> = key_bytes
        .into_iter()
        .filter_map(|n| n.as_u64().map(|n| n as u8))
        .collect();

    assert_eq!(key, b"ab");
}
