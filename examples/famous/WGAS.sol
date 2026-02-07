// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/**
 * @title WGAS — Wrapped GAS (NEP-17)
 * @notice Neo N3 adaptation of WETH9, the most deployed contract on Ethereum.
 *
 * Deposits GAS via onNEP17Payment and mints 1:1 WGAS tokens.
 * Withdrawals burn WGAS and return GAS via NativeCalls.gasTransfer().
 *
 * Compiler constraints respected:
 *   - No inline assembly
 *   - No {value: ...} — uses NativeCalls.gasTransfer()
 *   - No receive()/fallback() — uses onNEP17Payment() callback
 *   - Import devpack via -I devpack
 */
contract WGAS {
    address private constant GAS_TOKEN = NativeCalls.GAS_CONTRACT;

    string public name;
    string public symbol;
    uint8 public decimals;

    uint256 public totalSupply;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    event Deposit(address indexed from, uint256 amount);
    event Withdrawal(address indexed to, uint256 amount);
    event Transfer(address indexed from, address indexed to, uint256 amount);
    event Approval(address indexed owner, address indexed spender, uint256 amount);

    constructor() {
        name = "Wrapped GAS";
        symbol = "WGAS";
        decimals = 8;
    }

    /// @notice NEP-17 callback — receives GAS deposits and mints WGAS 1:1.
    function onNEP17Payment(address from, uint256 amount, Any calldata /*data*/) external {
        address caller = Syscalls.getCallingScriptHash();
        require(caller == GAS_TOKEN, "WGAS: only GAS accepted");
        require(amount > 0, "WGAS: zero deposit");

        balanceOf[from] += amount;
        totalSupply += amount;

        emit Deposit(from, amount);
        emit Transfer(address(0), from, amount);
    }

    /// @notice Burn WGAS and withdraw GAS back to caller.
    function withdraw(uint256 amount) external {
        require(balanceOf[msg.sender] >= amount, "WGAS: insufficient balance");

        balanceOf[msg.sender] -= amount;
        totalSupply -= amount;

        bool ok = NativeCalls.gasTransfer(address(this), msg.sender, amount, "");
        require(ok, "WGAS: GAS transfer failed");

        emit Withdrawal(msg.sender, amount);
        emit Transfer(msg.sender, address(0), amount);
    }

    /// @notice NEP-17 standard transfer (4-parameter signature).
    function transfer(address from, address to, uint256 amount, Any calldata data) external returns (bool) {
        require(Runtime.checkWitness(from), "WGAS: unauthorized");
        return _transfer(from, to, amount);
    }

    /// @notice Approve spender to transfer WGAS on behalf of caller.
    function approve(address spender, uint256 amount) external returns (bool) {
        allowance[msg.sender][spender] = amount;
        emit Approval(msg.sender, spender, amount);
        return true;
    }

    /// @notice Transfer WGAS from one address to another (requires allowance).
    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        uint256 allowed = allowance[from][msg.sender];
        require(allowed >= amount, "WGAS: allowance exceeded");

        allowance[from][msg.sender] = allowed - amount;
        return _transfer(from, to, amount);
    }

    function _transfer(address from, address to, uint256 amount) internal returns (bool) {
        require(from != address(0), "WGAS: from zero address");
        require(to != address(0), "WGAS: to zero address");
        require(balanceOf[from] >= amount, "WGAS: insufficient balance");

        balanceOf[from] -= amount;
        balanceOf[to] += amount;

        emit Transfer(from, to, amount);
        return true;
    }
}
