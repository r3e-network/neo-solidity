# Neo Solidity Compiler - Professional Build System
# Author: Jimmy <jimmy@r3e.network>
# Repository: https://github.com/r3e-network/neo-solidity

.PHONY: all build clean test docs install format lint release help

all: build

build:
	@echo "🔨 Building Neo Solidity Compiler..."
	cargo build --release
	@echo "✅ Build complete"

test:
	@echo "🧪 Running tests..."
	cargo test
	@echo "✅ Tests passed"

clean:
	@echo "🧹 Cleaning build artifacts..."
	cargo clean
	@echo "✅ Clean complete"

format:
	@echo "🎨 Formatting code..."
	cargo fmt
	@echo "✅ Code formatted"

lint:
	@echo "🔍 Linting code..."
	cargo clippy -- -D warnings
	@echo "✅ Linting passed"

install: build
	@echo "📦 Installing neo-solc..."
	cargo install --path .
	@echo "✅ neo-solc installed successfully"

docs:
	@echo "📚 Building documentation..."
	cargo doc --no-deps
	@echo "✅ Documentation built"

release: clean build test
	@echo "🚀 Creating release..."
	cargo package
	@echo "✅ Release ready"

help:
	@echo "Neo Solidity Compiler - Build System"
	@echo ""
	@echo "Available targets:"
	@echo "  build    - Build the compiler"
	@echo "  test     - Run all tests"
	@echo "  clean    - Clean build artifacts"
	@echo "  format   - Format code"
	@echo "  lint     - Lint code"
	@echo "  docs     - Build documentation"
	@echo "  install  - Install neo-solc binary"
	@echo "  release  - Create release package"
	@echo "  help     - Show this help message"
	@echo ""
	@echo "Repository: https://github.com/r3e-network/neo-solidity"
