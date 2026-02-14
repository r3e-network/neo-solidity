# Documentation Guide

The project documentation is served as a VitePress website from `docs/`.

## Local docs development

```bash
npm install
npm run docs:dev
```

## Build static docs

```bash
npm run docs:build
```

Output directory:

- `docs/.vitepress/dist`

## Documentation structure

- `docs/getting-started/` - installation and first-compile flow
- `docs/workflows/` - compile/deploy/test/production procedures
- `docs/solidity/` - support and behavior notes
- `docs/mapping/` - EVM to NeoVM semantic mapping
- `docs/neovm/` - native contracts and syscall model
- `docs/manifests/` - manifest fields and policy controls
- `docs/devpack/` - devpack capabilities and standards
- `docs/reference/` - CLI, architecture, runtime, errors, parity notes

## Canonical technical references

- `docs/SOLIDITY_SUPPORT_MATRIX.md`
- `docs/RUNTIME_SPEC.md`
- `docs/ARCHITECTURE.md`
- `docs/ERROR_REFERENCE.md`
- `docs/NEO_VM_PARITY_TODO.md`

## Archived content

Historical documents are tracked under `docs/archive/`.
