# Examples

This folder contains reference Solidity contracts you can compile with the Neo Solidity compiler (`neo-solc`).

The repo keeps a small set of compiled artifacts (e.g. `TestContract.nef` / `TestContract.manifest.json`)
checked in for quick inspection. Larger/iterative outputs should go under `examples/out/` (ignored by git).

## Quick compile

```bash
target/release/neo-solc examples/TestContract.sol -I devpack -O2 -o /tmp/TestContract
```

## Included contracts

Most contracts in this folder compile to deployable Neo N3 artifacts today. Some are EVM-style
reference ports adapted for Neo N3.

### Compiles on Neo N3 today

- `TestContract.sol`: minimal storage getter/setter used by integration tests.
- `ERC20Token.sol`: ERC20-like token exercising NEP-17 detection.
- `ERC721Token.sol`: ERC721-style NFT example (Neo-adapted receiver checks; prefer NEP-11 examples for production).
- `Escrow.sol`: escrow sample.
- `GovernanceToken.sol`: governance token sample (Neo-adapted: `values[]` must be `0`; use NEP-17 transfers instead).
- `Lottery.sol`: lottery sample.
- `MultiSigWallet.sol`: multisig wallet sample (Neo-adapted; receives GAS via `onNEP17Payment`).
- `NameService.sol`: name service sample.
- `SimpleStorage.sol`: basic storage sample.
- `Staking.sol`: staking sample.
- `UniswapV2Pair.sol`: AMM pair sample (Neo-adapted; no native ETH-style value transfers).
- `new/Counter.sol`: counter with increment/decrement events.
- `new/Bank.sol`: banking contract with deposits, withdrawals, and transfers.
- `new/MultiSigWalletNEP17.sol`: multisig wallet for GAS/NEO via NEP-17 transfers.
- `new/Vault.sol`: token vault with deposits/withdrawals.
- `new/NFT.sol`: minimal NEP-11-style NFT implementation.

### Notes

- Prefer `new/NFT.sol` or `devpack/examples/CompleteNEP11NFT.sol` for a production NEP-11-style NFT on Neo N3.

To compile all new samples:

```bash
for f in examples/new/*.sol; do base=$(basename "$f" .sol); cargo run --quiet -- "$f" -o examples/out/$base; done
```

Manifests in `examples/out/` (ignored by git) will describe supported standards and ABI entries for inspection.
