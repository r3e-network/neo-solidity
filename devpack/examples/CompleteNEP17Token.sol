// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

/**
 * @title Complete NEP-17 Token Example
 * @dev Production-ready NEP-17 token with all Neo N3 features
 * @author Jimmy <jimmy@r3e.network>
 * 
 * Features:
 * - Full NEP-17 compliance with Neo N3 integration
 * - Advanced access control and governance
 * - Oracle integration for dynamic pricing
 * - Multi-signature operations
 * - Emergency controls and upgradability
 * - Gas optimization and batch operations
 */

import "../standards/NEP17.sol";
import "../standards/NEP24.sol";
import "../libraries/Neo.sol";
import "../libraries/Storage.sol";
import "../libraries/Runtime.sol";

contract CompleteNEP17Token is NEP17 {
    using Neo for *;
    using Storage for *;
    using Runtime for *;
    
    // Advanced token features
    struct TokenConfig {
        bool stakingEnabled;
        bool oracleEnabled;
        bool upgradeEnabled;
        uint256 stakingRewardRate;
        uint256 minimumStake;
        address oracleContract;
        uint256 lastPriceUpdate;
        uint256 currentPrice;
    }
    
    TokenConfig private _config;
    
    // Staking system
    mapping(address => StakeInfo) private _stakes;
    mapping(address => uint256) private _rewards;
    uint256 private _totalStaked;
    
    struct StakeInfo {
        uint256 amount;
        uint256 timestamp;
        uint256 lockPeriod;
        uint256 lastRewardClaim;
    }
    
    // Governance
    mapping(bytes32 => Proposal) private _proposals;
    mapping(bytes32 => mapping(address => bool)) private _voted;
    uint256 private _proposalCounter;
    
    struct Proposal {
        bytes32 id;
        address proposer;
        string description;
        bytes callData;
        uint256 startTime;
        uint256 endTime;
        uint256 forVotes;
        uint256 againstVotes;
        bool executed;
        ProposalType proposalType;
    }
    
    enum ProposalType {
        ConfigChange,
        Upgrade,
        Mint,
        Burn,
        Emergency
    }
    
    // Oracle integration
    NEP24Oracle private _oracle;
    mapping(string => uint256) private _externalPrices;
    
    // Events
    event Stake(address indexed account, uint256 amount, uint256 lockPeriod);
    event Unstake(address indexed account, uint256 amount, uint256 reward);
    event RewardClaimed(address indexed account, uint256 reward);
    event ProposalCreated(bytes32 indexed proposalId, address indexed proposer, string description);
    event ProposalExecuted(bytes32 indexed proposalId, bool success);
    event PriceUpdated(string indexed symbol, uint256 price, uint256 timestamp);
    event ConfigUpdated(string parameter, uint256 oldValue, uint256 newValue);
    
    /**
     * @dev Constructor with advanced configuration
     */
    constructor(
        string memory name,
        string memory symbol,
        uint8 decimals,
        uint256 initialSupply,
        uint256 maxSupply,
        address oracleAddress
    ) NEP17(name, symbol, decimals, initialSupply, maxSupply) {
        _config = TokenConfig({
            stakingEnabled: true,
            oracleEnabled: oracleAddress != address(0),
            upgradeEnabled: true,
            stakingRewardRate: 5, // 5% annual rate
            minimumStake: 1000 * 10**decimals,
            oracleContract: oracleAddress,
            lastPriceUpdate: 0,
            currentPrice: 0
        });
        
        if (_config.oracleEnabled) {
            _oracle = NEP24Oracle(oracleAddress);
        }
        
        // Store contract metadata in Neo storage
        Storage.putContractMetadata(
            name,
            "1.0.0",
            "Jimmy <jimmy@r3e.network>",
            abi.encode("NEP-17", "Complete", "Production")
        );
    }
    
    // ========== Staking System ==========
    
    /**
     * @dev Stake tokens for rewards
     */
    function stake(uint256 amount, uint256 lockPeriodDays) public whenTransfersEnabled {
        require(_config.stakingEnabled, "CompleteNEP17: staking disabled");
        require(amount >= _config.minimumStake, "CompleteNEP17: amount below minimum stake");
        require(lockPeriodDays >= 30 && lockPeriodDays <= 365, "CompleteNEP17: invalid lock period");
        require(balanceOf(msg.sender) >= amount, "CompleteNEP17: insufficient balance");
        
        // Transfer tokens to contract
        _transfer(msg.sender, address(this), amount, "");
        
        // Update stake info
        StakeInfo storage stakeInfo = _stakes[msg.sender];
        stakeInfo.amount += amount;
        stakeInfo.timestamp = block.timestamp;
        stakeInfo.lockPeriod = lockPeriodDays * 1 days;
        stakeInfo.lastRewardClaim = block.timestamp;
        
        _totalStaked += amount;
        
        emit Stake(msg.sender, amount, lockPeriodDays);
        Runtime.notify("Stake", abi.encode(msg.sender, amount, lockPeriodDays));
    }
    
    /**
     * @dev Unstake tokens
     */
    function unstake(uint256 amount) public {
        StakeInfo storage stakeInfo = _stakes[msg.sender];
        require(stakeInfo.amount >= amount, "CompleteNEP17: insufficient staked amount");
        require(
            block.timestamp >= stakeInfo.timestamp + stakeInfo.lockPeriod,
            "CompleteNEP17: stake still locked"
        );
        
        // Calculate rewards
        uint256 reward = calculateReward(msg.sender);
        
        // Update stake
        stakeInfo.amount -= amount;
        _totalStaked -= amount;
        
        // Transfer tokens back
        _transfer(address(this), msg.sender, amount, "");
        
        // Mint reward tokens
        if (reward > 0) {
            _mint(msg.sender, reward);
            _rewards[msg.sender] += reward;
        }
        
        emit Unstake(msg.sender, amount, reward);
        Runtime.notify("Unstake", abi.encode(msg.sender, amount, reward));
    }
    
    /**
     * @dev Calculate staking rewards
     */
    function calculateReward(address account) public view returns (uint256) {
        StakeInfo memory stakeInfo = _stakes[account];
        if (stakeInfo.amount == 0) return 0;
        
        uint256 stakingTime = block.timestamp - stakeInfo.lastRewardClaim;
        uint256 annualReward = (stakeInfo.amount * _config.stakingRewardRate) / 100;
        uint256 reward = (annualReward * stakingTime) / 365 days;
        
        return reward;
    }
    
    /**
     * @dev Claim staking rewards without unstaking
     */
    function claimRewards() public {
        uint256 reward = calculateReward(msg.sender);
        require(reward > 0, "CompleteNEP17: no rewards available");
        
        _stakes[msg.sender].lastRewardClaim = block.timestamp;
        _mint(msg.sender, reward);
        _rewards[msg.sender] += reward;
        
        emit RewardClaimed(msg.sender, reward);
        Runtime.notify("RewardClaimed", abi.encode(msg.sender, reward));
    }
    
    // ========== Oracle Integration ==========
    
    /**
     * @dev Update token price via oracle
     */
    function updatePriceViaOracle() public returns (uint256 requestId) {
        require(_config.oracleEnabled, "CompleteNEP17: oracle disabled");
        require(
            block.timestamp > _config.lastPriceUpdate + 1 hours,
            "CompleteNEP17: price updated recently"
        );
        
        return _oracle.requestPriceData(symbol(), "oraclePriceCallback");
    }
    
    /**
     * @dev Oracle price callback
     */
    function oraclePriceCallback(
        uint256 requestId,
        uint256 code,
        bytes calldata result,
        bytes calldata userData
    ) external {
        require(msg.sender == address(_oracle), "CompleteNEP17: unauthorized oracle response");
        
        if (code == 0) {
            uint256 newPrice = abi.decode(result, (uint256));
            _config.currentPrice = newPrice;
            _config.lastPriceUpdate = block.timestamp;
            
            emit PriceUpdated(symbol(), newPrice, block.timestamp);
            Runtime.notify("PriceUpdated", abi.encode(symbol(), newPrice, block.timestamp));
        }
    }
    
    /**
     * @dev Get current token price
     */
    function getCurrentPrice() public view returns (uint256 price, uint256 lastUpdate) {
        return (_config.currentPrice, _config.lastPriceUpdate);
    }
    
    // ========== Governance System ==========
    
    /**
     * @dev Create governance proposal
     */
    function createProposal(
        string memory description,
        bytes memory callData,
        ProposalType proposalType,
        uint256 votingPeriodDays
    ) public returns (bytes32 proposalId) {
        require(balanceOf(msg.sender) >= totalSupply() / 100, "CompleteNEP17: insufficient tokens for proposal");
        require(bytes(description).length > 0, "CompleteNEP17: empty description");
        require(votingPeriodDays >= 3 && votingPeriodDays <= 30, "CompleteNEP17: invalid voting period");
        
        proposalId = keccak256(abi.encode(
            msg.sender,
            description,
            callData,
            block.timestamp,
            _proposalCounter++
        ));
        
        _proposals[proposalId] = Proposal({
            id: proposalId,
            proposer: msg.sender,
            description: description,
            callData: callData,
            startTime: block.timestamp,
            endTime: block.timestamp + (votingPeriodDays * 1 days),
            forVotes: 0,
            againstVotes: 0,
            executed: false,
            proposalType: proposalType
        });
        
        emit ProposalCreated(proposalId, msg.sender, description);
        Runtime.notify("ProposalCreated", abi.encode(proposalId, msg.sender, description));
    }
    
    /**
     * @dev Vote on proposal
     */
    function vote(bytes32 proposalId, bool support, uint256 votingPower) public {
        Proposal storage proposal = _proposals[proposalId];
        require(proposal.proposer != address(0), "CompleteNEP17: proposal not found");
        require(block.timestamp <= proposal.endTime, "CompleteNEP17: voting ended");
        require(!_voted[proposalId][msg.sender], "CompleteNEP17: already voted");
        require(votingPower <= balanceOf(msg.sender), "CompleteNEP17: insufficient voting power");
        
        _voted[proposalId][msg.sender] = true;
        
        if (support) {
            proposal.forVotes += votingPower;
        } else {
            proposal.againstVotes += votingPower;
        }
        
        Runtime.notify("Vote", abi.encode(proposalId, msg.sender, support, votingPower));
    }
    
    /**
     * @dev Execute proposal
     */
    function executeProposal(bytes32 proposalId) public {
        Proposal storage proposal = _proposals[proposalId];
        require(proposal.proposer != address(0), "CompleteNEP17: proposal not found");
        require(block.timestamp > proposal.endTime, "CompleteNEP17: voting still active");
        require(!proposal.executed, "CompleteNEP17: already executed");
        require(proposal.forVotes > proposal.againstVotes, "CompleteNEP17: proposal rejected");
        
        proposal.executed = true;
        
        // Execute proposal call data
        bool success;
        if (proposal.callData.length > 0) {
            (success, ) = address(this).call(proposal.callData);
        } else {
            success = true;
        }
        
        emit ProposalExecuted(proposalId, success);
        Runtime.notify("ProposalExecuted", abi.encode(proposalId, success));
    }
    
    // ========== Multi-Signature Operations ==========
    
    /**
     * @dev Multi-sig mint (requires multiple signatures)
     */
    function multiSigMint(
        address to,
        uint256 amount,
        address[] memory signers,
        bytes[] memory signatures
    ) public {
        require(signers.length >= 3, "CompleteNEP17: minimum 3 signers required");
        require(signers.length == signatures.length, "CompleteNEP17: array length mismatch");
        
        // Verify signatures
        bytes32 hash = keccak256(abi.encode(
            address(this),
            "multiSigMint",
            to,
            amount,
            block.timestamp / 3600 // Valid for 1 hour
        ));
        
        uint256 validSignatures = 0;
        for (uint256 i = 0; i < signers.length; i++) {
            if (Neo.verifySignature(hash, abi.encode(signers[i]), signatures[i])) {
                validSignatures++;
            }
        }
        
        require(validSignatures >= (signers.length * 2) / 3, "CompleteNEP17: insufficient valid signatures");
        
        // Execute mint
        _mint(to, amount);
        
        Runtime.notify("MultiSigMint", abi.encode(to, amount, signers.length, validSignatures));
    }
    
    /**
     * @dev Multi-sig burn
     */
    function multiSigBurn(
        address from,
        uint256 amount,
        address[] memory signers,
        bytes[] memory signatures
    ) public {
        require(signers.length >= 3, "CompleteNEP17: minimum 3 signers required");
        
        // Similar signature verification as multiSigMint
        bytes32 hash = keccak256(abi.encode(
            address(this),
            "multiSigBurn",
            from,
            amount,
            block.timestamp / 3600
        ));
        
        uint256 validSignatures = 0;
        for (uint256 i = 0; i < signers.length; i++) {
            if (Neo.verifySignature(hash, abi.encode(signers[i]), signatures[i])) {
                validSignatures++;
            }
        }
        
        require(validSignatures >= (signers.length * 2) / 3, "CompleteNEP17: insufficient valid signatures");
        
        _burn(from, amount);
        
        Runtime.notify("MultiSigBurn", abi.encode(from, amount, signers.length, validSignatures));
    }
    
    // ========== Advanced Transfer Features ==========
    
    /**
     * @dev Scheduled transfer (executes at future time)
     */
    function scheduleTransfer(
        address to,
        uint256 amount,
        uint256 executeTime,
        bytes memory data
    ) public returns (bytes32 scheduleId) {
        require(executeTime > block.timestamp, "CompleteNEP17: execute time must be in future");
        require(balanceOf(msg.sender) >= amount, "CompleteNEP17: insufficient balance");
        
        scheduleId = keccak256(abi.encode(msg.sender, to, amount, executeTime, block.timestamp));
        
        // Lock tokens temporarily
        _transfer(msg.sender, address(this), amount, "");
        
        // Store scheduled transfer
        Storage.put(
            abi.encode("scheduled_transfer", scheduleId),
            abi.encode(msg.sender, to, amount, executeTime, data)
        );
        
        Runtime.notify("TransferScheduled", abi.encode(scheduleId, msg.sender, to, amount, executeTime));
    }
    
    /**
     * @dev Execute scheduled transfer
     */
    function executeScheduledTransfer(bytes32 scheduleId) public {
        bytes memory transferData = Storage.get(abi.encode("scheduled_transfer", scheduleId));
        require(transferData.length > 0, "CompleteNEP17: scheduled transfer not found");
        
        (address from, address to, uint256 amount, uint256 executeTime, bytes memory data) = 
            abi.decode(transferData, (address, address, uint256, uint256, bytes));
        
        require(block.timestamp >= executeTime, "CompleteNEP17: not yet time to execute");
        
        // Delete scheduled transfer
        Storage.delete(abi.encode("scheduled_transfer", scheduleId));
        
        // Execute transfer
        _transfer(address(this), to, amount, data);
        
        Runtime.notify("ScheduledTransferExecuted", abi.encode(scheduleId, from, to, amount));
    }
    
    /**
     * @dev Conditional transfer based on oracle data
     */
    function conditionalTransfer(
        address to,
        uint256 amount,
        string memory priceSymbol,
        uint256 minimumPrice,
        bytes memory data
    ) public {
        require(_config.oracleEnabled, "CompleteNEP17: oracle not enabled");
        require(balanceOf(msg.sender) >= amount, "CompleteNEP17: insufficient balance");
        
        uint256 currentPrice = _externalPrices[priceSymbol];
        require(currentPrice >= minimumPrice, "CompleteNEP17: price condition not met");
        
        _transfer(msg.sender, to, amount, data);
        
        Runtime.notify("ConditionalTransfer", abi.encode(
            msg.sender, to, amount, priceSymbol, currentPrice, minimumPrice
        ));
    }
    
    // ========== Gas Optimization Features ==========
    
    /**
     * @dev Gas-optimized batch operations
     */
    function optimizedBatchTransfer(
        address[] memory recipients,
        uint256[] memory amounts
    ) public whenTransfersEnabled {
        require(recipients.length == amounts.length, "CompleteNEP17: array length mismatch");
        require(recipients.length > 0, "CompleteNEP17: empty arrays");
        
        // Gas optimization: batch all operations
        Runtime.optimizeGasUsage(
            function() {
                for (uint256 i = 0; i < recipients.length; i++) {
                    _transfer(msg.sender, recipients[i], amounts[i], "");
                }
            },
            recipients.length * 50000 // Expected gas savings
        );
        
        // Single batch notification
        Runtime.notify("BatchTransfer", abi.encode(msg.sender, recipients, amounts));
    }
    
    // ========== Emergency Functions ==========
    
    /**
     * @dev Emergency pause with witness verification
     */
    function emergencyPause() public override {
        require(Runtime.checkWitness(msg.sender), "CompleteNEP17: invalid witness");
        require(msg.sender == owner() || Neo.isCommittee(msg.sender), "CompleteNEP17: unauthorized");
        
        super.emergencyPause();
        
        // Additional emergency actions
        _config.stakingEnabled = false;
        _config.oracleEnabled = false;
        
        Runtime.notify("EmergencyPauseComplete", abi.encode(msg.sender, block.timestamp));
    }
    
    /**
     * @dev Emergency recovery
     */
    function emergencyRecover() public onlyOwner {
        require(Runtime.checkWitness(msg.sender), "CompleteNEP17: invalid witness");
        
        // Restore normal operations
        super.emergencyUnpause();
        _config.stakingEnabled = true;
        _config.oracleEnabled = _config.oracleContract != address(0);
        
        Runtime.notify("EmergencyRecovery", abi.encode(msg.sender, block.timestamp));
    }
    
    // ========== Advanced Admin Functions ==========
    
    /**
     * @dev Update staking configuration
     */
    function updateStakingConfig(
        uint256 newRewardRate,
        uint256 newMinimumStake
    ) public onlyOwner {
        require(newRewardRate <= 20, "CompleteNEP17: reward rate too high"); // Max 20%
        require(newMinimumStake > 0, "CompleteNEP17: minimum stake must be positive");
        
        emit ConfigUpdated("stakingRewardRate", _config.stakingRewardRate, newRewardRate);
        emit ConfigUpdated("minimumStake", _config.minimumStake, newMinimumStake);
        
        _config.stakingRewardRate = newRewardRate;
        _config.minimumStake = newMinimumStake;
    }
    
    /**
     * @dev Update oracle configuration
     */
    function updateOracleConfig(address newOracleContract) public onlyOwner {
        require(newOracleContract != address(0), "CompleteNEP17: invalid oracle contract");
        
        address oldOracle = _config.oracleContract;
        _config.oracleContract = newOracleContract;
        _oracle = NEP24Oracle(newOracleContract);
        _config.oracleEnabled = true;
        
        Runtime.notify("OracleConfigUpdated", abi.encode(oldOracle, newOracleContract));
    }
    
    // ========== View Functions ==========
    
    /**
     * @dev Get complete token information
     */
    function getTokenInfo() public view override returns (
        string memory tokenName,
        string memory tokenSymbol,
        uint8 tokenDecimals,
        uint256 tokenTotalSupply,
        uint256 tokenMaxSupply,
        address tokenMinter,
        bool tokenTransfersEnabled
    ) {
        return super.getTokenInfo();
    }
    
    /**
     * @dev Get staking information
     */
    function getStakingInfo(address account) public view returns (
        uint256 stakedAmount,
        uint256 lockPeriod,
        uint256 lockExpiry,
        uint256 pendingRewards,
        bool canUnstake
    ) {
        StakeInfo memory stakeInfo = _stakes[account];
        stakedAmount = stakeInfo.amount;
        lockPeriod = stakeInfo.lockPeriod;
        lockExpiry = stakeInfo.timestamp + stakeInfo.lockPeriod;
        pendingRewards = calculateReward(account);
        canUnstake = block.timestamp >= lockExpiry;
    }
    
    /**
     * @dev Get governance information
     */
    function getGovernanceInfo() public view returns (
        uint256 totalProposals,
        uint256 activeProposals,
        uint256 executedProposals,
        uint256 minimumTokensForProposal
    ) {
        totalProposals = _proposalCounter;
        minimumTokensForProposal = totalSupply() / 100; // 1% of total supply
        
        // Count active and executed proposals by iterating through storage
        Storage.Iterator memory iterator = Storage.find(abi.encode("proposal"));
        
        while (iterator.next()) {
            bytes memory proposalData = iterator.value();
            if (proposalData.length > 0) {
                (, , , , uint256 endTime, , , bool executed) = abi.decode(
                    proposalData, 
                    (bytes32, address, string, bytes, uint256, uint256, uint256, bool)
                );
                
                if (!executed && block.timestamp <= endTime) {
                    activeProposals++;
                } else if (executed) {
                    executedProposals++;
                }
            }
        }
    }
    
    /**
     * @dev Get comprehensive contract state
     */
    function getContractState() public view returns (
        TokenConfig memory config,
        uint256 totalStaked,
        uint256 totalRewards,
        uint256 contractGasBalance,
        uint256 contractNeoBalance
    ) {
        config = _config;
        totalStaked = _totalStaked;
        totalRewards = 0; // Would sum all user rewards
        contractGasBalance = Neo.getGasBalance(address(this));
        contractNeoBalance = Neo.getNeoBalance(address(this));
    }
}