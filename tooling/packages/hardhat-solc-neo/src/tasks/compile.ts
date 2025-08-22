import { task } from "hardhat/config";
import { HardhatRuntimeEnvironment } from "hardhat/types";
import { promises as fs } from "fs";
import path from "path";
import chalk from "chalk";
import { BuildArtifact, BuildInfo } from "@neo-solidity/types";

task("neo-compile", "Compile contracts using Neo-Solidity compiler")
  .addOptionalParam("force", "Force recompilation", false, types.boolean)
  .addOptionalParam("quiet", "Suppress output", false, types.boolean)
  .setAction(async (taskArgs, hre: HardhatRuntimeEnvironment) => {
    const { force, quiet } = taskArgs;
    
    if (!quiet) {
      console.log(chalk.blue("🔧 Compiling contracts with Neo-Solidity..."));
    }

    try {
      // Get source files
      const sourceFiles = await getSourceFiles(hre.config.paths.sources);
      
      if (sourceFiles.length === 0) {
        if (!quiet) {
          console.log(chalk.yellow("No Solidity files found"));
        }
        return;
      }

      // Check if compilation is needed
      if (!force && !quiet) {
        const needsCompilation = await checkCompilationNeeded(sourceFiles, hre);
        if (!needsCompilation) {
          console.log(chalk.green("✅ Contracts are up to date"));
          return;
        }
      }

      // Read source content
      const sources: { [fileName: string]: { content: string } } = {};
      for (const filePath of sourceFiles) {
        const content = await fs.readFile(filePath, "utf-8");
        const relativePath = path.relative(hre.config.paths.sources, filePath);
        sources[relativePath] = { content };
      }

      if (!quiet) {
        console.log(chalk.blue(`📝 Compiling ${sourceFiles.length} files...`));
      }

      // Compile with Neo-Solidity
      const compilationOutput = await hre.neoSolc.compiler.compile(sources);

      // Save build info
      const buildInfo: BuildInfo = {
        id: generateBuildId(),
        solcVersion: "0.8.19", // Would get from actual compiler
        neoSolcVersion: "0.1.0", // Would get from Neo compiler
        input: { language: "Solidity", sources, settings: hre.config.neoSolc },
        output: compilationOutput,
        metadata: {
          timestamp: new Date().toISOString(),
          duration: 0, // Would measure actual duration
          sourceFiles: Object.keys(sources),
          optimization: hre.config.neoSolc.optimizer || { enabled: false, runs: 0 },
          neo: hre.config.neoSolc.neo || {
            gasCostModel: "hybrid",
            storageOptimization: true,
            eventOptimization: true
          }
        }
      };

      // Save artifacts
      let compiledContracts = 0;
      for (const fileName of Object.keys(compilationOutput.contracts)) {
        for (const contractName of Object.keys(compilationOutput.contracts[fileName])) {
          const contract = compilationOutput.contracts[fileName][contractName];
          
          const artifact: BuildArtifact = {
            contractName,
            sourceName: fileName,
            metadata: {
              compiler: {
                version: buildInfo.neoSolcVersion,
                settings: hre.config.neoSolc
              },
              buildTime: buildInfo.metadata.timestamp,
              environment: {
                nodeVersion: process.version,
                platform: process.platform,
                architecture: process.arch
              },
              dependencies: {} // Would extract from package.json
            },
            contract,
            buildInfo: buildInfo.id
          };

          await hre.neoSolc.artifacts.saveBuildArtifact(artifact);
          compiledContracts++;
        }
      }

      if (!quiet) {
        console.log(chalk.green(`✅ Successfully compiled ${compiledContracts} contracts`));
        
        // Print contract sizes
        await printContractSizes(compilationOutput);
        
        // Print warnings if any
        if (compilationOutput.errors) {
          const warnings = compilationOutput.errors.filter(e => e.severity === "warning");
          if (warnings.length > 0) {
            console.log(chalk.yellow(`⚠️  ${warnings.length} warnings`));
          }
        }
      }

    } catch (error) {
      console.error(chalk.red("❌ Compilation failed:"));
      console.error(error instanceof Error ? error.message : String(error));
      throw error;
    }
  });

/**
 * Get all Solidity source files
 */
async function getSourceFiles(sourcesPath: string): Promise<string[]> {
  const files: string[] = [];
  
  async function scan(dir: string) {
    const entries = await fs.readdir(dir, { withFileTypes: true });
    
    for (const entry of entries) {
      const fullPath = path.join(dir, entry.name);
      
      if (entry.isDirectory()) {
        await scan(fullPath);
      } else if (entry.name.endsWith(".sol")) {
        files.push(fullPath);
      }
    }
  }
  
  try {
    await scan(sourcesPath);
  } catch (error) {
    if ((error as any).code !== "ENOENT") {
      throw error;
    }
  }
  
  return files;
}

/**
 * Check if compilation is needed
 */
async function checkCompilationNeeded(sourceFiles: string[], hre: HardhatRuntimeEnvironment): Promise<boolean> {
  try {
    // Get modification times of source files
    const sourceStats = await Promise.all(
      sourceFiles.map(async (file) => {
        const stat = await fs.stat(file);
        return { file, mtime: stat.mtime };
      })
    );
    
    const latestSource = sourceStats.reduce((latest, current) => 
      current.mtime > latest.mtime ? current : latest
    );

    // Check if any artifacts exist and their modification time
    const artifacts = await hre.neoSolc.artifacts.getAllBuildArtifacts();
    if (artifacts.length === 0) {
      return true; // No artifacts, need to compile
    }

    // Check if source is newer than any artifact
    for (const artifact of artifacts) {
      const buildTime = new Date(artifact.metadata.buildTime);
      if (latestSource.mtime > buildTime) {
        return true;
      }
    }

    return false;
  } catch (error) {
    // If we can't determine, assume compilation is needed
    return true;
  }
}

/**
 * Print contract sizes
 */
async function printContractSizes(compilationOutput: any) {
  console.log(chalk.blue("\n📊 Contract sizes:"));
  
  for (const fileName of Object.keys(compilationOutput.contracts)) {
    for (const contractName of Object.keys(compilationOutput.contracts[fileName])) {
      const contract = compilationOutput.contracts[fileName][contractName];
      
      // Calculate sizes
      const evmBytecodeSize = contract.evm.bytecode.object.length / 2; // hex to bytes
      const neoScriptSize = contract.neo.nef.script.length / 2;
      
      console.log(`  ${contractName}:`);
      console.log(`    EVM bytecode: ${chalk.cyan(evmBytecodeSize.toLocaleString())} bytes`);
      console.log(`    Neo script: ${chalk.cyan(neoScriptSize.toLocaleString())} bytes`);
      
      // Size warnings
      if (evmBytecodeSize > 24576) { // Ethereum contract size limit
        console.log(chalk.yellow(`    ⚠️  Large contract size (>${24576} bytes)`));
      }
    }
  }
}

/**
 * Generate unique build ID
 */
function generateBuildId(): string {
  return `build-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
}