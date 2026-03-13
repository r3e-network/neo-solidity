# Keyword Index

This page serves as a reference for all keywords reserved in the Neo Solidity compiler. Because Neo Solidity uses the upstream `solang-parser`, it reserves the exact same keywords as mainline Ethereum Solidity.

## Reserved Keywords

The following keywords are reserved and cannot be used as identifiers (e.g., variable or function names):

*   `after`
*   `alias`
*   `apply`
*   `auto`
*   `byte`
*   `case`
*   `catch`
*   `copyof`
*   `default`
*   `define`
*   `final`
*   `implements`
*   `in`
*   `inline`
*   `let`
*   `macro`
*   `match`
*   `mutable`
*   `null`
*   `of`
*   `partial`
*   `promise`
*   `reference`
*   `relocatable`
*   `sealed`
*   `sizeof`
*   `static`
*   `supports`
*   `switch`
*   `try`
*   `typedef`
*   `typeof`
*   `unchecked`

::: tip 💡 NeoVM Difference: Ignored Keywords
While keywords like `unchecked` and `assembly` are fully parsed by the frontend, the Neo Solidity compiler explicitly ignores or no-ops them during code generation because their concepts (like integer boundary overflows or low-level EVM stack access) do not apply to the Neo N3 architecture.
:::