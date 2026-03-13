# List of Known Bugs

Below, you can find a JSON-formatted list of some of the known security-relevant bugs in the Solidity compiler. The file itself is hosted in the Github repository. The list stretches back as far as version 0.3.0, bugs present only in older versions are not included.

There is another file called `bugs_by_version.json`, which can be used to check which bugs affect a specific version of the compiler.

Contract source verification tools and also other tools interacting with contracts should consult this list according to the following criteria:

- It is mildly suspicious if a contract was compiled with a nightly compiler version instead of a released version. This list does not keep track of unreleased or nightly versions.
- It is also mildly suspicious if a contract was compiled with a version that was not the most recent at the time of creation. For contracts created from other contracts, you have to follow the creation chain back to a transaction and use the date of that transaction as creation date.
- It is highly suspicious if a contract was compiled with a compiler that contains a known bug and the contract was created at a time where a newer compiler version containing a fix was already available.

## Neo Solidity Bugs

Because Neo Solidity uses the `solang-parser` for its frontend syntax analysis, it is immune to many of the code-generation bugs found in standard Solidity releases. 

However, Neo Solidity maintains its own tracking for semantic or compilation bugs related specifically to the EVM-to-NeoVM conversion process. 

Currently, there are no known security-critical bugs in the `0.14.x` release lineage.

Any future bugs discovered that affect the execution behavior of deployed `.nef` bytecode will be tracked here and in the GitHub issue tracker under the `security` label.