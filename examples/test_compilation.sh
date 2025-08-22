#!/bin/bash
# Test script for Neo Solidity Compiler
# Author: Jimmy <jimmy@r3e.network>
# Tests complete compilation to Neo N3 contract format

set -e

echo "🔨 Testing Neo Solidity Compiler - Neo N3 Contract Generation"
echo "============================================================"

# Create a simple test contract
cat > TestContract.sol << 'EOF'
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

contract TestContract {
    uint256 private value;
    
    event ValueChanged(uint256 newValue);
    
    constructor(uint256 _initialValue) {
        value = _initialValue;
        emit ValueChanged(_initialValue);
    }
    
    function setValue(uint256 _value) public {
        value = _value;
        emit ValueChanged(_value);
    }
    
    function getValue() public view returns (uint256) {
        return value;
    }
}
EOF

echo "📄 Created test contract: TestContract.sol"

# Test 1: Default compilation (should generate .nef + .manifest.json)
echo
echo "Test 1: Default compilation (complete format)"
echo "--------------------------------------------"
neo-solc TestContract.sol -o TestContract

# Verify outputs
if [ -f "TestContract.nef" ]; then
    echo "✅ TestContract.nef generated successfully"
    echo "   Size: $(stat -c%s TestContract.nef) bytes"
else
    echo "❌ TestContract.nef not found"
    exit 1
fi

if [ -f "TestContract.manifest.json" ]; then
    echo "✅ TestContract.manifest.json generated successfully"
    echo "   Size: $(stat -c%s TestContract.manifest.json) bytes"
    
    # Validate manifest structure
    if jq -e '.abi.methods' TestContract.manifest.json > /dev/null; then
        METHOD_COUNT=$(jq '.abi.methods | length' TestContract.manifest.json)
        echo "   Methods: $METHOD_COUNT"
    fi
    
    if jq -e '.abi.events' TestContract.manifest.json > /dev/null; then
        EVENT_COUNT=$(jq '.abi.events | length' TestContract.manifest.json)
        echo "   Events: $EVENT_COUNT"
    fi
else
    echo "❌ TestContract.manifest.json not found"
    exit 1
fi

# Test 2: NEF-only format
echo
echo "Test 2: NEF-only format"
echo "----------------------"
neo-solc TestContract.sol -f nef -o TestOnly.nef

if [ -f "TestOnly.nef" ]; then
    echo "✅ NEF-only output generated successfully"
    echo "   Size: $(stat -c%s TestOnly.nef) bytes"
else
    echo "❌ NEF-only output failed"
    exit 1
fi

# Test 3: Manifest-only format
echo
echo "Test 3: Manifest-only format"
echo "---------------------------"
neo-solc TestContract.sol -f manifest -o TestOnly.manifest.json

if [ -f "TestOnly.manifest.json" ]; then
    echo "✅ Manifest-only output generated successfully"
    echo "   Size: $(stat -c%s TestOnly.manifest.json) bytes"
else
    echo "❌ Manifest-only output failed"
    exit 1
fi

# Test 4: JSON format with all information
echo
echo "Test 4: Complete JSON format"
echo "---------------------------"
neo-solc TestContract.sol -f json -o TestContract.json

if [ -f "TestContract.json" ]; then
    echo "✅ JSON output generated successfully"
    echo "   Size: $(stat -c%s TestContract.json) bytes"
    
    # Validate JSON structure
    if jq -e '.nef' TestContract.json > /dev/null; then
        echo "   ✓ Contains NEF data"
    fi
    
    if jq -e '.manifest' TestContract.json > /dev/null; then
        echo "   ✓ Contains manifest data"
    fi
else
    echo "❌ JSON output failed"
    exit 1
fi

# Test 5: Optimized compilation
echo
echo "Test 5: Optimized compilation (-O3)"
echo "---------------------------------"
neo-solc TestContract.sol -O3 -o TestContractOptimized

if [ -f "TestContractOptimized.nef" ] && [ -f "TestContractOptimized.manifest.json" ]; then
    echo "✅ Optimized compilation successful"
    
    # Compare sizes
    ORIGINAL_SIZE=$(stat -c%s TestContract.nef)
    OPTIMIZED_SIZE=$(stat -c%s TestContractOptimized.nef)
    
    echo "   Original NEF size: $ORIGINAL_SIZE bytes"
    echo "   Optimized NEF size: $OPTIMIZED_SIZE bytes"
    
    if [ $OPTIMIZED_SIZE -le $ORIGINAL_SIZE ]; then
        echo "   ✓ Optimization reduced or maintained size"
    else
        echo "   ⚠️ Optimization increased size (may be expected for small contracts)"
    fi
else
    echo "❌ Optimized compilation failed"
    exit 1
fi

# Test 6: Validate Neo contract format
echo
echo "Test 6: Validate Neo contract format"
echo "-----------------------------------"

# Check NEF magic number
NEF_MAGIC=$(hexdump -C TestContract.nef | head -1 | cut -d' ' -f2-5 | tr -d ' ')
if [ "$NEF_MAGIC" = "4e454633" ]; then  # "NEF3" in little endian hex
    echo "✅ NEF file has correct magic number (NEF3)"
else
    echo "❌ NEF file has incorrect magic number: $NEF_MAGIC"
    exit 1
fi

# Check manifest structure
if jq -e '.name' TestContract.manifest.json > /dev/null && \
   jq -e '.abi' TestContract.manifest.json > /dev/null && \
   jq -e '.permissions' TestContract.manifest.json > /dev/null; then
    echo "✅ Manifest has correct Neo N3 structure"
else
    echo "❌ Manifest structure is invalid"
    exit 1
fi

# Test 7: Real-world contract compilation
echo
echo "Test 7: Real-world ERC20 contract"
echo "--------------------------------"
if [ -f "../examples/ERC20Token.sol" ]; then
    neo-solc ../examples/ERC20Token.sol -O2 -o ERC20
    
    if [ -f "ERC20.nef" ] && [ -f "ERC20.manifest.json" ]; then
        echo "✅ ERC20 contract compiled successfully"
        echo "   NEF size: $(stat -c%s ERC20.nef) bytes"
        echo "   Manifest size: $(stat -c%s ERC20.manifest.json) bytes"
        
        # Check for ERC20 methods in manifest
        TRANSFER_METHOD=$(jq '.abi.methods[] | select(.name == "transfer")' ERC20.manifest.json)
        if [ -n "$TRANSFER_METHOD" ]; then
            echo "   ✓ Transfer method found in manifest"
        fi
        
        TRANSFER_EVENT=$(jq '.abi.events[] | select(.name == "Transfer")' ERC20.manifest.json)
        if [ -n "$TRANSFER_EVENT" ]; then
            echo "   ✓ Transfer event found in manifest"
        fi
    else
        echo "❌ ERC20 contract compilation failed"
        exit 1
    fi
fi

# Cleanup
echo
echo "🧹 Cleaning up test files..."
rm -f TestContract.sol TestContract.nef TestContract.manifest.json
rm -f TestOnly.nef TestOnly.manifest.json TestContract.json
rm -f TestContractOptimized.nef TestContractOptimized.manifest.json
rm -f ERC20.nef ERC20.manifest.json

echo
echo "🎉 All tests passed! Neo Solidity Compiler correctly generates:"
echo "   ✓ .nef files (Neo Executable Format)"
echo "   ✓ .manifest.json files (Contract metadata)"
echo "   ✓ Proper Neo N3 contract structure"
echo "   ✓ Ready for deployment to Neo blockchain"
echo
echo "Usage examples:"
echo "  neo-solc MyContract.sol -o MyContract     # Generates MyContract.nef + MyContract.manifest.json"
echo "  neo-cli contract deploy MyContract.nef MyContract.manifest.json"