import { NeoRpcProvider } from "@neo-solidity/types";
import Debug from "debug";

const debug = Debug("neo-solidity:transaction-builder");

/**
 * Transaction builder for Neo blockchain
 */
export class TransactionBuilder {
  private rpcProvider: NeoRpcProvider;

  constructor(rpcProvider: NeoRpcProvider) {
    this.rpcProvider = rpcProvider;
  }

  /**
   * Send transaction to contract method
   */
  async sendTransaction(
    contractAddress: string,
    methodName: string,
    args: any[] = [],
    options: {
      gasLimit?: string;
      gasPrice?: string;
      value?: string;
    } = {}
  ): Promise<{ hash: string }> {
    debug(`Building transaction for ${methodName} on ${contractAddress}`);

    try {
      // Convert Ethereum-style parameters to Neo format
      const scriptHash = this.addressToScriptHash(contractAddress);
      const neoParams = this.convertArgsToNeoParams(args);
      
      // Build Neo transaction
      const transaction = await this.buildNeoTransaction({
        scriptHash,
        operation: methodName,
        args: neoParams,
        gasLimit: options.gasLimit ? BigInt(options.gasLimit) : BigInt('30000000'),
        gasPrice: options.gasPrice ? BigInt(options.gasPrice) : BigInt('1000'),
        value: options.value ? BigInt(options.value) : BigInt('0')
      });
      
      // Send the transaction
      const result = await this.rpcProvider.sendTransaction(transaction);
      const txHash = result.hash;

      debug(`Transaction sent with hash: ${txHash}`);
      return { hash: txHash };
    } catch (error) {
      debug(`Transaction failed: ${error}`);
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
      gasLimit?: string;
      gasPrice?: string;
      value?: string;
    } = {}
  ): Promise<{ address: string; hash: string }> {
    debug("Building contract deployment transaction");

    try {
      // Deploy contract to Neo blockchain
      const deploymentTx = await this.buildDeploymentTransaction({
        nef: bytecode.nef,
        manifest: bytecode.manifest,
        gasLimit: options.gasLimit ? BigInt(options.gasLimit) : BigInt('50000000'),
        gasPrice: options.gasPrice ? BigInt(options.gasPrice) : BigInt('1000')
      });
      
      const result = await this.rpcProvider.sendTransaction(deploymentTx);
      const address = result.contractAddress || this.calculateContractAddress(result.hash);
      const hash = result.hash;

      debug(`Contract deployed at: ${address}, tx: ${hash}`);
      return { address, hash };
    } catch (error) {
      debug(`Contract deployment failed: ${error}`);
      throw error;
    }
  }

  /**
   * Convert Ethereum address to Neo script hash
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
   * Convert arguments to Neo parameter format
   */
  private convertArgsToNeoParams(args: any[]): any[] {
    return args.map(arg => {
      if (typeof arg === 'string') {
        return { type: 'String', value: arg };
      } else if (typeof arg === 'number' || typeof arg === 'bigint') {
        return { type: 'Integer', value: arg.toString() };
      } else if (typeof arg === 'boolean') {
        return { type: 'Boolean', value: arg };
      } else if (Array.isArray(arg)) {
        return { type: 'Array', value: this.convertArgsToNeoParams(arg) };
      } else {
        return { type: 'ByteString', value: Buffer.from(JSON.stringify(arg)).toString('hex') };
      }
    });
  }

  /**
   * Build Neo transaction structure
   */
  private async buildNeoTransaction(params: {
    scriptHash: string;
    operation: string;
    args: any[];
    gasLimit: bigint;
    gasPrice: bigint;
    value: bigint;
  }): Promise<any> {
    // Create Neo script for contract invocation
    const script = this.createInvocationScript(params.scriptHash, params.operation, params.args);
    
    return {
      version: 0,
      script,
      gas: params.gasLimit,
      systemFee: params.gasLimit * params.gasPrice,
      networkFee: BigInt('1000000'), // 0.01 GAS
      validUntilBlock: await this.getValidUntilBlock(),
      attributes: [],
      witnesses: []
    };
  }

  /**
   * Build deployment transaction
   */
  private async buildDeploymentTransaction(params: {
    nef: string;
    manifest: any;
    gasLimit: bigint;
    gasPrice: bigint;
  }): Promise<any> {
    // Create deployment script
    const script = this.createDeploymentScript(params.nef, params.manifest);
    
    return {
      version: 0,
      script,
      gas: params.gasLimit,
      systemFee: params.gasLimit * params.gasPrice,
      networkFee: BigInt('2000000'), // 0.02 GAS for deployment
      validUntilBlock: await this.getValidUntilBlock(),
      attributes: [],
      witnesses: []
    };
  }

  /**
   * Create script for contract invocation
   */
  private createInvocationScript(scriptHash: string, operation: string, args: any[]): string {
    // This would create the actual Neo VM script
    // Simplified version for demonstration
    const opCodes = {
      PUSH0: '0x10',
      PUSHDATA1: '0x0c',
      SYSCALL: '0x41',
      System_Contract_Call: '0x627d5b52'
    };

    // Build script bytes (simplified)
    let script = '';
    
    // Push arguments
    for (const arg of args.reverse()) {
      script += this.encodeArgument(arg);
    }
    
    // Push operation name
    script += opCodes.PUSHDATA1;
    script += operation.length.toString(16).padStart(2, '0');
    script += Buffer.from(operation, 'utf8').toString('hex');
    
    // Push script hash
    script += opCodes.PUSHDATA1;
    script += '14'; // 20 bytes
    script += scriptHash.replace('0x', '');
    
    // System call
    script += opCodes.SYSCALL;
    script += opCodes.System_Contract_Call;
    
    return '0x' + script;
  }

  /**
   * Create deployment script
   */
  private createDeploymentScript(nef: string, manifest: any): string {
    // Create script for contract deployment
    const manifestJson = JSON.stringify(manifest);
    const manifestBytes = Buffer.from(manifestJson, 'utf8').toString('hex');
    
    let script = '';
    
    // Push manifest
    script += '0x0c'; // PUSHDATA1
    script += manifestBytes.length.toString(16).padStart(4, '0');
    script += manifestBytes;
    
    // Push NEF
    script += '0x0c'; // PUSHDATA1
    script += nef.length.toString(16).padStart(4, '0');
    script += nef.replace('0x', '');
    
    // System call for deployment
    script += '0x41'; // SYSCALL
    script += '0x627d5b52'; // System.Contract.Create
    
    return '0x' + script;
  }

  /**
   * Encode argument for Neo script
   */
  private encodeArgument(arg: any): string {
    switch (arg.type) {
      case 'String':
        const stringBytes = Buffer.from(arg.value, 'utf8').toString('hex');
        return '0x0c' + stringBytes.length.toString(16).padStart(2, '0') + stringBytes;
      
      case 'Integer':
        const intValue = BigInt(arg.value);
        if (intValue >= 0 && intValue <= 16) {
          // Small positive integers
          return '0x' + (0x10 + Number(intValue)).toString(16);
        } else {
          // Larger integers
          const bytes = this.bigIntToBytes(intValue);
          return '0x0c' + bytes.length.toString(16).padStart(2, '0') + bytes;
        }
      
      case 'Boolean':
        return arg.value ? '0x11' : '0x10'; // PUSH1 or PUSH0
      
      case 'Array':
        let arrayScript = '';
        for (const item of arg.value) {
          arrayScript += this.encodeArgument(item);
        }
        arrayScript += '0x' + (0x10 + arg.value.length).toString(16); // PUSH length
        arrayScript += '0xc0'; // PACK
        return arrayScript;
      
      default:
        // ByteString
        const hexBytes = arg.value;
        return '0x0c' + hexBytes.length.toString(16).padStart(2, '0') + hexBytes;
    }
  }

  /**
   * Convert BigInt to byte array
   */
  private bigIntToBytes(value: bigint): string {
    if (value === 0n) return '00';
    
    let hex = value.toString(16);
    if (hex.length % 2) hex = '0' + hex;
    
    return hex;
  }

  /**
   * Get valid until block
   */
  private async getValidUntilBlock(): Promise<number> {
    try {
      const currentBlock = await this.rpcProvider.getBlockCount();
      return currentBlock + 86400; // Valid for ~24 hours (assuming 15s blocks)
    } catch (error) {
      debug(`Failed to get current block: ${error}`);
      return 999999999; // Large number as fallback
    }
  }

  /**
   * Calculate contract address from deployment transaction
   */
  private calculateContractAddress(txHash: string): string {
    // Contract address is derived from the deployment transaction hash
    const crypto = require('crypto');
    const hash = crypto.createHash('ripemd160')
      .update(crypto.createHash('sha256').update(Buffer.from(txHash.slice(2), 'hex')).digest())
      .digest();
    
    // Convert to Neo address format
    const versionByte = Buffer.from([0x35]); // N3 version
    const addressPayload = Buffer.concat([versionByte, hash]);
    
    // Calculate checksum
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
    
    const leadingOnes = address.match(/^1*/)?.[0]?.length || 0;
    const leadingZeros = Buffer.alloc(leadingOnes);
    
    return Buffer.concat([leadingZeros, buffer]);
  }
}