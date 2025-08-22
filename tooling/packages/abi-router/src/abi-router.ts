import { 
  Interface, 
  FunctionFragment, 
  EventFragment,
  Result,
  LogDescription
} from "ethers";
import { 
  NeoRpcProvider,
  InvokeResult,
  ContractMethod,
  ContractEvent
} from "@neo-solidity/types";
import { RpcAdapter } from "./rpc-adapter";
import { TransactionBuilder } from "./transaction-builder";
import { EventDecoder } from "./event-decoder";
import { ContractWrapper } from "./contract-wrapper";
import Debug from "debug";

const debug = Debug("neo-solidity:abi-router");

/**
 * ABI Router - Provides Ethereum-compatible interface for Neo contracts
 */
export class AbiRouter {
  private rpcAdapter: RpcAdapter;
  private transactionBuilder: TransactionBuilder;
  private eventDecoder: EventDecoder;

  constructor(rpcProvider: NeoRpcProvider) {
    this.rpcAdapter = new RpcAdapter(rpcProvider);
    this.transactionBuilder = new TransactionBuilder(rpcProvider);
    this.eventDecoder = new EventDecoder();
  }

  /**
   * Create contract wrapper with Ethereum-compatible interface
   */
  createContract(
    address: string,
    abi: any[],
    signerOrProvider?: any
  ): ContractWrapper {
    debug(`Creating contract wrapper for ${address}`);
    
    return new ContractWrapper(
      address,
      abi,
      this.rpcAdapter,
      this.transactionBuilder,
      this.eventDecoder,
      signerOrProvider
    );
  }

  /**
   * Get contract at address with ABI
   */
  getContract(
    address: string,
    abi: any[],
    signerOrProvider?: any
  ): ContractWrapper {
    return this.createContract(address, abi, signerOrProvider);
  }

  /**
   * Deploy contract with Ethereum-compatible interface
   */
  async deployContract(
    bytecode: string,
    abi: any[],
    constructorArgs: any[] = [],
    options: {
      gasLimit?: string;
      gasPrice?: string;
      value?: string;
    } = {}
  ): Promise<ContractWrapper> {
    debug("Deploying contract with ABI compatibility");

    try {
      // Deploy contract using transaction builder
      const deployment = await this.transactionBuilder.deployContract(
        bytecode,
        constructorArgs,
        options
      );

      // Create and return contract wrapper
      return this.createContract(deployment.address, abi);
    } catch (error) {
      debug(`Contract deployment failed: ${error}`);
      throw error;
    }
  }

  /**
   * Encode function call data
   */
  encodeFunctionData(
    functionFragment: string | FunctionFragment,
    args: any[] = []
  ): string {
    debug(`Encoding function data for ${functionFragment}`);

    const iface = new Interface([
      typeof functionFragment === "string" 
        ? `function ${functionFragment}` 
        : functionFragment
    ]);

    const fragment = typeof functionFragment === "string"
      ? iface.getFunction(functionFragment.split('(')[0])!
      : functionFragment;

    return iface.encodeFunctionData(fragment, args);
  }

  /**
   * Decode function result data
   */
  decodeFunctionResult(
    functionFragment: string | FunctionFragment,
    data: string
  ): Result {
    debug(`Decoding function result for ${functionFragment}`);

    const iface = new Interface([
      typeof functionFragment === "string" 
        ? `function ${functionFragment}` 
        : functionFragment
    ]);

    const fragment = typeof functionFragment === "string"
      ? iface.getFunction(functionFragment.split('(')[0])!
      : functionFragment;

    return iface.decodeFunctionResult(fragment, data);
  }

  /**
   * Encode event filter topics
   */
  encodeFilterTopics(
    eventFragment: string | EventFragment,
    values: any[] = []
  ): string[] {
    debug(`Encoding filter topics for ${eventFragment}`);

    const iface = new Interface([
      typeof eventFragment === "string" 
        ? `event ${eventFragment}` 
        : eventFragment
    ]);

    const fragment = typeof eventFragment === "string"
      ? iface.getEvent(eventFragment.split('(')[0])!
      : eventFragment;

    return iface.encodeFilterTopics(fragment, values);
  }

  /**
   * Decode event log
   */
  decodeEventLog(
    eventFragment: string | EventFragment,
    data: string,
    topics: string[]
  ): LogDescription {
    debug(`Decoding event log for ${eventFragment}`);

    const iface = new Interface([
      typeof eventFragment === "string" 
        ? `event ${eventFragment}` 
        : eventFragment
    ]);

    return iface.parseLog({ data, topics })!;
  }

  /**
   * Convert Neo stack result to Ethereum format
   */
  convertNeoResultToEthereum(
    neoResult: InvokeResult,
    returnTypes: string[]
  ): any {
    debug("Converting Neo result to Ethereum format");

    if (!neoResult.stack || neoResult.stack.length === 0) {
      return null;
    }

    // Single return value
    if (returnTypes.length === 1) {
      return this.convertStackItem(neoResult.stack[0], returnTypes[0]);
    }

    // Multiple return values
    const result: any = {};
    for (let i = 0; i < Math.min(neoResult.stack.length, returnTypes.length); i++) {
      result[i] = this.convertStackItem(neoResult.stack[i], returnTypes[i]);
    }

    return result;
  }

  /**
   * Convert Ethereum arguments to Neo format
   */
  convertEthereumArgsToNeo(args: any[], paramTypes: string[]): any[] {
    debug("Converting Ethereum args to Neo format");

    const neoArgs: any[] = [];

    for (let i = 0; i < args.length; i++) {
      const arg = args[i];
      const paramType = paramTypes[i] || "string";
      
      neoArgs.push(this.convertEthereumArgToNeo(arg, paramType));
    }

    return neoArgs;
  }

  /**
   * Get function selector from signature
   */
  getFunctionSelector(signature: string): string {
    const iface = new Interface([`function ${signature}`]);
    const functionName = signature.split('(')[0];
    const fragment = iface.getFunction(functionName)!;
    return iface.getFunction(fragment.name)!.selector;
  }

  /**
   * Get event topic hash from signature
   */
  getEventTopicHash(signature: string): string {
    const iface = new Interface([`event ${signature}`]);
    const eventName = signature.split('(')[0];
    const fragment = iface.getEvent(eventName)!;
    return iface.getEvent(fragment.name)!.topicHash;
  }

  // Private methods

  private convertStackItem(item: any, targetType: string): any {
    if (!item) return null;

    switch (item.type) {
      case 'Boolean':
        return targetType === 'bool' ? item.value : Boolean(item.value);

      case 'Integer':
        if (targetType.startsWith('uint') || targetType.startsWith('int')) {
          return BigInt(item.value);
        }
        return Number(item.value);

      case 'ByteString':
        if (targetType === 'address') {
          return this.bytesToAddress(item.value);
        }
        if (targetType === 'string') {
          return this.bytesToString(item.value);
        }
        if (targetType.startsWith('bytes')) {
          return '0x' + item.value;
        }
        return item.value;

      case 'Array':
        return item.value?.map((subItem: any, index: number) => 
          this.convertStackItem(subItem, this.getArrayElementType(targetType))
        ) || [];

      case 'Map':
        // Convert Neo map to object
        const obj: any = {};
        for (const [key, value] of Object.entries(item.value || {})) {
          obj[key] = this.convertStackItem(value, 'string');
        }
        return obj;

      default:
        return item.value;
    }
  }

  private convertEthereumArgToNeo(arg: any, paramType: string): any {
    switch (paramType) {
      case 'bool':
        return {
          type: 'Boolean',
          value: Boolean(arg)
        };

      case 'address':
        return {
          type: 'Hash160',
          value: this.addressToScriptHash(arg)
        };

      case 'string':
        return {
          type: 'String',
          value: String(arg)
        };

      default:
        if (paramType.startsWith('uint') || paramType.startsWith('int')) {
          return {
            type: 'Integer',
            value: String(arg)
          };
        }
        
        if (paramType.startsWith('bytes')) {
          return {
            type: 'ByteArray',
            value: arg.startsWith('0x') ? arg.slice(2) : arg
          };
        }

        // Default to string representation
        return {
          type: 'String',
          value: String(arg)
        };
    }
  }

  private getArrayElementType(arrayType: string): string {
    // Extract element type from array type (e.g., "uint256[]" -> "uint256")
    return arrayType.replace(/\[\]$/, '');
  }

  private bytesToAddress(bytes: string): string {
    // Convert Neo script hash to address format
    // This is a simplified conversion - actual implementation would use Neo address encoding
    return "0x" + bytes.slice(-40);
  }

  private bytesToString(bytes: string): string {
    try {
      return Buffer.from(bytes, 'hex').toString('utf8');
    } catch {
      return bytes;
    }
  }

  private addressToScriptHash(address: string): string {
    // Convert Ethereum address to Neo script hash
    // This is a simplified conversion - actual implementation would handle proper conversion
    if (address.startsWith('0x')) {
      return address;
    }
    return '0x' + address.slice(-40);
  }
}