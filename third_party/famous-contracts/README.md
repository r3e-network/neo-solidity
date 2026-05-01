# Famous Solidity Contracts (Vendored Upstream Sources)

This directory stores upstream Solidity contract sources used by the Neo DevPack for Solidity compatibility audit.

- Source root: `third_party/famous-contracts/sources/`
- Target list: `docs/data/famous-contracts-targets.json` (entries with `source: "npm"`)
- Vendor sync script: `node scripts/vendor_famous_contracts.js`
- Integrity check script: `node scripts/verify_famous_contract_sources.js`

Rules:

1. Vendored files must remain byte-identical to upstream package sources.
1. Do not hand-edit vendored contract files.
1. Update by re-running the vendor script, then run audit + verify scripts.

Licensing:

- Each vendored file keeps its upstream SPDX/license header.
- Respect upstream package licenses when redistributing.
