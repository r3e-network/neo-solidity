import { NeoRpcProvider, InvokeResult } from "@neo-solidity/types";
import Debug from "debug";

const debug = Debug("neo-solidity:rpc-adapter");

/**
 * RPC adapter that bridges Ethereum-style calls to Neo RPC
 */
export class RpcAdapter {
  private rpcProvider: NeoRpcProvider;

  constructor(rpcProvider: NeoRpcProvider) {
    this.rpcProvider = rpcProvider;
  }

  /**
   * Invoke contract function (read-only)
   */
  async invokeFunction(
    contractAddress: string,
    methodName: string,
    args: any[] = [],
    blockTag?: string | number
  ): Promise<InvokeResult> {
    debug(`Invoking function ${methodName} on ${contractAddress}`);

    try {
      const scriptHash = this.addressToScriptHash(contractAddress);
      
      // Convert arguments to Neo RPC format
      const neoParams = this.convertArgsToNeoParams(args);
      
      const result = await this.rpcProvider.invokeFunction(
        scriptHash,
        methodName,
        neoParams
      );

      if (result.state === "FAULT") {
        throw new Error(`Function call failed: ${result.exception || "Unknown error"}`);
      }

      return result;
    } catch (error) {
      debug(`Function invocation failed: ${error}`);
      throw error;
    }
  }

  /**
   * Estimate gas for transaction
   */
  async estimateGas(
    contractAddress: string,
    methodName: string,
    args: any[] = [],
    options: {
      from?: string;
      value?: string;
    } = {}
  ): Promise<{
    systemFee: string;
    networkFee: string;
    totalGas: string;
  }> {
    debug(`Estimating gas for ${methodName} on ${contractAddress}`);

    try {
      // Invoke function to get gas consumption
      const scriptHash = this.addressToScriptHash(contractAddress);
      const neoParams = this.convertArgsToNeoParams(args);
      
      const result = await this.rpcProvider.invokeFunction(
        scriptHash,
        methodName,
        neoParams
      );

      // Calculate fees based on gas consumption
      const systemFee = result.gasConsumed;
      const networkFee = "1000000"; // 0.01 GAS for network fee (rough estimate)
      const totalGas = (BigInt(systemFee) + BigInt(networkFee)).toString();

      return {
        systemFee,
        networkFee,
        totalGas
      };
    } catch (error) {
      debug(`Gas estimation failed: ${error}`);
      throw error;
    }
  }

  /**
   * Get transaction receipt
   */
  async getTransactionReceipt(txHash: string): Promise<any> {
    debug(`Getting transaction receipt for ${txHash}`);

    try {
      // Get transaction details
      const tx = await this.rpcProvider.getTransaction(txHash);
      
      // Get application log for events and execution details
      const appLog = await this.rpcProvider.getApplicationLog(txHash);
      
      // Convert to Ethereum-style receipt
      return this.convertToEthereumReceipt(tx, appLog);
    } catch (error) {
      debug(`Failed to get transaction receipt: ${error}`);
      return null; // Transaction not found or not yet confirmed
    }
  }

  /**
   * Get current block number
   */
  async getBlockNumber(): Promise<number> {
    return this.rpcProvider.getBlockCount();
  }

  /**
   * Get block by number or hash
   */
  async getBlock(blockHashOrNumber: string | number): Promise<any> {
    debug(`Getting block ${blockHashOrNumber}`);

    try {
      const block = await this.rpcProvider.getBlock(blockHashOrNumber);
      return this.convertToEthereumBlock(block);
    } catch (error) {
      debug(`Failed to get block: ${error}`);
      throw error;
    }
  }

  /**
   * Get contract events
   */
  async getEvents(
    contractAddress: string,
    eventName: string,
    fromBlock?: number | string,
    toBlock?: number | string
  ): Promise<any[]> {
    debug(`Getting events ${eventName} from ${contractAddress}`);

    try {
      const events = await this.fetchEventsFromNeo(filter);
      return events.map(event => this.convertNeoEventToEthLog(event));
    } catch (error) {
      debug(`Failed to get events: ${error}`);
      throw error;
    }
  }

  /**
   * Get account balance
   */
  async getBalance(address: string, blockTag?: string | number): Promise<string> {
    debug(`Getting balance for ${address}`);

    try {
      const balances = await this.rpcProvider.getBalance(address);
      
      // Find GAS balance
      const gasBalance = balances.find(b => b.symbol === 'GAS');
      
      if (!gasBalance) {
        return "0";
      }

      // Convert to Wei-like format (GAS has 8 decimals, we need 18 for ETH compatibility)
      const gasAmount = BigInt(gasBalance.amount);
      const weiAmount = gasAmount * BigInt(10 ** 10); // Convert 8 decimals to 18 decimals
      
      return weiAmount.toString();
    } catch (error) {
      debug(`Failed to get balance: ${error}`);
      throw error;
    }
  }

  /**
   * Get transaction count (nonce)
   */
  async getTransactionCount(address: string, blockTag?: string | number): Promise<number> {
    try {
      // Neo doesn't use nonces like Ethereum, but we can return the number of transactions
      // sent by this address as an approximation
      const scriptHash = this.addressToScriptHash(address);
      const transactions = await this.rpcCall('getaddresshistory', [scriptHash]);
      return transactions?.length || 0;
    } catch (error) {
      debug(`Failed to get transaction count: ${error}`);
      return 0;
    }
  }

  /**
   * Get code at address
   */
  async getCode(address: string, blockTag?: string | number): Promise<string> {
    debug(`Getting code for ${address}`);

    try {
      const scriptHash = this.addressToScriptHash(address);
      const contractState = await this.rpcProvider.getContractState(scriptHash);
      
      // Return NEF script as bytecode
      return '0x' + contractState.nef.script;
    } catch (error) {
      debug(`Failed to get code: ${error}`);
      return "0x"; // No code found
    }
  }

  /**
   * Get storage at position
   */
  async getStorageAt(
    address: string,
    position: string,
    blockTag?: string | number
  ): Promise<string> {
    debug(`Getting storage at ${position} for ${address}`);

    try {
      const scriptHash = this.addressToScriptHash(address);
      const value = await this.rpcProvider.getStorage(scriptHash, position);
      
      return value || "0x0000000000000000000000000000000000000000000000000000000000000000";
    } catch (error) {
      debug(`Failed to get storage: ${error}`);
      return "0x0000000000000000000000000000000000000000000000000000000000000000";
    }
  }

  /**
   * Send raw transaction
   */
  async sendRawTransaction(signedTx: string): Promise<string> {
    debug("Sending raw transaction");

    try {
      const result = await this.rpcProvider.sendRawTransaction(signedTx);
      return result.hash;
    } catch (error) {
      debug(`Failed to send raw transaction: ${error}`);
      throw error;
    }
  }

  // Private methods

  private addressToScriptHash(address: string): string {
    // Convert Ethereum-style address to Neo script hash
    if (address.startsWith('0x')) {
      return address.toLowerCase();
    }
    
    // If it's already a Neo address, convert to script hash format
    try {
      const scriptHash = this.addressToScriptHash(address);
      return scriptHash;
    } catch {
      // If conversion fails, try to extract from the address
      const cleanAddress = address.replace(/^0x/, '');
      if (cleanAddress.length === 40) {
        return '0x' + cleanAddress.toLowerCase();
      }
      throw new Error(`Invalid address format: ${address}`);
    }
  }

  private convertArgsToNeoParams(args: any[]): any[] {
    return args.map(arg => {
      if (typeof arg === 'object' && arg !== null && arg.type) {
        // Already in Neo format
        return arg;
      }

      // Convert primitive types to Neo format
      if (typeof arg === 'boolean') {
        return { type: 'Boolean', value: arg };
      }
      
      if (typeof arg === 'number' || typeof arg === 'bigint') {
        return { type: 'Integer', value: String(arg) };
      }
      
      if (typeof arg === 'string') {
        if (arg.startsWith('0x')) {
          return { type: 'ByteArray', value: arg.slice(2) };
        }
        return { type: 'String', value: arg };
      }

      // Default to string representation
      return { type: 'String', value: String(arg) };
    });
  }

  private convertToEthereumReceipt(tx: any, appLog: any): any {
    return {
      transactionHash: tx.hash,
      transactionIndex: 0,
      blockNumber: tx.blockTime ? tx.blockTime : null,
      blockHash: tx.blockHash || null,
      cumulativeGasUsed: tx.sysFee,
      gasUsed: tx.sysFee,
      contractAddress: null, // Would be set for contract creation
      logs: this.convertNeoLogsToEthereumLogs(appLog.executions?.[0]?.notifications || []),
      status: appLog.executions?.[0]?.vmState === 'HALT' ? 1 : 0,
      from: tx.sender,
      to: null, // Would need to extract from transaction
      type: 0,
      effectiveGasPrice: "1000"
    };
  }

  private convertToEthereumBlock(neoBlock: any): any {
    return {
      number: neoBlock.index,
      hash: neoBlock.hash,
      parentHash: neoBlock.previousBlockHash,
      timestamp: neoBlock.time,
      size: neoBlock.size,
      gasLimit: "0xffffffff",
      gasUsed: "0x0", // Would need to calculate from transactions
      transactions: neoBlock.tx?.map((tx: any) => tx.hash) || [],
      miner: neoBlock.nextConsensus,
      difficulty: "0x0",
      totalDifficulty: "0x0"
    };
  }

  private convertNeoLogsToEthereumLogs(notifications: any[]): any[] {
    return notifications.map((notification, index) => ({
      address: notification.contract,
      topics: [
        // First topic is event signature hash
        this.getEventTopicHash(notification.eventName),
        // Additional topics from indexed parameters would go here
      ],
      data: this.encodeLogData(notification.state),
      logIndex: index,
      transactionIndex: 0,
      blockNumber: 0, // Would be filled from actual block
      blockHash: "0x0000000000000000000000000000000000000000000000000000000000000000"
    }));
  }

  private getEventTopicHash(eventName: string): string {
    // For Neo events, we use a different hashing approach than Ethereum's Keccak256
    // We'll use SHA256 which is available in Neo
    const crypto = require('crypto');
    const hash = crypto.createHash('sha256').update(eventName).digest('hex');
    return '0x' + hash;
  }

  private encodeLogData(state: any[]): string {
    // Encode Neo state array as Ethereum-compatible log data
    const encoded = state.map(item => {
      if (typeof item === 'string') {
        // Convert string to hex
        return Buffer.from(item, 'utf8').toString('hex');
      } else if (typeof item === 'number' || typeof item === 'bigint') {
        // Convert number to 32-byte hex
        return BigInt(item).toString(16).padStart(64, '0');
      } else if (typeof item === 'boolean') {
        // Convert boolean to 32-byte hex
        return item ? '0'.repeat(63) + '1' : '0'.repeat(64);
      } else {
        // Convert object to JSON then hex
        return Buffer.from(JSON.stringify(item), 'utf8').toString('hex');
      }
    }).join('');
    
    return '0x' + encoded;
  }

  /**
   * Fetch events from Neo blockchain
   */
  private async fetchEventsFromNeo(filter: {
    address?: string;
    topics?: string[];
    fromBlock?: string | number;
    toBlock?: string | number;
  }): Promise<any[]> {
    const events: any[] = [];
    
    try {
      const fromBlock = this.parseBlockTag(filter.fromBlock || 'earliest');
      const toBlock = this.parseBlockTag(filter.toBlock || 'latest');
      
      // Get application logs for the block range
      for (let blockHeight = fromBlock; blockHeight <= toBlock; blockHeight++) {
        const block = await this.rpcCall('getblock', [blockHeight, 1]);
        if (!block || !block.tx) continue;
        
        for (const tx of block.tx) {
          const appLog = await this.rpcCall('getapplicationlog', [tx.hash]);
          if (appLog && appLog.executions) {
            for (const execution of appLog.executions) {
              if (execution.notifications) {
                events.push(...this.filterNotifications(execution.notifications, filter));
              }
            }
          }
        }
      }
    } catch (error) {
      debug(`Failed to fetch events: ${error}`);
    }
    
    return events;
  }

  /**
   * Filter Neo notifications based on event filter criteria
   */
  private filterNotifications(notifications: any[], filter: any): any[] {
    return notifications.filter(notification => {
      // Filter by contract address
      if (filter.address) {
        const targetScriptHash = this.addressToScriptHash(filter.address);
        if (notification.contract !== targetScriptHash) {
          return false;
        }
      }
      
      // Filter by topics (event names)
      if (filter.topics && filter.topics.length > 0) {
        const eventName = notification.eventname;
        const eventHash = this.getEventTopicHash(eventName);
        
        // Check if any topic matches
        const topicMatches = filter.topics.some((topic: string) => 
          topic === eventHash || topic === eventName
        );
        
        if (!topicMatches) {
          return false;
        }
      }
      
      return true;
    });
  }

  /**
   * Convert Neo event notification to Ethereum-style log
   */
  private convertNeoEventToEthLog(notification: any): any {
    return {
      address: this.scriptHashToAddress(notification.contract),
      topics: [
        this.getEventTopicHash(notification.eventname),
        // Additional topics would be extracted from notification.state
      ],
      data: this.encodeLogData(notification.state || []),
      blockNumber: notification.blockIndex || 0,
      blockHash: notification.blockHash || '0x0000000000000000000000000000000000000000000000000000000000000000',
      transactionHash: notification.txHash || '0x0000000000000000000000000000000000000000000000000000000000000000',
      transactionIndex: 0,
      logIndex: 0,
      removed: false
    };
  }

  /**
   * Parse block tag to block number
   */
  private parseBlockTag(blockTag: string | number): number {
    if (typeof blockTag === 'number') {
      return blockTag;
    }
    
    switch (blockTag) {
      case 'earliest':
        return 0;
      case 'latest':
        // Would get latest block number from RPC
        return 999999999; // Large number as placeholder
      case 'pending':
        return 999999999;
      default:
        if (blockTag.startsWith('0x')) {
          return parseInt(blockTag, 16);
        }
        return parseInt(blockTag, 10);
    }
  }

  /**
   * Convert script hash to Neo address
   */
  private scriptHashToAddress(scriptHash: string): string {
    try {
      // Remove 0x prefix if present
      const cleanHash = scriptHash.replace(/^0x/, '');
      
      // Reverse the script hash (little endian to big endian)
      const reversedHash = Buffer.from(cleanHash, 'hex').reverse();
      
      // Add version byte (0x35 for N3)
      const versionByte = Buffer.from([0x35]);
      const addressPayload = Buffer.concat([versionByte, reversedHash]);
      
      // Calculate checksum
      const crypto = require('crypto');
      const checksum1 = crypto.createHash('sha256').update(addressPayload).digest();
      const checksum2 = crypto.createHash('sha256').update(checksum1).digest();
      const checksum = checksum2.slice(0, 4);
      
      // Combine and encode
      const fullAddress = Buffer.concat([addressPayload, checksum]);
      return this.base58Encode(fullAddress);
    } catch (error) {
      debug(`Failed to convert script hash to address: ${error}`);
      return scriptHash; // Return original if conversion fails
    }
  }

  /**
   * Convert Neo address to script hash
   */
  private addressToScriptHash(address: string): string {
    try {
      // Decode base58 address
      const decoded = this.base58Decode(address);
      
      // Remove version byte and checksum
      const scriptHash = decoded.slice(1, 21);
      
      // Reverse for little endian format
      const reversedHash = Buffer.from(scriptHash).reverse();
      
      return '0x' + reversedHash.toString('hex');
    } catch (error) {
      debug(`Failed to convert address to script hash: ${error}`);
      throw new Error(`Invalid Neo address: ${address}`);
    }
  }

  /**
   * Base58 encode
   */
  private base58Encode(buffer: Buffer): string {
    const alphabet = '123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz';
    let num = BigInt('0x' + buffer.toString('hex'));
    let result = '';
    
    while (num > 0) {
      const remainder = num % 58n;
      result = alphabet[Number(remainder)] + result;
      num = num / 58n;
    }
    
    // Add leading 1s for leading zeros
    for (let i = 0; i < buffer.length && buffer[i] === 0; i++) {
      result = '1' + result;
    }
    
    return result;
  }

  /**
   * Base58 decode
   */
  private base58Decode(address: string): Buffer {
    const alphabet = '123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz';
    let num = 0n;
    
    for (const char of address) {
      const index = alphabet.indexOf(char);
      if (index === -1) {
        throw new Error(`Invalid character in address: ${char}`);
      }
      num = num * 58n + BigInt(index);
    }
    
    const hex = num.toString(16);
    const buffer = Buffer.from(hex.length % 2 ? '0' + hex : hex, 'hex');
    
    // Add leading zeros for leading 1s
    const leadingOnes = address.match(/^1*/)?.[0]?.length || 0;
    const leadingZeros = Buffer.alloc(leadingOnes);
    
    return Buffer.concat([leadingZeros, buffer]);
  }

  /**
   * Make RPC call to Neo node
   */
  private async rpcCall(method: string, params: any[] = []): Promise<any> {
    try {
      return await this.rpcProvider.call(method, params);
    } catch (error) {
      debug(`RPC call failed: ${method} - ${error}`);
      throw error;
    }
  }
}