---
title: "Standards and Contracts: Ethereum to Neo Standard Mapping"
description: "Ethereum to Neo Standard Mapping from Standards and Contracts."
---

# Ethereum to Neo Standard Mapping

[Back to Standards and Contracts](/additional-material/neo-standards)

## Overview

| Ethereum          | Neo                                  | Key Differences                                                                       |
| ----------------- | ------------------------------------ | ------------------------------------------------------------------------------------- |
| ERC-20            | NEP-17                               | 4-parameter transfer, witness auth, no approve/allowance                              |
| ERC-721           | NEP-11                               | `bytes32` token IDs, 3-param transfer, witness auth, required `tokensOf`/`properties` |
| ERC-2981          | NEP-24                               | Multiple royalty recipients, `royaltyToken` parameter                                 |
| EIP-165           | Manifest `supportedstandards`        | Interface detection is manifest-based, no `supportsInterface()`                       |
| EIP-2612 (Permit) | `Runtime.checkWitness()`             | No permit needed — witness model replaces approvals                                   |
| EIP-1967 (Proxy)  | NEP-22/29/31                          | Native update + deploy callback + optional destroy, no proxy pattern                  |
