import axios, { AxiosInstance } from "axios";
import { 
  NeoRpcProvider,
  NeoBlock,
  NeoTransaction,
  ContractState,
  InvokeResult,
  SendResult,
  Balance,
  ApplicationLog,
  NeoNetworkConfig
} from "@neo-solidity/types";
import { HardhatPluginError } from "hardhat/plugins";
import Debug from "debug";

const debug = Debug("hardhat:neo-deployer:rpc");

/**
 * Neo RPC client implementation
 */
export class NeoRpcClient implements NeoRpcProvider {
  public readonly url: string;
  public readonly magic: number;
  private client: AxiosInstance;
  private requestId = 1;

  constructor(config: NeoNetworkConfig) {
    this.url = config.rpcUrls[0]; // Use first RPC URL
    this.magic = config.magic;
    
    this.client = axios.create({
      baseURL: this.url,
      timeout: 30000,
      headers: {
        'Content-Type': 'application/json'
      }
    });
  }

  /**
   * Make RPC call
   */
  async call<T = any>(method: string, params: any[] = []): Promise<T> {
    debug(`RPC call: ${method}`, params);
    
    try {
      const response = await this.client.post('/', {
        jsonrpc: '2.0',
        method,
        params,
        id: this.requestId++
      });

      if (response.data.error) {
        throw new Error(`RPC error: ${response.data.error.message}`);
      }

      return response.data.result;
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      throw new HardhatPluginError(
        "@neo-solidity/hardhat-neo-deployer",
        `RPC call failed: ${message}`
      );
    }
  }

  /**
   * Get block by hash or index
   */
  async getBlock(hashOrIndex: string | number): Promise<NeoBlock> {
    return this.call('getblock', [hashOrIndex, 1]); // verbose = 1
  }

  /**
   * Get transaction by hash
   */
  async getTransaction(hash: string): Promise<NeoTransaction> {
    return this.call('getrawtransaction', [hash, 1]); // verbose = 1
  }

  /**
   * Get contract state
   */
  async getContractState(scriptHash: string): Promise<ContractState> {
    return this.call('getcontractstate', [scriptHash]);
  }

  /**
   * Invoke contract function (read-only)
   */
  async invokeFunction(
    scriptHash: string,
    method: string,
    params: any[] = []
  ): Promise<InvokeResult> {
    return this.call('invokefunction', [scriptHash, method, params]);
  }

  /**
   * Send raw transaction
   */
  async sendRawTransaction(signedTransaction: string): Promise<SendResult> {
    return this.call('sendrawtransaction', [signedTransaction]);
  }

  /**
   * Get balance for address
   */
  async getBalance(address: string): Promise<Balance[]> {
    const nep17Balances = await this.call('getnep17balances', [address]);
    
    return nep17Balances.balance.map((balance: any) => ({
      assetHash: balance.assethash,
      symbol: balance.symbol || 'UNKNOWN',
      decimals: balance.decimals || 0,
      amount: balance.amount
    }));
  }

  /**
   * Get storage item
   */
  async getStorage(scriptHash: string, key: string): Promise<string | null> {
    try {
      return await this.call('getstorage', [scriptHash, key]);
    } catch (error) {
      // Storage item not found
      return null;
    }
  }

  /**
   * Get block count
   */
  async getBlockCount(): Promise<number> {
    return this.call('getblockcount');
  }

  /**
   * Get best block hash
   */
  async getBestBlockHash(): Promise<string> {
    return this.call('getbestblockhash');
  }

  /**
   * Get transaction height
   */
  async getTransactionHeight(hash: string): Promise<number> {
    return this.call('gettransactionheight', [hash]);
  }

  /**
   * Get application log
   */
  async getApplicationLog(hash: string): Promise<ApplicationLog> {
    return this.call('getapplicationlog', [hash]);
  }

  /**
   * Calculate network fee
   */
  async calculateNetworkFee(tx: string): Promise<string> {
    return this.call('calculatenetworkfee', [tx]);
  }

  /**
   * Get peers
   */
  async getPeers(): Promise<any> {
    return this.call('getpeers');
  }

  /**
   * Get version
   */
  async getVersion(): Promise<any> {
    return this.call('getversion');
  }

  /**
   * Submit block
   */
  async submitBlock(block: string): Promise<boolean> {
    return this.call('submitblock', [block]);
  }

  /**
   * Get mempool
   */
  async getRawMemPool(): Promise<string[]> {
    return this.call('getrawmempool');
  }

  /**
   * Validate address
   */
  async validateAddress(address: string): Promise<any> {
    return this.call('validateaddress', [address]);
  }

  /**
   * List plugins
   */
  async listPlugins(): Promise<any[]> {
    return this.call('listplugins');
  }

  /**
   * Get committee
   */
  async getCommittee(): Promise<string[]> {
    return this.call('getcommittee');
  }

  /**
   * Get next block validators
   */
  async getNextBlockValidators(): Promise<string[]> {
    return this.call('getnextblockvalidators');
  }

  /**
   * Get candidates
   */
  async getCandidates(): Promise<any[]> {
    return this.call('getcandidates');
  }

  /**
   * Invoke script
   */
  async invokeScript(script: string, signers?: any[]): Promise<InvokeResult> {
    return this.call('invokescript', [script, signers]);
  }

  /**
   * Get state root
   */
  async getStateRoot(index: number): Promise<any> {
    return this.call('getstateroot', [index]);
  }

  /**
   * Get proof
   */
  async getProof(rootHash: string, contractHash: string, key: string): Promise<string> {
    return this.call('getproof', [rootHash, contractHash, key]);
  }

  /**
   * Verify proof
   */
  async verifyProof(rootHash: string, proof: string): Promise<string> {
    return this.call('verifyproof', [rootHash, proof]);
  }

  /**
   * Find states
   */
  async findStates(rootHash: string, contractHash: string, prefix: string): Promise<any> {
    return this.call('findstates', [rootHash, contractHash, prefix]);
  }

  /**
   * Get NEP-11 balances
   */
  async getNep11Balances(address: string): Promise<any> {
    return this.call('getnep11balances', [address]);
  }

  /**
   * Get NEP-11 transfers
   */
  async getNep11Transfers(address: string, startTime?: number, endTime?: number): Promise<any> {
    const params = [address];
    if (startTime !== undefined) params.push(startTime);
    if (endTime !== undefined) params.push(endTime);
    return this.call('getnep11transfers', params);
  }

  /**
   * Get NEP-17 transfers
   */
  async getNep17Transfers(address: string, startTime?: number, endTime?: number): Promise<any> {
    const params = [address];
    if (startTime !== undefined) params.push(startTime);
    if (endTime !== undefined) params.push(endTime);
    return this.call('getnep17transfers', params);
  }

  /**
   * Test connection
   */
  async testConnection(): Promise<boolean> {
    try {
      await this.getVersion();
      return true;
    } catch {
      return false;
    }
  }

  /**
   * Get network information
   */
  async getNetworkInfo(): Promise<{
    magic: number;
    version: any;
    blockCount: number;
    peers: number;
  }> {
    const [version, blockCount, peers] = await Promise.all([
      this.getVersion(),
      this.getBlockCount(),
      this.getPeers().then(p => p.connected?.length || 0).catch(() => 0)
    ]);

    return {
      magic: this.magic,
      version,
      blockCount,
      peers
    };
  }
}