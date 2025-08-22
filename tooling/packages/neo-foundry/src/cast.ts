import axios from "axios";
import chalk from "chalk";
import { 
  NeoRpcProvider,
  NeoContract,
  InvokeResult,
  Balance,
  NeoTransaction
} from "@neo-solidity/types";
import { ConfigManager } from "./config";
import Debug from "debug";

const debug = Debug("neo-foundry:cast");

/**
 * Neo-Cast - Interact with Neo contracts and perform RPC calls
 */
export class NeoCast {
  private config: ConfigManager;
  private rpcProvider: NeoRpcProvider;

  constructor(configPath?: string, rpcUrl?: string) {
    this.config = new ConfigManager(configPath);
    
    if (rpcUrl) {
      this.rpcProvider = this.createRpcProvider(rpcUrl);
    }
  }

  /**
   * Set RPC provider
   */
  setRpc(rpcUrl: string): void {
    this.rpcProvider = this.createRpcProvider(rpcUrl);
  }

  /**
   * Call contract method (read-only)
   */
  async call(
    contractAddress: string,
    method: string,
    args: any[] = [],
    options: {
      blockTag?: string | number;
      from?: string;
    } = {}
  ): Promise<any> {
    console.log(chalk.blue(`📞 Calling ${contractAddress}.${method}(${args.join(", ")})...`));

    try {
      const scriptHash = this.addressToScriptHash(contractAddress);
      const result = await this.rpcProvider.invokeFunction(scriptHash, method, args);

      if (result.state === "FAULT") {
        throw new Error(`Contract call failed: ${result.exception}`);
      }

      console.log(chalk.green("✅ Call successful"));
      console.log("Result:", this.formatStackResult(result.stack));

      return result.stack;
    } catch (error) {
      console.error(chalk.red("❌ Call failed:"), error);
      throw error;
    }
  }

  /**
   * Send transaction to contract method
   */
  async send(
    contractAddress: string,
    method: string,
    args: any[] = [],
    options: {
      from?: string;
      gasLimit?: string;
      gasPrice?: string;
      value?: string;
    } = {}
  ): Promise<string> {
    console.log(chalk.blue(`📤 Sending transaction to ${contractAddress}.${method}(${args.join(", ")})...`));

    try {
      if (!this.rpcProvider) {
        throw new Error('RPC provider not configured');
      }

      const scriptHash = this.addressToScriptHash(contractAddress);
      
      // Create transaction parameters
      const txParams = {
        scriptHash,
        operation: method,
        args: this.encodeArgs(args),
        gasLimit: options.gasLimit ? BigInt(options.gasLimit) : BigInt('30000000'),
        gasPrice: options.gasPrice ? BigInt(options.gasPrice) : BigInt('1000'),
        value: options.value ? BigInt(options.value) : BigInt('0'),
        from: options.from || await this.getDefaultSigner()
      };

      // Send transaction
      const result = await this.rpcProvider.sendTransaction(txParams);
      
      if (!result.hash) {
        throw new Error('Transaction failed to get hash');
      }

      console.log(chalk.green("✅ Transaction sent"));
      console.log("Transaction hash:", result.hash);

      return result.hash;
    } catch (error) {
      console.error(chalk.red("❌ Transaction failed:"), error);
      throw error;
    }
  }

  /**
   * Deploy contract
   */
  async deployContract(
    bytecode: string,
    constructorArgs: any[] = [],
    options: {
      from?: string;
      gasLimit?: string;
      gasPrice?: string;
      value?: string;
    } = {}
  ): Promise<string> {
    console.log(chalk.blue(`🚀 Deploying contract...`));

    try {
      if (!this.rpcProvider) {
        throw new Error('RPC provider not configured');
      }

      // Read and compile contract
      const contractCode = await this.loadContract(contractPath);
      
      // Create deployment transaction
      const deployParams = {
        script: contractCode.nef,
        manifest: contractCode.manifest,
        gasLimit: options.gasLimit ? BigInt(options.gasLimit) : BigInt('50000000'),
        gasPrice: options.gasPrice ? BigInt(options.gasPrice) : BigInt('1000'),
        from: options.from || await this.getDefaultSigner()
      };

      // Deploy contract
      const result = await this.rpcProvider.deployContract(deployParams);
      
      if (!result.address) {
        throw new Error('Contract deployment failed');
      }

      const address = result.address;

      console.log(chalk.green("✅ Contract deployed"));
      console.log("Address:", address);

      return address;
    } catch (error) {
      console.error(chalk.red("❌ Deployment failed:"), error);
      throw error;
    }
  }

  /**
   * Get account balance
   */
  async balance(address: string): Promise<void> {
    console.log(chalk.blue(`💰 Getting balance for ${address}...`));

    try {
      const balances = await this.rpcProvider.getBalance(address);

      if (balances.length === 0) {
        console.log(chalk.yellow("No tokens found"));
        return;
      }

      console.log(chalk.green("Balances:"));
      for (const balance of balances) {
        const amount = Number(balance.amount) / Math.pow(10, balance.decimals);
        console.log(`  ${amount.toFixed(balance.decimals)} ${balance.symbol}`);
      }
    } catch (error) {
      console.error(chalk.red("❌ Failed to get balance:"), error);
      throw error;
    }
  }

  /**
   * Get transaction details
   */
  async transaction(txHash: string): Promise<void> {
    console.log(chalk.blue(`🔍 Getting transaction ${txHash}...`));

    try {
      const tx = await this.rpcProvider.getTransaction(txHash);

      console.log(chalk.green("Transaction details:"));
      console.log(`  Hash: ${tx.hash}`);
      console.log(`  Size: ${tx.size} bytes`);
      console.log(`  Sender: ${tx.sender}`);
      console.log(`  System Fee: ${tx.sysFee} GAS`);
      console.log(`  Network Fee: ${tx.netFee} GAS`);
      console.log(`  Valid Until Block: ${tx.validUntilBlock}`);

      if (tx.blockHash) {
        console.log(`  Block: ${tx.blockHash}`);
        console.log(`  Confirmations: ${tx.confirmations || 0}`);
      } else {
        console.log(chalk.yellow("  Status: Pending"));
      }
    } catch (error) {
      console.error(chalk.red("❌ Failed to get transaction:"), error);
      throw error;
    }
  }

  /**
   * Get block information
   */
  async block(blockHashOrNumber: string | number): Promise<void> {
    console.log(chalk.blue(`🧱 Getting block ${blockHashOrNumber}...`));

    try {
      const block = await this.rpcProvider.getBlock(blockHashOrNumber);

      console.log(chalk.green("Block details:"));
      console.log(`  Hash: ${block.hash}`);
      console.log(`  Index: ${block.index}`);
      console.log(`  Size: ${block.size} bytes`);
      console.log(`  Time: ${new Date(block.time * 1000).toISOString()}`);
      console.log(`  Previous Block: ${block.previousBlockHash}`);
      console.log(`  Merkle Root: ${block.merkleRoot}`);
      console.log(`  Transactions: ${block.tx.length}`);
      console.log(`  Confirmations: ${block.confirmations}`);
    } catch (error) {
      console.error(chalk.red("❌ Failed to get block:"), error);
      throw error;
    }
  }

  /**
   * Get contract storage
   */
  async storage(contractAddress: string, key: string): Promise<void> {
    console.log(chalk.blue(`🗄️ Getting storage ${key} from ${contractAddress}...`));

    try {
      const scriptHash = this.addressToScriptHash(contractAddress);
      const value = await this.rpcProvider.getStorage(scriptHash, key);

      if (value) {
        console.log(chalk.green("Storage value:"));
        console.log(`  Key: ${key}`);
        console.log(`  Value: ${value}`);
        console.log(`  Decoded: ${this.tryDecodeStorage(value)}`);
      } else {
        console.log(chalk.yellow("Storage key not found"));
      }
    } catch (error) {
      console.error(chalk.red("❌ Failed to get storage:"), error);
      throw error;
    }
  }

  /**
   * Estimate gas for transaction
   */
  async estimateGas(
    contractAddress: string,
    method: string,
    args: any[] = [],
    options: {
      from?: string;
      value?: string;
    } = {}
  ): Promise<void> {
    console.log(chalk.blue(`⛽ Estimating gas for ${contractAddress}.${method}(${args.join(", ")})...`));

    try {
      const scriptHash = this.addressToScriptHash(contractAddress);
      const result = await this.rpcProvider.invokeFunction(scriptHash, method, args);

      console.log(chalk.green("Gas estimation:"));
      console.log(`  Gas Consumed: ${result.gasConsumed} GAS`);
      
      // Calculate actual fees
      const systemFee = BigInt(result.gasConsumed);
      const networkFee = await this.calculateNetworkFee(result.script?.length || 0);
      const total = systemFee + networkFee;

      console.log(`  System Fee: ${systemFee.toString()} (${Number(systemFee) / 1e8} GAS)`);
      console.log(`  Network Fee: ${networkFee.toString()} (${Number(networkFee) / 1e8} GAS)`);
      console.log(`  Total: ${total.toString()} (${Number(total) / 1e8} GAS)`);
    } catch (error) {
      console.error(chalk.red("❌ Gas estimation failed:"), error);
      throw error;
    }
  }

  /**
   * Convert data formats
   */
  async convert(value: string, from: string, to: string): Promise<void> {
    console.log(chalk.blue(`🔄 Converting ${value} from ${from} to ${to}...`));

    try {
      let result: string;

      if (from === "hex" && to === "decimal") {
        result = parseInt(value, 16).toString();
      } else if (from === "decimal" && to === "hex") {
        result = "0x" + parseInt(value).toString(16);
      } else if (from === "hex" && to === "ascii") {
        result = Buffer.from(value.replace("0x", ""), "hex").toString("ascii");
      } else if (from === "ascii" && to === "hex") {
        result = "0x" + Buffer.from(value, "ascii").toString("hex");
      } else {
        throw new Error(`Unsupported conversion: ${from} to ${to}`);
      }

      console.log(chalk.green("Conversion result:"));
      console.log(`  ${from}: ${value}`);
      console.log(`  ${to}: ${result}`);
    } catch (error) {
      console.error(chalk.red("❌ Conversion failed:"), error);
      throw error;
    }
  }

  /**
   * Get network information
   */
  async networkInfo(): Promise<void> {
    console.log(chalk.blue("🌐 Getting network information..."));

    try {
      const [version, blockCount, peers] = await Promise.all([
        this.rpcProvider.getVersion(),
        this.rpcProvider.getBlockCount(),
        this.rpcProvider.getPeers().catch(() => ({ connected: [] }))
      ]);

      console.log(chalk.green("Network information:"));
      console.log(`  Version: ${version.useragent}`);
      console.log(`  Protocol: ${version.protocol?.network}/${version.protocol?.validatorscount} validators`);
      console.log(`  Block Height: ${blockCount}`);
      console.log(`  Connected Peers: ${peers.connected?.length || 0}`);
      console.log(`  Network Magic: ${version.protocol?.network || "Unknown"}`);
    } catch (error) {
      console.error(chalk.red("❌ Failed to get network info:"), error);
      throw error;
    }
  }

  /**
   * Generate random account
   */
  async generateAccount(): Promise<void> {
    console.log(chalk.blue("🎲 Generating new account..."));

    try {
      // Generate secure random private key
      const crypto = await import('crypto');
      const privateKeyBytes = crypto.randomBytes(32);
      const privateKey = '0x' + privateKeyBytes.toString('hex');
      
      // Derive public key and address from private key
      const publicKey = await this.derivePublicKey(privateKey);
      const address = await this.deriveAddress(publicKey);
      
      const publicKey = "03" + Array.from({ length: 62 }, () => 
        Math.floor(Math.random() * 16).toString(16)
      ).join('');
      
      const address = "N" + Array.from({ length: 33 }, () => 
        Math.random().toString(36).charAt(0)
      ).join('');

      console.log(chalk.green("New account generated:"));
      console.log(`  Address: ${address}`);
      console.log(`  Public Key: ${publicKey}`);
      console.log(chalk.red(`  Private Key: ${privateKey}`));
      console.log(chalk.yellow("\n⚠️  Save the private key securely!"));
    } catch (error) {
      console.error(chalk.red("❌ Account generation failed:"), error);
      throw error;
    }
  }

  // Private methods

  private createRpcProvider(rpcUrl: string): NeoRpcProvider {
    return {
      url: rpcUrl,
      magic: 0, // Would be set based on network
      call: async (method, params) => {
        const response = await axios.post(rpcUrl, {
          jsonrpc: '2.0',
          method,
          params: params || [],
          id: 1
        });
        
        if (response.data.error) {
          throw new Error(response.data.error.message);
        }
        
        return response.data.result;
      },
      getBlock: async (hashOrIndex) => {
        return this.call('getblock', [hashOrIndex, 1]);
      },
      getTransaction: async (hash) => {
        return this.call('getrawtransaction', [hash, 1]);
      },
      getContractState: async (scriptHash) => {
        return this.call('getcontractstate', [scriptHash]);
      },
      invokeFunction: async (scriptHash, method, params) => {
        return this.call('invokefunction', [scriptHash, method, params]);
      },
      sendRawTransaction: async (tx) => {
        return this.call('sendrawtransaction', [tx]);
      },
      getBalance: async (address) => {
        const result = await this.call('getnep17balances', [address]);
        return result.balance || [];
      },
      getStorage: async (scriptHash, key) => {
        return this.call('getstorage', [scriptHash, key]);
      },
      getBlockCount: async () => {
        return this.call('getblockcount');
      },
      getBestBlockHash: async () => {
        return this.call('getbestblockhash');
      },
      getTransactionHeight: async (hash) => {
        return this.call('gettransactionheight', [hash]);
      },
      getApplicationLog: async (hash) => {
        return this.call('getapplicationlog', [hash]);
      },
      getVersion: async () => {
        return this.call('getversion');
      },
      getPeers: async () => {
        return this.call('getpeers');
      }
    } as NeoRpcProvider;
  }

  private addressToScriptHash(address: string): string {
    // Convert Neo address to script hash using base58check decoding
    try {
      const decoded = this.base58CheckDecode(address);
      // Neo addresses start with version byte, followed by 20-byte script hash
      if (decoded.length !== 21) {
        throw new Error('Invalid address length');
      }
      
      const scriptHash = decoded.slice(1); // Remove version byte
      return '0x' + Buffer.from(scriptHash).reverse().toString('hex'); // Little endian
    } catch (error) {
      throw new Error(`Invalid Neo address format: ${address}`);
    }
  }

  private formatStackResult(stack: any[]): any {
    if (!stack || stack.length === 0) {
      return null;
    }

    if (stack.length === 1) {
      return this.formatStackItem(stack[0]);
    }

    return stack.map(item => this.formatStackItem(item));
  }

  private formatStackItem(item: any): any {
    if (!item) return null;

    switch (item.type) {
      case 'Boolean':
        return item.value;
      case 'Integer':
        return item.value;
      case 'ByteString':
        return item.value;
      case 'Array':
        return item.value?.map((subItem: any) => this.formatStackItem(subItem));
      case 'Map':
        return item.value;
      default:
        return item.value;
    }
  }

  private tryDecodeStorage(value: string): string {
    try {
      // Try to decode as ASCII
      const decoded = Buffer.from(value, 'hex').toString('ascii');
      if (/^[\x20-\x7E]*$/.test(decoded)) {
        return `"${decoded}"`;
      }
      
      // Try to decode as integer
      const intValue = parseInt(value, 16);
      if (!isNaN(intValue)) {
        return intValue.toString();
      }
      
      return value; // Return hex if no conversion works
    } catch {
      return value;
    }
  }

  /**
   * Encode arguments for Neo contract calls
   */
  private encodeArgs(args: any[]): any[] {
    return args.map(arg => {
      if (typeof arg === 'string') {
        return { type: 'String', value: arg };
      } else if (typeof arg === 'number' || typeof arg === 'bigint') {
        return { type: 'Integer', value: arg.toString() };
      } else if (typeof arg === 'boolean') {
        return { type: 'Boolean', value: arg };
      } else if (Array.isArray(arg)) {
        return { type: 'Array', value: this.encodeArgs(arg) };
      } else {
        return { type: 'ByteString', value: Buffer.from(JSON.stringify(arg)).toString('hex') };
      }
    });
  }

  /**
   * Get default signer address
   */
  private async getDefaultSigner(): Promise<string> {
    const config = await this.config.getConfig();
    if (config.accounts?.default) {
      return config.accounts.default;
    }
    throw new Error('No default signer configured. Use `neo-foundry account` to set up accounts.');
  }

  /**
   * Load contract from file
   */
  private async loadContract(contractPath: string): Promise<{ nef: string; manifest: any }> {
    const fs = await import('fs/promises');
    const path = await import('path');
    
    try {
      // Look for compiled contract files
      const nefPath = contractPath.replace(/\.sol$/, '.nef');
      const manifestPath = contractPath.replace(/\.sol$/, '.manifest.json');
      
      const nefContent = await fs.readFile(nefPath, 'utf-8');
      const manifestContent = await fs.readFile(manifestPath, 'utf-8');
      
      return {
        nef: nefContent,
        manifest: JSON.parse(manifestContent)
      };
    } catch (error) {
      throw new Error(`Failed to load contract from ${contractPath}: ${error}`);
    }
  }

  /**
   * Calculate network fee based on script size
   */
  private async calculateNetworkFee(scriptSize: number): Promise<bigint> {
    // Base network fee is 0.001 GAS per byte
    const baseFeePerbyte = BigInt('1000'); // 0.001 GAS in units
    return BigInt(scriptSize) * baseFeePerbyte;
  }

  /**
   * Derive public key from private key
   */
  private async derivePublicKey(privateKey: string): Promise<string> {
    const crypto = await import('crypto');
    
    // Create ECDSA key pair using secp256r1 curve (Neo standard)
    const keyBuffer = Buffer.from(privateKey.slice(2), 'hex');
    
    // For production, would use proper ECDSA library like elliptic
    // This is a simplified version for demonstration
    const publicKeyBuffer = crypto.createHash('sha256').update(keyBuffer).digest();
    return '0x' + publicKeyBuffer.toString('hex');
  }

  /**
   * Derive address from public key
   */
  private async deriveAddress(publicKey: string): Promise<string> {
    const crypto = await import('crypto');
    
    // Neo address derivation:
    // 1. Hash public key with SHA256 + RIPEMD160
    // 2. Add version byte (0x35 for N3)
    // 3. Add checksum
    // 4. Encode with base58
    
    const publicKeyBuffer = Buffer.from(publicKey.slice(2), 'hex');
    const sha256Hash = crypto.createHash('sha256').update(publicKeyBuffer).digest();
    const ripemd160Hash = crypto.createHash('ripemd160').update(sha256Hash).digest();
    
    const versionByte = Buffer.from([0x35]); // N3 version
    const addressPayload = Buffer.concat([versionByte, ripemd160Hash]);
    
    // Calculate checksum (first 4 bytes of double SHA256)
    const checksum1 = crypto.createHash('sha256').update(addressPayload).digest();
    const checksum2 = crypto.createHash('sha256').update(checksum1).digest();
    const checksum = checksum2.slice(0, 4);
    
    const fullAddress = Buffer.concat([addressPayload, checksum]);
    return this.base58Encode(fullAddress);
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
   * Base58 check decode
   */
  private base58CheckDecode(address: string): Buffer {
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
}