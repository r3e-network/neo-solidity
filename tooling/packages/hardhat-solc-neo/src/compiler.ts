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
    // This would parse the actual version output from neo-solc
    // For now, return a default version
    return ["0.1.0"];
  }

  /**
   * Download and install compiler version
   */
  async downloadCompiler(version: string): Promise<void> {
    debug(`Downloading Neo-Solidity compiler version ${version}`);
    
    // Implementation would download the specified compiler version
    // For now, this is a placeholder
    throw new Error("Compiler download not yet implemented");
  }
}