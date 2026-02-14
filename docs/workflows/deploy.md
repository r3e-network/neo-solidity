# Deploy Contracts

Deployment on Neo N3 uses two files:

- `<contract>.nef`
- `<contract>.manifest.json`

## Local deployment (Neo-Express)

The repository ships smoke scripts that create/reset local chains and validate invocation:

```bash
make test-deploy-smoke
make test-deploy-constructor-smoke
make test-deploy-permissions-smoke
```

## CLI deployment flow

```bash
neo-cli contract deploy build/MyContract.nef build/MyContract.manifest.json
```

Then invoke methods through your preferred Neo CLI/SDK flow.

## Constructor arguments on Neo

Neo deployment enters `_deploy(data, update)`.

For parameterized Solidity constructors:

1. Pass constructor data as a JSON array string in Neo-Express/CLI tooling (example: `"[7,\"hello\"]"`).
2. SDKs that support StackItems can pass an array directly.
3. Contract-to-contract deploy flows may pass serialized bytes (`abi.encode(...)` path).

Compiler note: deploy stubs rely on StdLib decode helpers and manifest permissions must allow required StdLib methods.

## Upgrade and destroy lifecycle

Use native ContractManagement calls (typically via devpack wrappers):

- `deploy`
- `update`
- `destroy`
- `getContract`

See [NeoVM Native Contracts](/neovm/native-contracts).
