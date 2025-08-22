import { spawn } from "child_process";
import { promises as fs } from "fs";
import path from "path";
import { HardhatPluginError } from "hardhat/plugins";
import { 
  CompilationInput, 
  CompilationOutput, 
  NeoSolidityConfig,
  CompilerOptions
} from "@neo-solidity/types";
import chalk from "chalk";
import Debug from "debug";

const debug = Debug("hardhat:neo-solidity:compiler");

/**
 * Neo-Solidity compiler wrapper for Hardhat integration
 */
export class NeoSolidityCompiler {
  private config: NeoSolidityConfig;
  private paths: any;

  constructor(config: NeoSolidityConfig, paths: any) {
    this.config = config;
    this.paths = paths;
  }

  /**
   * Compile Solidity sources to NeoVM bytecode
   */
  async compile(sources: { [fileName: string]: { content: string } }): Promise<CompilationOutput> {
    debug("Starting Neo-Solidity compilation");
    
    try {
      // Prepare compilation input
      const input: CompilationInput = {
        language: "Solidity",
        sources,
        settings: this.config
      };

      // Write input to temporary file
      const inputFile = path.join(this.paths.cache, "neo-solc-input.json");
      await fs.mkdir(path.dirname(inputFile), { recursive: true });
      await fs.writeFile(inputFile, JSON.stringify(input, null, 2));

      // Execute Neo-Solidity compiler
      const outputFile = path.join(this.paths.cache, "neo-solc-output.json");
      await this.executeCompiler(inputFile, outputFile);

      // Read compilation output
      const outputContent = await fs.readFile(outputFile, "utf-8");
      const output: CompilationOutput = JSON.parse(outputContent);

      // Validate compilation results
      this.validateOutput(output);

      debug("Neo-Solidity compilation completed successfully");
      return output;

    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      throw new HardhatPluginError(
        "@neo-solidity/hardhat-solc-neo",
        `Compilation failed: ${message}`
      );
    }
  }

  /**
   * Execute the Neo-Solidity compiler binary
   */
  private async executeCompiler(inputFile: string, outputFile: string): Promise<void> {
    return new Promise((resolve, reject) => {
      const compilerPath = this.getCompilerPath();
      const args = [
        "--standard-json",
        "--input", inputFile,
        "--output", outputFile
      ];

      debug(`Executing compiler: ${compilerPath} ${args.join(" ")}`);

      const child = spawn(compilerPath, args, {
        stdio: ["pipe", "pipe", "pipe"],
        env: { ...process.env }
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
          debug("Compiler executed successfully");
          resolve();
        } else {
          debug(`Compiler failed with code ${code}`);
          debug(`Stdout: ${stdout}`);
          debug(`Stderr: ${stderr}`);
          reject(new Error(`Compiler failed with exit code ${code}\n${stderr}`));
        }
      });

      child.on("error", (error) => {
        debug(`Compiler process error: ${error.message}`);
        reject(new Error(`Failed to start compiler: ${error.message}`));
      });
    });
  }

  /**
   * Get path to Neo-Solidity compiler binary
   */
  private getCompilerPath(): string {
    // Try different potential locations
    const possiblePaths = [
      path.join(process.cwd(), "bin/neo-solc"),
      path.join(process.cwd(), "target/release/neo-solc"),
      "neo-solc" // Assume it's in PATH
    ];

    for (const compilerPath of possiblePaths) {
      try {
        // Check if file exists and is executable
        if (path.isAbsolute(compilerPath)) {
          require("fs").accessSync(compilerPath, require("fs").constants.X_OK);
        }
        return compilerPath;
      } catch {
        continue;
      }
    }

    throw new Error(
      "Neo-Solidity compiler not found. Please ensure neo-solc is installed and available in PATH."
    );
  }

  /**
   * Validate compilation output
   */
  private validateOutput(output: CompilationOutput): void {
    // Check for compilation errors
    if (output.errors) {
      const errors = output.errors.filter(error => error.severity === "error");
      if (errors.length > 0) {
        const errorMessages = errors.map(error => error.formattedMessage || error.message);
        throw new Error(`Compilation errors:\n${errorMessages.join("\n")}`);
      }

      // Print warnings
      const warnings = output.errors.filter(error => error.severity === "warning");
      if (warnings.length > 0) {
        console.log(chalk.yellow("Compilation warnings:"));
        warnings.forEach(warning => {
          console.log(chalk.yellow(`  ${warning.formattedMessage || warning.message}`));
        });
      }
    }

    // Validate that we have contracts
    if (!output.contracts || Object.keys(output.contracts).length === 0) {
      throw new Error("No contracts were compiled");
    }

    // Validate Neo-specific outputs
    for (const fileName of Object.keys(output.contracts)) {
      for (const contractName of Object.keys(output.contracts[fileName])) {
        const contract = output.contracts[fileName][contractName];
        
        if (!contract.neo) {
          throw new Error(`Missing Neo-specific compilation output for ${contractName}`);
        }

        if (!contract.neo.nef || !contract.neo.manifest) {
          throw new Error(`Missing NEF or manifest for ${contractName}`);
        }
      }
    }
  }

  /**
   * Get available compiler versions
   */
  async getAvailableVersions(): Promise<string[]> {
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
            // Parse version information
            const versions = this.parseVersionOutput(stdout);
            resolve(versions);
          } else {
            reject(new Error(`Failed to get compiler versions`));
          }
        });
      });
    } catch (error) {
      debug(`Failed to get available versions: ${error}`);
      return ["latest"];
    }
  }

  /**
   * Parse version output from compiler
   */
  private parseVersionOutput(output: string): string[] {
    const versions: string[] = [];
    const lines = output.split('\n');
    
    for (const line of lines) {
      const trimmed = line.trim();
      // Look for version patterns like "neo-solidity: 0.1.0"
      const versionMatch = trimmed.match(/neo-solidity[:\s]+([\d\.\w-]+)/);
      if (versionMatch) {
        versions.push(versionMatch[1]);
      }
      
      // Also look for supported Solidity versions
      const solidityMatch = trimmed.match(/solidity[:\s]+([\d\.\w-]+)/);
      if (solidityMatch) {
        versions.push(`solidity-${solidityMatch[1]}`);
      }
    }
    
    return versions.length > 0 ? versions : ['0.1.0'];
  }

  /**
   * Download and install compiler version
   */
  async downloadCompiler(version: string): Promise<void> {
    debug(`Downloading Neo-Solidity compiler version ${version}`);
    
    if (!this.isValidVersion(version)) {
      throw new Error(`Invalid version format: ${version}`);
    }
    
    const compilerDir = path.join(this.paths.cache, 'compilers');
    const compilerPath = path.join(compilerDir, `neo-solc-${version}`);
    
    // Check if already downloaded
    try {
      await fs.access(compilerPath);
      debug(`Compiler version ${version} already exists`);
      return;
    } catch {
      // Not downloaded, continue
    }
    
    await fs.mkdir(compilerDir, { recursive: true });
    
    try {
      const downloadUrl = this.getDownloadUrl(version);
      debug(`Downloading from: ${downloadUrl}`);
      
      const response = await fetch(downloadUrl);
      if (!response.ok) {
        throw new Error(`Download failed: ${response.statusText}`);
      }
      
      const buffer = await response.arrayBuffer();
      await fs.writeFile(compilerPath, Buffer.from(buffer));
      await fs.chmod(compilerPath, 0o755);
      
      debug(`Successfully downloaded compiler version ${version}`);
    } catch (error) {
      throw new Error(`Failed to download compiler version ${version}: ${error}`);
    }
  }

  /**
   * Validate version format
   */
  private isValidVersion(version: string): boolean {
    return /^\d+\.\d+\.\d+(-[\w\.]+)?$/.test(version) || version === 'latest';
  }

  /**
   * Get download URL for compiler version
   */
  private getDownloadUrl(version: string): string {
    const platform = process.platform;
    const arch = process.arch;
    const ext = platform === 'win32' ? '.exe' : '';
    
    if (version === 'latest') {
      return `https://github.com/neo-project/neo-solidity/releases/latest/download/neo-solc-${platform}-${arch}${ext}`;
    } else {
      return `https://github.com/neo-project/neo-solidity/releases/download/v${version}/neo-solc-${platform}-${arch}${ext}`;
    }
  }

  /**
   * Get current compiler version
   */
  async getCurrentVersion(): Promise<string> {
    try {
      const compilerPath = this.getCompilerPath();
      
      return new Promise((resolve, reject) => {
        const child = spawn(compilerPath, ['--version'], {
          stdio: ['pipe', 'pipe', 'pipe']
        });

        let stdout = '';
        child.stdout.on('data', (data) => {
          stdout += data.toString();
        });

        child.on('close', (code) => {
          if (code === 0) {
            const versions = this.parseVersionOutput(stdout);
            resolve(versions[0] || 'unknown');
          } else {
            reject(new Error('Failed to get current version'));
          }
        });
      });
    } catch (error) {
      debug(`Failed to get current version: ${error}`);
      return 'unknown';
    }
  }

  /**
   * Check if compiler is installed and available
   */
  async isCompilerAvailable(): Promise<boolean> {
    try {
      await this.getCurrentVersion();
      return true;
    } catch {
      return false;
    }
  }

  /**
   * Get compilation statistics
   */
  getCompilationStats(output: CompilationOutput): {
    contractCount: number;
    errorCount: number;
    warningCount: number;
    totalSize: number;
  } {
    let contractCount = 0;
    let totalSize = 0;
    let errorCount = 0;
    let warningCount = 0;

    // Count contracts and calculate total size
    for (const fileName of Object.keys(output.contracts)) {
      for (const contractName of Object.keys(output.contracts[fileName])) {
        contractCount++;
        const contract = output.contracts[fileName][contractName];
        if (contract.neo?.nef) {
          // Estimate size from NEF bytecode
          totalSize += contract.neo.nef.length / 2; // Convert hex to bytes
        }
      }
    }

    // Count errors and warnings
    if (output.errors) {
      errorCount = output.errors.filter(e => e.severity === 'error').length;
      warningCount = output.errors.filter(e => e.severity === 'warning').length;
    }

    return {
      contractCount,
      errorCount,
      warningCount,
      totalSize
    };
  }
}