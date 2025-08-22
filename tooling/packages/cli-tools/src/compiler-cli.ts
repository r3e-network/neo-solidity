import { spawn } from "child_process";
import { promises as fs } from "fs";
import path from "path";
import { glob } from "glob";
import { 
  CompilationInput,
  CompilationOutput,
  CompilerOptions,
  NeoSolidityConfig
} from "@neo-solidity/types";
import chalk from "chalk";
import Debug from "debug";

const debug = Debug("neo-solidity:cli:compiler");

/**
 * Compiler CLI implementation
 */
export class CompilerCLI {

  /**
   * Compile Solidity files
   */
  async compile(
    files: string[],
    options: {
      output?: string;
      optimize?: boolean;
      optimizeRuns?: string;
      gasModel?: string;
      storageOpt?: boolean;
      eventOpt?: boolean;
      includePaths?: string;
      libraries?: string;
      metadata?: boolean;
      combinedJson?: string;
      standardJson?: boolean;
      verbose?: boolean;
      quiet?: boolean;
    }
  ): Promise<{
    contractCount: number;
    duration: number;
    outputDir: string;
    warnings: number;
  }> {
    const startTime = Date.now();
    
    debug("Starting compilation with options:", options);

    try {
      // Resolve input files
      const sourceFiles = await this.resolveSourceFiles(files);
      
      if (sourceFiles.length === 0) {
        throw new Error("No Solidity files found");
      }

      if (options.verbose) {
        console.log(chalk.blue(`Found ${sourceFiles.length} source files`));
        sourceFiles.forEach(file => console.log(chalk.gray(`  ${file}`)));
      }

      // Prepare compilation configuration
      const config = this.buildCompilerConfig(options);
      
      // Read source files
      const sources = await this.readSourceFiles(sourceFiles);

      // Execute compilation
      const output = await this.executeCompilation(sources, config);

      // Save artifacts
      const outputDir = options.output || "build";
      await this.saveArtifacts(output, outputDir, options);

      const duration = Date.now() - startTime;
      const contractCount = this.countContracts(output);
      const warnings = this.countWarnings(output);

      return {
        contractCount,
        duration,
        outputDir,
        warnings
      };
    } catch (error) {
      debug("Compilation failed:", error);
      throw error;
    }
  }

  /**
   * Get compiler version information
   */
  async getVersion(): Promise<{
    compiler: string;
    solidity: string;
    neovm: string;
  }> {
    try {
      const compilerPath = this.getCompilerPath();
      
      return new Promise((resolve, reject) => {
        const child = spawn(compilerPath, ["--version"], {
          stdio: ["pipe", "pipe", "pipe"]
        });

        let stdout = "";
        child.stdout.on("data", (data) => {
          stdout += data.toString();
        });

        child.on("close", (code) => {
          if (code === 0) {
            const version = this.parseVersionOutput(stdout);
            resolve(version);
          } else {
            reject(new Error("Failed to get version information"));
          }
        });

        child.on("error", reject);
      });
    } catch (error) {
      throw new Error("Neo-Solidity compiler not found");
    }
  }

  /**
   * Install specific compiler version
   */
  async install(version?: string): Promise<void> {
    debug(`Installing compiler version: ${version || 'latest'}`);
    
    if (!version || version === 'latest') {
      version = await this.getLatestVersion();
    }

    if (!this.isValidVersion(version)) {
      throw new Error(`Invalid version format: ${version}`);
    }

    const compilerDir = path.join(process.cwd(), '.neo-solidity', 'compilers');
    const compilerPath = path.join(compilerDir, `neo-solc-${version}`);
    
    // Check if already installed
    try {
      await fs.access(compilerPath);
      debug(`Compiler version ${version} already installed`);
      return;
    } catch {
      // Not installed, continue with installation
    }

    await fs.mkdir(compilerDir, { recursive: true });
    
    try {
      const downloadUrl = this.getDownloadUrl(version);
      await this.downloadCompiler(downloadUrl, compilerPath);
      await fs.chmod(compilerPath, 0o755);
      debug(`Successfully installed compiler version ${version}`);
    } catch (error) {
      throw new Error(`Failed to install compiler version ${version}: ${error}`);
    }
  }

  /**
   * List available compiler versions
   */
  async listVersions(): Promise<Array<{
    version: string;
    current: boolean;
    prerelease: boolean;
  }>> {
    try {
      const response = await fetch('https://api.github.com/repos/neo-project/neo-solidity/releases');
      if (!response.ok) {
        throw new Error(`GitHub API request failed: ${response.statusText}`);
      }
      
      const releases = await response.json();
      const currentVersion = await this.getCurrentVersion();
      
      return releases.map((release: any) => ({
        version: release.tag_name.replace(/^v/, ''),
        current: release.tag_name.replace(/^v/, '') === currentVersion,
        prerelease: release.prerelease
      })).sort((a: any, b: any) => {
        // Sort by version number (newest first)
        return this.compareVersions(b.version, a.version);
      });
    } catch (error) {
      debug('Failed to fetch versions from GitHub, falling back to local versions');
      return await this.getLocalVersions();
    }
  }

  /**
   * Standard JSON compilation
   */
  async standardJson(inputFile?: string, outputFile?: string): Promise<void> {
    debug("Standard JSON compilation");

    try {
      // Read input
      let input: string;
      if (inputFile) {
        input = await fs.readFile(inputFile, "utf-8");
      } else {
        // Read from stdin
        input = await this.readStdin();
      }

      const compilationInput: CompilationInput = JSON.parse(input);
      
      // Execute compilation
      const output = await this.executeStandardJsonCompilation(compilationInput);

      // Write output
      const outputJson = JSON.stringify(output, null, 2);
      if (outputFile) {
        await fs.writeFile(outputFile, outputJson);
      } else {
        console.log(outputJson);
      }
    } catch (error) {
      throw new Error(`Standard JSON compilation failed: ${error}`);
    }
  }

  /**
   * Analyze contracts for optimization opportunities
   */
  async analyze(
    files: string[],
    options: {
      gasReport?: boolean;
      sizeReport?: boolean;
      output?: string;
    }
  ): Promise<{
    gasAnalysis?: any[];
    sizeAnalysis?: any[];
    recommendations: string[];
  }> {
    debug("Analyzing contracts for optimization");

    const sourceFiles = await this.resolveSourceFiles(files);
    
    // Compile contracts first to get bytecode
    const sources = await this.readSourceFiles(sourceFiles);
    const config = this.buildCompilerConfig({ optimize: true });
    const output = await this.executeCompilation(sources, config);
    
    const analysis: any = {
      recommendations: []
    };
    
    if (options.gasReport) {
      analysis.gasAnalysis = await this.analyzeGasUsage(output);
    }
    
    if (options.sizeReport) {
      analysis.sizeAnalysis = await this.analyzeContractSizes(output);
    }
    
    // Generate recommendations based on analysis
    analysis.recommendations = this.generateOptimizationRecommendations(output, analysis);

    return analysis;
  }

  /**
   * Display analysis results
   */
  displayAnalysis(analysis: any, format: string): void {
    switch (format) {
      case "table":
        this.displayAnalysisTable(analysis);
        break;
      case "json":
        console.log(JSON.stringify(analysis, null, 2));
        break;
      case "csv":
        this.displayAnalysisCSV(analysis);
        break;
    }
  }

  /**
   * Flatten Solidity file
   */
  async flatten(filePath: string): Promise<string> {
    debug(`Flattening file: ${filePath}`);

    try {
      const content = await fs.readFile(filePath, "utf-8");
      
      const flattened = await this.flattenRecursively(filePath, new Set());
      return `// SPDX-License-Identifier: UNLICENSED
// File: ${filePath}
// Flattened by solc-neo

${flattened}`;
    } catch (error) {
      throw new Error(`Failed to flatten ${filePath}: ${error}`);
    }
  }

  /**
   * Verify contract on blockchain explorer
   */
  async verifyContract(options: {
    address: string;
    source: string;
    network: string;
    constructorArgs?: string;
  }): Promise<{
    success: boolean;
    error?: string;
    explorerUrl?: string;
  }> {
    debug(`Verifying contract at ${options.address}`);

    try {
      const explorerConfig = this.getExplorerConfig(options.network);
      if (!explorerConfig) {
        throw new Error(`Unsupported network: ${options.network}`);
      }
      
      const verificationPayload = {
        address: options.address,
        sourceCode: options.source,
        constructorArguments: options.constructorArgs || '',
        compilerVersion: await this.getCurrentVersion()
      };
      
      const response = await this.submitVerification(explorerConfig, verificationPayload);
      
      if (response.success) {
        return {
          success: true,
          explorerUrl: `${explorerConfig.baseUrl}/contract/${options.address}`
        };
      } else {
        throw new Error(response.error || 'Verification failed');
      }
    } catch (error) {
      return {
        success: false,
        error: error instanceof Error ? error.message : String(error)
      };
    }
  }

  // Private methods

  private async resolveSourceFiles(files: string[]): Promise<string[]> {
    if (files.length === 0) {
      // Default to all .sol files in current directory
      return glob("**/*.sol", { ignore: "node_modules/**" });
    }

    const resolved: string[] = [];
    
    for (const file of files) {
      const stats = await fs.stat(file).catch(() => null);
      
      if (stats?.isDirectory()) {
        const dirFiles = await glob("**/*.sol", { 
          cwd: file,
          ignore: "node_modules/**"
        });
        resolved.push(...dirFiles.map(f => path.join(file, f)));
      } else if (file.includes("*")) {
        const globFiles = await glob(file);
        resolved.push(...globFiles);
      } else {
        resolved.push(file);
      }
    }

    return resolved;
  }

  private buildCompilerConfig(options: any): NeoSolidityConfig {
    return {
      version: "latest",
      optimizer: {
        enabled: options.optimize || false,
        runs: parseInt(options.optimizeRuns || "200")
      },
      outputSelection: {
        "*": {
          "*": [
            "abi",
            "evm.bytecode",
            "evm.deployedBytecode",
            "metadata"
          ]
        }
      },
      neo: {
        gasCostModel: options.gasModel as any || "hybrid",
        storageOptimization: options.storageOpt || false,
        eventOptimization: options.eventOpt || false
      }
    };
  }

  private async readSourceFiles(files: string[]): Promise<{ [fileName: string]: { content: string } }> {
    const sources: { [fileName: string]: { content: string } } = {};
    
    for (const file of files) {
      const content = await fs.readFile(file, "utf-8");
      sources[file] = { content };
    }
    
    return sources;
  }

  private async executeCompilation(
    sources: { [fileName: string]: { content: string } },
    config: NeoSolidityConfig
  ): Promise<CompilationOutput> {
    const input: CompilationInput = {
      language: "Solidity",
      sources,
      settings: config
    };

    return this.executeStandardJsonCompilation(input);
  }

  private async executeStandardJsonCompilation(input: CompilationInput): Promise<CompilationOutput> {
    const compilerPath = this.getCompilerPath();
    
    return new Promise((resolve, reject) => {
      const child = spawn(compilerPath, ["--standard-json"], {
        stdio: ["pipe", "pipe", "pipe"]
      });

      let stdout = "";
      let stderr = "";

      child.stdout.on("data", (data) => {
        stdout += data.toString();
      });

      child.stderr.on("data", (data) => {
        stderr += data.toString();
      });

      child.on("close", (code) => {
        if (code === 0) {
          try {
            const output: CompilationOutput = JSON.parse(stdout);
            resolve(output);
          } catch (error) {
            reject(new Error(`Failed to parse compiler output: ${error}`));
          }
        } else {
          reject(new Error(`Compiler failed: ${stderr}`));
        }
      });

      child.on("error", reject);

      // Send input
      child.stdin.write(JSON.stringify(input));
      child.stdin.end();
    });
  }

  private async saveArtifacts(
    output: CompilationOutput,
    outputDir: string,
    options: any
  ): Promise<void> {
    await fs.mkdir(outputDir, { recursive: true });

    if (options.combinedJson) {
      // Save combined JSON
      const combined: any = {};
      
      for (const fileName of Object.keys(output.contracts)) {
        for (const contractName of Object.keys(output.contracts[fileName])) {
          const contract = output.contracts[fileName][contractName];
          combined[contractName] = {};
          
          if (options.combinedJson.includes("abi")) {
            combined[contractName].abi = contract.abi;
          }
          if (options.combinedJson.includes("bin")) {
            combined[contractName].bin = contract.evm.bytecode.object;
          }
          if (options.combinedJson.includes("metadata")) {
            combined[contractName].metadata = contract.metadata;
          }
        }
      }
      
      await fs.writeFile(
        path.join(outputDir, "combined.json"),
        JSON.stringify(combined, null, 2)
      );
    } else {
      // Save individual artifacts
      for (const fileName of Object.keys(output.contracts)) {
        for (const contractName of Object.keys(output.contracts[fileName])) {
          const contract = output.contracts[fileName][contractName];
          
          const artifactPath = path.join(outputDir, `${contractName}.json`);
          await fs.writeFile(artifactPath, JSON.stringify(contract, null, 2));
        }
      }
    }
  }

  private getCompilerPath(): string {
    // Try to find neo-solc compiler
    const possiblePaths = [
      path.join(process.cwd(), "bin/neo-solc"),
      path.join(process.cwd(), "target/release/neo-solc"),
      "neo-solc" // Assume in PATH
    ];

    // For this implementation, we'll assume it exists
    return possiblePaths[2];
  }

  private parseVersionOutput(output: string): {
    compiler: string;
    solidity: string;
    neovm: string;
  } {
    const lines = output.split('\n');
    let compiler = 'unknown';
    let solidity = 'unknown';
    let neovm = 'unknown';
    
    for (const line of lines) {
      const trimmed = line.trim();
      if (trimmed.includes('neo-solidity')) {
        const match = trimmed.match(/neo-solidity[:\s]+([\d\.\w-]+)/);
        if (match) compiler = match[1];
      } else if (trimmed.includes('solidity')) {
        const match = trimmed.match(/solidity[:\s]+([\d\.\w-]+)/);
        if (match) solidity = match[1];
      } else if (trimmed.includes('neo-vm') || trimmed.includes('neovm')) {
        const match = trimmed.match(/neo-?vm[:\s]+([\d\.\w-]+)/);
        if (match) neovm = match[1];
      }
    }
    
    return { compiler, solidity, neovm };
  }

  private countContracts(output: CompilationOutput): number {
    let count = 0;
    for (const fileName of Object.keys(output.contracts)) {
      count += Object.keys(output.contracts[fileName]).length;
    }
    return count;
  }

  private countWarnings(output: CompilationOutput): number {
    if (!output.errors) return 0;
    return output.errors.filter(error => error.severity === "warning").length;
  }

  private async readStdin(): Promise<string> {
    return new Promise((resolve) => {
      let input = "";
      process.stdin.setEncoding("utf8");
      
      process.stdin.on("data", (chunk) => {
        input += chunk;
      });
      
      process.stdin.on("end", () => {
        resolve(input);
      });
    });
  }

  private displayAnalysisTable(analysis: any): void {
    if (analysis.gasAnalysis) {
      console.log(chalk.blue("\n⛽ Gas Analysis:"));
      console.table(analysis.gasAnalysis);
    }

    if (analysis.sizeAnalysis) {
      console.log(chalk.blue("\n📏 Size Analysis:"));
      console.table(analysis.sizeAnalysis);
    }

    if (analysis.recommendations.length > 0) {
      console.log(chalk.blue("\n💡 Recommendations:"));
      analysis.recommendations.forEach((rec: string, i: number) => {
        console.log(`  ${i + 1}. ${rec}`);
      });
    }
  }

  private displayAnalysisCSV(analysis: any): void {
    if (analysis.gasAnalysis) {
      console.log('Contract,Method,Gas Cost');
      analysis.gasAnalysis.forEach((item: any) => {
        console.log(`${item.contract},${item.method},${item.gas}`);
      });
    }
    
    if (analysis.sizeAnalysis) {
      console.log('\nContract,Size (bytes),Limit (bytes),Percentage');
      analysis.sizeAnalysis.forEach((item: any) => {
        const percentage = ((item.size / item.limit) * 100).toFixed(2);
        console.log(`${item.contract},${item.size},${item.limit},${percentage}%`);
      });
    }
    
    if (analysis.recommendations.length > 0) {
      console.log('\nRecommendations');
      analysis.recommendations.forEach((rec: string) => {
        console.log(`"${rec}"`);
      });
    }
  }

  private async getLatestVersion(): Promise<string> {
    try {
      const response = await fetch('https://api.github.com/repos/neo-project/neo-solidity/releases/latest');
      if (!response.ok) {
        throw new Error('Failed to fetch latest version');
      }
      const release = await response.json();
      return release.tag_name.replace(/^v/, '');
    } catch (error) {
      debug('Failed to fetch latest version, using default');
      return '0.1.0';
    }
  }

  private isValidVersion(version: string): boolean {
    return /^\d+\.\d+\.\d+(-[\w\.]+)?$/.test(version);
  }

  private getDownloadUrl(version: string): string {
    const platform = process.platform;
    const arch = process.arch;
    const ext = platform === 'win32' ? '.exe' : '';
    return `https://github.com/neo-project/neo-solidity/releases/download/v${version}/neo-solc-${platform}-${arch}${ext}`;
  }

  private async downloadCompiler(url: string, outputPath: string): Promise<void> {
    const response = await fetch(url);
    if (!response.ok) {
      throw new Error(`Download failed: ${response.statusText}`);
    }
    
    const buffer = await response.arrayBuffer();
    await fs.writeFile(outputPath, Buffer.from(buffer));
  }

  private async getCurrentVersion(): Promise<string> {
    try {
      const versionInfo = await this.getVersion();
      return versionInfo.compiler;
    } catch {
      return 'unknown';
    }
  }

  private compareVersions(a: string, b: string): number {
    const parseVersion = (v: string) => v.split('.').map(Number);
    const versionA = parseVersion(a);
    const versionB = parseVersion(b);
    
    for (let i = 0; i < Math.max(versionA.length, versionB.length); i++) {
      const partA = versionA[i] || 0;
      const partB = versionB[i] || 0;
      if (partA !== partB) {
        return partA - partB;
      }
    }
    return 0;
  }

  private async getLocalVersions(): Promise<Array<{
    version: string;
    current: boolean;
    prerelease: boolean;
  }>> {
    const compilerDir = path.join(process.cwd(), '.neo-solidity', 'compilers');
    try {
      const files = await fs.readdir(compilerDir);
      const currentVersion = await this.getCurrentVersion();
      
      return files
        .filter(file => file.startsWith('neo-solc-'))
        .map(file => {
          const version = file.replace('neo-solc-', '');
          return {
            version,
            current: version === currentVersion,
            prerelease: version.includes('-')
          };
        });
    } catch {
      return [];
    }
  }

  private async analyzeGasUsage(output: CompilationOutput): Promise<any[]> {
    const gasAnalysis: any[] = [];
    
    for (const fileName of Object.keys(output.contracts)) {
      for (const contractName of Object.keys(output.contracts[fileName])) {
        const contract = output.contracts[fileName][contractName];
        if (contract.abi) {
          for (const item of contract.abi) {
            if (item.type === 'function' && item.stateMutability !== 'view' && item.stateMutability !== 'pure') {
              // Estimate gas based on function complexity
              const gasEstimate = this.estimateGasCost(item);
              gasAnalysis.push({
                contract: contractName,
                method: item.name,
                gas: gasEstimate
              });
            }
          }
        }
      }
    }
    
    return gasAnalysis;
  }

  private async analyzeContractSizes(output: CompilationOutput): Promise<any[]> {
    const sizeAnalysis: any[] = [];
    const NEO_CONTRACT_SIZE_LIMIT = 1024 * 1024; // 1MB limit for Neo contracts
    
    for (const fileName of Object.keys(output.contracts)) {
      for (const contractName of Object.keys(output.contracts[fileName])) {
        const contract = output.contracts[fileName][contractName];
        if (contract.evm?.bytecode?.object) {
          const bytecodeSize = contract.evm.bytecode.object.length / 2; // Convert hex to bytes
          sizeAnalysis.push({
            contract: contractName,
            size: bytecodeSize,
            limit: NEO_CONTRACT_SIZE_LIMIT
          });
        }
      }
    }
    
    return sizeAnalysis;
  }

  private generateOptimizationRecommendations(output: CompilationOutput, analysis: any): string[] {
    const recommendations: string[] = [];
    
    // Size-based recommendations
    if (analysis.sizeAnalysis) {
      for (const item of analysis.sizeAnalysis) {
        const percentage = (item.size / item.limit) * 100;
        if (percentage > 90) {
          recommendations.push(`Contract ${item.contract} is ${percentage.toFixed(1)}% of size limit. Consider splitting into multiple contracts.`);
        } else if (percentage > 70) {
          recommendations.push(`Contract ${item.contract} is large (${percentage.toFixed(1)}% of limit). Consider optimization.`);
        }
      }
    }
    
    // Gas-based recommendations
    if (analysis.gasAnalysis) {
      const highGasFunctions = analysis.gasAnalysis.filter((item: any) => item.gas > 100000);
      for (const func of highGasFunctions) {
        recommendations.push(`Function ${func.contract}.${func.method} has high gas cost (${func.gas}). Consider optimization.`);
      }
    }
    
    // Static analysis recommendations
    for (const fileName of Object.keys(output.contracts)) {
      for (const contractName of Object.keys(output.contracts[fileName])) {
        const contract = output.contracts[fileName][contractName];
        if (contract.abi) {
          recommendations.push(...this.analyzeContractStructure(contract.abi, contractName));
        }
      }
    }
    
    return recommendations;
  }

  private estimateGasCost(abiItem: any): number {
    let baseCost = 21000; // Base transaction cost
    
    // Add cost based on input parameters
    if (abiItem.inputs) {
      baseCost += abiItem.inputs.length * 1000;
    }
    
    // Estimate based on function name patterns
    const name = abiItem.name.toLowerCase();
    if (name.includes('transfer') || name.includes('send')) {
      baseCost += 5000;
    }
    if (name.includes('approve')) {
      baseCost += 3000;
    }
    if (name.includes('mint') || name.includes('burn')) {
      baseCost += 10000;
    }
    
    return baseCost;
  }

  private analyzeContractStructure(abi: any[], contractName: string): string[] {
    const recommendations: string[] = [];
    
    const functions = abi.filter(item => item.type === 'function');
    const events = abi.filter(item => item.type === 'event');
    
    if (functions.length > 20) {
      recommendations.push(`Contract ${contractName} has many functions (${functions.length}). Consider using proxy pattern or splitting functionality.`);
    }
    
    if (events.length === 0) {
      recommendations.push(`Contract ${contractName} has no events. Consider adding events for better tracking and debugging.`);
    }
    
    // Check for missing view functions
    const viewFunctions = functions.filter(f => f.stateMutability === 'view' || f.stateMutability === 'pure');
    if (viewFunctions.length === 0 && functions.length > 0) {
      recommendations.push(`Contract ${contractName} has no view functions. Consider adding getter functions for better transparency.`);
    }
    
    return recommendations;
  }

  private async flattenRecursively(filePath: string, processed: Set<string>): Promise<string> {
    if (processed.has(filePath)) {
      return ''; // Avoid circular dependencies
    }
    
    processed.add(filePath);
    
    try {
      const content = await fs.readFile(filePath, 'utf-8');
      const lines = content.split('\n');
      const result: string[] = [];
      
      for (const line of lines) {
        const importMatch = line.match(/^import\s+[^"']*["']([^"']+)["'];?/);
        if (importMatch) {
          const importPath = importMatch[1];
          let resolvedPath: string;
          
          if (importPath.startsWith('.')) {
            // Relative import
            resolvedPath = path.resolve(path.dirname(filePath), importPath);
          } else {
            // Try to resolve from node_modules or other standard locations
            resolvedPath = await this.resolveImport(importPath, filePath);
          }
          
          if (resolvedPath.endsWith('.sol')) {
            try {
              const importedContent = await this.flattenRecursively(resolvedPath, processed);
              if (importedContent.trim()) {
                result.push(`// File: ${resolvedPath}`);
                result.push(importedContent);
                result.push('');
              }
            } catch (error) {
              result.push(`// Failed to import: ${importPath} - ${error}`);
            }
          }
        } else {
          result.push(line);
        }
      }
      
      return result.join('\n');
    } catch (error) {
      throw new Error(`Failed to read file ${filePath}: ${error}`);
    }
  }

  private async resolveImport(importPath: string, fromFile: string): Promise<string> {
    const possiblePaths = [
      path.resolve(path.dirname(fromFile), importPath),
      path.resolve(path.dirname(fromFile), importPath + '.sol'),
      path.resolve(process.cwd(), 'node_modules', importPath),
      path.resolve(process.cwd(), 'contracts', importPath),
      path.resolve(process.cwd(), 'src', importPath)
    ];
    
    for (const possiblePath of possiblePaths) {
      try {
        await fs.access(possiblePath);
        return possiblePath;
      } catch {
        continue;
      }
    }
    
    throw new Error(`Could not resolve import: ${importPath}`);
  }

  private getExplorerConfig(network: string): { baseUrl: string; apiKey?: string } | null {
    const configs: Record<string, { baseUrl: string; apiKey?: string }> = {
      'mainnet': { baseUrl: 'https://explorer.neo.org' },
      'testnet': { baseUrl: 'https://testnet.neo.org' },
      'private': { baseUrl: 'http://localhost:4000' }
    };
    
    return configs[network] || null;
  }

  private async submitVerification(
    explorerConfig: { baseUrl: string; apiKey?: string },
    payload: any
  ): Promise<{ success: boolean; error?: string }> {
    try {
      const response = await fetch(`${explorerConfig.baseUrl}/api/verify-contract`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          ...(explorerConfig.apiKey && { 'Authorization': `Bearer ${explorerConfig.apiKey}` })
        },
        body: JSON.stringify(payload)
      });
      
      if (!response.ok) {
        return { success: false, error: `HTTP ${response.status}: ${response.statusText}` };
      }
      
      const result = await response.json();
      return { success: result.success, error: result.error };
    } catch (error) {
      return { success: false, error: `Network error: ${error}` };
    }
  }
}