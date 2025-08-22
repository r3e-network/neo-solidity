import { HardhatRuntimeEnvironment } from 'hardhat/types';
import {
  NeoHardhatConfig,
  DeploymentOptions,
  DeploymentResult,
  ContractABI,
  TransactionResponse,
  TransactionReceipt
} from '@neo-solidity/types';
import { ethers } from 'ethers';
import { EventEmitter } from 'events';
import * as fs from 'fs-extra';
import * as path from 'path';

export class DeploymentManager extends EventEmitter {
  private config: NeoHardhatConfig;
  private network: any;
  private provider: any;
  private signer: any;
  private deployments: Map<string, DeploymentResult> = new Map();

  constructor(config: NeoHardhatConfig, network: any) {
    super();
    this.config = config;
    this.network = network;
    this.initializeProvider();
  }

  private initializeProvider(): void {
    const networkConfig = this.config.networks[this.network.name];
    if (!networkConfig) {
      throw new Error(`Network ${this.network.name} not configured`);
    }

    this.provider = new ethers.JsonRpcProvider(networkConfig.rpc.url);
    
    if (Array.isArray(networkConfig.accounts) && networkConfig.accounts.length > 0) {
      this.signer = new ethers.Wallet(networkConfig.accounts[0], this.provider);
    }
  }

  async deployContract(
    contractName: string,
    constructorArgs: any[] = [],
    options: DeploymentOptions = {}
  ): Promise<DeploymentResult> {
    this.emit('deploymentStarted', { contractName, args: constructorArgs });

    try {
      // Load contract artifact
      const artifact = await this.loadArtifact(contractName);
      
      // Create contract factory
      const factory = new ethers.ContractFactory(
        artifact.abi,
        artifact.bytecode,
        this.signer
      );

      // Estimate gas if not provided
      if (!options.gasLimit) {
        const gasEstimate = await factory.getDeployTransaction(...constructorArgs).then(tx => 
          this.provider.estimateGas(tx)
        );
        options.gasLimit = (gasEstimate * BigInt(120) / BigInt(100)).toString(); // 20% buffer
      }

      // Deploy contract
      const deployTx = {
        gasLimit: options.gasLimit,
        gasPrice: options.gasPrice || this.config.neo?.gasPrice,
        value: options.value || '0',
        nonce: options.nonce
      };

      if (!options.skipDryRun) {
        await this.performDryRun(factory, constructorArgs, deployTx);
      }

      const contract = await factory.deploy(...constructorArgs, deployTx);
      const receipt = await contract.waitForDeployment();
      
      // Get deployment receipt
      const txReceipt = await contract.deploymentTransaction()?.wait(options.confirmations || 1);
      
      const deploymentResult: DeploymentResult = {
        contractName,
        contractAddress: await contract.getAddress(),
        transactionHash: contract.deploymentTransaction()?.hash || '',
        gasUsed: txReceipt?.gasUsed?.toString() || '0',
        gasPrice: txReceipt?.gasPrice?.toString() || '0',
        deploymentData: {
          bytecode: artifact.bytecode,
          constructorArgs,
          libraries: options.libraries || {}
        },
        receipt: txReceipt,
        deployedAt: new Date(),
        network: this.network.name,
        block: {
          number: txReceipt?.blockNumber || 0,
          hash: txReceipt?.blockHash || '',
          timestamp: Date.now()
        }
      };

      // Save deployment
      this.deployments.set(contractName, deploymentResult);
      await this.saveDeployment(deploymentResult);
      
      this.emit('deploymentCompleted', deploymentResult);
      return deploymentResult;

    } catch (error) {
      this.emit('deploymentFailed', { contractName, error });
      throw error;
    }
  }

  private async performDryRun(
    factory: ethers.ContractFactory,
    args: any[],
    txOptions: any
  ): Promise<void> {
    try {
      // Simulate deployment on fork
      const forkProvider = new ethers.JsonRpcProvider(this.config.networks.hardhat.rpc.url);
      const forkSigner = new ethers.Wallet(this.config.neo?.privateKey || '', forkProvider);
      
      const forkFactory = new ethers.ContractFactory(
        factory.interface,
        factory.bytecode,
        forkSigner
      );

      const contract = await forkFactory.deploy(...args, {
        ...txOptions,
        gasLimit: txOptions.gasLimit || '9007199254740991' // Max safe integer
      });
      
      await contract.waitForDeployment();
      console.log('✅ Dry run successful');
    } catch (error) {
      console.log('❌ Dry run failed:', error);
      throw new Error(`Deployment simulation failed: ${error}`);
    }
  }

  async deployMultiple(
    contracts: Array<{
      name: string;
      args?: any[];
      options?: DeploymentOptions;
    }>
  ): Promise<DeploymentResult[]> {
    const results: DeploymentResult[] = [];
    
    for (const contract of contracts) {
      const result = await this.deployContract(
        contract.name,
        contract.args || [],
        contract.options || {}
      );
      results.push(result);
    }
    
    return results;
  }

  async verifyContract(
    contractName: string,
    constructorArgs: any[] = []
  ): Promise<boolean> {
    const deployment = this.deployments.get(contractName);
    if (!deployment) {
      throw new Error(`No deployment found for ${contractName}`);
    }

    try {
      // Load source code
      const artifact = await this.loadArtifact(contractName);
      const sourceCode = await this.getSourceCode(contractName);
      
      // Create verification request
      const verificationData = {
        contractAddress: deployment.contractAddress,
        sourceCode,
        contractName,
        compilerVersion: this.config.solidity.version,
        optimizationUsed: this.config.solidity.settings.optimizer.enabled,
        runs: this.config.solidity.settings.optimizer.runs,
        constructorArguments: this.encodeConstructorArgs(artifact.abi, constructorArgs),
        abi: artifact.abi,
        bytecode: artifact.bytecode
      };

      // Submit to block explorer
      const verified = await this.submitVerification(verificationData);
      
      if (verified) {
        console.log('✅ Contract verification submitted successfully');
        // Save verification status
        await this.saveVerificationResult(deployment.contractAddress, {
          verified: true,
          submittedAt: new Date().toISOString(),
          verificationData
        });
        return true;
      } else {
        console.log('❌ Contract verification submission failed');
        return false;
      }
    } catch (error) {
      console.error('❌ Verification failed:', error);
      return false;
    }
  }

  async upgradeContract(
    proxyAddress: string,
    newImplementationName: string,
    upgradeOptions: DeploymentOptions = {}
  ): Promise<DeploymentResult> {
    // Deploy new implementation
    const newImplementation = await this.deployContract(
      newImplementationName,
      [],
      { ...upgradeOptions, skipDryRun: false }
    );

    // Execute proxy upgrade transaction
    try {
      const proxyContract = await this.getContractAt('IProxy', proxyAddress);
      
      // Prepare upgrade transaction
      const upgradeData = proxyContract.interface.encodeFunctionData('upgrade', [
        newImplementation.contractAddress
      ]);
      
      const upgradeTx = await this.executeTransaction({
        to: proxyAddress,
        data: upgradeData,
        gasLimit: upgradeOptions.gasLimit || 500000
      });
      
      console.log(`✅ Proxy upgraded successfully. Tx: ${upgradeTx.hash}`);
      
      // Update deployment record
      const upgradeResult = {
        ...newImplementation,
        upgradeTransaction: upgradeTx,
        proxyAddress,
        upgradeType: 'proxy'
      };
      
      await this.saveDeployment(newImplementationName, upgradeResult);
      
      return upgradeResult;
    } catch (error) {
      console.error('❌ Proxy upgrade failed:', error);
      throw new Error(`Proxy upgrade failed: ${error}`);
    }
  }

  async getDeployment(contractName: string): Promise<DeploymentResult | undefined> {
    return this.deployments.get(contractName) || await this.loadDeployment(contractName);
  }

  async getAllDeployments(): Promise<DeploymentResult[]> {
    const deploymentDir = path.join(process.cwd(), 'deployments', this.network.name);
    const files = await fs.readdir(deploymentDir).catch(() => []);
    
    const deployments: DeploymentResult[] = [];
    for (const file of files) {
      if (file.endsWith('.json')) {
        const deployment = await this.loadDeployment(file.replace('.json', ''));
        if (deployment) {
          deployments.push(deployment);
        }
      }
    }
    
    return deployments;
  }

  private async loadArtifact(contractName: string): Promise<any> {
    const artifactPath = path.join(
      this.config.paths.artifacts,
      'contracts',
      `${contractName}.sol`,
      `${contractName}.json`
    );
    
    if (!await fs.pathExists(artifactPath)) {
      throw new Error(`Artifact not found for ${contractName}. Run 'npx hardhat compile' first.`);
    }
    
    return fs.readJson(artifactPath);
  }

  private async getSourceCode(contractName: string): Promise<string> {
    const sourcePath = path.join(this.config.paths.sources, `${contractName}.sol`);
    return fs.readFile(sourcePath, 'utf8');
  }

  private encodeConstructorArgs(abi: ContractABI, args: any[]): string {
    if (args.length === 0) return '';
    
    const iface = new ethers.Interface(abi);
    const constructor = abi.find(item => item.type === 'constructor');
    
    if (!constructor) return '';
    
    return ethers.AbiCoder.defaultAbiCoder()
      .encode(constructor.inputs.map(input => input.type), args)
      .slice(2); // Remove 0x prefix
  }

  private async saveDeployment(deployment: DeploymentResult): Promise<void> {
    const deploymentDir = path.join(process.cwd(), 'deployments', this.network.name);
    await fs.ensureDir(deploymentDir);
    
    const deploymentPath = path.join(deploymentDir, `${deployment.contractName}.json`);
    await fs.writeJson(deploymentPath, deployment, { spaces: 2 });
  }

  private async loadDeployment(contractName: string): Promise<DeploymentResult | undefined> {
    const deploymentPath = path.join(
      process.cwd(),
      'deployments',
      this.network.name,
      `${contractName}.json`
    );
    
    if (!await fs.pathExists(deploymentPath)) {
      return undefined;
    }
    
    return fs.readJson(deploymentPath);
  }

  // Gas optimization utilities
  async estimateDeploymentCost(
    contractName: string,
    constructorArgs: any[] = []
  ): Promise<{
    gasEstimate: string;
    costInWei: string;
    costInEth: string;
  }> {
    const artifact = await this.loadArtifact(contractName);
    const factory = new ethers.ContractFactory(
      artifact.abi,
      artifact.bytecode,
      this.signer
    );

    const gasEstimate = await factory.getDeployTransaction(...constructorArgs)
      .then(tx => this.provider.estimateGas(tx));
    
    const gasPrice = await this.provider.getFeeData().then(fee => fee.gasPrice || BigInt(0));
    const costInWei = gasEstimate * gasPrice;
    const costInEth = ethers.formatEther(costInWei);

    return {
      gasEstimate: gasEstimate.toString(),
      costInWei: costInWei.toString(),
      costInEth
    };
  }

  async batchDeploy(
    deployments: Array<{
      contractName: string;
      args: any[];
      options?: DeploymentOptions;
    }>,
    batchOptions: {
      maxConcurrency?: number;
      delayBetween?: number;
    } = {}
  ): Promise<DeploymentResult[]> {
    const { maxConcurrency = 3, delayBetween = 1000 } = batchOptions;
    const results: DeploymentResult[] = [];
    
    // Process in batches to avoid overwhelming the network
    for (let i = 0; i < deployments.length; i += maxConcurrency) {
      const batch = deployments.slice(i, i + maxConcurrency);
      
      const batchResults = await Promise.all(
        batch.map(deployment => 
          this.deployContract(deployment.contractName, deployment.args, deployment.options)
        )
      );
      
      results.push(...batchResults);
      
      // Delay between batches
      if (i + maxConcurrency < deployments.length && delayBetween > 0) {
        await new Promise(resolve => setTimeout(resolve, delayBetween));
      }
    }
    
    return results;
  }

  // Deployment analytics
  async generateDeploymentReport(): Promise<{
    totalDeployments: number;
    totalGasUsed: string;
    totalCost: string;
    deploymentsByStatus: { [status: string]: number };
    topGasConsumers: Array<{ contract: string; gasUsed: string }>;
  }> {
    const deployments = await this.getAllDeployments();
    
    const totalGasUsed = deployments.reduce(
      (sum, deployment) => sum + BigInt(deployment.gasUsed),
      BigInt(0)
    );
    
    const totalCost = deployments.reduce(
      (sum, deployment) => sum + (BigInt(deployment.gasUsed) * BigInt(deployment.gasPrice)),
      BigInt(0)
    );

    const topGasConsumers = deployments
      .sort((a, b) => Number(BigInt(b.gasUsed) - BigInt(a.gasUsed)))
      .slice(0, 10)
      .map(d => ({
        contract: d.contractName,
        gasUsed: d.gasUsed
      }));

    return {
      totalDeployments: deployments.length,
      totalGasUsed: totalGasUsed.toString(),
      totalCost: ethers.formatEther(totalCost),
      deploymentsByStatus: {
        successful: deployments.filter(d => d.status === 'success').length,
        failed: deployments.filter(d => d.status === 'failed').length,
        pending: deployments.filter(d => d.status === 'pending').length,
        verified: deployments.filter(d => d.verified).length
      },
      topGasConsumers
    };
  }

  /**
   * Submit contract for verification to block explorer
   */
  private async submitVerification(verificationData: any): Promise<boolean> {
    try {
      const explorerConfig = this.getExplorerConfig();
      if (!explorerConfig) {
        console.warn('No explorer configuration found, skipping verification');
        return false;
      }

      const response = await fetch(`${explorerConfig.baseUrl}/api/verify-contract`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          ...(explorerConfig.apiKey && { 'Authorization': `Bearer ${explorerConfig.apiKey}` })
        },
        body: JSON.stringify(verificationData)
      });

      if (!response.ok) {
        console.error(`Verification request failed: ${response.status} ${response.statusText}`);
        return false;
      }

      const result = await response.json();
      return result.success === true;
    } catch (error) {
      console.error('Verification submission error:', error);
      return false;
    }
  }

  /**
   * Save verification result
   */
  private async saveVerificationResult(contractAddress: string, result: any): Promise<void> {
    const verificationPath = path.join(
      this.config.paths.deployments,
      this.network.name,
      '.verifications',
      `${contractAddress}.json`
    );

    await fs.mkdir(path.dirname(verificationPath), { recursive: true });
    await fs.writeFile(verificationPath, JSON.stringify(result, null, 2));
  }

  /**
   * Get contract interface
   */
  private async getContractAt(contractName: string, address: string): Promise<any> {
    const artifact = await this.loadArtifact(contractName);
    return new ethers.Contract(address, artifact.abi, this.signer);
  }

  /**
   * Execute transaction with proper error handling
   */
  private async executeTransaction(txParams: any): Promise<any> {
    try {
      const tx = await this.signer.sendTransaction(txParams);
      const receipt = await tx.wait();
      
      if (!receipt || receipt.status === 0) {
        throw new Error('Transaction failed');
      }
      
      return receipt;
    } catch (error) {
      throw new Error(`Transaction execution failed: ${error}`);
    }
  }

  /**
   * Get explorer configuration for current network
   */
  private getExplorerConfig(): { baseUrl: string; apiKey?: string } | null {
    const networkName = this.network.name;
    const configs: Record<string, { baseUrl: string; apiKey?: string }> = {
      'mainnet': { 
        baseUrl: 'https://explorer.neo.org',
        apiKey: process.env.NEO_EXPLORER_API_KEY
      },
      'testnet': { 
        baseUrl: 'https://testnet.neo.org',
        apiKey: process.env.NEO_TESTNET_EXPLORER_API_KEY
      },
      'private': { 
        baseUrl: 'http://localhost:4000'
      }
    };

    return configs[networkName] || null;
  }

  /**
   * Load artifact from file system
   */
  private async loadArtifact(contractName: string): Promise<any> {
    const artifactPath = path.join(
      this.config.paths.artifacts,
      'contracts',
      `${contractName}.sol`,
      `${contractName}.json`
    );

    try {
      const artifactContent = await fs.readFile(artifactPath, 'utf-8');
      return JSON.parse(artifactContent);
    } catch (error) {
      throw new Error(`Failed to load artifact for ${contractName}: ${error}`);
    }
  }

  /**
   * Get source code for contract
   */
  private async getSourceCode(contractName: string): Promise<string> {
    const sourcePath = path.join(
      this.config.paths.sources,
      `${contractName}.sol`
    );

    try {
      return await fs.readFile(sourcePath, 'utf-8');
    } catch (error) {
      throw new Error(`Failed to load source code for ${contractName}: ${error}`);
    }
  }

  /**
   * Load existing deployment
   */
  private async loadDeployment(contractName: string): Promise<DeploymentResult | undefined> {
    const deploymentPath = path.join(
      this.config.paths.deployments,
      this.network.name,
      `${contractName}.json`
    );

    try {
      const deploymentContent = await fs.readFile(deploymentPath, 'utf-8');
      return JSON.parse(deploymentContent);
    } catch (error) {
      if ((error as any).code === 'ENOENT') {
        return undefined;
      }
      throw error;
    }
  }
}