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
      // This would implement event filtering
      // For now, return empty array as placeholder
      return [];
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
    // Neo doesn't have nonces like Ethereum
    // This is a placeholder implementation
    return 0;
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
    
    // If it's already a Neo address, we need to convert it
    // This is a simplified implementation
    return '0x' + address.slice(-40).toLowerCase();
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
    // This would calculate the Keccak256 hash of the event signature
    // For now, return a mock hash
    return "0x" + eventName.split('').map(c => c.charCodeAt(0).toString(16)).join('').padStart(64, '0');
  }

  private encodeLogData(state: any[]): string {
    // Encode Neo state array as Ethereum log data
    // This is a simplified implementation
    return "0x" + state.map(item => 
      typeof item === 'string' ? item : JSON.stringify(item)
    ).join('');
  }
}