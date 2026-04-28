# Standards Mirror — TestNet Deployments

- Generated: `2026-04-28T01:26:19.159Z`
- RPC: `http://seed1t5.neo.org:20332`
- Network magic: `894710606`
- Deployer: `NhMYxG5ATmRjSy6ocnPxrA2DiYba6xhFqu`

| Pair | Implementation | Address | Deploy Tx | Tests |
|---|---|---|---|---|
| ERC-20 ↔ NEP-17 — Fungible Token | solidity | `NaKd35AaXvYiLfKngxfuPuxQrFqtiRG1Ns` | `0x47da2da24d82cc25b7c827cc24e950722242600978c7ffca49a4ce9c5f0e949e` | 5/5 |
| ERC-20 ↔ NEP-17 — Fungible Token | csharp | `NRGNZQRrb5TuDo4fA5KPiqZQB29Uybp1zJ` | `-` | 4/4 |
| ERC-721 ↔ NEP-11 — Non-Fungible Token | solidity | `Ndfq3zG5NEe85tBZQBC4NJLjbeVRu9TwFn` | `0x28821263f9132b576acb4153a81eba8e2878e83200ccc7d53b1b0e1f54e96fb8` | 3/3 |
| ERC-721 ↔ NEP-11 — Non-Fungible Token | csharp | `NbuB1V5es6YBtPfVrW4R9bDtxDieuZoK38` | `-` | 3/5 |
| ERC-2981 ↔ NEP-24 — NFT Royalty | solidity | `NQhcPMzycbfy5h4ZBg7vrbAvioa41KdR6i` | `0x36e5acd55c1ebf99a425fae9f93e52385a0bf871e003f6c591de0c888e417ee5` | 3/3 |
| ERC-2981 ↔ NEP-24 — NFT Royalty | csharp | `NgTke4MQShakWQpPvskjqX1XEmpMF4EmSC` | `-` | 3/3 |
| ERC-3525 — Semi-Fungible Bond | solidity | `NeBJFQY6UAqyQdYXqT6sn4A45gJRqSFkdN` | `0x7a244abbc2c77cf26ef907a0a3b7bfbddd11221bb90fbbddef9b71c796b0eaf9` | 4/4 |
| ERC-3525 — Semi-Fungible Bond | csharp | `NVpt23PJU2ZbEHXmDkzEqCfoE9NQfEopNZ` | `0x9b8dc510c18c27aad853f177c54ef85dd040f35aab14e1cde147d05a2b1cefba` | 3/3 |

## ERC-20 ↔ NEP-17 — Fungible Token

### solidity
- Contract address: `NaKd35AaXvYiLfKngxfuPuxQrFqtiRG1Ns`
- Contract hash: `f6f1318d3a215df624202590929ae53686ec0a9e`
- Deploy tx: `0x47da2da24d82cc25b7c827cc24e950722242600978c7ffca49a4ce9c5f0e949e`
  - ✅ `read` symbol
  - ✅ `read` decimals
  - ✅ `write` faucet tx=`0x91b9eb5e3ac8c4d84db51bde2ceb7e9d8bb3c6091347413ed01bd8f410c60d43`
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
- Contract address: `Ndfq3zG5NEe85tBZQBC4NJLjbeVRu9TwFn`
- Contract hash: `e12679b0246e4a0a7b546f1e994fcf9199a6c5c2`
- Deploy tx: `0x28821263f9132b576acb4153a81eba8e2878e83200ccc7d53b1b0e1f54e96fb8`
  - ✅ `read` symbol
  - ✅ `write` mint tx=`0x66f1f611154d0cd203a5e539c752a469ae29b62d7c4df57417d76a227f77e055`
  - ✅ `read` balanceOf

### csharp
- Contract address: `NbuB1V5es6YBtPfVrW4R9bDtxDieuZoK38`
- Contract hash: `15c664d51340a102490dbf5dec5647f541775baf`
- Deploy tx: `-`
  - ✅ `read` symbol
  - ✅ `read` decimals
  - ✅ `write` mint tx=`0x013fc7873dfb7d3d2afdf2a538763b8c10b2dee902c6bb510368937788f21c9e`
  - ❌ `read` balanceOf reason=`actual=2 (raw type=Integer, value=2), expected 1`
  - ❌ `read` totalSupply reason=`actual=2 (raw type=ByteString, value=Ag==), expected 1`

## ERC-2981 ↔ NEP-24 — NFT Royalty

### solidity
- Contract address: `NQhcPMzycbfy5h4ZBg7vrbAvioa41KdR6i`
- Contract hash: `ade57dfd9ad85fff8dca3845cf22206346468234`
- Deploy tx: `0x36e5acd55c1ebf99a425fae9f93e52385a0bf871e003f6c591de0c888e417ee5`
  - ✅ `read` symbol
  - ✅ `write` mint tx=`0xe93230aa3dc33254f41893c13f20e8b4cf1c32cc5832d676c61e888e4adbe0a2`
  - ✅ `write` setDefaultRoyalty tx=`0x6e1971653ca91a8ffacde2b907eb62fe6ccffdec3165aa6559647b46f25755a0`

### csharp
- Contract address: `NgTke4MQShakWQpPvskjqX1XEmpMF4EmSC`
- Contract hash: `bf3fe7eb875750c81c2915d53123c380685a65e1`
- Deploy tx: `-`
  - ✅ `read` symbol
  - ✅ `write` mint tx=`0x50930d0824161ee5283a362162c6fb290d4e2a8e666d8a5a93b9ce91a7df956b`
  - ✅ `read` totalSupply

## ERC-3525 — Semi-Fungible Bond

### solidity
- Contract address: `NeBJFQY6UAqyQdYXqT6sn4A45gJRqSFkdN`
- Contract hash: `cee12844b9a9555c29184d59a042384a9e6a58c8`
- Deploy tx: `0x7a244abbc2c77cf26ef907a0a3b7bfbddd11221bb90fbbddef9b71c796b0eaf9`
  - ✅ `read` symbol
  - ✅ `write` mint tx=`0xec86aa0594916823c0836955c56f73f816b3fe522aa8358abb94b853b4552d1a`
  - ✅ `read` balanceOfToken
  - ✅ `read` slotOf

### csharp
- Contract address: `NVpt23PJU2ZbEHXmDkzEqCfoE9NQfEopNZ`
- Contract hash: `fcfde62a4764cbcd9b35615084e0075c4bddba6c`
- Deploy tx: `0x9b8dc510c18c27aad853f177c54ef85dd040f35aab14e1cde147d05a2b1cefba`
  - ✅ `read` symbol
  - ✅ `read` valueDecimals
  - ✅ `write` mint tx=`0x7f59cd51819bf2aae5a258b72bbba2d4ecf9e1aa3caee7d2d3a13d091d8f8eaa`
