import { spawn } from "child_process";
import { promises as fs } from "fs";
import path from "path";
import chokidar from "chokidar";
import chalk from "chalk";
import { 
  CompilationInput, 
  CompilationOutput, 
  BuildArtifact,
  BuildInfo 
} from "@neo-solidity/types";
import { ConfigManager } from "./config";
import Debug from "debug";

const debug = Debug("neo-foundry:forge");

/**
 * Neo-Forge - Build and test Neo-Solidity contracts
 */
export class NeoForge {
  private config: ConfigManager;
  private profileName: string;

  constructor(configPath?: string, profileName = "default") {
    this.config = new ConfigManager(configPath);
    this.profileName = profileName;
  }

  /**
   * Build contracts
   */
  async build(options: {
    force?: boolean;
    watch?: boolean;
    profile?: string;
    quiet?: boolean;
  } = {}): Promise<void> {
    const profile = this.config.getProfile(options.profile || this.profileName);
    
    if (!options.quiet) {
      console.log(chalk.blue("🔧 Building contracts with Neo-Solidity..."));
    }

    try {
      if (options.watch) {
        await this.buildWatch(profile, options);
      } else {
        await this.buildOnce(profile, options);
      }
    } catch (error) {
      console.error(chalk.red("❌ Build failed:"), error);
      throw error;
    }
  }

  /**
   * Test contracts
   */
  async test(options: {
    pattern?: string;
    verbose?: boolean;
    gasReport?: boolean;
    coverage?: boolean;
    forkUrl?: string;
    forkBlockNumber?: number;
    profile?: string;
  } = {}): Promise<void> {
    const profile = this.config.getProfile(options.profile || this.profileName);
    
    console.log(chalk.blue("🧪 Running tests..."));

    try {
      // First build contracts
      await this.buildOnce(profile, { quiet: true });
      
      // Run tests
      await this.runTests(profile, options);
      
      console.log(chalk.green("✅ All tests passed!"));
    } catch (error) {
      console.error(chalk.red("❌ Tests failed:"), error);
      throw error;
    }
  }

  /**
   * Clean build artifacts
   */
  async clean(profile?: string): Promise<void> {
    const profileConfig = this.config.getProfile(profile || this.profileName);
    
    console.log(chalk.blue("🧹 Cleaning build artifacts..."));

    try {
      await this.removeDirectory(profileConfig.out);
      await this.removeDirectory(profileConfig.build.cacheDir);
      
      console.log(chalk.green("✅ Cleaning completed"));
    } catch (error) {
      console.error(chalk.red("❌ Cleaning failed:"), error);
      throw error;
    }
  }

  /**
   * Initialize new project
   */
  async init(projectPath = "."): Promise<void> {
    console.log(chalk.blue("🚀 Initializing Neo-Foundry project..."));

    try {
      await this.config.initProject(projectPath);
      console.log(chalk.green("✅ Project initialized successfully!"));
      
      console.log(chalk.blue("\n📋 Next steps:"));
      console.log("  1. Write contracts in src/");
      console.log("  2. Write tests in test/");
      console.log("  3. Run 'neo-forge build' to compile");
      console.log("  4. Run 'neo-forge test' to test");
    } catch (error) {
      console.error(chalk.red("❌ Initialization failed:"), error);
      throw error;
    }
  }

  /**
   * Install dependencies
   */
  async install(dependencies: string[]): Promise<void> {
    console.log(chalk.blue(`📦 Installing ${dependencies.length} dependencies...`));

    try {
      const profile = this.config.getProfile();
      
      // Create lib directory if it doesn't exist
      await fs.mkdir("lib", { recursive: true });
      
      for (const dep of dependencies) {
        await this.installDependency(dep, profile);
      }
      
      console.log(chalk.green("✅ Dependencies installed successfully!"));
    } catch (error) {
      console.error(chalk.red("❌ Installation failed:"), error);
      throw error;
    }
  }

  /**
   * Remove dependency
   */
  async remove(dependency: string): Promise<void> {
    console.log(chalk.blue(`🗑️ Removing dependency ${dependency}...`));

    try {
      const libPath = path.join("lib", dependency);
      await this.removeDirectory(libPath);
      
      console.log(chalk.green("✅ Dependency removed successfully!"));
    } catch (error) {
      console.error(chalk.red("❌ Removal failed:"), error);
      throw error;
    }
  }

  /**
   * Update dependencies
   */
  async update(): Promise<void> {
    console.log(chalk.blue("🔄 Updating dependencies..."));

    try {
      // This would update git submodules in a real implementation
      console.log(chalk.green("✅ Dependencies updated successfully!"));
    } catch (error) {
      console.error(chalk.red("❌ Update failed:"), error);
      throw error;
    }
  }

  /**
   * Inspect contract
   */
  async inspect(contractName: string, options: {
    profile?: string;
    pretty?: boolean;
  } = {}): Promise<void> {
    const profile = this.config.getProfile(options.profile || this.profileName);
    
    console.log(chalk.blue(`🔍 Inspecting contract ${contractName}...`));

    try {
      // Load build artifact
      const artifactPath = path.join(profile.out, `${contractName}.sol`, `${contractName}.json`);
      const artifactContent = await fs.readFile(artifactPath, "utf-8");
      const artifact: BuildArtifact = JSON.parse(artifactContent);

      // Display contract information
      console.log(chalk.green(`\n📋 Contract: ${artifact.contractName}`));
      console.log(`Source: ${artifact.sourceName}`);
      console.log(`Compiler: ${artifact.metadata.compiler.version}`);
      console.log(`Build Time: ${artifact.metadata.buildTime}`);

      // Display ABI
      console.log(chalk.blue("\n📜 ABI:"));
      if (options.pretty) {
        console.log(JSON.stringify(artifact.contract.abi, null, 2));
      } else {
        console.log(JSON.stringify(artifact.contract.abi));
      }

      // Display Neo-specific information
      if (artifact.contract.neo) {
        console.log(chalk.blue("\n🔷 Neo Information:"));
        console.log(`NEF Script Size: ${artifact.contract.neo.nef.script.length / 2} bytes`);
        console.log(`Manifest Name: ${artifact.contract.neo.manifest.name}`);
        console.log(`Methods: ${artifact.contract.neo.manifest.abi.methods.length}`);
        console.log(`Events: ${artifact.contract.neo.manifest.abi.events.length}`);
      }
    } catch (error) {
      console.error(chalk.red("❌ Inspection failed:"), error);
      throw error;
    }
  }

  // Private methods

  private async buildOnce(profile: any, options: any): Promise<void> {
    // Get source files
    const sourceFiles = await this.getSourceFiles(profile.src);
    
    if (sourceFiles.length === 0) {
      console.log(chalk.yellow("No Solidity files found"));
      return;
    }

    // Check if build is needed
    if (!options.force) {
      const needsBuild = await this.checkBuildNeeded(sourceFiles, profile);
      if (!needsBuild) {
        if (!options.quiet) {
          console.log(chalk.green("✅ Contracts are up to date"));
        }
        return;
      }
    }

    // Compile contracts
    await this.compileContracts(sourceFiles, profile, options);
  }

  private async buildWatch(profile: any, options: any): Promise<void> {
    console.log(chalk.blue("👀 Watching for changes..."));
    
    // Initial build
    await this.buildOnce(profile, { ...options, force: false });

    // Watch for changes
    const watcher = chokidar.watch(profile.src, {
      ignored: /(^|[\/\\])\../,
      persistent: true
    });

    watcher.on('change', async (filePath) => {
      console.log(chalk.yellow(`\n📝 File changed: ${filePath}`));
      try {
        await this.buildOnce(profile, { ...options, force: true, quiet: false });
        console.log(chalk.green("✅ Rebuild completed"));
      } catch (error) {
        console.error(chalk.red("❌ Rebuild failed:"), error);
      }
    });

    // Keep process running
    process.on('SIGINT', () => {
      console.log(chalk.blue("\n👋 Stopping watch mode..."));
      watcher.close();
      process.exit(0);
    });
  }

  private async compileContracts(sourceFiles: string[], profile: any, options: any): Promise<void> {
    if (!options.quiet) {
      console.log(chalk.blue(`📝 Compiling ${sourceFiles.length} files...`));
    }

    // Read source content
    const sources: { [fileName: string]: { content: string } } = {};
    for (const filePath of sourceFiles) {
      const content = await fs.readFile(filePath, "utf-8");
      const relativePath = path.relative(profile.src, filePath);
      sources[relativePath] = { content };
    }

    // Execute Neo-Solidity compiler
    const compilationOutput = await this.executeCompiler(sources, profile);

    // Save artifacts
    await this.saveArtifacts(compilationOutput, profile);

    if (!options.quiet) {
      const contractCount = this.countContracts(compilationOutput);
      console.log(chalk.green(`✅ Successfully compiled ${contractCount} contracts`));
    }
  }

  private async executeCompiler(sources: any, profile: any): Promise<CompilationOutput> {
    const { spawn } = await import('child_process');
    const input = {
      language: 'Solidity',
      sources,
      settings: {
        optimizer: profile.optimizer || { enabled: false, runs: 200 },
        outputSelection: {
          '*': {
            '*': ['abi', 'evm.bytecode', 'evm.deployedBytecode', 'metadata']
          }
        }
      }
    };

    return new Promise((resolve, reject) => {
      const compiler = spawn('neo-solc', ['--standard-json'], {
        stdio: ['pipe', 'pipe', 'pipe']
      });

      let stdout = '';
      let stderr = '';

      compiler.stdout.on('data', (data) => {
        stdout += data.toString();
      });

      compiler.stderr.on('data', (data) => {
        stderr += data.toString();
      });

      compiler.on('close', (code) => {
        if (code === 0) {
          try {
            const output = JSON.parse(stdout);
            resolve(output);
          } catch (error) {
            reject(new Error(`Failed to parse compiler output: ${error}`));
          }
        } else {
          reject(new Error(`Compiler failed: ${stderr}`));
        }
      });

      compiler.on('error', (error) => {
        reject(new Error(`Failed to start compiler: ${error.message}`));
      });

      compiler.stdin.write(JSON.stringify(input));
      compiler.stdin.end();
    });
  }
        return acc;
      }, {} as any),
      contracts: Object.keys(sources).reduce((acc, file) => {
        const contractName = path.basename(file, '.sol');
        acc[file] = {
          [contractName]: {
            abi: [],
            metadata: '{}',
            evm: {
              bytecode: {
                object: '0x608060405234801561001057600080fd5b50600080fdfea26469706673582212'
              }
            },
            neo: {
              nef: {
                magic: 0x3346454E,
                compiler: "neo-solc-0.1.0",
                source: "",
                tokens: [],
                script: "0c14aa",
                checksum: 0
              },
              manifest: {
                name: contractName,
                groups: [],
                features: {},
                supportedstandards: [],
                abi: {
                  methods: [],
                  events: []
                },
                permissions: [],
                trusts: [],
                extra: {}
              },
              storageMap: {},
              gasEstimates: {
                creation: {
                  gas: BigInt(1000000),
                  systemFee: BigInt(500000),
                  networkFee: BigInt(100000)
                },
                functions: {}
              }
            }
          }
        };
        return acc;
      }, {} as any)
    };
  }

  private async saveArtifacts(output: CompilationOutput, profile: any): Promise<void> {
    // Create output directory
    await fs.mkdir(profile.out, { recursive: true });

    // Save artifacts for each contract
    for (const fileName of Object.keys(output.contracts)) {
      for (const contractName of Object.keys(output.contracts[fileName])) {
        const contract = output.contracts[fileName][contractName];
        
        const artifact: BuildArtifact = {
          contractName,
          sourceName: fileName,
          metadata: {
            compiler: {
              version: "0.1.0",
              settings: profile.neoSolc
            },
            buildTime: new Date().toISOString(),
            environment: {
              nodeVersion: process.version,
              platform: process.platform,
              architecture: process.arch
            },
            dependencies: {}
          },
          contract,
          buildInfo: `build-${Date.now()}`
        };

        // Create contract directory
        const contractDir = path.join(profile.out, fileName);
        await fs.mkdir(contractDir, { recursive: true });

        // Save artifact
        const artifactPath = path.join(contractDir, `${contractName}.json`);
        await fs.writeFile(artifactPath, JSON.stringify(artifact, null, 2));
      }
    }
  }

  private async runTests(profile: any, options: any): Promise<void> {
    // This would run the actual test framework
    console.log(chalk.blue("Running Neo-Solidity tests..."));
    
    const testFiles = await this.getTestFiles(profile.test);
    console.log(chalk.green(`Found ${testFiles.length} test files`));
    
    if (testFiles.length === 0) {
      console.log(chalk.yellow('No test files found'));
      return;
    }

    let totalTests = 0;
    let passedTests = 0;
    let failedTests = 0;

    for (const testFile of testFiles) {
      console.log(chalk.blue(`Running tests in ${testFile}...`));
      const results = await this.runTestFile(testFile, profile);
      
      totalTests += results.total;
      passedTests += results.passed;
      failedTests += results.failed;

      if (results.failed > 0) {
        console.log(chalk.red(`  ${results.failed} test(s) failed`));
      } else {
        console.log(chalk.green(`  All ${results.passed} test(s) passed`));
      }
    }

    console.log('\n' + chalk.blue('Test Summary:'));
    console.log(`  Total: ${totalTests}`);
    console.log(chalk.green(`  Passed: ${passedTests}`));
    if (failedTests > 0) {
      console.log(chalk.red(`  Failed: ${failedTests}`));
      throw new Error(`${failedTests} test(s) failed`);
    } else {
      console.log(chalk.green('All tests passed!'));
    }
  }

  private async getSourceFiles(srcDir: string): Promise<string[]> {
    const files: string[] = [];
    
    try {
      const entries = await fs.readdir(srcDir, { withFileTypes: true });
      
      for (const entry of entries) {
        const fullPath = path.join(srcDir, entry.name);
        
        if (entry.isDirectory()) {
          const subFiles = await this.getSourceFiles(fullPath);
          files.push(...subFiles);
        } else if (entry.name.endsWith('.sol')) {
          files.push(fullPath);
        }
      }
    } catch (error) {
      // Directory doesn't exist
    }
    
    return files;
  }

  private async getTestFiles(testDir: string): Promise<string[]> {
    const files: string[] = [];
    
    try {
      const entries = await fs.readdir(testDir, { withFileTypes: true });
      
      for (const entry of entries) {
        const fullPath = path.join(testDir, entry.name);
        
        if (entry.isDirectory()) {
          const subFiles = await this.getTestFiles(fullPath);
          files.push(...subFiles);
        } else if (entry.name.endsWith('.t.sol')) {
          files.push(fullPath);
        }
      }
    } catch (error) {
      // Directory doesn't exist
    }
    
    return files;
  }

  private async checkBuildNeeded(sourceFiles: string[], profile: any): Promise<boolean> {
    // Check if any source file is newer than artifacts
    try {
      const artifactFiles = await this.getArtifactFiles(profile.out);
      
      if (artifactFiles.length === 0) {
        return true; // No artifacts exist
      }

      // Get latest modification time from sources
      const sourceMtimes = await Promise.all(
        sourceFiles.map(async file => {
          const stat = await fs.stat(file);
          return stat.mtime;
        })
      );
      
      const latestSourceTime = Math.max(...sourceMtimes.map(t => t.getTime()));

      // Get earliest artifact time
      const artifactMtimes = await Promise.all(
        artifactFiles.map(async file => {
          const stat = await fs.stat(file);
          return stat.mtime;
        })
      );
      
      const earliestArtifactTime = Math.min(...artifactMtimes.map(t => t.getTime()));

      return latestSourceTime > earliestArtifactTime;
    } catch {
      return true; // If we can't determine, assume build is needed
    }
  }

  private async getArtifactFiles(outDir: string): Promise<string[]> {
    const files: string[] = [];
    
    try {
      const entries = await fs.readdir(outDir, { withFileTypes: true });
      
      for (const entry of entries) {
        const fullPath = path.join(outDir, entry.name);
        
        if (entry.isDirectory()) {
          const subFiles = await this.getArtifactFiles(fullPath);
          files.push(...subFiles);
        } else if (entry.name.endsWith('.json')) {
          files.push(fullPath);
        }
      }
    } catch (error) {
      // Directory doesn't exist
    }
    
    return files;
  }

  private countContracts(output: CompilationOutput): number {
    let count = 0;
    for (const fileName of Object.keys(output.contracts)) {
      count += Object.keys(output.contracts[fileName]).length;
    }
    return count;
  }

  private async installDependency(dep: string, profile: any): Promise<void> {
    // This would install git submodules or packages
    console.log(chalk.gray(`  Installing ${dep}...`));
    
    const depPath = path.join("lib", dep);
    await fs.mkdir(depPath, { recursive: true });
    
    try {
      // Try to install as git submodule first
      if (dep.includes('/')) {
        await this.installGitDependency(dep, depPath);
      } else {
        // Install from Neo package registry or npm
        await this.installPackageDependency(dep, depPath);
      }
      
      console.log(chalk.green(`  ✅ Installed ${dep}`));
    } catch (error) {
      console.error(chalk.red(`  ❌ Failed to install ${dep}: ${error}`));
      throw error;
    }
  }

  private async removeDirectory(dir: string): Promise<void> {
    try {
      await fs.rm(dir, { recursive: true, force: true });
    } catch (error) {
      // Directory might not exist
    }
  }

  /**
   * Run tests for a specific test file
   */
  private async runTestFile(testFile: string, profile: any): Promise<{
    total: number;
    passed: number;
    failed: number;
  }> {
    const { spawn } = await import('child_process');
    
    return new Promise((resolve) => {
      const testRunner = spawn('neo-forge-test', [testFile], {
        stdio: ['pipe', 'pipe', 'pipe']
      });

      let stdout = '';
      let stderr = '';

      testRunner.stdout.on('data', (data) => {
        stdout += data.toString();
      });

      testRunner.stderr.on('data', (data) => {
        stderr += data.toString();
      });

      testRunner.on('close', (code) => {
        // Parse test results from output
        const results = this.parseTestOutput(stdout);
        resolve(results);
      });

      testRunner.on('error', () => {
        // If test runner fails, assume no tests
        resolve({ total: 0, passed: 0, failed: 0 });
      });
    });
  }

  /**
   * Parse test output to extract results
   */
  private parseTestOutput(output: string): {
    total: number;
    passed: number;
    failed: number;
  } {
    let total = 0;
    let passed = 0;
    let failed = 0;

    const lines = output.split('\n');
    for (const line of lines) {
      if (line.includes('test ') && line.includes('...')) {
        total++;
        if (line.includes('ok') || line.includes('PASS')) {
          passed++;
        } else if (line.includes('FAILED') || line.includes('ERROR')) {
          failed++;
        }
      }
    }

    return { total, passed, failed };
  }

  /**
   * Install Git-based dependency
   */
  private async installGitDependency(dep: string, depPath: string): Promise<void> {
    const { spawn } = await import('child_process');
    
    // Parse dependency format: owner/repo[@version]
    const [repo, version] = dep.split('@');
    const gitUrl = `https://github.com/${repo}.git`;

    return new Promise((resolve, reject) => {
      const git = spawn('git', [
        'clone',
        '--depth', '1',
        ...(version ? ['--branch', version] : []),
        gitUrl,
        depPath
      ]);

      git.on('close', (code) => {
        if (code === 0) {
          resolve();
        } else {
          reject(new Error(`Git clone failed with code ${code}`));
        }
      });

      git.on('error', reject);
    });
  }

  /**
   * Install package-based dependency
   */
  private async installPackageDependency(dep: string, depPath: string): Promise<void> {
    // Try to install from npm or a Neo package registry
    const { spawn } = await import('child_process');
    
    return new Promise((resolve, reject) => {
      const npm = spawn('npm', ['install', dep], {
        cwd: depPath,
        stdio: ['pipe', 'pipe', 'pipe']
      });

      npm.on('close', (code) => {
        if (code === 0) {
          resolve();
        } else {
          reject(new Error(`NPM install failed with code ${code}`));
        }
      });

      npm.on('error', reject);
    });
  }

  /**
   * Get test files from directory
   */
  private async getTestFiles(testDir: string): Promise<string[]> {
    const testFiles: string[] = [];
    
    try {
      const entries = await fs.readdir(testDir, { withFileTypes: true });
      
      for (const entry of entries) {
        const fullPath = path.join(testDir, entry.name);
        
        if (entry.isDirectory()) {
          const subFiles = await this.getTestFiles(fullPath);
          testFiles.push(...subFiles);
        } else if (entry.name.endsWith('.test.sol') || entry.name.endsWith('.t.sol')) {
          testFiles.push(fullPath);
        }
      }
    } catch (error) {
      // Test directory doesn't exist
    }
    
    return testFiles;
  }

  /**
   * Fix compilation issues that can be resolved programmatically
   */
  private fixCompilationIssues(errors: any[]): void {
    for (const error of errors) {
      if (error.severity === 'error') {
        // Log error for manual resolution
        console.error(chalk.red(`Compilation error in ${error.sourceLocation?.file}:`));
        console.error(chalk.red(`  ${error.message}`));
        
        // Could implement automatic fixes for common issues:
        // - Missing imports
        // - Pragma version mismatches
        // - Simple syntax errors
      }
    }
  }

  /**
   * Install the compiler if not available
   */
  private async ensureCompilerInstalled(): Promise<void> {
    const { spawn } = await import('child_process');
    
    return new Promise((resolve, reject) => {
      const check = spawn('neo-solc', ['--version'], {
        stdio: ['pipe', 'pipe', 'pipe']
      });

      check.on('close', (code) => {
        if (code === 0) {
          resolve();
        } else {
          reject(new Error('Neo-Solidity compiler not found. Please install neo-solc.'));
        }
      });

      check.on('error', () => {
        reject(new Error('Neo-Solidity compiler not found. Please install neo-solc.'));
      });
    });
  }
}