// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

/**
 * @title ERC721Token
 * @dev Complete ERC721 NFT implementation for Neo blockchain
 * Features: minting, burning, transfers, approvals, metadata, enumerable, royalties
 */
contract ERC721Token {
    // Token name
    string private _name;
    
    // Token symbol
    string private _symbol;
    
    // Base URI for token metadata
    string private _baseTokenURI;
    
    // Contract URI for contract metadata
    string private _contractURI;
    
    // Owner of the contract
    address private _owner;
    
    // Paused state
    bool private _paused;
    
    // Current token ID counter
    uint256 private _currentTokenId;
    
    // Total supply of tokens
    uint256 private _totalSupply;
    
    // Default royalty information
    address private _defaultRoyaltyReceiver;
    uint96 private _defaultRoyaltyFraction; // out of 10,000
    
    // Maximum supply of tokens (0 = unlimited)
    uint256 private _maxSupply;
    
    // Mapping from token ID to owner address
    mapping(uint256 => address) private _owners;
    
    // Mapping owner address to token count
    mapping(address => uint256) private _balances;
    
    // Mapping from token ID to approved address
    mapping(uint256 => address) private _tokenApprovals;
    
    // Mapping from owner to operator approvals
    mapping(address => mapping(address => bool)) private _operatorApprovals;
    
    // Mapping from token ID to token URI (if different from base)
    mapping(uint256 => string) private _tokenURIs;
    
    // Mapping from token ID to royalty info
    mapping(uint256 => RoyaltyInfo) private _tokenRoyaltyInfo;
    
    // Array of all token IDs (for enumeration)
    uint256[] private _allTokens;
    
    // Mapping from token ID to index in _allTokens array
    mapping(uint256 => uint256) private _allTokensIndex;
    
    // Mapping from owner to list of owned token IDs
    mapping(address => mapping(uint256 => uint256)) private _ownedTokens;
    
    // Mapping from token ID to index in owner's token list
    mapping(uint256 => uint256) private _ownedTokensIndex;
    
    struct RoyaltyInfo {
        address receiver;
        uint96 royaltyFraction;
    }
    
    // Events
    event Transfer(address indexed from, address indexed to, uint256 indexed tokenId);
    event Approval(address indexed owner, address indexed approved, uint256 indexed tokenId);
    event ApprovalForAll(address indexed owner, address indexed operator, bool approved);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event Pause();
    event Unpause();
    event BaseURIUpdated(string newBaseURI);
    event ContractURIUpdated(string newContractURI);
    event DefaultRoyaltySet(address receiver, uint96 feeNumerator);
    event TokenRoyaltySet(uint256 tokenId, address receiver, uint96 feeNumerator);
    event MaxSupplySet(uint256 maxSupply);
    
    // Custom errors
    error NotOwnerNorApproved();
    error NonexistentToken();
    error TransferToNonERC721Receiver();
    error InvalidAddress();
    error ContractPaused();
    error MaxSupplyExceeded();
    error RoyaltyTooHigh();
    error OnlyOwner();
    error TokenAlreadyExists();
    error InvalidTokenId();
    
    // Modifiers
    modifier onlyOwner() {
        if (msg.sender != _owner) revert OnlyOwner();
        _;
    }
    
    modifier whenNotPaused() {
        if (_paused) revert ContractPaused();
        _;
    }
    
    modifier validAddress(address account) {
        if (account == address(0)) revert InvalidAddress();
        _;
    }
    
    modifier tokenExists(uint256 tokenId) {
        if (!_exists(tokenId)) revert NonexistentToken();
        _;
    }
    
    /**
     * @dev Constructor sets the initial NFT collection metadata
     * @param name_ The name of the token collection
     * @param symbol_ The symbol of the token collection
     * @param baseTokenURI_ The base URI for token metadata
     * @param maxSupply_ Maximum supply of tokens (0 for unlimited)
     */
    constructor(
        string memory name_,
        string memory symbol_,
        string memory baseTokenURI_,
        uint256 maxSupply_
    ) {
        require(bytes(name_).length > 0, "ERC721: name cannot be empty");
        require(bytes(symbol_).length > 0, "ERC721: symbol cannot be empty");
        
        _name = name_;
        _symbol = symbol_;
        _baseTokenURI = baseTokenURI_;
        _maxSupply = maxSupply_;
        _owner = msg.sender;
        _paused = false;
        _currentTokenId = 1; // Start from token ID 1
        
        emit OwnershipTransferred(address(0), msg.sender);
        if (maxSupply_ > 0) {
            emit MaxSupplySet(maxSupply_);
        }
    }
    
    /**
     * @dev See {IERC165-supportsInterface}
     */
    function supportsInterface(bytes4 interfaceId) public pure returns (bool) {
        return
            interfaceId == 0x01ffc9a7 || // ERC165
            interfaceId == 0x80ac58cd || // ERC721
            interfaceId == 0x5b5e139f || // ERC721Metadata
            interfaceId == 0x780e9d63 || // ERC721Enumerable
            interfaceId == 0x2a55205a;   // ERC2981 (Royalties)
    }
    
    /**
     * @dev Returns the token collection name
     */
    function name() public view returns (string memory) {
        return _name;
    }
    
    /**
     * @dev Returns the token collection symbol
     */
    function symbol() public view returns (string memory) {
        return _symbol;
    }
    
    /**
     * @dev Returns the total number of tokens in existence
     */
    function totalSupply() public view returns (uint256) {
        return _totalSupply;
    }
    
    /**
     * @dev Returns the maximum supply of tokens
     */
    function maxSupply() public view returns (uint256) {
        return _maxSupply;
    }
    
    /**
     * @dev Returns the number of tokens owned by owner
     */
    function balanceOf(address owner) public view validAddress(owner) returns (uint256) {
        return _balances[owner];
    }
    
    /**
     * @dev Returns the owner of the tokenId token
     */
    function ownerOf(uint256 tokenId) public view tokenExists(tokenId) returns (address) {
        return _owners[tokenId];
    }
    
    /**
     * @dev Returns the account approved for tokenId token
     */
    function getApproved(uint256 tokenId) public view tokenExists(tokenId) returns (address) {
        return _tokenApprovals[tokenId];
    }
    
    /**
     * @dev Returns if the operator is allowed to manage all of the assets of owner
     */
    function isApprovedForAll(address owner, address operator) public view returns (bool) {
        return _operatorApprovals[owner][operator];
    }
    
    /**
     * @dev Returns the contract owner
     */
    function owner() public view returns (address) {
        return _owner;
    }
    
    /**
     * @dev Returns if the contract is paused
     */
    function paused() public view returns (bool) {
        return _paused;
    }
    
    /**
     * @dev Returns the base URI for tokens
     */
    function baseURI() public view returns (string memory) {
        return _baseTokenURI;
    }
    
    /**
     * @dev Returns the contract URI for contract metadata
     */
    function contractURI() public view returns (string memory) {
        return _contractURI;
    }
    
    /**
     * @dev Returns the token URI for a given token ID
     */
    function tokenURI(uint256 tokenId) public view tokenExists(tokenId) returns (string memory) {
        string memory _tokenURI = _tokenURIs[tokenId];
        string memory base = _baseTokenURI;
        
        // If there is no base URI, return the token URI
        if (bytes(base).length == 0) {
            return _tokenURI;
        }
        
        // If both are set, concatenate the baseURI and tokenURI
        if (bytes(_tokenURI).length > 0) {
            return string(abi.encodePacked(base, _tokenURI));
        }
        
        // If there is a baseURI but no tokenURI, concatenate the tokenId to the baseURI
        return string(abi.encodePacked(base, _toString(tokenId)));
    }
    
    /**
     * @dev Returns a token ID owned by owner at a given index
     */
    function tokenOfOwnerByIndex(address owner, uint256 index) 
        public 
        view 
        validAddress(owner) 
        returns (uint256) 
    {
        require(index < balanceOf(owner), "ERC721: owner index out of bounds");
        return _ownedTokens[owner][index];
    }
    
    /**
     * @dev Returns a token ID at a given index of all tokens
     */
    function tokenByIndex(uint256 index) public view returns (uint256) {
        require(index < totalSupply(), "ERC721: global index out of bounds");
        return _allTokens[index];
    }
    
    /**
     * @dev Approve or remove operator as an operator for the caller
     */
    function setApprovalForAll(address operator, bool approved) public whenNotPaused {
        if (msg.sender == operator) revert InvalidAddress();
        _operatorApprovals[msg.sender][operator] = approved;
        emit ApprovalForAll(msg.sender, operator, approved);
    }
    
    /**
     * @dev Gives permission to to transfer tokenId token to another account
     */
    function approve(address to, uint256 tokenId) public whenNotPaused tokenExists(tokenId) {
        address owner = ownerOf(tokenId);
        if (to == owner) revert InvalidAddress();
        
        if (msg.sender != owner && !isApprovedForAll(owner, msg.sender)) {
            revert NotOwnerNorApproved();
        }
        
        _approve(to, tokenId);
    }
    
    /**
     * @dev Transfers tokenId token from from to to
     */
    function transferFrom(address from, address to, uint256 tokenId) 
        public 
        whenNotPaused 
        tokenExists(tokenId)
    {
        if (!_isApprovedOrOwner(msg.sender, tokenId)) revert NotOwnerNorApproved();
        _transfer(from, to, tokenId);
    }
    
    /**
     * @dev Safely transfers tokenId token from from to to
     */
    function safeTransferFrom(address from, address to, uint256 tokenId) public {
        safeTransferFrom(from, to, tokenId, "");
    }
    
    /**
     * @dev Safely transfers tokenId token from from to to with data
     */
    function safeTransferFrom(
        address from,
        address to,
        uint256 tokenId,
        bytes memory data
    ) public whenNotPaused tokenExists(tokenId) {
        if (!_isApprovedOrOwner(msg.sender, tokenId)) revert NotOwnerNorApproved();
        _safeTransfer(from, to, tokenId, data);
    }
    
    /**
     * @dev Mints a new token to the specified address
     */
    function mint(address to) public onlyOwner validAddress(to) returns (uint256) {
        if (_maxSupply > 0 && _totalSupply >= _maxSupply) revert MaxSupplyExceeded();
        
        uint256 tokenId = _currentTokenId;
        _currentTokenId++;
        
        _mint(to, tokenId);
        return tokenId;
    }
    
    /**
     * @dev Mints a new token with custom URI to the specified address
     */
    function mintWithURI(address to, string memory uri) 
        public 
        onlyOwner 
        validAddress(to) 
        returns (uint256) 
    {
        uint256 tokenId = mint(to);
        _setTokenURI(tokenId, uri);
        return tokenId;
    }
    
    /**
     * @dev Batch mint tokens to multiple addresses
     */
    function batchMint(address[] memory recipients) public onlyOwner returns (uint256[] memory) {
        require(recipients.length > 0, "ERC721: empty recipients array");
        require(recipients.length <= 100, "ERC721: too many recipients");
        
        if (_maxSupply > 0) {
            require(_totalSupply + recipients.length <= _maxSupply, "ERC721: exceeds max supply");
        }
        
        uint256[] memory tokenIds = new uint256[](recipients.length);
        
        for (uint256 i = 0; i < recipients.length; i++) {
            if (recipients[i] == address(0)) revert InvalidAddress();
            
            uint256 tokenId = _currentTokenId;
            _currentTokenId++;
            
            _mint(recipients[i], tokenId);
            tokenIds[i] = tokenId;
        }
        
        return tokenIds;
    }
    
    /**
     * @dev Burns a token
     */
    function burn(uint256 tokenId) public tokenExists(tokenId) {
        if (!_isApprovedOrOwner(msg.sender, tokenId)) revert NotOwnerNorApproved();
        _burn(tokenId);
    }
    
    /**
     * @dev Sets the base URI for all tokens
     */
    function setBaseURI(string memory newBaseURI) public onlyOwner {
        _baseTokenURI = newBaseURI;
        emit BaseURIUpdated(newBaseURI);
    }
    
    /**
     * @dev Sets the contract URI for contract metadata
     */
    function setContractURI(string memory newContractURI) public onlyOwner {
        _contractURI = newContractURI;
        emit ContractURIUpdated(newContractURI);
    }
    
    /**
     * @dev Sets the URI for a specific token
     */
    function setTokenURI(uint256 tokenId, string memory uri) 
        public 
        onlyOwner 
        tokenExists(tokenId) 
    {
        _setTokenURI(tokenId, uri);
    }
    
    /**
     * @dev Sets the default royalty for all tokens
     */
    function setDefaultRoyalty(address receiver, uint96 feeNumerator) 
        public 
        onlyOwner 
        validAddress(receiver) 
    {
        if (feeNumerator > 1000) revert RoyaltyTooHigh(); // Max 10%
        _defaultRoyaltyReceiver = receiver;
        _defaultRoyaltyFraction = feeNumerator;
        emit DefaultRoyaltySet(receiver, feeNumerator);
    }
    
    /**
     * @dev Sets the royalty for a specific token
     */
    function setTokenRoyalty(uint256 tokenId, address receiver, uint96 feeNumerator) 
        public 
        onlyOwner 
        tokenExists(tokenId)
        validAddress(receiver)
    {
        if (feeNumerator > 1000) revert RoyaltyTooHigh(); // Max 10%
        _tokenRoyaltyInfo[tokenId] = RoyaltyInfo(receiver, feeNumerator);
        emit TokenRoyaltySet(tokenId, receiver, feeNumerator);
    }
    
    /**
     * @dev Returns royalty information for a token
     */
    function royaltyInfo(uint256 tokenId, uint256 salePrice) 
        public 
        view 
        tokenExists(tokenId)
        returns (address receiver, uint256 royaltyAmount) 
    {
        RoyaltyInfo memory royalty = _tokenRoyaltyInfo[tokenId];
        
        if (royalty.receiver == address(0)) {
            royalty.receiver = _defaultRoyaltyReceiver;
            royalty.royaltyFraction = _defaultRoyaltyFraction;
        }
        
        receiver = royalty.receiver;
        royaltyAmount = (salePrice * royalty.royaltyFraction) / 10000;
    }
    
    /**
     * @dev Pauses all token transfers
     */
    function pause() public onlyOwner {
        require(!_paused, "ERC721: already paused");
        _paused = true;
        emit Pause();
    }
    
    /**
     * @dev Unpauses all token transfers
     */
    function unpause() public onlyOwner {
        require(_paused, "ERC721: not paused");
        _paused = false;
        emit Unpause();
    }
    
    /**
     * @dev Transfers ownership of the contract to a new account
     */
    function transferOwnership(address newOwner) public onlyOwner validAddress(newOwner) {
        emit OwnershipTransferred(_owner, newOwner);
        _owner = newOwner;
    }
    
    /**
     * @dev Renounces ownership of the contract
     */
    function renounceOwnership() public onlyOwner {
        emit OwnershipTransferred(_owner, address(0));
        _owner = address(0);
    }
    
    /**
     * @dev Returns all token IDs owned by an address
     */
    function tokensOfOwner(address owner) public view validAddress(owner) returns (uint256[] memory) {
        uint256 tokenCount = balanceOf(owner);
        if (tokenCount == 0) {
            return new uint256[](0);
        }
        
        uint256[] memory tokenIds = new uint256[](tokenCount);
        for (uint256 i = 0; i < tokenCount; i++) {
            tokenIds[i] = tokenOfOwnerByIndex(owner, i);
        }
        return tokenIds;
    }
    
    // Internal functions
    
    /**
     * @dev Returns whether tokenId exists
     */
    function _exists(uint256 tokenId) internal view returns (bool) {
        return _owners[tokenId] != address(0);
    }
    
    /**
     * @dev Returns whether spender is allowed to manage tokenId
     */
    function _isApprovedOrOwner(address spender, uint256 tokenId) internal view returns (bool) {
        address owner = ownerOf(tokenId);
        return (spender == owner || getApproved(tokenId) == spender || isApprovedForAll(owner, spender));
    }
    
    /**
     * @dev Internal mint function
     */
    function _mint(address to, uint256 tokenId) internal {
        if (to == address(0)) revert InvalidAddress();
        if (_exists(tokenId)) revert TokenAlreadyExists();
        
        _beforeTokenTransfer(address(0), to, tokenId);
        
        _balances[to] += 1;
        _owners[tokenId] = to;
        
        _addTokenToAllTokensEnumeration(tokenId);
        _addTokenToOwnerEnumeration(to, tokenId);
        
        emit Transfer(address(0), to, tokenId);
        
        _totalSupply++;
    }
    
    /**
     * @dev Internal burn function
     */
    function _burn(uint256 tokenId) internal {
        address owner = ownerOf(tokenId);
        
        _beforeTokenTransfer(owner, address(0), tokenId);
        
        // Clear approvals
        _approve(address(0), tokenId);
        
        _balances[owner] -= 1;
        delete _owners[tokenId];
        
        // Clear token URI if it exists
        if (bytes(_tokenURIs[tokenId]).length != 0) {
            delete _tokenURIs[tokenId];
        }
        
        // Clear token royalty if it exists
        if (_tokenRoyaltyInfo[tokenId].receiver != address(0)) {
            delete _tokenRoyaltyInfo[tokenId];
        }
        
        _removeTokenFromOwnerEnumeration(owner, tokenId);
        _removeTokenFromAllTokensEnumeration(tokenId);
        
        emit Transfer(owner, address(0), tokenId);
        
        _totalSupply--;
    }
    
    /**
     * @dev Internal transfer function
     */
    function _transfer(address from, address to, uint256 tokenId) internal {
        if (ownerOf(tokenId) != from) revert InvalidAddress();
        if (to == address(0)) revert InvalidAddress();
        
        _beforeTokenTransfer(from, to, tokenId);
        
        // Clear approvals from the previous owner
        _approve(address(0), tokenId);
        
        _balances[from] -= 1;
        _balances[to] += 1;
        _owners[tokenId] = to;
        
        _removeTokenFromOwnerEnumeration(from, tokenId);
        _addTokenToOwnerEnumeration(to, tokenId);
        
        emit Transfer(from, to, tokenId);
    }
    
    /**
     * @dev Internal approve function
     */
    function _approve(address to, uint256 tokenId) internal {
        _tokenApprovals[tokenId] = to;
        emit Approval(ownerOf(tokenId), to, tokenId);
    }
    
    /**
     * @dev Internal safe transfer function
     */
    function _safeTransfer(address from, address to, uint256 tokenId, bytes memory data) internal {
        _transfer(from, to, tokenId);
        if (!_checkOnERC721Received(from, to, tokenId, data)) {
            revert TransferToNonERC721Receiver();
        }
    }
    
    /**
     * @dev Internal function to set token URI
     */
    function _setTokenURI(uint256 tokenId, string memory uri) internal {
        _tokenURIs[tokenId] = uri;
    }
    
    /**
     * @dev Hook that is called before any token transfer
     */
    function _beforeTokenTransfer(address from, address to, uint256 tokenId) internal {
        // Can be overridden for additional functionality
    }
    
    /**
     * @dev Private function to add a token to all tokens enumeration
     */
    function _addTokenToAllTokensEnumeration(uint256 tokenId) private {
        _allTokensIndex[tokenId] = _allTokens.length;
        _allTokens.push(tokenId);
    }
    
    /**
     * @dev Private function to remove a token from all tokens enumeration
     */
    function _removeTokenFromAllTokensEnumeration(uint256 tokenId) private {
        uint256 lastTokenIndex = _allTokens.length - 1;
        uint256 tokenIndex = _allTokensIndex[tokenId];
        uint256 lastTokenId = _allTokens[lastTokenIndex];
        
        _allTokens[tokenIndex] = lastTokenId;
        _allTokensIndex[lastTokenId] = tokenIndex;
        
        delete _allTokensIndex[tokenId];
        _allTokens.pop();
    }
    
    /**
     * @dev Private function to add a token to owner enumeration
     */
    function _addTokenToOwnerEnumeration(address to, uint256 tokenId) private {
        uint256 length = balanceOf(to);
        _ownedTokens[to][length] = tokenId;
        _ownedTokensIndex[tokenId] = length;
    }
    
    /**
     * @dev Private function to remove a token from owner enumeration
     */
    function _removeTokenFromOwnerEnumeration(address from, uint256 tokenId) private {
        uint256 lastTokenIndex = balanceOf(from) - 1;
        uint256 tokenIndex = _ownedTokensIndex[tokenId];
        
        if (tokenIndex != lastTokenIndex) {
            uint256 lastTokenId = _ownedTokens[from][lastTokenIndex];
            _ownedTokens[from][tokenIndex] = lastTokenId;
            _ownedTokensIndex[lastTokenId] = tokenIndex;
        }
        
        delete _ownedTokensIndex[tokenId];
        delete _ownedTokens[from][lastTokenIndex];
    }
    
    /**
     * @dev Internal function to check if address is a contract that can receive NFTs
     */
    function _checkOnERC721Received(
        address from,
        address to,
        uint256 tokenId,
        bytes memory data
    ) private returns (bool) {
        if (to.code.length > 0) {
            try IERC721Receiver(to).onERC721Received(msg.sender, from, tokenId, data) returns (bytes4 retval) {
                return retval == IERC721Receiver.onERC721Received.selector;
            } catch (bytes memory reason) {
                if (reason.length == 0) {
                    revert TransferToNonERC721Receiver();
                } else {
                    assembly {
                        revert(add(32, reason), mload(reason))
                    }
                }
            }
        } else {
            return true;
        }
    }
    
    /**
     * @dev Converts a uint256 to its ASCII string decimal representation
     */
    function _toString(uint256 value) internal pure returns (string memory) {
        if (value == 0) {
            return "0";
        }
        uint256 temp = value;
        uint256 digits;
        while (temp != 0) {
            digits++;
            temp /= 10;
        }
        bytes memory buffer = new bytes(digits);
        while (value != 0) {
            digits -= 1;
            buffer[digits] = bytes1(uint8(48 + uint256(value % 10)));
            value /= 10;
        }
        return string(buffer);
    }
}

/**
 * @title IERC721Receiver
 * @dev Interface for contracts that can receive NFTs
 */
interface IERC721Receiver {
    function onERC721Received(
        address operator,
        address from,
        uint256 tokenId,
        bytes calldata data
    ) external returns (bytes4);
}