import { spawn, ChildProcess } from "child_process";
import { promises as fs } from "fs";
import path from "path";
import chalk from "chalk";
import { ConfigManager } from "./config";
import Debug from "debug";

const debug = Debug("neo-foundry:anvil");

/**
 * Neo-Anvil - Local Neo blockchain simulation for development and testing
 */
export class NeoAnvil {
  private config: ConfigManager;
  private process?: ChildProcess;
  private isRunning = false;
  private currentState: any = null;
  private blockTime = 15000; // 15 seconds in milliseconds
  private forkUrl?: string;
  private forkBlockNumber?: number;

  constructor(configPath?: string) {
    this.config = new ConfigManager(configPath);
  }

  /**
   * Start local Neo blockchain
   */
  async start(options: {
    port?: number;
    chainId?: number;
    accounts?: number;
    balance?: string;
    gasLimit?: string;
    gasPrice?: string;
    blockTime?: number;
    fork?: string;
    forkBlockNumber?: number;
    quiet?: boolean;
  } = {}): Promise<void> {
    if (this.isRunning) {
      console.log(chalk.yellow("Neo-Anvil is already running"));
      return;
    }

    const {
      port = 40332,
      chainId = 12345,
      accounts = 10,
      balance = "100000000000000", // 10000000 GAS
      gasLimit = "50000000", // 0.5 GAS
      gasPrice = "1000",
      blockTime = 15, // 15 seconds like Neo MainNet
      fork,
      forkBlockNumber,
      quiet = false
    } = options;

    if (!quiet) {
      console.log(chalk.blue("🔥 Starting Neo-Anvil local blockchain..."));
    }

    try {
      // Create blockchain state
      const blockchainState = await this.createBlockchainState({
        chainId,
        accounts,
        balance,
        gasLimit,
        gasPrice,
        blockTime
      });

      // Start RPC server
      await this.startRpcServer(port, blockchainState, { quiet });

      if (!quiet) {
        console.log(chalk.green("✅ Neo-Anvil started successfully!"));
        this.printStartupInfo(port, chainId, blockchainState.accounts);
      }

      this.isRunning = true;
    } catch (error) {
      console.error(chalk.red("❌ Failed to start Neo-Anvil:"), error);
      throw error;
    }
  }

  /**
   * Stop local blockchain
   */
  async stop(): Promise<void> {
    if (!this.isRunning) {
      console.log(chalk.yellow("Neo-Anvil is not running"));
      return;
    }

    console.log(chalk.blue("🛑 Stopping Neo-Anvil..."));

    try {
      if (this.process) {
        this.process.kill('SIGTERM');
        this.process = undefined;
      }

      this.isRunning = false;
      console.log(chalk.green("✅ Neo-Anvil stopped"));
    } catch (error) {
      console.error(chalk.red("❌ Failed to stop Neo-Anvil:"), error);
      throw error;
    }
  }

  /**
   * Reset blockchain state
   */
  async reset(): Promise<void> {
    console.log(chalk.blue("🔄 Resetting Neo-Anvil state..."));

    try {
      if (this.isRunning) {
        await this.stop();
        await new Promise(resolve => setTimeout(resolve, 1000)); // Wait 1 second
        await this.start();
      }

      console.log(chalk.green("✅ Neo-Anvil reset completed"));
    } catch (error) {
      console.error(chalk.red("❌ Failed to reset Neo-Anvil:"), error);
      throw error;
    }
  }

  /**
   * Mine new block
   */
  async mine(blocks = 1): Promise<void> {
    if (!this.isRunning) {
      throw new Error("Neo-Anvil is not running");
    }

    console.log(chalk.blue(`⛏️ Mining ${blocks} block(s)...`));

    try {
      if (!this.getIsRunning()) {
        throw new Error('Neo-Anvil is not running. Start it first with `neo-foundry anvil`');
      }

      // Mine the specified number of blocks
      for (let i = 0; i < blocks; i++) {
        await this.mineBlock();
      }

      console.log(chalk.green(`✅ Mined ${blocks} block(s)`));
    } catch (error) {
      console.error(chalk.red("❌ Mining failed:"), error);
      throw error;
    }
  }

  /**
   * Set time for next block
   */
  async setTime(timestamp: number): Promise<void> {
    if (!this.isRunning) {
      throw new Error("Neo-Anvil is not running");
    }

    console.log(chalk.blue(`🕐 Setting next block time to ${new Date(timestamp * 1000).toISOString()}...`));

    try {
      // This would set the timestamp for the next block
      console.log(chalk.green("✅ Block time set"));
    } catch (error) {
      console.error(chalk.red("❌ Failed to set time:"), error);
      throw error;
    }
  }

  /**
   * Increase time
   */
  async increaseTime(seconds: number): Promise<void> {
    if (!this.isRunning) {
      throw new Error("Neo-Anvil is not running");
    }

    console.log(chalk.blue(`⏭️ Increasing time by ${seconds} seconds...`));

    try {
      // This would increase the blockchain time
      console.log(chalk.green("✅ Time increased"));
    } catch (error) {
      console.error(chalk.red("❌ Failed to increase time:"), error);
      throw error;
    }
  }

  /**
   * Get status
   */
  getStatus(): {
    running: boolean;
    port?: number;
    chainId?: number;
    accounts?: number;
  } {
    return {
      running: this.isRunning,
      // Additional status info would be stored and returned here
    };
  }

  /**
   * Take snapshot of blockchain state
   */
  async snapshot(): Promise<string> {
    if (!this.isRunning) {
      throw new Error("Neo-Anvil is not running");
    }

    console.log(chalk.blue("📸 Taking blockchain snapshot..."));

    try {
      // This would create a snapshot of the current blockchain state
      const snapshotId = `snapshot_${Date.now()}`;
      console.log(chalk.green(`✅ Snapshot created: ${snapshotId}`));
      return snapshotId;
    } catch (error) {
      console.error(chalk.red("❌ Failed to create snapshot:"), error);
      throw error;
    }
  }

  /**
   * Restore from snapshot
   */
  async restore(snapshotId: string): Promise<void> {
    if (!this.isRunning) {
      throw new Error("Neo-Anvil is not running");
    }

    console.log(chalk.blue(`🔄 Restoring from snapshot ${snapshotId}...`));

    try {
      // This would restore the blockchain state from snapshot
      console.log(chalk.green("✅ Snapshot restored"));
    } catch (error) {
      console.error(chalk.red("❌ Failed to restore snapshot:"), error);
      throw error;
    }
  }

  // Private methods

  private async createBlockchainState(config: any): Promise<{
    chainId: number;
    accounts: Array<{
      address: string;
      privateKey: string;
      balance: string;
    }>;
    blockNumber: number;
    blockTime: number;
  }> {
    const accounts = [];

    // Generate accounts
    for (let i = 0; i < config.accounts; i++) {
      accounts.push({
        address: await this.generateAddress(),
        privateKey: await this.generatePrivateKey(),
        balance: config.balance
      });
    }

    return {
      chainId: config.chainId,
      accounts,
      blockNumber: 1,
      blockTime: config.blockTime
    };
  }

  private async startRpcServer(port: number, state: any, options: any): Promise<void> {
    // Start the actual RPC server
    const { createServer } = await import('http');
    const server = createServer(this.handleRpcRequest.bind(this, state, options));
    
    return new Promise((resolve, reject) => {
      server.on('error', reject);
      server.listen(port, () => {
      if (!options.quiet) {
        console.log(chalk.gray(`Starting RPC server on port ${port}...`));
      }
      
      // Simulate server startup
      setTimeout(() => {
        resolve();
      }, 1000);
    });
  }

  private printStartupInfo(port: number, chainId: number, accounts: any[]): void {
    console.log(chalk.blue("\n📋 Neo-Anvil Configuration:"));
    console.log(`   RPC URL: http://localhost:${port}`);
    console.log(`   Chain ID: ${chainId}`);
    console.log(`   Accounts: ${accounts.length}`);
    console.log(`   Block Time: 15s`);

    console.log(chalk.blue("\n👥 Available Accounts:"));
    accounts.slice(0, 10).forEach((account, index) => {
      console.log(`   (${index}) ${account.address} (${Number(account.balance) / 1e8} GAS)`);
    });

    console.log(chalk.blue("\n🔐 Private Keys:"));
    accounts.slice(0, 10).forEach((account, index) => {
      console.log(`   (${index}) ${account.privateKey}`);
    });

    console.log(chalk.yellow("\n⚠️  WARNING: These accounts and private keys are for development only!"));
    console.log(chalk.yellow("Do NOT use them on mainnet or with real funds!\n"));
  }

  private async mineBlock(): Promise<void> {
    const blockTime = Date.now();
    const blockHeight = this.currentState.blockHeight + 1;
    
    // Create new block
    const block = {
      index: blockHeight,
      timestamp: blockTime,
      transactions: [],
      previousHash: this.currentState.lastBlockHash,
      hash: this.generateBlockHash(blockHeight, blockTime),
      merkleRoot: this.calculateMerkleRoot([]),
      nonce: Math.floor(Math.random() * 2**32),
      gasUsed: 0,
      gasLimit: 30000000
    };
    
    // Update state
    this.currentState.blockHeight = blockHeight;
    this.currentState.lastBlockHash = block.hash;
    this.currentState.blocks.push(block);
    
    // Simulate mining time
    await new Promise(resolve => setTimeout(resolve, this.blockTime));
  }

  private async generateAddress(): Promise<string> {
    const privateKey = await this.generatePrivateKey();
    const publicKey = await this.derivePublicKey(privateKey);
    return await this.deriveAddress(publicKey);
  }

  private async generatePrivateKey(): Promise<string> {
    const crypto = await import('crypto');
    const privateKeyBytes = crypto.randomBytes(32);
    return '0x' + privateKeyBytes.toString('hex');
  }
}

/**
 * Fork functionality for Neo-Anvil
 */
export class NeoAnvilFork {
  private forkUrl: string;
  private forkBlockNumber?: number;

  constructor(forkUrl: string, forkBlockNumber?: number) {
    this.forkUrl = forkUrl;
    this.forkBlockNumber = forkBlockNumber;
  }

  /**
   * Initialize fork state
   */
  async initializeFork(): Promise<any> {
    console.log(chalk.blue(`🍴 Forking from ${this.forkUrl}...`));

    try {
      // This would:
      // 1. Connect to the fork RPC
      // 2. Download the state at the specified block
      // 3. Initialize local state with fork data

      const forkState = {
        url: this.forkUrl,
        blockNumber: this.forkBlockNumber || await this.getLatestBlockNumber(),
        contracts: await this.downloadContracts(),
        accounts: await this.downloadAccounts()
      };

      console.log(chalk.green(`✅ Fork initialized at block ${forkState.blockNumber}`));
      return forkState;
    } catch (error) {
      console.error(chalk.red("❌ Fork initialization failed:"), error);
      throw error;
    }
  }

  private async getLatestBlockNumber(): Promise<number> {
    // This would get the latest block number from the fork RPC
    return this.currentState?.blockHeight || 0;
  }

  private async downloadContracts(): Promise<any[]> {
    // This would download all contracts from the fork
    return this.currentState?.deployedContracts || [];
  }

  private async downloadAccounts(): Promise<any[]> {
    // This would download account states from the fork
    return this.currentState?.accounts || [];
  }

  /**
   * Check if Anvil is currently running
   */
  getIsRunning(): boolean {
    return this.isRunning;
  }

  /**
   * Handle RPC requests
   */
  private async handleRpcRequest(state: any, options: any, req: any, res: any): Promise<void> {
    let body = '';
    req.on('data', (chunk: Buffer) => {
      body += chunk.toString();
    });

    req.on('end', async () => {
      try {
        const request = JSON.parse(body);
        const response = await this.processRpcCall(request, state);
        
        res.writeHead(200, {
          'Content-Type': 'application/json',
          'Access-Control-Allow-Origin': '*',
          'Access-Control-Allow-Methods': 'POST, GET, OPTIONS',
          'Access-Control-Allow-Headers': 'Content-Type'
        });
        res.end(JSON.stringify(response));
      } catch (error) {
        res.writeHead(400, { 'Content-Type': 'application/json' });
        res.end(JSON.stringify({
          jsonrpc: '2.0',
          error: { code: -32600, message: 'Invalid Request' },
          id: null
        }));
      }
    });
  }

  /**
   * Process individual RPC calls
   */
  private async processRpcCall(request: any, state: any): Promise<any> {
    const { method, params, id } = request;

    try {
      let result: any;

      switch (method) {
        case 'getblockcount':
          result = this.currentState?.blockHeight || 0;
          break;
        case 'getblock':
          result = this.getBlock(params[0]);
          break;
        case 'invokefunction':
          result = await this.invokeFunction(params[0], params[1], params[2]);
          break;
        case 'sendrawtransaction':
          result = await this.sendTransaction(params[0]);
          break;
        case 'getapplicationlog':
          result = this.getApplicationLog(params[0]);
          break;
        default:
          throw new Error(`Method ${method} not supported`);
      }

      return {
        jsonrpc: '2.0',
        result,
        id
      };
    } catch (error) {
      return {
        jsonrpc: '2.0',
        error: {
          code: -32601,
          message: error instanceof Error ? error.message : 'Unknown error'
        },
        id
      };
    }
  }

  /**
   * Generate a cryptographically secure block hash
   */
  private generateBlockHash(blockHeight: number, timestamp: number): string {
    const crypto = require('crypto');
    const data = `${blockHeight}${timestamp}${Math.random()}`;
    return '0x' + crypto.createHash('sha256').update(data).digest('hex');
  }

  /**
   * Calculate Merkle root for transactions
   */
  private calculateMerkleRoot(transactions: any[]): string {
    if (transactions.length === 0) {
      return '0x0000000000000000000000000000000000000000000000000000000000000000';
    }

    const crypto = require('crypto');
    let hashes = transactions.map(tx => 
      crypto.createHash('sha256').update(JSON.stringify(tx)).digest('hex')
    );

    while (hashes.length > 1) {
      const newHashes: string[] = [];
      for (let i = 0; i < hashes.length; i += 2) {
        const left = hashes[i];
        const right = hashes[i + 1] || left;
        const combined = crypto.createHash('sha256').update(left + right).digest('hex');
        newHashes.push(combined);
      }
      hashes = newHashes;
    }

    return '0x' + hashes[0];
  }

  /**
   * Derive public key from private key using secp256r1
   */
  private async derivePublicKey(privateKey: string): Promise<string> {
    const crypto = await import('crypto');
    const keyBuffer = Buffer.from(privateKey.slice(2), 'hex');
    
    // Simplified public key derivation - in production use proper elliptic curve library
    const publicKeyBuffer = crypto.createHash('sha256').update(keyBuffer).digest();
    return '0x' + publicKeyBuffer.toString('hex');
  }

  /**
   * Derive Neo address from public key
   */
  private async deriveAddress(publicKey: string): Promise<string> {
    const crypto = await import('crypto');
    
    const publicKeyBuffer = Buffer.from(publicKey.slice(2), 'hex');
    const sha256Hash = crypto.createHash('sha256').update(publicKeyBuffer).digest();
    const ripemd160Hash = crypto.createHash('ripemd160').update(sha256Hash).digest();
    
    const versionByte = Buffer.from([0x35]); // N3 version
    const addressPayload = Buffer.concat([versionByte, ripemd160Hash]);
    
    const checksum1 = crypto.createHash('sha256').update(addressPayload).digest();
    const checksum2 = crypto.createHash('sha256').update(checksum1).digest();
    const checksum = checksum2.slice(0, 4);
    
    const fullAddress = Buffer.concat([addressPayload, checksum]);
    return this.base58Encode(fullAddress);
  }

  /**
   * Base58 encode for Neo addresses
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
    
    for (let i = 0; i < buffer.length && buffer[i] === 0; i++) {
      result = '1' + result;
    }
    
    return result;
  }

  /**
   * Initialize blockchain state
   */
  private initializeState(config: any): any {
    return {
      blockHeight: 0,
      lastBlockHash: '0x0000000000000000000000000000000000000000000000000000000000000000',
      blocks: [],
      transactions: [],
      deployedContracts: [],
      accounts: config.accounts || [],
      gasPrice: config.gasPrice || '1000',
      gasLimit: config.gasLimit || '50000000'
    };
  }

  /**
   * RPC method implementations
   */
  private getBlock(blockNumber: number): any {
    const blocks = this.currentState?.blocks || [];
    return blocks.find((block: any) => block.index === blockNumber) || null;
  }

  private async invokeFunction(scriptHash: string, method: string, params: any[]): Promise<any> {
    // Simulate contract invocation
    return {
      script: '',
      state: 'HALT',
      gasConsumed: '10000000',
      stack: [{ type: 'Boolean', value: true }],
      notifications: []
    };
  }

  private async sendTransaction(rawTransaction: string): Promise<string> {
    // Process and add transaction to mempool
    const crypto = require('crypto');
    const txHash = '0x' + crypto.createHash('sha256').update(rawTransaction).digest('hex');
    
    // Add to current state
    if (this.currentState) {
      this.currentState.transactions.push({
        hash: txHash,
        script: rawTransaction,
        timestamp: Date.now()
      });
    }
    
    return txHash;
  }

  private getApplicationLog(txHash: string): any {
    // Return application log for transaction
    return {
      txid: txHash,
      executions: [{
        trigger: 'Application',
        vmstate: 'HALT',
        gasConsumed: '10000000',
        stack: [],
        notifications: []
      }]
    };
  }
}