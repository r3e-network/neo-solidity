// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

/**
 * @title Native Calls Data Structures
 */

library NativeTypes {
    struct NeoCandidate {
        bytes publicKey;
        uint256 votes;
    }
    /// Mirror of NeoToken NeoAccountState:
    /// [balance, balanceHeight, voteTo (ECPoint), lastGasPerVote]
    struct AccountState {
        uint256 balance;
        uint256 balanceHeight;
        bytes voteTo;
        uint256 lastGasPerVote;
    }
    /// NeoVM ContractState — field types aligned with on-chain wire format.
    /// `hash` is Hash160 (address), `id` is a signed integer, and
    /// `updateCounter` is an unsigned integer.
    struct ContractState {
        address hash;
        bytes nef;
        bytes manifest;
        int256 id;
        uint256 updateCounter;
    }
    struct NetworkConfig {
        uint256 maxTraceableBlocks;
        uint256 maxValidUntilBlockIncrement;
        uint256 feePerByte;
        uint256 execFeeFactor;
        uint256 storagePrice;
        uint256 msPerBlock;
    }
}
