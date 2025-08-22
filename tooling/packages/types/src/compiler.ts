import { BigNumber } from 'ethers';

/**
 * Configuration for the Neo-Solidity compiler
 */
export interface NeoSolidityConfig {
  /** Compiler version to use */
  version?: string;
  /** Optimization settings */
  optimizer?: {
    enabled: boolean;
    runs: number;
  };
  /** Output selection for compilation artifacts */
  outputSelection?: {
    [file: string]: {
      [contract: string]: string[];
    };
  };
  /** Libraries to link */
  libraries?: {
    [libraryName: string]: string;
  };
  /** Metadata settings */
  metadata?: {
    useLiteralContent?: boolean;
    bytecodeHash?: 'none' | 'ipfs' | 'bzzr1';
  };
  /** Neo-specific settings */
  neo?: {
    /** Gas cost model configuration */
    gasCostModel?: 'ethereum' | 'neo' | 'hybrid';
    /** Storage cost optimization */
    storageOptimization?: boolean;
    /** Event optimization for Neo notifications */
    eventOptimization?: boolean;
  };
}

/**
 * Compilation input format
 */
export interface CompilationInput {
  language: 'Solidity';
  sources: {
    [fileName: string]: {
      content?: string;
      keccak256?: string;
      urls?: string[];
    };
  };
  settings: NeoSolidityConfig;
}

/**
 * Compilation output format
 */
export interface CompilationOutput {
  errors?: CompilationError[];
  sources: {
    [fileName: string]: {
      id: number;
      ast?: any;
    };
  };
  contracts: {
    [fileName: string]: {
      [contractName: string]: CompiledContract;
    };
  };
}

/**
 * Individual compiled contract
 */
export interface CompiledContract {
  abi: any[];
  metadata: string;
  userdoc?: any;
  devdoc?: any;
  ir?: string;
  irOptimized?: string;
  storageLayout?: StorageLayout;
  evm: {
    assembly?: string;
    legacyAssembly?: any;
    bytecode: Bytecode;
    deployedBytecode?: Bytecode;
    methodIdentifiers?: {
      [signature: string]: string;
    };
    gasEstimates?: GasEstimates;
  };
  /** Neo-specific compilation outputs */
  neo: {
    /** Neo VM bytecode */
    nef: {
      magic: number;
      compiler: string;
      source: string;
      tokens: any[];
      script: string;
      checksum: number;
    };
    /** Neo manifest */
    manifest: {
      name: string;
      groups: any[];
      features: any;
      supportedstandards: string[];
      abi: NeoAbi;
      permissions: any[];
      trusts: any[];
      extra: any;
    };
    /** Storage layout for Neo storage */
    storageMap: {
      [key: string]: {
        slot: number;
        type: string;
        description: string;
      };
    };
    /** Gas cost estimates for Neo */
    gasEstimates: {
      creation: NeoGasEstimate;
      functions: {
        [methodName: string]: NeoGasEstimate;
      };
    };
  };
}

/**
 * Neo-specific ABI format
 */
export interface NeoAbi {
  methods: NeoMethod[];
  events: NeoEvent[];
}

export interface NeoMethod {
  name: string;
  offset: number;
  parameters: NeoParameter[];
  returntype: string;
  safe: boolean;
}

export interface NeoEvent {
  name: string;
  parameters: NeoParameter[];
}

export interface NeoParameter {
  name: string;
  type: string;
}

/**
 * Neo-specific gas estimation
 */
export interface NeoGasEstimate {
  gas: BigNumber;
  systemFee: BigNumber;
  networkFee: BigNumber;
}

/**
 * Bytecode information
 */
export interface Bytecode {
  object: string;
  opcodes?: string;
  sourceMap?: string;
  linkReferences?: {
    [fileName: string]: {
      [libraryName: string]: Array<{
        start: number;
        length: number;
      }>;
    };
  };
  generatedSources?: GeneratedSource[];
}

/**
 * Generated source information
 */
export interface GeneratedSource {
  ast: any;
  contents: string;
  id: number;
  language: string;
  name: string;
}

/**
 * Storage layout information
 */
export interface StorageLayout {
  storage: StorageItem[];
  types: {
    [typeName: string]: StorageType;
  };
}

export interface StorageItem {
  astId: number;
  contract: string;
  label: string;
  offset: number;
  slot: string;
  type: string;
}

export interface StorageType {
  encoding: string;
  label: string;
  numberOfBytes: string;
  base?: string;
  key?: string;
  value?: string;
  members?: StorageItem[];
}

/**
 * Gas estimation information
 */
export interface GasEstimates {
  creation?: {
    codeDepositCost: string;
    executionCost: string;
    totalCost: string;
  };
  external?: {
    [methodName: string]: string;
  };
  internal?: {
    [methodName: string]: string;
  };
}

/**
 * Compilation error
 */
export interface CompilationError {
  sourceLocation?: {
    file: string;
    start: number;
    end: number;
  };
  secondarySourceLocations?: Array<{
    file: string;
    start: number;
    end: number;
    message: string;
  }>;
  type: 'TypeError' | 'ParserError' | 'Warning' | 'Info';
  component: string;
  severity: 'error' | 'warning' | 'info';
  message: string;
  formattedMessage?: string;
}

/**
 * Compiler options for CLI tools
 */
export interface CompilerOptions {
  /** Input files or directories */
  input: string[];
  /** Output directory */
  output?: string;
  /** Compiler configuration */
  config?: NeoSolidityConfig;
  /** Include remappings */
  remappings?: string[];
  /** Base path for imports */
  basePath?: string;
  /** Include paths for imports */
  includePaths?: string[];
  /** Allow paths for imports */
  allowPaths?: string[];
  /** Overwrite output files */
  overwrite?: boolean;
  /** Verbose output */
  verbose?: boolean;
  /** Emit only specific outputs */
  emit?: string[];
}