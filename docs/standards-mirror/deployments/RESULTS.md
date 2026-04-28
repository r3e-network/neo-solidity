# Standards Mirror — TestNet Deployments

- Generated: `2026-04-28T03:47:02.543Z`
- RPC: `http://seed1t5.neo.org:20332`
- Network magic: `894710606`
- Deployer: `NhMYxG5ATmRjSy6ocnPxrA2DiYba6xhFqu`

| Pair | Implementation | Address | Deploy Tx | Tests |
|---|---|---|---|---|
| ERC-20 ↔ NEP-17 — Fungible Token | solidity | `NZbQsZAbH3eBdZZYShj6CgG1ZkVEbjZhwF` | `0x37897c9d8b04c7d87baf2e256621d9980145fa2ee8891c9b477c9783985be43e` | 5/5 |
| ERC-20 ↔ NEP-17 — Fungible Token | csharp | `NRGNZQRrb5TuDo4fA5KPiqZQB29Uybp1zJ` | `-` | 4/4 |
| ERC-721 ↔ NEP-11 — Non-Fungible Token | solidity | `NbTK8px52xHxJ5zSJvVFqBujZ5eQEV4dYt` | `0x2bac122c5803ea38cc90c26115564d82bd8cd54d4c430664a5da7166adf26674` | 3/3 |
| ERC-721 ↔ NEP-11 — Non-Fungible Token | csharp | `NbuB1V5es6YBtPfVrW4R9bDtxDieuZoK38` | `-` | 3/5 |
| ERC-2981 ↔ NEP-24 — NFT Royalty | solidity | `NQhcPMzycbfy5h4ZBg7vrbAvioa41KdR6i` | `-` | 3/3 |
| ERC-2981 ↔ NEP-24 — NFT Royalty | csharp | `NgTke4MQShakWQpPvskjqX1XEmpMF4EmSC` | `-` | 2/3 |
| ERC-3525 — Semi-Fungible Bond | solidity | `NdzbQnww1HMVDUgZtZzrfN5TvxFTBoBTW6` | `0x16953f75ec84751dd7ae3e6ce8804efdb9b09e6510ecd3716ef1534defa22a2e` | 4/4 |
| ERC-3525 — Semi-Fungible Bond | csharp | `NVpt23PJU2ZbEHXmDkzEqCfoE9NQfEopNZ` | `-` | 3/3 |

## ERC-20 ↔ NEP-17 — Fungible Token

### solidity
- Contract address: `NZbQsZAbH3eBdZZYShj6CgG1ZkVEbjZhwF`
- Contract hash: `d76434af829dc4c936c12648aa77932fa94c0f96`
- Deploy tx: `0x37897c9d8b04c7d87baf2e256621d9980145fa2ee8891c9b477c9783985be43e`
  - ✅ `read` symbol
  - ✅ `read` decimals
  - ✅ `write` faucet tx=`0x538ac547448db6f249bdb339332f6cc878f507d2e5db80f41f1e0d40456ce0f5`
  - ✅ `read` balanceOf
  - ✅ `read` totalSupply

### csharp
- Contract address: `NRGNZQRrb5TuDo4fA5KPiqZQB29Uybp1zJ`
- Contract hash: `1f3a9b414de1c60434543dd8a05ac5e08b75b43a`
- Deploy tx: `-`
  - ✅ `read` symbol
  - ✅ `read` decimals
  - ✅ `read` totalSupply
  - ✅ `read` balanceOf

## ERC-721 ↔ NEP-11 — Non-Fungible Token

### solidity
- Contract address: `NbTK8px52xHxJ5zSJvVFqBujZ5eQEV4dYt`
- Contract hash: `48b5f8f579810b402fed660844145fed406f77aa`
- Deploy tx: `0x2bac122c5803ea38cc90c26115564d82bd8cd54d4c430664a5da7166adf26674`
  - ✅ `read` symbol
  - ✅ `write` mint tx=`0xcf23cd1f8dde494c721139a8154aea51fe1ad29c87df8bb01f4266e100ba19f9`
  - ✅ `read` balanceOf

### csharp
- Contract address: `NbuB1V5es6YBtPfVrW4R9bDtxDieuZoK38`
- Contract hash: `15c664d51340a102490dbf5dec5647f541775baf`
- Deploy tx: `-`
  - ✅ `read` symbol
  - ✅ `read` decimals
  - ✅ `write` mint tx=`0xa19b9fbdf26dc3f1eb792d183db722fa7448252a3f88df20d2719635eab6b82b`
  - ❌ `read` balanceOf reason=`actual=3 (raw type=Integer, value=3), expected 1`
  - ❌ `read` totalSupply reason=`actual=3 (raw type=ByteString, value=Aw==), expected 1`

## ERC-2981 ↔ NEP-24 — NFT Royalty

### solidity
- Contract address: `NQhcPMzycbfy5h4ZBg7vrbAvioa41KdR6i`
- Contract hash: `ade57dfd9ad85fff8dca3845cf22206346468234`
- Deploy tx: `-`
  - ✅ `read` symbol
  - ✅ `write` mint tx=`0xb4ae32e8ffdfbe787610f635ab5e2d3eb8a517de86ceedbe6d5f1b94fbe8db1e`
  - ✅ `write` setDefaultRoyalty tx=`0x70c703a1cef771c540b6864b49d1bdbffb603f1e21e9247d04af075181ce66c4`

### csharp
- Contract address: `NgTke4MQShakWQpPvskjqX1XEmpMF4EmSC`
- Contract hash: `bf3fe7eb875750c81c2915d53123c380685a65e1`
- Deploy tx: `-`
  - ✅ `read` symbol
  - ✅ `write` mint tx=`0xf4bcf408ace3734019a2e76870f94e52d825583dcb18536de7281ef5ed77e990`
  - ❌ `read` totalSupply reason=`actual=2 (raw type=ByteString, value=Ag==), expected 1`

## ERC-3525 — Semi-Fungible Bond

### solidity
- Contract address: `NdzbQnww1HMVDUgZtZzrfN5TvxFTBoBTW6`
- Contract hash: `d0fd56dad510d54ca7877bab2c578d63b82a52c6`
- Deploy tx: `0x16953f75ec84751dd7ae3e6ce8804efdb9b09e6510ecd3716ef1534defa22a2e`
  - ✅ `read` symbol
  - ✅ `write` mint tx=`0x15bec1664e3ff23768d6ea519af1560b6f2989f329d0971bda93c51bfa1d9c1d`
  - ✅ `read` balanceOfToken
  - ✅ `read` slotOf

### csharp
- Contract address: `NVpt23PJU2ZbEHXmDkzEqCfoE9NQfEopNZ`
- Contract hash: `fcfde62a4764cbcd9b35615084e0075c4bddba6c`
- Deploy tx: `-`
  - ✅ `read` symbol
  - ✅ `read` valueDecimals
  - ✅ `write` mint tx=`0xfaa4a36ca80b7634ea447232d8e845ab8935fae19ecf8956491c58c1941211ab`
