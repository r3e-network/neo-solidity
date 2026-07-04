use super::*;

impl ExecutionContext {
    pub(crate) fn invoke_native_oracle(&mut self, method: &str, params: StackItem) -> StackItem {
        match method {
            "request" => {
                // Extract parameters: url, filter, callback_contract, callback_method, user_data, gas_for_response
                let (url, filter, cb_contract, cb_method, user_data, gas_for_response) =
                    if let StackItem::Array(args) = &params {
                        let borrowed = args.borrow();
                        let url = Self::stack_item_to_bytes(
                            borrowed.first().cloned().unwrap_or(StackItem::Null),
                        );
                        let filter = Self::stack_item_to_bytes(
                            borrowed.get(1).cloned().unwrap_or(StackItem::Null),
                        );
                        let cb_contract = Self::stack_item_to_bytes(
                            borrowed.get(2).cloned().unwrap_or(StackItem::Null),
                        );
                        let cb_method = Self::stack_item_to_bytes(
                            borrowed.get(3).cloned().unwrap_or(StackItem::Null),
                        );
                        let user_data = Self::stack_item_to_bytes(
                            borrowed.get(4).cloned().unwrap_or(StackItem::Null),
                        );
                        let gas = Self::extract_first_int(
                            &borrowed
                                .get(5)
                                .cloned()
                                .unwrap_or(StackItem::UnsignedInteger(0)),
                        );
                        (url, filter, cb_contract, cb_method, user_data, gas)
                    } else {
                        (
                            Vec::new(),
                            Vec::new(),
                            Vec::new(),
                            Vec::new(),
                            Vec::new(),
                            0,
                        )
                    };

                let id = self.oracle_next_request_id;
                self.oracle_next_request_id += 1;

                let request = OracleRequest {
                    id,
                    original_tx_hash: vec![0u8; 32],
                    url: String::from_utf8_lossy(&url).to_string(),
                    filter: String::from_utf8_lossy(&filter).to_string(),
                    callback_contract: cb_contract,
                    callback_method: String::from_utf8_lossy(&cb_method).to_string(),
                    user_data,
                    gas_for_response,
                };
                self.oracle_requests.insert(id, request);

                // Return 4-byte request id (matching original behavior)
                let id_bytes = (id as u32).to_le_bytes();
                StackItem::byte_array(id_bytes.to_vec())
            }
            "getprice" => StackItem::UnsignedInteger(self.oracle_price),
            "setprice" => {
                self.oracle_price = Self::extract_first_int(&params);
                StackItem::Null
            }
            "finish" => {
                // No-op in embedded runtime
                StackItem::Null
            }
            "verify" => StackItem::Boolean(true),
            "getoraclenodes" => {
                // Cockatrice: returns ECPoint[] of registered oracle nodes.
                // The embedded runtime returns a synthetic 1-node array
                // (matching the deterministic-test philosophy).
                let node_key: Vec<u8> = (0..33).map(|i| (i * 7 + 13) as u8).collect();
                StackItem::array(vec![StackItem::byte_array(node_key)])
            }
            "getrequests" => {
                // Cockatrice: returns an iterator over pending/finished requests.
                // The embedded runtime materializes the in-memory request set.
                let ids: Vec<u64> = self.oracle_requests.keys().copied().collect();
                StackItem::array(
                    ids.iter()
                        .map(|id| {
                            StackItem::byte_array((*id as u32).to_le_bytes().to_vec())
                        })
                        .collect(),
                )
            }
            "getrequest" => {
                // Cockatrice: returns a single OracleRequest struct by ID.
                // The embedded runtime serializes the request as a NeoVM Map
                // with the same field names used on-chain.
                let id = Self::extract_first_int(&params);
                if let Some(req) = self.oracle_requests.get(&id) {
                    let mut map = std::collections::HashMap::new();
                    map.insert(b"OriginalTxid".to_vec(), StackItem::byte_array(req.original_tx_hash.clone()));
                    map.insert(b"GasForResponse".to_vec(), StackItem::UnsignedInteger(req.gas_for_response));
                    map.insert(b"Url".to_vec(), StackItem::byte_array(req.url.as_bytes().to_vec()));
                    map.insert(b"Filter".to_vec(), StackItem::byte_array(req.filter.as_bytes().to_vec()));
                    map.insert(b"CallbackContract".to_vec(), StackItem::byte_array(req.callback_contract.clone()));
                    map.insert(b"CallbackMethod".to_vec(), StackItem::byte_array(req.callback_method.as_bytes().to_vec()));
                    map.insert(b"UserData".to_vec(), StackItem::byte_array(req.user_data.clone()));
                    StackItem::map(map)
                } else {
                    StackItem::Null
                }
            }
            _ => StackItem::Null,
        }
    }
}
