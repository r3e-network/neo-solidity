import {
  CastCallOptions,
  CastCallResult,
  CastSendOptions,
  CastSendResult,
  ContractABI,
  TransactionReceipt
} from '@neo-solidity/types';
import { ethers } from 'ethers';
import { EventEmitter } from 'events';
import * as fs from 'fs-extra';
import * as path from 'path';

export class NeoCast extends EventEmitter {
  private provider: ethers.Provider;
  private signer?: ethers.Wallet;
  private rpcUrl: string;

  constructor(rpcUrl: string = 'http://127.0.0.1:10332') {
    super();
    this.rpcUrl = rpcUrl;
    this.provider = new ethers.JsonRpcProvider(rpcUrl);
  }

  // Wallet Management
  async setupWallet(options: {
    privateKey?: string;
    mnemonic?: string;
    keystore?: string;
    password?: string;
  }): Promise<void> {
    if (options.privateKey) {
      this.signer = new ethers.Wallet(options.privateKey, this.provider);
    } else if (options.mnemonic) {
      this.signer = ethers.Wallet.fromPhrase(options.mnemonic).connect(this.provider);
    } else if (options.keystore && options.password) {
      const keystoreData = await fs.readFile(options.keystore, 'utf8');
      this.signer = await ethers.Wallet.fromEncryptedJson(keystoreData, options.password);
      this.signer = this.signer.connect(this.provider);
    }

    if (this.signer) {
      this.emit('walletSetup', { address: this.signer.address });
    }
  }

  // Contract Interaction
  async call(
    contractAddress: string,
    functionSignature: string,
    args: any[] = [],
    options: CastCallOptions = {}
  ): Promise<CastCallResult> {
    try {
      this.emit('callStarted', { contractAddress, functionSignature, args });

      // Parse function signature
      const { name, inputs, outputs } = this.parseFunctionSignature(functionSignature);
      
      // Create contract interface
      const abi = this.createMinimalABI(name, inputs, outputs);
      const contract = new ethers.Contract(contractAddress, abi, this.provider);

      // Execute call
      const result = await contract[name](...args, {
        blockTag: options.block || 'latest',
        from: options.from
      });

      const callResult: CastCallResult = {
        success: true,
        result: this.formatResult(result, outputs),
        gas_used: await this.estimateGas(contractAddress, functionSignature, args, options)
      };

      this.emit('callCompleted', callResult);
      return callResult;
    } catch (error) {
      const callResult: CastCallResult = {
        success: false,
        error: {
          code: -1,
          message: String(error),
          data: error instanceof Error ? error.stack : undefined
        }
      };

      this.emit('callFailed', { error, callResult });
      return callResult;
    }
  }

  async send(
    contractAddress: string,
    functionSignature: string,
    args: any[] = [],
    options: CastSendOptions = {}
  ): Promise<CastSendResult> {
    try {
      if (!this.signer) {
        throw new Error('No signer available. Setup wallet first.');
      }

      this.emit('sendStarted', { contractAddress, functionSignature, args });

      // Parse function signature
      const { name, inputs } = this.parseFunctionSignature(functionSignature);
      
      // Create contract interface
      const abi = this.createMinimalABI(name, inputs, []);
      const contract = new ethers.Contract(contractAddress, abi, this.signer);

      // Prepare transaction options
      const txOptions: any = {};
      if (options.gas) txOptions.gasLimit = options.gas;
      if (options.gas_price) txOptions.gasPrice = options.gas_price;
      if (options.value) txOptions.value = options.value;
      if (options.nonce) txOptions.nonce = options.nonce;

      // Send transaction
      const tx = await contract[name](...args, txOptions);
      
      // Wait for confirmation if not async
      let receipt;
      if (!options.async) {
        receipt = await tx.wait(options.confirmations || 1);
      }

      const sendResult: CastSendResult = {
        success: true,
        transaction_hash: tx.hash,
        block_number: receipt?.blockNumber,
        gas_used: receipt?.gasUsed?.toString(),
        gas_price: tx.gasPrice?.toString(),
        status: receipt?.status === 1 ? 'success' : 'failure',
        logs: receipt?.logs || [],
        receipt
      };

      this.emit('sendCompleted', sendResult);
      return sendResult;
    } catch (error) {
      const sendResult: CastSendResult = {
        success: false,
        error: {
          code: -1,
          message: String(error),
          data: error instanceof Error ? error.stack : undefined
        }
      };

      this.emit('sendFailed', { error, sendResult });
      return sendResult;
    }
  }

  // Utility Functions
  async balance(address: string, block?: string | number): Promise<string> {
    const balance = await this.provider.getBalance(address, block || 'latest');
    return ethers.formatEther(balance);
  }

  async nonce(address: string, block?: string | number): Promise<number> {
    return this.provider.getTransactionCount(address, block || 'latest');
  }

  async code(address: string, block?: string | number): Promise<string> {
    return this.provider.getCode(address, block || 'latest');
  }

  async storage(address: string, slot: string, block?: string | number): Promise<string> {
    return this.provider.getStorage(address, slot, block || 'latest');
  }

  // Transaction Utilities
  async receipt(txHash: string): Promise<TransactionReceipt | null> {
    const receipt = await this.provider.getTransactionReceipt(txHash);
    
    if (!receipt) {
      return null;
    }

    return {
      transactionHash: receipt.hash,
      transactionIndex: receipt.index,
      blockHash: receipt.blockHash,
      blockNumber: receipt.blockNumber,
      from: receipt.from,
      to: receipt.to || '',
      gasUsed: receipt.gasUsed.toString(),
      gasPrice: receipt.gasPrice?.toString() || '0',
      status: receipt.status || 0,
      logs: receipt.logs,
      events: [], // Would be decoded based on ABI
      contractAddress: receipt.contractAddress
    };
  }

  async transaction(txHash: string): Promise<any> {
    return this.provider.getTransaction(txHash);
  }

  async block(blockNumberOrHash: string | number): Promise<any> {
    if (typeof blockNumberOrHash === 'number') {
      return this.provider.getBlock(blockNumberOrHash);
    } else if (blockNumberOrHash.startsWith('0x') && blockNumberOrHash.length === 66) {
      return this.provider.getBlock(blockNumberOrHash);
    } else {
      return this.provider.getBlock(parseInt(blockNumberOrHash));
    }
  }

  // Encoding/Decoding Utilities
  encodeFunctionData(functionSignature: string, args: any[]): string {
    const { name, inputs } = this.parseFunctionSignature(functionSignature);
    const iface = new ethers.Interface([`function ${name}(${inputs.map(i => `${i.type} ${i.name}`).join(',')})`]);
    return iface.encodeFunctionData(name, args);
  }

  decodeFunctionData(functionSignature: string, data: string): any[] {
    const { name, inputs } = this.parseFunctionSignature(functionSignature);
    const iface = new ethers.Interface([`function ${name}(${inputs.map(i => `${i.type} ${i.name}`).join(',')})`]);
    const decoded = iface.decodeFunctionData(name, data);
    return Array.from(decoded);
  }

  encodeFunctionResult(functionSignature: string, values: any[]): string {
    const { name, inputs, outputs } = this.parseFunctionSignature(functionSignature);
    const iface = new ethers.Interface([`function ${name}(${inputs.map(i => `${i.type} ${i.name}`).join(',')}) returns (${outputs.map(o => `${o.type} ${o.name}`).join(',')})`]);
    return iface.encodeFunctionResult(name, values);
  }

  decodeFunctionResult(functionSignature: string, data: string): any[] {
    const { name, inputs, outputs } = this.parseFunctionSignature(functionSignature);
    const iface = new ethers.Interface([`function ${name}(${inputs.map(i => `${i.type} ${i.name}`).join(',')}) returns (${outputs.map(o => `${o.type} ${o.name}`).join(',')})`]);
    const decoded = iface.decodeFunctionResult(name, data);
    return Array.from(decoded);
  }

  // Event Utilities
  async events(
    contractAddress: string,
    eventSignature: string,
    fromBlock?: number,
    toBlock?: number
  ): Promise<any[]> {
    const { name, inputs } = this.parseEventSignature(eventSignature);
    const eventAbi = {
      type: 'event',
      name,
      inputs: inputs.map(input => ({
        name: input.name,
        type: input.type,
        indexed: input.indexed || false
      }))
    };

    const iface = new ethers.Interface([eventAbi]);
    const eventFilter = {
      address: contractAddress,
      topics: [iface.getEvent(name).topicHash],
      fromBlock: fromBlock || 0,
      toBlock: toBlock || 'latest'
    };

    const logs = await this.provider.getLogs(eventFilter);
    return logs.map(log => {
      try {
        const decoded = iface.parseLog(log);
        return {
          ...log,
          decoded: {
            name: decoded?.name,
            args: decoded?.args,
            signature: decoded?.signature
          }
        };
      } catch (error) {
        return { ...log, decoded: null, error: String(error) };
      }
    });
  }

  // ABI Utilities
  async loadABI(contractPath: string): Promise<ContractABI> {
    try {
      // Try to load from artifacts
      const artifactPath = path.join(process.cwd(), 'out', `${contractPath}.sol`, `${contractPath}.json`);
      const artifact = await fs.readJson(artifactPath);
      return artifact.abi;
    } catch (error) {
      throw new Error(`Could not load ABI for ${contractPath}: ${error}`);
    }
  }

  async callWithABI(
    contractAddress: string,
    abiPath: string,
    functionName: string,
    args: any[] = [],
    options: CastCallOptions = {}
  ): Promise<CastCallResult> {
    try {
      const abi = await this.loadABI(abiPath);
      const contract = new ethers.Contract(contractAddress, abi, this.provider);
      
      const result = await contract[functionName](...args, {
        blockTag: options.block || 'latest',
        from: options.from
      });

      return {
        success: true,
        result: this.formatComplexResult(result)
      };
    } catch (error) {
      return {
        success: false,
        error: {
          code: -1,
          message: String(error),
          data: error instanceof Error ? error.stack : undefined
        }
      };
    }
  }

  // Advanced Features
  async multicall(calls: Array<{
    target: string;
    callData: string;
    allowFailure?: boolean;
  }>): Promise<{
    success: boolean;
    results: Array<{ success: boolean; returnData: string }>;
  }> {
    // This would implement multicall functionality
    const results: Array<{ success: boolean; returnData: string }> = [];
    
    for (const call of calls) {
      try {
        const result = await this.provider.call({
          to: call.target,
          data: call.callData
        });
        
        results.push({ success: true, returnData: result });
      } catch (error) {
        if (call.allowFailure) {
          results.push({ success: false, returnData: '0x' });
        } else {
          throw error;
        }
      }
    }

    return { success: true, results };
  }

  async simulate(
    contractAddress: string,
    functionSignature: string,
    args: any[] = [],
    options: CastCallOptions = {}
  ): Promise<{
    success: boolean;
    gasUsed: string;
    returnData: any;
    traces?: any[];
  }> {
    try {
      // Simulate the transaction without sending it
      const callResult = await this.call(contractAddress, functionSignature, args, options);
      
      return {
        success: callResult.success,
        gasUsed: callResult.gas_used || '0',
        returnData: callResult.result,
        traces: [] // Would include execution traces
      };
    } catch (error) {
      return {
        success: false,
        gasUsed: '0',
        returnData: null
      };
    }
  }

  // Private Helper Methods
  private parseFunctionSignature(signature: string): {
    name: string;
    inputs: Array<{ name: string; type: string }>;
    outputs: Array<{ name: string; type: string }>;
  } {
    // Parse function signature like "transfer(address,uint256)" or "transfer(address to, uint256 amount) returns (bool)"
    const match = signature.match(/^(\w+)\(([^)]*)\)(?:\s+returns\s+\(([^)]*)\))?/);
    
    if (!match) {
      throw new Error(`Invalid function signature: ${signature}`);
    }

    const name = match[1];
    const inputsStr = match[2];
    const outputsStr = match[3] || '';

    const inputs = this.parseParameters(inputsStr);
    const outputs = this.parseParameters(outputsStr);

    return { name, inputs, outputs };
  }

  private parseEventSignature(signature: string): {
    name: string;
    inputs: Array<{ name: string; type: string; indexed?: boolean }>;
  } {
    // Parse event signature like "Transfer(address indexed from, address indexed to, uint256 value)"
    const match = signature.match(/^(\w+)\(([^)]*)\)/);
    
    if (!match) {
      throw new Error(`Invalid event signature: ${signature}`);
    }

    const name = match[1];
    const inputsStr = match[2];
    
    const inputs = this.parseParameters(inputsStr).map(param => ({
      ...param,
      indexed: inputsStr.includes(`indexed ${param.name}`) || inputsStr.includes(`${param.type} indexed`)
    }));

    return { name, inputs };
  }

  private parseParameters(paramsStr: string): Array<{ name: string; type: string }> {
    if (!paramsStr.trim()) {
      return [];
    }

    return paramsStr.split(',').map((param, index) => {
      const trimmed = param.trim();
      const parts = trimmed.split(/\s+/).filter(p => p !== 'indexed');
      
      if (parts.length >= 2) {
        return { type: parts[0], name: parts[1] };
      } else if (parts.length === 1) {
        return { type: parts[0], name: `param${index}` };
      } else {
        throw new Error(`Invalid parameter: ${param}`);
      }
    });
  }

  private createMinimalABI(
    name: string,
    inputs: Array<{ name: string; type: string }>,
    outputs: Array<{ name: string; type: string }>
  ): any[] {
    return [{
      type: 'function',
      name,
      inputs,
      outputs,
      stateMutability: outputs.length > 0 ? 'view' : 'nonpayable'
    }];
  }

  private formatResult(result: any, outputs: Array<{ name: string; type: string }>): any {
    if (outputs.length === 0) {
      return null;
    } else if (outputs.length === 1) {
      return this.convertValue(result, outputs[0].type);
    } else {
      // Multiple return values
      const formatted: any = {};
      for (let i = 0; i < outputs.length && i < result.length; i++) {
        formatted[i] = this.convertValue(result[i], outputs[i].type);
        if (outputs[i].name) {
          formatted[outputs[i].name] = formatted[i];
        }
      }
      return formatted;
    }
  }

  private formatComplexResult(result: any): any {
    if (typeof result === 'bigint') {
      return result.toString();
    } else if (Array.isArray(result)) {
      return result.map(item => this.formatComplexResult(item));
    } else if (result && typeof result === 'object') {
      const formatted: any = {};
      for (const [key, value] of Object.entries(result)) {
        formatted[key] = this.formatComplexResult(value);
      }
      return formatted;
    }
    return result;
  }

  private convertValue(value: any, type: string): any {
    if (type.startsWith('uint') || type.startsWith('int')) {
      return typeof value === 'bigint' ? value.toString() : value;
    }
    return value;
  }

  private async estimateGas(
    contractAddress: string,
    functionSignature: string,
    args: any[],
    options: CastCallOptions
  ): Promise<string> {
    try {
      const { name, inputs } = this.parseFunctionSignature(functionSignature);
      const abi = this.createMinimalABI(name, inputs, []);
      const contract = new ethers.Contract(contractAddress, abi, this.provider);
      
      const gasEstimate = await contract[name].estimateGas(...args);
      return gasEstimate.toString();
    } catch (error) {
      return '0';
    }
  }
}