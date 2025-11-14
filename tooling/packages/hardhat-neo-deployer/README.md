# @neo-solidity/hardhat-neo-deployer

Hardhat task collection that mirrors Foundry/Hardhat deployment ergonomics for Neo contracts. **This package is still a placeholder** – it wires the tasks into Hardhat so you can explore the intended UX, but none of the tasks broadcast real Neo transactions yet. All functionality currently terminates with a `HardhatPluginError` explaining that deployment is not implemented.

## Status

| Feature | Status |
| --- | --- |
| `neo-deploy` / `neo-deploy-batch` tasks | ⚠️ Stub – prints CLI flow but does not submit transactions |
| Gas estimation task (`neo-deploy-estimate`) | ⚠️ Stub – returns heuristic numbers, not RPC-calculated figures |
| Account utilities (`hardhat neo-account ...`) | ⚠️ Stub |
| RPC client (`NeoRpcClient`) | ✅ Typed wrapper around Neo JSON-RPC; usable outside of the tasks |

If you need to deploy contracts today, use native Neo tooling (`neo-cli`, `neo-express`, or the Neo Foundry CLI) and treat this package as a preview of the eventual Hardhat workflow.

## Installation

```bash
npm install --save-dev @neo-solidity/hardhat-neo-deployer
```

Then enable the plugin in your `hardhat.config.(ts|js)`:

```ts
import "@neo-solidity/hardhat-neo-deployer";

export default {
  neoNetworks: {
    testnet: {
      rpcUrls: ["https://testnet1.neo.coz.io:443"],
      magic: 894710606,
      accounts: ["0x..."] // private keys
    }
  }
};
```

## Tasks (prototype)

- `npx hardhat neo-deploy --contract MyToken` – runs through the deployment flow and immediately throws with a "not implemented" message.
- `npx hardhat neo-deploy-batch --config deployments.json` – reads your config, simulates output, then throws.
- `npx hardhat neo-deploy-estimate --contract MyToken` – prints heuristic gas numbers based purely on artifact size.

Because all tasks throw early, integrate them only if you want to exercise the future UX or hook custom scripts around the exposed API objects (`hre.neoDeploy.deployer`, `hre.neoDeploy.rpc`, etc.).

## When will this change?

The long‑term plan is to:

1. Implement NEF/manifest transaction assembly via `NeoRpcClient`.
2. Add account/key management (signers, hardware wallets, Ledger support).
3. Provide explorer verification hooks and deployment summaries identical to the Hardhat EVM flow.

Those pieces are being tracked in the project roadmap. Until they land, expect every task to terminate with `HardhatPluginError: <feature> is not available because Neo deployments are not implemented...`.
