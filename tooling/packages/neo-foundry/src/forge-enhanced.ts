import {
  FoundryConfig,
  ForgeProject,
  ForgeBuildOptions,
  ForgeBuildResult,
  ForgeTestOptions,
  ForgeTestResult,
  CompilationMessage,
  GasUsageReport,
  CoverageReport,
  ScriptOptions,
  ScriptResult,
  FlattenOptions,
  FlattenResult
} from '@neo-solidity/types';
import { EventEmitter } from 'events';
import * as fs from 'fs-extra';
import * as path from 'path';
import { spawn, ChildProcess } from 'child_process';
import glob from 'glob';
import semver from 'semver';

export class NeoForge extends EventEmitter {
  private config: FoundryConfig;
  private projectRoot: string;
  private processes: Map<string, ChildProcess> = new Map();

  constructor(config: FoundryConfig, projectRoot: string = process.cwd()) {
    super();
    this.config = config;
    this.projectRoot = projectRoot;
  }

  // Enhanced Build System
  async build(options: ForgeBuildOptions = {}): Promise<ForgeBuildResult> {
    const startTime = Date.now();
    this.emit('buildStarted', { options });

    try {
      // Prepare build environment
      await this.prepareBuildEnvironment(options);
      
      // Compile contracts
      const compilationResult = await this.compileContracts(options);
      
      // Post-process artifacts
      await this.postProcessArtifacts(compilationResult, options);
      
      const result: ForgeBuildResult = {
        success: true,
        contracts: compilationResult.contracts,
        compilation_time: Date.now() - startTime,
        compiler_version: options.compiler_version || this.config.solc_version,
        errors: compilationResult.errors,
        warnings: compilationResult.warnings
      };

      this.emit('buildCompleted', result);
      return result;
    } catch (error) {
      const result: ForgeBuildResult = {
        success: false,
        contracts: {},
        compilation_time: Date.now() - startTime,
        compiler_version: options.compiler_version || this.config.solc_version,
        errors: [{ 
          severity: 'error' as const, 
          type: 'build_error',
          component: 'neo-forge',
          message: String(error),
          formattedMessage: `Build failed: ${error}`
        }],
        warnings: []
      };

      this.emit('buildFailed', { error, result });
      return result;
    }
  }

  // Advanced Test System
  async test(options: ForgeTestOptions = {}): Promise<ForgeTestResult> {
    const startTime = Date.now();
    this.emit('testStarted', { options });

    try {
      // Build contracts first
      await this.build({
        root: options.root,
        cache: !options.json // Skip cache for JSON output
      });

      // Discover test files
      const testFiles = await this.discoverTestFiles(options);
      
      // Run tests
      const testResults = await this.runTests(testFiles, options);
      
      // Generate reports
      const result: ForgeTestResult = {
        success: testResults.every(test => test.success),
        test_results: this.organizeTestResults(testResults),
        gas_report: options.gas_report ? await this.generateGasReport(testResults) : undefined,
        coverage: options.coverage ? await this.generateCoverage(testResults) : undefined,
        summary: {
          total_tests: testResults.length,
          passed: testResults.filter(t => t.success).length,
          failed: testResults.filter(t => !t.success).length,
          skipped: 0, // Would be implemented with test filtering
          duration: Date.now() - startTime,
          gas_used: testResults.reduce((sum, test) => sum + test.gas_used, 0)
        }
      };

      this.emit('testCompleted', result);
      return result;
    } catch (error) {
      const result: ForgeTestResult = {
        success: false,
        test_results: {},
        summary: {
          total_tests: 0,
          passed: 0,
          failed: 1,
          skipped: 0,
          duration: Date.now() - startTime,
          gas_used: 0
        }
      };

      this.emit('testFailed', { error, result });
      return result;
    }
  }

  // Script Execution System
  async script(scriptPath: string, options: ScriptOptions = {}): Promise<ScriptResult> {
    this.emit('scriptStarted', { scriptPath, options });

    try {
      // Load and validate script
      const script = await this.loadScript(scriptPath);
      
      // Execute script
      const transactions = await this.executeScript(script, options);
      
      // Get receipts if broadcasting
      const receipts = options.broadcast 
        ? await this.getTransactionReceipts(transactions)
        : [];

      const result: ScriptResult = {
        success: true,
        transactions,
        receipts,
        gas_used: transactions.reduce((sum, tx) => sum + parseInt(tx.gas), 0).toString(),
        logs: []
      };

      this.emit('scriptCompleted', result);
      return result;
    } catch (error) {
      const result: ScriptResult = {
        success: false,
        transactions: [],
        receipts: [],
        gas_used: '0',
        logs: [],
        error: {
          message: String(error),
          stack: error instanceof Error ? error.stack : undefined
        }
      };

      this.emit('scriptFailed', { error, result });
      return result;
    }
  }

  // Contract Verification
  async verify(contractAddress: string, contractPath: string, options: any = {}): Promise<boolean> {
    this.emit('verificationStarted', { contractAddress, contractPath });

    try {
      // Load contract source and ABI
      const { source, abi, bytecode } = await this.loadContractData(contractPath);
      
      // Prepare verification data
      const verificationData = {
        address: contractAddress,
        source,
        abi,
        bytecode,
        compiler_version: options.compiler_version || this.config.solc_version,
        optimization: this.config.optimizer,
        constructor_args: options.constructor_args || ''
      };

      // Submit to verifier (this would integrate with block explorer APIs)
      const success = await this.submitVerification(verificationData);
      
      this.emit('verificationCompleted', { success, contractAddress });
      return success;
    } catch (error) {
      this.emit('verificationFailed', { error, contractAddress });
      return false;
    }
  }

  // Code Flattening
  async flatten(contractPath: string, options: FlattenOptions = {}): Promise<FlattenResult> {
    this.emit('flattenStarted', { contractPath, options });

    try {
      const result = await this.flattenContract(contractPath, options);
      this.emit('flattenCompleted', result);
      return result;
    } catch (error) {
      const result: FlattenResult = {
        success: false,
        flattened_source: '',
        contracts_found: [],
        imports_resolved: [],
        errors: [String(error)],
        warnings: []
      };

      this.emit('flattenFailed', { error, result });
      return result;
    }
  }

  // Advanced Features

  async watch(options: ForgeBuildOptions = {}): Promise<void> {
    const watchPaths = [
      path.join(this.projectRoot, this.config.src),
      path.join(this.projectRoot, 'test'),
      path.join(this.projectRoot, 'script')
    ];

    console.log('👀 Watching for changes...');
    
    // This would implement file watching and auto-rebuild
    // Using fs.watch or chokidar for better cross-platform support
  }

  async clean(): Promise<void> {
    const cachePath = path.join(this.projectRoot, 'cache');
    const outPath = path.join(this.projectRoot, this.config.out);
    
    await Promise.all([
      fs.remove(cachePath).catch(() => {}),
      fs.remove(outPath).catch(() => {})
    ]);

    console.log('🧹 Cleaned cache and artifacts');
  }

  async inspect(contractName: string): Promise<{
    abi: any[];
    bytecode: string;
    metadata: any;
    storageLayout: any;
    gasEstimates: any;
  }> {
    const artifact = await this.loadArtifact(contractName);
    
    return {
      abi: artifact.abi,
      bytecode: artifact.bytecode.object,
      metadata: JSON.parse(artifact.metadata),
      storageLayout: artifact.storageLayout,
      gasEstimates: artifact.gasEstimates
    };
  }

  async tree(options: { no_dedupe?: boolean } = {}): Promise<string[]> {
    // Generate dependency tree for contracts
    const contracts = await this.discoverContracts();
    const dependencies: string[] = [];
    
    for (const contract of contracts) {
      const deps = await this.analyzeDependencies(contract);
      dependencies.push(...deps);
    }
    
    return options.no_dedupe ? dependencies : [...new Set(dependencies)];
  }

  // Private Implementation Methods

  private async prepareBuildEnvironment(options: ForgeBuildOptions): Promise<void> {
    // Create output directories
    const outPath = path.join(this.projectRoot, options.out || this.config.out);
    await fs.ensureDir(outPath);
    
    // Clean if force rebuild
    if (options.force) {
      await fs.remove(outPath);
      await fs.ensureDir(outPath);
    }
  }

  private async compileContracts(options: ForgeBuildOptions): Promise<{
    contracts: any;
    errors: CompilationMessage[];
    warnings: CompilationMessage[];
  }> {
    const contracts: any = {};
    const errors: CompilationMessage[] = [];
    const warnings: CompilationMessage[] = [];

    // Discover source files
    const sourceFiles = await this.discoverContracts(options.root);
    
    for (const sourceFile of sourceFiles) {
      try {
        const compiled = await this.compileSingleContract(sourceFile, options);
        Object.assign(contracts, compiled.contracts);
        errors.push(...compiled.errors);
        warnings.push(...compiled.warnings);
      } catch (error) {
        errors.push({
          severity: 'error',
          type: 'compilation_error',
          component: 'compiler',
          message: String(error),
          formattedMessage: `Compilation failed for ${sourceFile}: ${error}`
        });
      }
    }

    return { contracts, errors, warnings };
  }

  private async compileSingleContract(sourceFile: string, options: ForgeBuildOptions): Promise<{
    contracts: any;
    errors: CompilationMessage[];
    warnings: CompilationMessage[];
  }> {
    // This would integrate with the Neo-Solidity compiler
    // For now, return placeholder data
    const contractName = path.basename(sourceFile, '.sol');
    
    return {
      contracts: {
        [sourceFile]: {
          [contractName]: {
            abi: [],
            bytecode: {
              object: '0x',
              sourceMap: '',
              linkReferences: {}
            },
            deployedBytecode: {
              object: '0x',
              sourceMap: '',
              linkReferences: {}
            },
            metadata: '{}',
            storageLayout: {
              storage: [],
              types: {}
            }
          }
        }
      },
      errors: [],
      warnings: []
    };
  }

  private async postProcessArtifacts(compilationResult: any, options: ForgeBuildOptions): Promise<void> {
    // Generate additional artifacts (ABIs, bindings, etc.)
    if (options.names) {
      await this.generateContractNames(compilationResult);
    }
    
    if (options.sizes) {
      await this.generateContractSizes(compilationResult);
    }
  }

  private async discoverTestFiles(options: ForgeTestOptions): Promise<string[]> {
    const testDir = path.join(this.projectRoot, 'test');
    const pattern = '**/*.sol';
    
    return new Promise((resolve, reject) => {
      glob(pattern, { cwd: testDir }, (err, files) => {
        if (err) reject(err);
        else resolve(files.map(f => path.join(testDir, f)));
      });
    });
  }

  private async runTests(testFiles: string[], options: ForgeTestOptions): Promise<any[]> {
    // This would execute test contracts using the Neo-Solidity test runner
    return [];
  }

  private organizeTestResults(testResults: any[]): { [testSuite: string]: any } {
    const organized: { [testSuite: string]: any } = {};
    
    for (const result of testResults) {
      const suiteName = result.contract || 'Unknown';
      if (!organized[suiteName]) {
        organized[suiteName] = {};
      }
      organized[suiteName][result.test] = result;
    }
    
    return organized;
  }

  private async generateGasReport(testResults: any[]): Promise<GasUsageReport> {
    return {
      contracts: {}
    };
  }

  private async generateCoverage(testResults: any[]): Promise<CoverageReport> {
    return {
      files: {},
      summary: {
        lines: { total: 0, covered: 0, skipped: 0, pct: 0 },
        functions: { total: 0, covered: 0, skipped: 0, pct: 0 },
        branches: { total: 0, covered: 0, skipped: 0, pct: 0 },
        statements: { total: 0, covered: 0, skipped: 0, pct: 0 }
      }
    };
  }

  private async loadScript(scriptPath: string): Promise<any> {
    const fullPath = path.resolve(this.projectRoot, scriptPath);
    
    if (!await fs.pathExists(fullPath)) {
      throw new Error(`Script not found: ${scriptPath}`);
    }
    
    return fs.readFile(fullPath, 'utf8');
  }

  private async executeScript(script: string, options: ScriptOptions): Promise<any[]> {
    // This would execute Solidity scripts using the Neo-Solidity runtime
    return [];
  }

  private async getTransactionReceipts(transactions: any[]): Promise<any[]> {
    // Get transaction receipts from the network
    return [];
  }

  private async loadContractData(contractPath: string): Promise<{
    source: string;
    abi: any[];
    bytecode: string;
  }> {
    // Load contract source, ABI, and bytecode
    const source = await fs.readFile(contractPath, 'utf8');
    
    return {
      source,
      abi: [],
      bytecode: '0x'
    };
  }

  private async submitVerification(verificationData: any): Promise<boolean> {
    // Submit to block explorer for verification
    // This would integrate with actual APIs
    return true;
  }

  private async flattenContract(contractPath: string, options: FlattenOptions): Promise<FlattenResult> {
    // Flatten contract by resolving all imports
    const source = await fs.readFile(contractPath, 'utf8');
    
    return {
      success: true,
      flattened_source: source,
      contracts_found: [path.basename(contractPath, '.sol')],
      imports_resolved: [],
      errors: [],
      warnings: []
    };
  }

  private async discoverContracts(root?: string): Promise<string[]> {
    const srcDir = path.join(root || this.projectRoot, this.config.src);
    const pattern = '**/*.sol';
    
    return new Promise((resolve, reject) => {
      glob(pattern, { cwd: srcDir }, (err, files) => {
        if (err) reject(err);
        else resolve(files.map(f => path.join(srcDir, f)));
      });
    });
  }

  private async compileSolcContract(sourceFile: string): Promise<any> {
    // Compile using solc
    return {};
  }

  private async loadArtifact(contractName: string): Promise<any> {
    const artifactPath = path.join(
      this.projectRoot, 
      this.config.out, 
      `${contractName}.sol`,
      `${contractName}.json`
    );
    
    return fs.readJson(artifactPath);
  }

  private async analyzeDependencies(contractPath: string): Promise<string[]> {
    // Analyze contract dependencies
    const source = await fs.readFile(contractPath, 'utf8');
    const imports = source.match(/import\s+[^;]+;/g) || [];
    
    return imports.map(imp => imp.replace(/import\s+["']([^"']+)["'][^;]*;/, '$1'));
  }

  private async generateContractNames(compilationResult: any): Promise<void> {
    // Generate contract names output
    console.log('📋 Contract Names:');
    for (const [file, contracts] of Object.entries(compilationResult.contracts)) {
      for (const contractName of Object.keys(contracts as any)) {
        console.log(`  ${contractName} (${file})`);
      }
    }
  }

  private async generateContractSizes(compilationResult: any): Promise<void> {
    // Generate contract sizes output
    console.log('📏 Contract Sizes:');
    for (const [file, contracts] of Object.entries(compilationResult.contracts)) {
      for (const [contractName, contract] of Object.entries(contracts as any)) {
        const bytecode = (contract as any).bytecode?.object || '0x';
        const sizeInBytes = (bytecode.length - 2) / 2;
        console.log(`  ${contractName}: ${sizeInBytes} bytes`);
      }
    }
  }

  // Process management
  private startProcess(command: string, args: string[], options: any = {}): ChildProcess {
    const proc = spawn(command, args, {
      cwd: this.projectRoot,
      stdio: options.silent ? 'pipe' : 'inherit',
      ...options
    });

    const processId = `${command}_${Date.now()}`;
    this.processes.set(processId, proc);

    proc.on('exit', () => {
      this.processes.delete(processId);
    });

    return proc;
  }

  async killAllProcesses(): Promise<void> {
    for (const [id, proc] of this.processes.entries()) {
      proc.kill();
      this.processes.delete(id);
    }
  }
}