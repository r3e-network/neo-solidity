# Contracts

Contracts in Solidity are similar to classes in object-oriented languages. 

## Creating Contracts

On NeoVM, the concept of a "constructor" maps to the `_deploy(data, update)` entry point. This method is called automatically by Neo's `ContractManagement` service when the contract is deployed or updated on the blockchain.

```solidity
contract MyToken {
    string public name;
    address public owner;

    constructor(string memory _name) {
        name = _name;
        owner = msg.sender;
    }
}
```

When compiled to NeoVM, the `constructor` parameters are serialized and passed through the `data` array parameter of the `_deploy` method. The compiler automatically handles the serialization and initialization.

## Visibility and Getters

Solidity has four types of visibilities for functions and state variables:
* `external`: Part of the contract interface (Neo Manifest ABI). Can be called from other contracts.
* `public`: Part of the contract interface (Neo Manifest ABI). Can be called internally or externally.
* `internal`: Not part of the ABI. Can only be accessed internally or by derived contracts.
* `private`: Not part of the ABI. Can only be accessed by the contract they are defined in.

Getter functions for `public` state variables are automatically generated and exposed in the Neo manifest.

## Inheritance

Multiple inheritance is supported and resolved using C3 linearization, exactly identical to Ethereum Solidity. The `virtual` and `override` keywords work identically.

## Abstract Contracts and Interfaces

* **Abstract Contracts:** Contracts are abstract if at least one of their functions lacks an implementation.
* **Interfaces:** Interfaces are similar to abstract contracts, but they cannot have any functions implemented, and they cannot inherit from other interfaces.

Both features are perfectly supported on NeoVM. Interfaces are crucial for executing cross-contract calls using `System.Contract.Call`.

## Libraries

Libraries are similar to contracts, but their purpose is that they are deployed only once at a specific address and their code is reused. 
On Ethereum, external libraries require `delegatecall`. 

Because NeoVM does not support `delegatecall` (each contract has entirely isolated storage), **external libraries are not supported on Neo N3.** All library functions must be `internal`, forcing them to be inlined into the consuming contract at compile time.

## Authorization Patterns

Ethereum contracts typically authorize actions based on `msg.sender`. Neo N3 uses an explicit cryptographic witness model via `Runtime.checkWitness()`.

While `msg.sender` successfully maps to `Runtime.getCallingScriptHash()`, relying on witnesses is the native Neo paradigm:

```solidity
// ✅ Neo-idiomatic — explicit witness check
function withdraw(uint256 amount) public {
    require(Runtime.checkWitness(owner), "not authorized");
    // ...
}
```