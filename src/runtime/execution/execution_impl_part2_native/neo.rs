impl ExecutionContext {
    fn invoke_native_neo(&mut self, method: &str, params: StackItem) -> StackItem {
        match method {
            "symbol" => StackItem::byte_array(b"NEO".to_vec()),
            "decimals" => StackItem::UnsignedInteger(0),
            "totalsupply" => StackItem::UnsignedInteger(self.neo_total_supply),
            "getgasperblock" => StackItem::UnsignedInteger(5),
            "getcommittee" => {
                // Return deterministic committee ECPoint public keys (ByteString).
                let member1 = (0u8..33u8).collect::<Vec<_>>();
                let member2 = (33u8..66u8).collect::<Vec<_>>();
                StackItem::array(vec![
                    StackItem::byte_array(member1),
                    StackItem::byte_array(member2),
                ])
            }
            "getnextblockvalidators" => {
                // Return deterministic validator ECPoint public keys (ByteString).
                let validator = (100u8..133u8).collect::<Vec<_>>();
                StackItem::array(vec![StackItem::byte_array(validator)])
            }
            "getcandidates" => {
                // Neo N3: getCandidates() -> Array<Struct(ECPoint publicKey, BigInteger votes)>
                //
                // Represent structs as arrays in the runtime, matching the compiler's
                // array-backed struct lowering.
                let cand1_key = (10u8..43u8).collect::<Vec<_>>();
                let cand2_key = (43u8..76u8).collect::<Vec<_>>();
                StackItem::array(vec![
                    StackItem::array(vec![
                        StackItem::byte_array(cand1_key),
                        StackItem::UnsignedInteger(1_000),
                    ]),
                    StackItem::array(vec![
                        StackItem::byte_array(cand2_key),
                        StackItem::UnsignedInteger(2_000),
                    ]),
                ])
            }
            "getaccountstate" => {
                // Neo N3: getAccountState(account) -> NeoAccountState? (nullable)
                // Shape: [balance, balanceHeight, voteTo (ECPoint bytes or null), lastGasPerVote]
                if let StackItem::Array(args) = params {
                    if let Some(StackItem::ByteArray(acc)) = args.borrow().first() {
                        if let Some(balance) =
                            self.neo_balances.get(acc.borrow().as_slice()).copied()
                        {
                            let height =
                                self.block_height.unwrap_or(self.default_block_height);
                            return StackItem::array(vec![
                                StackItem::UnsignedInteger(balance),
                                StackItem::UnsignedInteger(height),
                                // Return `null` for voteTo by default so the compiler's
                                // helper can normalize it to empty bytes.
                                StackItem::Null,
                                StackItem::UnsignedInteger(0),
                            ]);
                        }
                    }
                }
                StackItem::Null
            }
            "vote" | "registercandidate" | "unregistercandidate" | "setgasperblock" => {
                // Keep these as deterministic placeholders for now.
                StackItem::Boolean(true)
            }
            "balanceof" => {
                if let StackItem::Array(args) = params {
                    if let Some(StackItem::ByteArray(acc)) = args.borrow().first() {
                        let bal = *self.neo_balances.get(acc.borrow().as_slice()).unwrap_or(&0);
                        StackItem::UnsignedInteger(bal)
                    } else {
                        StackItem::UnsignedInteger(0)
                    }
                } else {
                    StackItem::UnsignedInteger(0)
                }
            }
            "transfer" => self.handle_native_transfer(true, params),
            _ => StackItem::Null,
        }
    }
}
