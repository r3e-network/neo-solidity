// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

/**
 * @title NEP-24 Centralized Oracle Standard
 * @dev Complete implementation of Neo N3 NEP-24 oracle standard for Solidity
 * @author Jimmy <jimmy@r3e.network>
 * 
 * NEP-24 provides standardized oracle services for Neo N3:
 * - External data fetching with URL requests
 * - Callback-based response handling
 * - Gas-efficient oracle operations
 * - Request validation and filtering
 * - Response verification and processing
 */

import "../contracts/Framework.sol";
import "../libraries/Neo.sol";
import "../libraries/Runtime.sol";

/**
 * @title INEP24Oracle
 * @dev Interface for NEP-24 oracle standard
 */
interface INEP24Oracle {
    function request(
        string calldata url,
        string calldata filter,
        string calldata callback,
        bytes calldata userData,
        uint256 gasForResponse
    ) external;
    
    function getPrice() external view returns (uint256);
    
    event OracleRequest(
        uint256 indexed requestId,
        address indexed requester,
        string url,
        string filter
    );
    
    event OracleResponse(
        uint256 indexed requestId,
        address indexed requester,
        uint256 code,
        bytes result
    );
}

/**
 * @title NEP24Oracle
 * @dev Complete NEP-24 oracle implementation with Neo N3 integration
 */
contract NEP24Oracle is INEP24Oracle, Framework {
    using Neo for *;
    using Runtime for *;
    
    // Oracle configuration
    uint256 private _requestCounter;
    uint256 private _oraclePrice;
    address private _oracleContract;
    
    // Request tracking
    mapping(uint256 => OracleRequest) private _requests;
    mapping(address => uint256[]) private _userRequests;
    mapping(bytes32 => uint256) private _urlRequests;
    
    // Oracle settings
    uint256 private constant MAX_URL_LENGTH = 256;
    uint256 private constant MAX_FILTER_LENGTH = 128;
    uint256 private constant MAX_CALLBACK_LENGTH = 64;
    uint256 private constant MIN_GAS_FOR_RESPONSE = 10000000; // 0.1 GAS
    uint256 private constant MAX_GAS_FOR_RESPONSE = 100000000; // 1 GAS
    
    struct OracleRequest {
        uint256 id;
        address requester;
        string url;
        string filter;
        string callback;
        bytes userData;
        uint256 gasForResponse;
        uint256 timestamp;
        uint256 blockHeight;
        RequestStatus status;
        uint256 responseCode;
        bytes responseData;
    }
    
    enum RequestStatus {
        Pending,
        InProgress,
        Completed,
        Failed,
        Expired
    }
    
    // Events
    event OracleRequest(
        uint256 indexed requestId,
        address indexed requester,
        string url,
        string filter
    );
    
    event OracleResponse(
        uint256 indexed requestId,
        address indexed requester,
        uint256 code,
        bytes result
    );
    
    event OracleRequestExpired(uint256 indexed requestId);
    event OraclePriceUpdated(uint256 oldPrice, uint256 newPrice);
    event OracleContractUpdated(address oldContract, address newContract);
    
    // Custom errors
    error NEP24InvalidURL(string url);
    error NEP24InvalidFilter(string filter);
    error NEP24InvalidCallback(string callback);
    error NEP24InsufficientGas(uint256 provided, uint256 required);
    error NEP24RequestNotFound(uint256 requestId);
    error NEP24UnauthorizedResponse();
    error NEP24RequestExpired(uint256 requestId);
    error NEP24InvalidPrice(uint256 price);
    
    // Modifiers
    modifier validURL(string memory url) {
        if (bytes(url).length == 0 || bytes(url).length > MAX_URL_LENGTH) {
            revert NEP24InvalidURL(url);
        }
        _;
    }
    
    modifier validFilter(string memory filter) {
        if (bytes(filter).length > MAX_FILTER_LENGTH) {
            revert NEP24InvalidFilter(filter);
        }
        _;
    }
    
    modifier validCallback(string memory callback) {
        if (bytes(callback).length == 0 || bytes(callback).length > MAX_CALLBACK_LENGTH) {
            revert NEP24InvalidCallback(callback);
        }
        _;
    }
    
    modifier sufficientGas(uint256 gasForResponse) {
        if (gasForResponse < MIN_GAS_FOR_RESPONSE || gasForResponse > MAX_GAS_FOR_RESPONSE) {
            revert NEP24InsufficientGas(gasForResponse, MIN_GAS_FOR_RESPONSE);
        }
        _;
    }
    
    modifier onlyOracleContract() {
        if (msg.sender != _oracleContract) revert NEP24UnauthorizedResponse();
        _;
    }
    
    /**
     * @dev Constructor
     */
    constructor(uint256 initialPrice) Framework() {
        _oraclePrice = initialPrice;
        _oracleContract = 0xfe924b7cfe89ddd271abaf7210a80a7e11178758; // Oracle native contract
        _requestCounter = 1;
    }
    
    // ========== Oracle Request Functions ==========
    
    /**
     * @dev Make oracle request (NEP-24 standard)
     */
    function request(
        string calldata url,
        string calldata filter,
        string calldata callback,
        bytes calldata userData,
        uint256 gasForResponse
    ) 
        external 
        override
        validURL(url)
        validFilter(filter)
        validCallback(callback)
        sufficientGas(gasForResponse)
    {
        // Check payment
        uint256 totalCost = _oraclePrice + gasForResponse;
        require(Neo.getGasBalance(msg.sender) >= totalCost, "NEP24: insufficient GAS balance");
        
        // Transfer payment
        require(
            Neo.transferGas(msg.sender, address(this), totalCost),
            "NEP24: payment transfer failed"
        );
        
        // Create request
        uint256 requestId = _requestCounter++;
        _requests[requestId] = OracleRequest({
            id: requestId,
            requester: msg.sender,
            url: url,
            filter: filter,
            callback: callback,
            userData: userData,
            gasForResponse: gasForResponse,
            timestamp: block.timestamp,
            blockHeight: block.number,
            status: RequestStatus.Pending,
            responseCode: 0,
            responseData: ""
        });
        
        // Track user requests
        _userRequests[msg.sender].push(requestId);
        
        // Track URL requests for rate limiting
        bytes32 urlHash = keccak256(bytes(url));
        _urlRequests[urlHash]++;
        
        // Make oracle request to native contract
        Syscalls.oracleRequest(url, filter, callback, userData, gasForResponse);
        
        emit OracleRequest(requestId, msg.sender, url, filter);
        
        // Emit Neo-compatible notification
        Runtime.notify("OracleRequest", abi.encode(requestId, msg.sender, url, filter));
    }
    
    /**
     * @dev Batch oracle requests
     */
    function batchRequest(
        string[] calldata urls,
        string[] calldata filters,
        string[] calldata callbacks,
        bytes[] calldata userDatas,
        uint256[] calldata gasAmounts
    ) external returns (uint256[] memory requestIds) {
        require(urls.length == filters.length, "NEP24: array length mismatch");
        require(urls.length == callbacks.length, "NEP24: array length mismatch");
        require(urls.length == userDatas.length, "NEP24: array length mismatch");
        require(urls.length == gasAmounts.length, "NEP24: array length mismatch");
        require(urls.length > 0, "NEP24: empty arrays");
        require(urls.length <= 10, "NEP24: too many requests");
        
        requestIds = new uint256[](urls.length);
        
        for (uint256 i = 0; i < urls.length; i++) {
            // Make individual oracle requests
            uint256 requestId = _requestCounter++;
            
            // Create request record
            _requests[requestId] = OracleRequest({
                id: requestId,
                requester: msg.sender,
                url: urls[i],
                filter: filters[i],
                callback: callbacks[i],
                userData: userDatas[i],
                gasForResponse: gasAmounts[i],
                timestamp: block.timestamp,
                blockHeight: block.number,
                status: RequestStatus.Pending,
                responseCode: 0,
                responseData: ""
            });
            
            // Make syscall
            Syscalls.oracleRequest(urls[i], filters[i], callbacks[i], userDatas[i], gasAmounts[i]);
            
            requestIds[i] = requestId;
        }
        
        return requestIds;
    }
    
    // ========== Oracle Response Handling ==========
    
    /**
     * @dev Handle oracle response (called by oracle contract)
     */
    function oracleResponse(
        uint256 requestId,
        uint256 code,
        bytes calldata result
    ) external onlyOracleContract {
        if (_requests[requestId].requester == address(0)) {
            revert NEP24RequestNotFound(requestId);
        }
        
        OracleRequest storage request_ = _requests[requestId];
        
        // Check if request is expired
        if (block.timestamp > request_.timestamp + 3600) { // 1 hour expiration
            request_.status = RequestStatus.Expired;
            emit OracleRequestExpired(requestId);
            return;
        }
        
        // Update request status
        request_.status = code == 0 ? RequestStatus.Completed : RequestStatus.Failed;
        request_.responseCode = code;
        request_.responseData = result;
        
        emit OracleResponse(requestId, request_.requester, code, result);
        
        // Emit Neo-compatible notification
        Runtime.notify("OracleResponse", abi.encode(requestId, request_.requester, code, result));
        
        // Execute callback if successful
        if (code == 0 && bytes(request_.callback).length > 0) {
            _executeCallback(request_);
        }
    }
    
    /**
     * @dev Execute oracle callback
     */
    function _executeCallback(OracleRequest memory request_) private {
        bytes memory callbackData = abi.encodeWithSignature(
            string(abi.encodePacked(request_.callback, "(uint256,uint256,bytes,bytes)")),
            request_.id,
            request_.responseCode,
            request_.responseData,
            request_.userData
        );
        
        // Call the requester contract with callback
        (bool success, ) = request_.requester.call{gas: request_.gasForResponse}(callbackData);
        
        if (!success) {
            Runtime.log(string(abi.encodePacked(
                "Oracle callback failed for request ",
                Runtime._uint256ToString(request_.id)
            )));
        }
    }
    
    // ========== View Functions ==========
    
    /**
     * @dev Get oracle price
     */
    function getPrice() public view override returns (uint256) {
        return _oraclePrice;
    }
    
    /**
     * @dev Get request details
     */
    function getRequest(uint256 requestId) public view returns (OracleRequest memory) {
        return _requests[requestId];
    }
    
    /**
     * @dev Get user requests
     */
    function getUserRequests(address user) public view returns (uint256[] memory) {
        return _userRequests[user];
    }
    
    /**
     * @dev Get request count
     */
    function getRequestCount() public view returns (uint256) {
        return _requestCounter - 1;
    }
    
    /**
     * @dev Get URL request count (for rate limiting)
     */
    function getURLRequestCount(string memory url) public view returns (uint256) {
        bytes32 urlHash = keccak256(bytes(url));
        return _urlRequests[urlHash];
    }
    
    // ========== Admin Functions ==========
    
    /**
     * @dev Set oracle price
     */
    function setPrice(uint256 newPrice) public onlyOwner {
        if (newPrice == 0) revert NEP24InvalidPrice(newPrice);
        
        uint256 oldPrice = _oraclePrice;
        _oraclePrice = newPrice;
        
        emit OraclePriceUpdated(oldPrice, newPrice);
    }
    
    /**
     * @dev Set oracle contract address
     */
    function setOracleContract(address newContract) public onlyOwner {
        require(newContract != address(0), "NEP24: invalid oracle contract");
        
        address oldContract = _oracleContract;
        _oracleContract = newContract;
        
        emit OracleContractUpdated(oldContract, newContract);
    }
    
    /**
     * @dev Emergency cancel request
     */
    function cancelRequest(uint256 requestId) public {
        OracleRequest storage request_ = _requests[requestId];
        
        require(request_.requester != address(0), "NEP24: request not found");
        require(
            msg.sender == request_.requester || msg.sender == owner(),
            "NEP24: unauthorized cancellation"
        );
        require(
            request_.status == RequestStatus.Pending || request_.status == RequestStatus.InProgress,
            "NEP24: cannot cancel completed request"
        );
        
        request_.status = RequestStatus.Failed;
        
        // Refund gas if request was cancelled by requester
        if (msg.sender == request_.requester) {
            Neo.transferGas(address(this), request_.requester, request_.gasForResponse);
        }
        
        Runtime.notify("OracleRequestCancelled", abi.encode(requestId, msg.sender));
    }
    
    /**
     * @dev Clean expired requests
     */
    function cleanExpiredRequests(uint256[] memory requestIds) public {
        for (uint256 i = 0; i < requestIds.length; i++) {
            OracleRequest storage request_ = _requests[requestIds[i]];
            
            if (request_.status == RequestStatus.Pending && 
                block.timestamp > request_.timestamp + 3600) {
                
                request_.status = RequestStatus.Expired;
                emit OracleRequestExpired(requestIds[i]);
                
                // Refund unused gas
                Neo.transferGas(address(this), request_.requester, request_.gasForResponse);
            }
        }
    }
    
    // ========== Oracle Utilities ==========
    
    /**
     * @dev Validate URL format
     */
    function isValidURL(string memory url) public pure returns (bool) {
        bytes memory urlBytes = bytes(url);
        
        // Check length
        if (urlBytes.length == 0 || urlBytes.length > MAX_URL_LENGTH) {
            return false;
        }
        
        // Check for http/https prefix
        if (urlBytes.length < 7) return false;
        
        bytes memory httpPrefix = bytes("http://");
        bytes memory httpsPrefix = bytes("https://");
        
        bool hasHttpPrefix = true;
        bool hasHttpsPrefix = true;
        
        // Check http prefix
        for (uint256 i = 0; i < 7 && i < urlBytes.length; i++) {
            if (urlBytes[i] != httpPrefix[i]) {
                hasHttpPrefix = false;
                break;
            }
        }
        
        // Check https prefix
        for (uint256 i = 0; i < 8 && i < urlBytes.length; i++) {
            if (urlBytes[i] != httpsPrefix[i]) {
                hasHttpsPrefix = false;
                break;
            }
        }
        
        return hasHttpPrefix || hasHttpsPrefix;
    }
    
    /**
     * @dev Validate JSON filter
     */
    function isValidFilter(string memory filter) public pure returns (bool) {
        bytes memory filterBytes = bytes(filter);
        
        // Empty filter is valid (returns full response)
        if (filterBytes.length == 0) return true;
        
        // Check length
        if (filterBytes.length > MAX_FILTER_LENGTH) return false;
        
        // Basic JSON path validation (starts with $ or .)
        return filterBytes[0] == bytes1("$") || filterBytes[0] == bytes1(".");
    }
    
    /**
     * @dev Estimate request cost
     */
    function estimateRequestCost(uint256 gasForResponse) public view returns (uint256) {
        return _oraclePrice + gasForResponse;
    }
    
    /**
     * @dev Get request statistics
     */
    function getRequestStats(address user) public view returns (
        uint256 totalRequests,
        uint256 pendingRequests,
        uint256 completedRequests,
        uint256 failedRequests
    ) {
        uint256[] memory userRequestIds = _userRequests[user];
        
        for (uint256 i = 0; i < userRequestIds.length; i++) {
            OracleRequest memory request_ = _requests[userRequestIds[i]];
            
            if (request_.status == RequestStatus.Pending || request_.status == RequestStatus.InProgress) {
                pendingRequests++;
            } else if (request_.status == RequestStatus.Completed) {
                completedRequests++;
            } else if (request_.status == RequestStatus.Failed || request_.status == RequestStatus.Expired) {
                failedRequests++;
            }
        }
        
        totalRequests = userRequestIds.length;
    }
    
    // ========== Common Oracle Patterns ==========
    
    /**
     * @dev Request price data
     */
    function requestPriceData(
        string memory symbol,
        string memory callback
    ) public returns (uint256 requestId) {
        string memory url = string(abi.encodePacked(
            "https://api.coinpaprika.com/v1/tickers/",
            symbol,
            "-",
            symbol
        ));
        string memory filter = "$.quotes.USD.price";
        
        request(url, filter, callback, abi.encode(symbol), MIN_GAS_FOR_RESPONSE);
        return _requestCounter - 1;
    }
    
    /**
     * @dev Request weather data
     */
    function requestWeatherData(
        string memory city,
        string memory callback
    ) public returns (uint256 requestId) {
        string memory url = string(abi.encodePacked(
            "https://api.openweathermap.org/data/2.5/weather?q=",
            city
        ));
        string memory filter = "$.main.temp";
        
        request(url, filter, callback, abi.encode(city), MIN_GAS_FOR_RESPONSE);
        return _requestCounter - 1;
    }
    
    /**
     * @dev Request random number
     */
    function requestRandomNumber(string memory callback) public returns (uint256 requestId) {
        string memory url = "https://api.random.org/json-rpc/1/invoke";
        string memory filter = "$.result.random.data[0]";
        
        request(url, filter, callback, "", MIN_GAS_FOR_RESPONSE);
        return _requestCounter - 1;
    }
    
    /**
     * @dev Request blockchain data from external chain
     */
    function requestExternalBlockchainData(
        string memory blockchain,
        string memory endpoint,
        string memory callback
    ) public returns (uint256 requestId) {
        string memory url = string(abi.encodePacked(
            "https://api.",
            blockchain,
            ".com/",
            endpoint
        ));
        
        request(url, "", callback, abi.encode(blockchain, endpoint), MIN_GAS_FOR_RESPONSE);
        return _requestCounter - 1;
    }
    
    // ========== Response Processing ==========
    
    /**
     * @dev Standard price callback
     */
    function priceCallback(
        uint256 requestId,
        uint256 code,
        bytes calldata result,
        bytes calldata userData
    ) external {
        require(msg.sender == address(this), "NEP24: internal callback only");
        
        if (code == 0) {
            // Parse price data
            uint256 price = abi.decode(result, (uint256));
            string memory symbol = abi.decode(userData, (string));
            
            // Store price data
            Storage.put(
                abi.encode("price", symbol),
                abi.encode(price, block.timestamp)
            );
            
            Runtime.notify("PriceUpdated", abi.encode(symbol, price, block.timestamp));
        }
    }
    
    /**
     * @dev Standard weather callback
     */
    function weatherCallback(
        uint256 requestId,
        uint256 code,
        bytes calldata result,
        bytes calldata userData
    ) external {
        require(msg.sender == address(this), "NEP24: internal callback only");
        
        if (code == 0) {
            // Parse weather data
            int256 temperature = abi.decode(result, (int256));
            string memory city = abi.decode(userData, (string));
            
            // Store weather data
            Storage.put(
                abi.encode("weather", city),
                abi.encode(temperature, block.timestamp)
            );
            
            Runtime.notify("WeatherUpdated", abi.encode(city, temperature, block.timestamp));
        }
    }
    
    /**
     * @dev Generic data callback
     */
    function dataCallback(
        uint256 requestId,
        uint256 code,
        bytes calldata result,
        bytes calldata userData
    ) external {
        require(msg.sender == address(this), "NEP24: internal callback only");
        
        OracleRequest storage request_ = _requests[requestId];
        request_.status = code == 0 ? RequestStatus.Completed : RequestStatus.Failed;
        request_.responseCode = code;
        request_.responseData = result;
        
        // Store generic result
        Storage.put(
            abi.encode("oracle_result", requestId),
            abi.encode(code, result, userData, block.timestamp)
        );
        
        Runtime.notify("OracleDataReceived", abi.encode(requestId, code, result));
    }
    
    // ========== Oracle Management ==========
    
    /**
     * @dev Get oracle statistics
     */
    function getOracleStats() public view returns (
        uint256 totalRequests,
        uint256 pendingRequests,
        uint256 completedRequests,
        uint256 failedRequests,
        uint256 currentPrice
    ) {
        for (uint256 i = 1; i < _requestCounter; i++) {
            RequestStatus status = _requests[i].status;
            
            if (status == RequestStatus.Pending || status == RequestStatus.InProgress) {
                pendingRequests++;
            } else if (status == RequestStatus.Completed) {
                completedRequests++;
            } else if (status == RequestStatus.Failed || status == RequestStatus.Expired) {
                failedRequests++;
            }
        }
        
        totalRequests = _requestCounter - 1;
        currentPrice = _oraclePrice;
    }
    
    /**
     * @dev Get popular URLs
     */
    function getPopularURLs() public view returns (
        string[] memory urls,
        uint256[] memory counts
    ) {
        // Get all URL request counts from storage
        Storage.Iterator memory iterator = Storage.find(abi.encode("url_count"));
        
        string[] memory tempUrls = new string[](100);
        uint256[] memory tempCounts = new uint256[](100);
        uint256 foundCount = 0;
        
        while (iterator.next() && foundCount < 100) {
            bytes memory countData = iterator.value();
            if (countData.length > 0) {
                uint256 count = abi.decode(countData, (uint256));
                if (count > 0) {
                    // Extract URL from key
                    bytes memory key = iterator.currentKey;
                    string memory url = abi.decode(key, (string));
                    
                    tempUrls[foundCount] = url;
                    tempCounts[foundCount] = count;
                    foundCount++;
                }
            }
        }
        
        // Sort by count (descending) and resize arrays
        for (uint256 i = 0; i < foundCount - 1; i++) {
            for (uint256 j = i + 1; j < foundCount; j++) {
                if (tempCounts[j] > tempCounts[i]) {
                    // Swap
                    uint256 tempCount = tempCounts[i];
                    string memory tempUrl = tempUrls[i];
                    tempCounts[i] = tempCounts[j];
                    tempUrls[i] = tempUrls[j];
                    tempCounts[j] = tempCount;
                    tempUrls[j] = tempUrl;
                }
            }
        }
        
        urls = new string[](foundCount);
        counts = new uint256[](foundCount);
        
        for (uint256 i = 0; i < foundCount; i++) {
            urls[i] = tempUrls[i];
            counts[i] = tempCounts[i];
        }
    }
    
    /**
     * @dev Emergency oracle shutdown
     */
    function emergencyShutdown() public onlyOwner {
        // Disable new requests by setting extremely high price
        uint256 oldPrice = _oraclePrice;
        _oraclePrice = type(uint256).max;
        
        emit OraclePriceUpdated(oldPrice, _oraclePrice);
        Runtime.notify("OracleEmergencyShutdown", abi.encode(msg.sender, block.timestamp));
    }
    
    /**
     * @dev Restore oracle service
     */
    function restoreService(uint256 normalPrice) public onlyOwner {
        require(normalPrice > 0 && normalPrice < type(uint256).max, "NEP24: invalid price");
        
        uint256 oldPrice = _oraclePrice;
        _oraclePrice = normalPrice;
        
        emit OraclePriceUpdated(oldPrice, normalPrice);
        Runtime.notify("OracleServiceRestored", abi.encode(msg.sender, block.timestamp));
    }
}