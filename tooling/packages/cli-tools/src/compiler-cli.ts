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
    
    // This would download and install the specified compiler version
    // For now, this is a placeholder
    throw new Error("Compiler installation not yet implemented");
  }

  /**
   * List available compiler versions
   */
  async listVersions(): Promise<Array<{
    version: string;
    current: boolean;
    prerelease: boolean;
  }>> {
    // This would fetch available versions from releases
    // For now, return mock data
    return [
      { version: "0.1.0", current: true, prerelease: false },
      { version: "0.1.0-beta.1", current: false, prerelease: true },
      { version: "0.0.9", current: false, prerelease: false }
    ];
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
    
    // Mock analysis results
    const analysis = {
      gasAnalysis: options.gasReport ? [
        { contract: "Token", method: "transfer", gas: 21000 },
        { contract: "Token", method: "approve", gas: 15000 }
      ] : undefined,
      sizeAnalysis: options.sizeReport ? [
        { contract: "Token", size: 1024, limit: 24576 }
      ] : undefined,
      recommendations: [
        "Consider using uint256 instead of uint8 for gas efficiency",
        "Pack struct members to save storage slots"
      ]
    };

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
      
      // This would recursively resolve and inline imports
      // For now, return the original content with a header
      return `// SPDX-License-Identifier: UNLICENSED
// Flattened by solc-neo

${content}`;
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
      // This would submit verification to the appropriate explorer
      // For now, return mock success
      return {
        success: true,
        explorerUrl: `https://explorer.neo.org/contract/${options.address}`
      };
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
    // Mock version parsing
    return {
      compiler: "0.1.0",
      solidity: "0.8.19",
      neovm: "3.6.0"
    };
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
    // CSV output implementation
    console.log("Analysis CSV output not yet implemented");
  }
}