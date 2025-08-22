import {
  NeoNetworkConfig,
  NetworkConfig,
  NeoHardhatConfig
} from '@neo-solidity/types';
import { ethers } from 'ethers';
import { EventEmitter } from 'events';
import * as fs from 'fs-extra';
import * as path from 'path';

export class NetworkManager extends EventEmitter {
  private networks: Map<string, NeoNetworkConfig> = new Map();
  private providers: Map<string, ethers.Provider> = new Map();
  private signers: Map<string, ethers.Wallet> = new Map();
  private currentNetwork?: string;

  constructor(networks: { [networkName: string]: NeoNetworkConfig }) {
    super();
    
    // Register networks
    for (const [name, config] of Object.entries(networks)) {
      this.addNetwork(name, config);
    }
  }

  // Network Management
  addNetwork(name: string, config: NeoNetworkConfig): void {
    // Validate network configuration
    this.validateNetworkConfig(config);
    
    this.networks.set(name, config);
    this.emit('networkAdded', { name, config });
  }

  removeNetwork(name: string): boolean {
    if (this.networks.has(name)) {
      this.networks.delete(name);
      this.providers.delete(name);
      this.signers.delete(name);
      this.emit('networkRemoved', { name });
      return true;
    }
    return false;
  }

  getNetwork(name: string): NeoNetworkConfig | undefined {
    return this.networks.get(name);
  }

  listNetworks(): string[] {
    return Array.from(this.networks.keys());
  }

  getCurrentNetwork(): string | undefined {
    return this.currentNetwork;
  }

  // Provider Management
  async getProvider(networkName: string): Promise<ethers.Provider> {
    if (!this.providers.has(networkName)) {
      const network = this.networks.get(networkName);
      if (!network) {
        throw new Error(`Network ${networkName} not found`);
      }

      const provider = await this.createProvider(network);
      this.providers.set(networkName, provider);
    }

    return this.providers.get(networkName)!;
  }

  async getSigner(networkName: string, accountIndex: number = 0): Promise<ethers.Wallet> {
    const signerKey = `${networkName}:${accountIndex}`;
    
    if (!this.signers.has(signerKey)) {
      const network = this.networks.get(networkName);
      if (!network) {
        throw new Error(`Network ${networkName} not found`);
      }

      const provider = await this.getProvider(networkName);
      const signer = await this.createSigner(network, provider, accountIndex);
      this.signers.set(signerKey, signer);
    }

    return this.signers.get(signerKey)!;
  }

  // Network Connection
  async connectToNetwork(networkName: string): Promise<void> {
    const network = this.networks.get(networkName);
    if (!network) {
      throw new Error(`Network ${networkName} not found`);
    }

    this.emit('networkConnecting', { networkName });

    try {
      // Test connection
      const provider = await this.getProvider(networkName);
      const blockNumber = await provider.getBlockNumber();
      
      this.currentNetwork = networkName;
      this.emit('networkConnected', { networkName, blockNumber });
    } catch (error) {
      this.emit('networkConnectionFailed', { networkName, error });
      throw error;
    }
  }

  async disconnectFromNetwork(): Promise<void> {
    if (this.currentNetwork) {
      this.emit('networkDisconnected', { networkName: this.currentNetwork });
      this.currentNetwork = undefined;
    }
  }

  // Network Status
  async getNetworkStatus(networkName: string): Promise<{
    connected: boolean;
    blockNumber: number;
    chainId: number;
    gasPrice: string;
    peerCount?: number;
    sync?: {
      syncing: boolean;
      currentBlock?: number;
      highestBlock?: number;
    };
  }> {
    try {
      const provider = await this.getProvider(networkName);
      const network = await provider.getNetwork();
      const blockNumber = await provider.getBlockNumber();
      const feeData = await provider.getFeeData();

      return {
        connected: true,
        blockNumber,
        chainId: Number(network.chainId),
        gasPrice: feeData.gasPrice?.toString() || '0'
      };
    } catch (error) {
      return {
        connected: false,
        blockNumber: 0,
        chainId: 0,
        gasPrice: '0'
      };
    }
  }

  async waitForConnection(networkName: string, timeout: number = 30000): Promise<void> {
    const startTime = Date.now();
    
    while (Date.now() - startTime < timeout) {
      try {
        await this.connectToNetwork(networkName);
        return;
      } catch (error) {
        await new Promise(resolve => setTimeout(resolve, 1000));
      }
    }

    throw new Error(`Failed to connect to ${networkName} within ${timeout}ms`);
  }

  // Account Management
  async getAccounts(networkName: string): Promise<string[]> {
    const network = this.networks.get(networkName);
    if (!network) {
      throw new Error(`Network ${networkName} not found`);
    }

    if (Array.isArray(network.accounts)) {
      return network.accounts.map((privateKey, index) => {
        const wallet = new ethers.Wallet(privateKey);
        return wallet.address;
      });
    } else if (network.accounts.mnemonic) {
      const accounts: string[] = [];
      for (let i = 0; i < network.accounts.count; i++) {
        const wallet = ethers.Wallet.fromPhrase(
          network.accounts.mnemonic,
          `${network.accounts.path}/${i}`
        );
        accounts.push(wallet.address);
      }
      return accounts;
    }

    return [];
  }

  async getBalance(networkName: string, address: string): Promise<string> {
    const provider = await this.getProvider(networkName);
    const balance = await provider.getBalance(address);
    return ethers.formatEther(balance);
  }

  async getTransactionCount(networkName: string, address: string): Promise<number> {
    const provider = await this.getProvider(networkName);
    return provider.getTransactionCount(address);
  }

  // Neo-specific Features
  async getNeoBlockHeight(networkName: string): Promise<number> {
    const provider = await this.getProvider(networkName);
    return provider.getBlockNumber();
  }

  async getNeoAddressVersion(networkName: string): Promise<number> {
    const network = this.networks.get(networkName);
    return network?.addressVersion || 53;
  }

  async getMagicNumber(networkName: string): Promise<number> {
    const network = this.networks.get(networkName);
    return network?.magic || 860833102;
  }

  // Configuration Management
  async saveNetworkConfig(filePath: string): Promise<void> {
    const config: { [name: string]: NeoNetworkConfig } = {};
    
    for (const [name, network] of this.networks.entries()) {
      // Remove sensitive data
      const safeConfig = { ...network };
      if (Array.isArray(safeConfig.accounts)) {
        safeConfig.accounts = safeConfig.accounts.map(() => '<private_key>');
      } else if (safeConfig.accounts.mnemonic) {
        safeConfig.accounts.mnemonic = '<mnemonic>';
      }
      
      config[name] = safeConfig;
    }

    await fs.writeJson(filePath, config, { spaces: 2 });
  }

  async loadNetworkConfig(filePath: string): Promise<void> {
    if (!await fs.pathExists(filePath)) {
      throw new Error(`Network config file not found: ${filePath}`);
    }

    const config = await fs.readJson(filePath);
    
    for (const [name, networkConfig] of Object.entries(config)) {
      this.addNetwork(name, networkConfig as NeoNetworkConfig);
    }
  }

  // Monitoring
  startNetworkMonitoring(networkName: string, interval: number = 10000): void {
    const monitoringId = setInterval(async () => {
      try {
        const status = await this.getNetworkStatus(networkName);
        this.emit('networkStatus', { networkName, status });
      } catch (error) {
        this.emit('networkError', { networkName, error });
      }
    }, interval);

    this.once('networkDisconnected', () => {
      clearInterval(monitoringId);
    });
  }

  // Private Implementation
  private validateNetworkConfig(config: NeoNetworkConfig): void {
    if (!config.rpc?.url) {
      throw new Error('Network RPC URL is required');
    }

    if (!config.magic) {
      throw new Error('Network magic number is required');
    }

    if (!config.addressVersion) {
      throw new Error('Network address version is required');
    }

    try {
      new URL(config.rpc.url);
    } catch {
      throw new Error('Invalid RPC URL format');
    }
  }

  private async createProvider(network: NeoNetworkConfig): Promise<ethers.Provider> {
    const providerOptions: any = {
      timeout: network.rpc.timeout || 30000
    };

    if (network.rpc.headers) {
      providerOptions.headers = network.rpc.headers;
    }

    return new ethers.JsonRpcProvider(network.rpc.url, undefined, providerOptions);
  }

  private async createSigner(
    network: NeoNetworkConfig,
    provider: ethers.Provider,
    accountIndex: number
  ): Promise<ethers.Wallet> {
    if (Array.isArray(network.accounts)) {
      if (accountIndex >= network.accounts.length) {
        throw new Error(`Account index ${accountIndex} out of range`);
      }
      
      return new ethers.Wallet(network.accounts[accountIndex], provider);
    } else if (network.accounts.mnemonic) {
      const derivationPath = `${network.accounts.path}/${network.accounts.initialIndex + accountIndex}`;
      return ethers.Wallet.fromPhrase(network.accounts.mnemonic, derivationPath).connect(provider);
    } else {
      throw new Error('No accounts configured for network');
    }
  }

  // Utilities
  async estimateGas(
    networkName: string,
    transaction: any
  ): Promise<{
    gasLimit: string;
    gasPrice: string;
    totalCost: string;
  }> {
    const provider = await this.getProvider(networkName);
    
    const gasLimit = await provider.estimateGas(transaction);
    const feeData = await provider.getFeeData();
    const gasPrice = feeData.gasPrice || BigInt(0);
    const totalCost = gasLimit * gasPrice;

    return {
      gasLimit: gasLimit.toString(),
      gasPrice: gasPrice.toString(),
      totalCost: totalCost.toString()
    };
  }

  async sendTransaction(
    networkName: string,
    transaction: any,
    accountIndex: number = 0
  ): Promise<{
    hash: string;
    wait: (confirmations?: number) => Promise<any>;
  }> {
    const signer = await this.getSigner(networkName, accountIndex);
    const tx = await signer.sendTransaction(transaction);
    
    return {
      hash: tx.hash,
      wait: async (confirmations = 1) => {
        const receipt = await tx.wait(confirmations);
        this.emit('transactionConfirmed', { 
          networkName, 
          hash: tx.hash, 
          receipt 
        });
        return receipt;
      }
    };
  }

  // Network Discovery
  async discoverNetworks(): Promise<string[]> {
    // This could discover networks from common configuration files
    const discoveries: string[] = [];
    
    // Check for Hardhat config
    const hardhatConfigPath = path.join(process.cwd(), 'hardhat.config.js');
    if (await fs.pathExists(hardhatConfigPath)) {
      discoveries.push('hardhat');
    }

    // Check for Foundry config
    const foundryConfigPath = path.join(process.cwd(), 'foundry.toml');
    if (await fs.pathExists(foundryConfigPath)) {
      discoveries.push('foundry');
    }

    return discoveries;
  }

  async autoConfigureFromHardhat(configPath?: string): Promise<void> {
    const hardhatPath = configPath || path.join(process.cwd(), 'hardhat.config.js');
    
    if (!await fs.pathExists(hardhatPath)) {
      throw new Error('Hardhat config not found');
    }

    try {
      // This would parse the Hardhat config and extract networks
      // For now, add a default local network
      this.addNetwork('hardhat', {
        magic: 860833102,
        addressVersion: 53,
        rpc: {
          url: 'http://127.0.0.1:8545'
        },
        accounts: [],
        gasLimit: '9007199254740991',
        gasPrice: '0',
        blockGasLimit: '9007199254740991'
      });
    } catch (error) {
      throw new Error(`Failed to parse Hardhat config: ${error}`);
    }
  }

  async autoConfigureFromFoundry(configPath?: string): Promise<void> {
    const foundryPath = configPath || path.join(process.cwd(), 'foundry.toml');
    
    if (!await fs.pathExists(foundryPath)) {
      throw new Error('Foundry config not found');
    }

    try {
      // This would parse the Foundry TOML config
      // For now, add a default local network
      this.addNetwork('anvil', {
        magic: 860833102,
        addressVersion: 53,
        rpc: {
          url: 'http://127.0.0.1:8545'
        },
        accounts: [],
        gasLimit: '9007199254740991',
        gasPrice: '0',
        blockGasLimit: '9007199254740991'
      });
    } catch (error) {
      throw new Error(`Failed to parse Foundry config: ${error}`);
    }
  }

  // Health Checks
  async healthCheck(networkName: string): Promise<{
    healthy: boolean;
    checks: Array<{
      name: string;
      status: 'pass' | 'fail' | 'warn';
      message: string;
      duration?: number;
    }>;
  }> {
    const checks: Array<{
      name: string;
      status: 'pass' | 'fail' | 'warn';
      message: string;
      duration?: number;
    }> = [];

    // Connection check
    const startTime = Date.now();
    try {
      await this.getNetworkStatus(networkName);
      checks.push({
        name: 'connection',
        status: 'pass',
        message: 'Successfully connected to network',
        duration: Date.now() - startTime
      });
    } catch (error) {
      checks.push({
        name: 'connection',
        status: 'fail',
        message: `Connection failed: ${error}`,
        duration: Date.now() - startTime
      });
    }

    // Account balance check
    try {
      const accounts = await this.getAccounts(networkName);
      if (accounts.length > 0) {
        const balance = await this.getBalance(networkName, accounts[0]);
        if (parseFloat(balance) > 0) {
          checks.push({
            name: 'balance',
            status: 'pass',
            message: `Account has balance: ${balance} ETH`
          });
        } else {
          checks.push({
            name: 'balance',
            status: 'warn',
            message: 'Account has zero balance'
          });
        }
      }
    } catch (error) {
      checks.push({
        name: 'balance',
        status: 'fail',
        message: `Balance check failed: ${error}`
      });
    }

    const healthy = checks.every(check => check.status !== 'fail');
    
    return { healthy, checks };
  }
}