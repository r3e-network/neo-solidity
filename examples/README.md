# Examples

This folder contains reference Solidity contracts you can compile with the Neo Solidity compiler (`cargo run -- <file>`). We keep compiled artifacts out of the repo to avoid churn; run the commands below to regenerate them locally.

## Quick compile

```bash
cargo run --quiet -- examples/TestContract.sol -o /tmp/TestContract
```

## Included contracts

- `TestContract.sol`: minimal storage getter/setter used by integration tests.
- `ERC20Token.sol`: ERC20-like token exercising NEP-17 detection.
- `ERC721Token.sol`: ERC721-like NFT exercising NEP-11 detection.
- `GovernanceToken.sol`: governance token sample.
- `MultiSigWallet.sol`: simple multisig wallet.
- `UniswapV2Pair.sol`: AMM pair sample.
- `new/Counter.sol`: counter with increment/decrement events.
- `new/Vault.sol`: token vault with deposits/withdrawals.
- `new/NFT.sol`: minimal NEP-11-style NFT implementation.

To compile all new samples:

```bash
for f in examples/new/*.sol; do base=$(basename "$f" .sol); cargo run --quiet -- "$f" -o examples/out/$base; done
```

Manifests in `examples/out/` (ignored by git) will describe supported standards and ABI entries for inspection.
