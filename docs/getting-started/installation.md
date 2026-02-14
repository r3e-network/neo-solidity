# Installation

## Build From Source

```bash
git clone https://github.com/r3e-network/neo-solidity.git
cd neo-solidity
cargo build --release
```

Compiler binary:

- `target/release/neo-solc`

## Validate Installation

```bash
./target/release/neo-solc --help
```

## Optional Tooling

Install and build TypeScript tooling packages:

```bash
npm --prefix tooling install
npm --prefix tooling run build
```

## Optional Runtime (C#)

```bash
dotnet build src/Neo.Sol.Runtime/Neo.Sol.Runtime.csproj --configuration Release
```

## Install Neo-Express (Local Deploy Testing)

If you do not already have `neoxp`:

```bash
dotnet tool install Neo.Express --tool-path ./build/dotnet-tools
```

## Environment Verification

```bash
make build
make test
```

For release confidence:

```bash
make production-gate
```
