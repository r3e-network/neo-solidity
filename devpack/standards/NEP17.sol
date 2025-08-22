// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

/**
 * @title NEP-17 Fungible Token Standard
 * @dev Complete implementation of Neo N3 NEP-17 standard for Solidity
 * @author Jimmy <jimmy@r3e.network>
 * 
 * NEP-17 is Neo's enhanced fungible token standard, providing:
 * - Standard ERC-20 compatibility
 * - Neo-specific features (onNEP17Payment callback)
 * - Advanced transfer capabilities
 * - Integration with Neo native tokens
 * - Event system compatible with Neo Runtime.Notify
 */

import "../contracts/Framework.sol";
import "../libraries/Neo.sol";
import "../libraries/Runtime.sol";

/**
 * @title INEP17
 * @dev Interface for NEP-17 fungible token standard
 */
interface INEP17 {
    // Standard NEP-17 functions
    function symbol() external view returns (string memory);
    function decimals() external view returns (uint8);
    function totalSupply() external view returns (uint256);
    function balanceOf(address account) external view returns (uint256);
    function transfer(address from, address to, uint256 amount, bytes calldata data) external returns (bool);
    
    // Events
    event Transfer(address indexed from, address indexed to, uint256 amount);
}

/**
 * @title INEP17Receiver
 * @dev Interface for contracts that can receive NEP-17 tokens
 */
interface INEP17Receiver {
    function onNEP17Payment(address from, uint256 amount, bytes calldata data) external;
}

/**
 * @title NEP17
 * @dev Complete NEP-17 token implementation with Neo N3 integration
 */
contract NEP17 is INEP17, Framework {
    using Neo for *;
    using Runtime for *;
    
    // Token metadata
    string private _name;
    string private _symbol;
    uint8 private _decimals;
    uint256 private _totalSupply;
    
    // Balances and allowances
    mapping(address => uint256) private _balances;
    mapping(address => mapping(address => uint256)) private _allowances;
    
    // NEP-17 specific features
    bool private _transfersEnabled = true;
    address private _minter;
    uint256 private _maxSupply;
    
    // Events (NEP-17 compatible)
    event Transfer(address indexed from, address indexed to, uint256 amount);
    event Approval(address indexed owner, address indexed spender, uint256 amount);
    event Mint(address indexed to, uint256 amount);
    event Burn(address indexed from, uint256 amount);
    
    // NEP-17 specific events
    event TransfersEnabled();
    event TransfersDisabled();
    event MinterChanged(address indexed oldMinter, address indexed newMinter);
    event MaxSupplySet(uint256 maxSupply);
    
    // Custom errors
    error NEP17InsufficientBalance(address account, uint256 balance, uint256 needed);
    error NEP17InvalidReceiver(address receiver);
    error NEP17TransfersDisabled();
    error NEP17ExceedsMaxSupply(uint256 amount, uint256 maxSupply);
    error NEP17InvalidAmount(uint256 amount);
    error NEP17NotMinter(address caller);
    
    // Modifiers
    modifier whenTransfersEnabled() {
        if (!_transfersEnabled) revert NEP17TransfersDisabled();
        _;
    }
    
    modifier onlyMinter() {
        if (msg.sender != _minter) revert NEP17NotMinter(msg.sender);
        _;
    }
    
    modifier validAmount(uint256 amount) {
        if (amount == 0) revert NEP17InvalidAmount(amount);
        _;
    }
    
    modifier validReceiver(address to) {
        if (to == address(0)) revert NEP17InvalidReceiver(to);
        _;
    }
    
    /**
     * @dev Constructor
     */
    constructor(
        string memory name_,
        string memory symbol_,
        uint8 decimals_,
        uint256 initialSupply,
        uint256 maxSupply_
    ) Framework() {
        require(bytes(name_).length > 0, "NEP17: name cannot be empty");
        require(bytes(symbol_).length > 0, "NEP17: symbol cannot be empty");
        require(decimals_ <= 18, "NEP17: decimals cannot exceed 18");
        
        _name = name_;
        _symbol = symbol_;
        _decimals = decimals_;
        _minter = msg.sender;
        _maxSupply = maxSupply_;
        
        if (initialSupply > 0) {
            _mint(msg.sender, initialSupply);
        }
    }
    
    // ========== View Functions ==========
    
    /**
     * @dev Returns the name of the token
     */
    function name() public view returns (string memory) {
        return _name;
    }
    
    /**
     * @dev Returns the symbol of the token
     */
    function symbol() public view override returns (string memory) {
        return _symbol;
    }
    
    /**
     * @dev Returns the number of decimals
     */
    function decimals() public view override returns (uint8) {
        return _decimals;
    }
    
    /**
     * @dev Returns the total supply
     */
    function totalSupply() public view override returns (uint256) {
        return _totalSupply;
    }
    
    /**
     * @dev Returns the balance of an account
     */
    function balanceOf(address account) public view override returns (uint256) {
        return _balances[account];
    }
    
    /**
     * @dev Returns the allowance
     */
    function allowance(address owner, address spender) public view returns (uint256) {
        return _allowances[owner][spender];
    }
    
    /**
     * @dev Returns if transfers are enabled
     */
    function transfersEnabled() public view returns (bool) {
        return _transfersEnabled;
    }
    
    /**
     * @dev Returns the minter address
     */
    function minter() public view returns (address) {
        return _minter;
    }
    
    /**
     * @dev Returns the maximum supply
     */
    function maxSupply() public view returns (uint256) {
        return _maxSupply;
    }
    
    // ========== Transfer Functions ==========
    
    /**
     * @dev NEP-17 transfer function
     */
    function transfer(
        address from,
        address to,
        uint256 amount,
        bytes calldata data
    ) public override whenTransfersEnabled validReceiver(to) validAmount(amount) returns (bool) {
        // Check authorization
        require(
            from == msg.sender || 
            _allowances[from][msg.sender] >= amount ||
            Runtime.checkWitness(from),
            "NEP17: unauthorized transfer"
        );
        
        _transfer(from, to, amount, data);
        
        // Update allowance if needed
        if (from != msg.sender && _allowances[from][msg.sender] != type(uint256).max) {
            _approve(from, msg.sender, _allowances[from][msg.sender] - amount);
        }
        
        return true;
    }
    
    /**
     * @dev Standard ERC-20 transfer
     */
    function transfer(address to, uint256 amount) public returns (bool) {
        return transfer(msg.sender, to, amount, "");
    }
    
    /**
     * @dev Transfer from (ERC-20 compatibility)
     */
    function transferFrom(address from, address to, uint256 amount) public returns (bool) {
        return transfer(from, to, amount, "");
    }
    
    /**
     * @dev Approve spender
     */
    function approve(address spender, uint256 amount) public returns (bool) {
        _approve(msg.sender, spender, amount);
        return true;
    }
    
    /**
     * @dev Increase allowance
     */
    function increaseAllowance(address spender, uint256 addedValue) public returns (bool) {
        _approve(msg.sender, spender, _allowances[msg.sender][spender] + addedValue);
        return true;
    }
    
    /**
     * @dev Decrease allowance
     */
    function decreaseAllowance(address spender, uint256 subtractedValue) public returns (bool) {
        uint256 currentAllowance = _allowances[msg.sender][spender];
        require(currentAllowance >= subtractedValue, "NEP17: decreased allowance below zero");
        _approve(msg.sender, spender, currentAllowance - subtractedValue);
        return true;
    }
    
    // ========== Minting and Burning ==========
    
    /**
     * @dev Mint tokens
     */
    function mint(address to, uint256 amount) public onlyMinter validReceiver(to) validAmount(amount) {
        if (_maxSupply > 0 && _totalSupply + amount > _maxSupply) {
            revert NEP17ExceedsMaxSupply(amount, _maxSupply);
        }
        
        _mint(to, amount);
    }
    
    /**
     * @dev Burn tokens
     */
    function burn(uint256 amount) public validAmount(amount) {
        _burn(msg.sender, amount);
    }
    
    /**
     * @dev Burn tokens from account (with allowance)
     */
    function burnFrom(address from, uint256 amount) public validAmount(amount) {
        uint256 currentAllowance = _allowances[from][msg.sender];
        require(currentAllowance >= amount, "NEP17: burn amount exceeds allowance");
        
        _burn(from, amount);
        _approve(from, msg.sender, currentAllowance - amount);
    }
    
    // ========== Admin Functions ==========
    
    /**
     * @dev Enable transfers
     */
    function enableTransfers() public onlyOwner {
        require(!_transfersEnabled, "NEP17: transfers already enabled");
        _transfersEnabled = true;
        emit TransfersEnabled();
    }
    
    /**
     * @dev Disable transfers
     */
    function disableTransfers() public onlyOwner {
        require(_transfersEnabled, "NEP17: transfers already disabled");
        _transfersEnabled = false;
        emit TransfersDisabled();
    }
    
    /**
     * @dev Change minter
     */
    function changeMinter(address newMinter) public onlyOwner {
        require(newMinter != address(0), "NEP17: new minter is zero address");
        address oldMinter = _minter;
        _minter = newMinter;
        emit MinterChanged(oldMinter, newMinter);
    }
    
    /**
     * @dev Set maximum supply
     */
    function setMaxSupply(uint256 newMaxSupply) public onlyOwner {
        require(newMaxSupply >= _totalSupply, "NEP17: max supply below current supply");
        _maxSupply = newMaxSupply;
        emit MaxSupplySet(newMaxSupply);
    }
    
    // ========== Batch Operations ==========
    
    /**
     * @dev Batch transfer to multiple recipients
     */
    function batchTransfer(
        address[] memory recipients,
        uint256[] memory amounts,
        bytes[] memory data
    ) public whenTransfersEnabled returns (bool) {
        require(recipients.length == amounts.length, "NEP17: array length mismatch");
        require(recipients.length == data.length, "NEP17: array length mismatch");
        require(recipients.length > 0, "NEP17: empty arrays");
        require(recipients.length <= 100, "NEP17: too many recipients");
        
        for (uint256 i = 0; i < recipients.length; i++) {
            transfer(msg.sender, recipients[i], amounts[i], data[i]);
        }
        
        return true;
    }
    
    /**
     * @dev Batch mint to multiple recipients
     */
    function batchMint(address[] memory recipients, uint256[] memory amounts) 
        public 
        onlyMinter 
        returns (bool) 
    {
        require(recipients.length == amounts.length, "NEP17: array length mismatch");
        require(recipients.length > 0, "NEP17: empty arrays");
        require(recipients.length <= 100, "NEP17: too many recipients");
        
        for (uint256 i = 0; i < recipients.length; i++) {
            mint(recipients[i], amounts[i]);
        }
        
        return true;
    }
    
    // ========== Internal Functions ==========
    
    /**
     * @dev Internal transfer function
     */
    function _transfer(address from, address to, uint256 amount, bytes memory data) internal {
        uint256 fromBalance = _balances[from];
        if (fromBalance < amount) {
            revert NEP17InsufficientBalance(from, fromBalance, amount);
        }
        
        unchecked {
            _balances[from] = fromBalance - amount;
        }
        _balances[to] += amount;
        
        emit Transfer(from, to, amount);
        
        // Emit Neo-compatible notification
        Runtime.notify("Transfer", abi.encode(from, to, amount));
        
        // Call onNEP17Payment if recipient is a contract
        if (to.code.length > 0) {
            try INEP17Receiver(to).onNEP17Payment(from, amount, data) {
                // Success
            } catch {
                // Revert if recipient doesn't implement interface correctly
                revert NEP17InvalidReceiver(to);
            }
        }
    }
    
    /**
     * @dev Internal mint function
     */
    function _mint(address to, uint256 amount) internal {
        _totalSupply += amount;
        _balances[to] += amount;
        
        emit Transfer(address(0), to, amount);
        emit Mint(to, amount);
        
        // Emit Neo-compatible notification
        Runtime.notify("Transfer", abi.encode(address(0), to, amount));
        
        // Call onNEP17Payment if recipient is a contract
        if (to.code.length > 0) {
            try INEP17Receiver(to).onNEP17Payment(address(0), amount, "") {
                // Success
            } catch {
                // Mint can proceed even if recipient doesn't implement interface
            }
        }
    }
    
    /**
     * @dev Internal burn function
     */
    function _burn(address from, uint256 amount) internal {
        uint256 accountBalance = _balances[from];
        if (accountBalance < amount) {
            revert NEP17InsufficientBalance(from, accountBalance, amount);
        }
        
        unchecked {
            _balances[from] = accountBalance - amount;
            _totalSupply -= amount;
        }
        
        emit Transfer(from, address(0), amount);
        emit Burn(from, amount);
        
        // Emit Neo-compatible notification
        Runtime.notify("Transfer", abi.encode(from, address(0), amount));
    }
    
    /**
     * @dev Internal approve function
     */
    function _approve(address owner, address spender, uint256 amount) internal {
        require(owner != address(0), "NEP17: approve from zero address");
        require(spender != address(0), "NEP17: approve to zero address");
        
        _allowances[owner][spender] = amount;
        emit Approval(owner, spender, amount);
    }
    
    // ========== Neo Integration Functions ==========
    
    /**
     * @dev Get token holders count (expensive operation)
     */
    function getHoldersCount() public view returns (uint256) {
        // Use storage iterator to count all balance entries
        Storage.Iterator memory iterator = Storage.find(abi.encode("balance"));
        uint256 count = 0;
        
        while (iterator.next() && count < 10000) { // Limit to prevent gas exhaustion
            bytes memory balance = iterator.value();
            if (balance.length > 0) {
                uint256 amount = abi.decode(balance, (uint256));
                if (amount > 0) {
                    count++;
                }
            }
        }
        
        return count;
    }
    
    /**
     * @dev Get token info for Neo blockchain
     */
    function getTokenInfo() public view returns (
        string memory tokenName,
        string memory tokenSymbol,
        uint8 tokenDecimals,
        uint256 tokenTotalSupply,
        uint256 tokenMaxSupply,
        address tokenMinter,
        bool tokenTransfersEnabled
    ) {
        return (_name, _symbol, _decimals, _totalSupply, _maxSupply, _minter, _transfersEnabled);
    }
    
    /**
     * @dev Check NEP-17 compliance
     */
    function supportsInterface(bytes4 interfaceId) public pure returns (bool) {
        return interfaceId == type(INEP17).interfaceId ||
               interfaceId == 0x01ffc9a7; // ERC165
    }
    
    /**
     * @dev Get contract metadata for Neo
     */
    function getContractMetadata() public view returns (
        string memory standard,
        string memory name,
        string memory version,
        string memory author
    ) {
        return (
            "NEP-17",
            "Neo N3 Fungible Token",
            "1.0.0",
            "Jimmy <jimmy@r3e.network>"
        );
    }
    
    // ========== Emergency Functions ==========
    
    /**
     * @dev Emergency token recovery
     */
    function emergencyTokenRecovery(
        address token,
        address to,
        uint256 amount
    ) public onlyOwner {
        require(token != address(this), "NEP17: cannot recover own tokens");
        require(to != address(0), "NEP17: cannot recover to zero address");
        
        // Call the token's transfer function
        (bool success, ) = token.call(
            abi.encodeWithSignature("transfer(address,address,uint256,bytes)", 
                                  address(this), to, amount, "")
        );
        
        require(success, "NEP17: token recovery failed");
    }
    
    /**
     * @dev Emergency pause (disable transfers)
     */
    function emergencyPause() public onlyOwner {
        disableTransfers();
        
        // Emit emergency notification
        Runtime.notify("EmergencyPause", abi.encode(msg.sender, block.timestamp));
    }
    
    /**
     * @dev Emergency unpause (enable transfers)
     */
    function emergencyUnpause() public onlyOwner {
        enableTransfers();
        
        // Emit emergency notification
        Runtime.notify("EmergencyUnpause", abi.encode(msg.sender, block.timestamp));
    }
    
    // ========== Advanced Features ==========
    
    /**
     * @dev Time-locked transfer
     */
    function transferWithTimelock(
        address to,
        uint256 amount,
        uint256 releaseTime
    ) public whenTransfersEnabled validReceiver(to) validAmount(amount) {
        require(releaseTime > block.timestamp, "NEP17: release time must be in future");
        
        // Store the time-locked transfer
        bytes32 timelockId = keccak256(abi.encode(msg.sender, to, amount, releaseTime, block.timestamp));
        
        // Transfer to this contract temporarily
        _transfer(msg.sender, address(this), amount, "");
        
        // Store timelock info
        Storage.put(
            abi.encode("timelock", timelockId),
            abi.encode(msg.sender, to, amount, releaseTime)
        );
        
        // Emit event
        Runtime.notify("TimelockCreated", abi.encode(timelockId, msg.sender, to, amount, releaseTime));
    }
    
    /**
     * @dev Claim time-locked tokens
     */
    function claimTimelock(bytes32 timelockId) public {
        bytes memory timelockData = Storage.get(abi.encode("timelock", timelockId));
        require(timelockData.length > 0, "NEP17: timelock not found");
        
        (address from, address to, uint256 amount, uint256 releaseTime) = 
            abi.decode(timelockData, (address, address, uint256, uint256));
        
        require(block.timestamp >= releaseTime, "NEP17: timelock not yet expired");
        require(msg.sender == to, "NEP17: only recipient can claim");
        
        // Delete timelock
        Storage.delete(abi.encode("timelock", timelockId));
        
        // Transfer tokens
        _transfer(address(this), to, amount, "");
        
        // Emit event
        Runtime.notify("TimelockClaimed", abi.encode(timelockId, to, amount));
    }
    
    /**
     * @dev Multi-signature transfer
     */
    function multiSigTransfer(
        address to,
        uint256 amount,
        address[] memory signers,
        bytes[] memory signatures
    ) public whenTransfersEnabled validReceiver(to) validAmount(amount) {
        require(signers.length == signatures.length, "NEP17: array length mismatch");
        require(signers.length >= 2, "NEP17: minimum 2 signers required");
        require(signers.length <= 10, "NEP17: maximum 10 signers allowed");
        
        // Verify signatures
        bytes32 hash = keccak256(abi.encode(address(this), to, amount, block.timestamp));
        
        for (uint256 i = 0; i < signers.length; i++) {
            require(
                Neo.verifySignature(hash, abi.encode(signers[i]), signatures[i]),
                "NEP17: invalid signature"
            );
        }
        
        // Execute transfer from multisig pool
        _transfer(address(this), to, amount, abi.encode("multisig", signers));
    }
    
    /**
     * @dev Conditional transfer based on oracle data
     */
    function conditionalTransfer(
        address to,
        uint256 amount,
        string memory oracleUrl,
        string memory condition
    ) public whenTransfersEnabled validReceiver(to) validAmount(amount) {
        // Escrow tokens in contract until condition is met
        _transfer(msg.sender, address(this), amount, "");
        
        // Create oracle request with callback
        bytes memory userData = abi.encode(msg.sender, to, amount, condition);
        bytes32 requestId = keccak256(abi.encode(msg.sender, to, amount, block.timestamp));
        
        // Store pending transfer
        Storage.put(
            abi.encode("conditional_transfer", requestId),
            userData
        );
        
        // Make oracle request via syscall
        Syscalls.oracleRequest(oracleUrl, condition, "conditionalTransferCallback", userData, 10000000);
        
        Runtime.notify("ConditionalTransferCreated", abi.encode(requestId, msg.sender, to, amount));
    }
    
    /**
     * @dev Oracle callback for conditional transfers
     */
    function conditionalTransferCallback(
        uint256 requestId,
        uint256 code,
        bytes calldata result,
        bytes calldata userData
    ) external {
        require(msg.sender == address(this), "NEP17: unauthorized callback");
        
        if (code == 0) {
            // Parse oracle result to determine if condition is met
            bool conditionMet = abi.decode(result, (bool));
            
            if (conditionMet) {
                (address from, address to, uint256 amount,) = abi.decode(userData, (address, address, uint256, string));
                
                // Execute transfer
                _transfer(address(this), to, amount, "");
                
                Runtime.notify("ConditionalTransferExecuted", abi.encode(from, to, amount));
            } else {
                // Return tokens to sender
                (address from,, uint256 amount,) = abi.decode(userData, (address, address, uint256, string));
                _transfer(address(this), from, amount, "");
                
                Runtime.notify("ConditionalTransferFailed", abi.encode(from, amount));
            }
        }
    }
    
    /**
     * @dev Get all balances (expensive operation, use carefully)
     */
    function getAllBalances() public view returns (address[] memory accounts, uint256[] memory balances) {
        // Use storage iterator to get all balance entries
        Storage.Iterator memory iterator = Storage.find(abi.encode("balance"));
        
        // Temporary arrays with maximum size
        address[] memory tempAccounts = new address[](1000);
        uint256[] memory tempBalances = new uint256[](1000);
        uint256 count = 0;
        
        while (iterator.next() && count < 1000) {
            bytes memory balanceData = iterator.value();
            if (balanceData.length > 0) {
                uint256 balance = abi.decode(balanceData, (uint256));
                if (balance > 0) {
                    // Extract address from key
                    bytes memory key = iterator.currentKey;
                    address account = abi.decode(key, (address));
                    
                    tempAccounts[count] = account;
                    tempBalances[count] = balance;
                    count++;
                }
            }
        }
        
        // Resize arrays to actual count
        accounts = new address[](count);
        balances = new uint256[](count);
        
        for (uint256 i = 0; i < count; i++) {
            accounts[i] = tempAccounts[i];
            balances[i] = tempBalances[i];
        }
    }
    
    /**
     * @dev NEP-17 specific metadata
     */
    function nep17Metadata() public view returns (
        string memory standard,
        bytes memory logo,
        string memory website,
        string memory description
    ) {
        return (
            "NEP-17",
            "", // Logo data (optional)
            "https://r3e.network",
            string(abi.encodePacked("NEP-17 token: ", _name))
        );
    }
}