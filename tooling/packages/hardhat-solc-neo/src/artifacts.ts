import { promises as fs } from "fs";
import path from "path";
import { 
  BuildArtifact, 
  BuildInfo, 
  DeploymentArtifact,
  ArtifactManager as IArtifactManager,
  ArtifactStorageConfig,
  ArtifactValidationResult,
  ArtifactComparison,
  ArtifactSearchCriteria,
  ArtifactStatistics
} from "@neo-solidity/types";
import { HardhatPluginError } from "hardhat/plugins";
import Debug from "debug";

const debug = Debug("hardhat:neo-solidity:artifacts");

/**
 * Artifact manager implementation for Neo-Solidity
 */
export class ArtifactManager implements IArtifactManager {
  private config: ArtifactStorageConfig;

  constructor(baseDir: string) {
    this.config = {
      baseDir,
      buildDir: path.join(baseDir, "contracts"),
      deploymentDir: path.join(baseDir, "deployments"),
      buildInfoDir: path.join(baseDir, "build-info"),
      cacheDir: path.join(baseDir, ".cache"),
      compression: {
        enabled: false,
        algorithm: "gzip"
      },
      cleanup: {
        maxAge: 30, // days
        maxSize: 100 // MB
      }
    };
  }

  /**
   * Get build artifact by contract name
   */
  async getBuildArtifact(contractName: string): Promise<BuildArtifact | null> {
    debug(`Getting build artifact for ${contractName}`);
    
    try {
      const artifactPath = path.join(this.config.buildDir, `${contractName}.json`);
      const content = await fs.readFile(artifactPath, "utf-8");
      return JSON.parse(content);
    } catch (error) {
      if ((error as any).code === "ENOENT") {
        return null;
      }
      throw new HardhatPluginError(
        "@neo-solidity/hardhat-solc-neo",
        `Failed to read build artifact: ${error}`
      );
    }
  }

  /**
   * Save build artifact
   */
  async saveBuildArtifact(artifact: BuildArtifact): Promise<void> {
    debug(`Saving build artifact for ${artifact.contractName}`);
    
    try {
      await fs.mkdir(this.config.buildDir, { recursive: true });
      
      const artifactPath = path.join(this.config.buildDir, `${artifact.contractName}.json`);
      await fs.writeFile(artifactPath, JSON.stringify(artifact, null, 2));
      
      debug(`Build artifact saved to ${artifactPath}`);
    } catch (error) {
      throw new HardhatPluginError(
        "@neo-solidity/hardhat-solc-neo",
        `Failed to save build artifact: ${error}`
      );
    }
  }

  /**
   * Get all build artifacts
   */
  async getAllBuildArtifacts(): Promise<BuildArtifact[]> {
    debug("Getting all build artifacts");
    
    try {
      const files = await this.getArtifactFiles(this.config.buildDir);
      const artifacts: BuildArtifact[] = [];
      
      for (const file of files) {
        if (file.endsWith(".json")) {
          const content = await fs.readFile(path.join(this.config.buildDir, file), "utf-8");
          artifacts.push(JSON.parse(content));
        }
      }
      
      return artifacts;
    } catch (error) {
      throw new HardhatPluginError(
        "@neo-solidity/hardhat-solc-neo",
        `Failed to get build artifacts: ${error}`
      );
    }
  }

  /**
   * Get deployment artifact
   */
  async getDeploymentArtifact(contractName: string, networkName: string): Promise<DeploymentArtifact | null> {
    debug(`Getting deployment artifact for ${contractName} on ${networkName}`);
    
    try {
      const deploymentPath = path.join(this.config.deploymentDir, networkName, `${contractName}.json`);
      const content = await fs.readFile(deploymentPath, "utf-8");
      return JSON.parse(content);
    } catch (error) {
      if ((error as any).code === "ENOENT") {
        return null;
      }
      throw new HardhatPluginError(
        "@neo-solidity/hardhat-solc-neo",
        `Failed to read deployment artifact: ${error}`
      );
    }
  }

  /**
   * Save deployment artifact
   */
  async saveDeploymentArtifact(artifact: DeploymentArtifact): Promise<void> {
    debug(`Saving deployment artifact for ${artifact.contractName} on ${artifact.networkName}`);
    
    try {
      const networkDir = path.join(this.config.deploymentDir, artifact.networkName);
      await fs.mkdir(networkDir, { recursive: true });
      
      const deploymentPath = path.join(networkDir, `${artifact.contractName}.json`);
      await fs.writeFile(deploymentPath, JSON.stringify(artifact, null, 2));
      
      debug(`Deployment artifact saved to ${deploymentPath}`);
    } catch (error) {
      throw new HardhatPluginError(
        "@neo-solidity/hardhat-solc-neo",
        `Failed to save deployment artifact: ${error}`
      );
    }
  }

  /**
   * Get all deployments for a network
   */
  async getNetworkDeployments(networkName: string): Promise<DeploymentArtifact[]> {
    debug(`Getting deployments for network ${networkName}`);
    
    try {
      const networkDir = path.join(this.config.deploymentDir, networkName);
      const files = await this.getArtifactFiles(networkDir);
      const deployments: DeploymentArtifact[] = [];
      
      for (const file of files) {
        if (file.endsWith(".json")) {
          const content = await fs.readFile(path.join(networkDir, file), "utf-8");
          deployments.push(JSON.parse(content));
        }
      }
      
      return deployments;
    } catch (error) {
      if ((error as any).code === "ENOENT") {
        return [];
      }
      throw new HardhatPluginError(
        "@neo-solidity/hardhat-solc-neo",
        `Failed to get network deployments: ${error}`
      );
    }
  }

  /**
   * Clear all artifacts
   */
  async clearArtifacts(): Promise<void> {
    debug("Clearing all artifacts");
    
    try {
      await this.removeDirectory(this.config.buildDir);
      await this.removeDirectory(this.config.deploymentDir);
      await this.removeDirectory(this.config.buildInfoDir);
      await this.removeDirectory(this.config.cacheDir);
    } catch (error) {
      throw new HardhatPluginError(
        "@neo-solidity/hardhat-solc-neo",
        `Failed to clear artifacts: ${error}`
      );
    }
  }

  /**
   * Export artifacts to directory
   */
  async exportArtifacts(outputDir: string): Promise<void> {
    debug(`Exporting artifacts to ${outputDir}`);
    
    try {
      await fs.mkdir(outputDir, { recursive: true });
      
      // Export build artifacts
      const buildArtifacts = await this.getAllBuildArtifacts();
      for (const artifact of buildArtifacts) {
        const exportPath = path.join(outputDir, "contracts", `${artifact.contractName}.json`);
        await fs.mkdir(path.dirname(exportPath), { recursive: true });
        await fs.writeFile(exportPath, JSON.stringify(artifact, null, 2));
      }
      
      // Export deployment artifacts
      const networkDirs = await this.getDirectories(this.config.deploymentDir);
      for (const networkName of networkDirs) {
        const deployments = await this.getNetworkDeployments(networkName);
        for (const deployment of deployments) {
          const exportPath = path.join(outputDir, "deployments", networkName, `${deployment.contractName}.json`);
          await fs.mkdir(path.dirname(exportPath), { recursive: true });
          await fs.writeFile(exportPath, JSON.stringify(deployment, null, 2));
        }
      }
      
      debug("Artifacts exported successfully");
    } catch (error) {
      throw new HardhatPluginError(
        "@neo-solidity/hardhat-solc-neo",
        `Failed to export artifacts: ${error}`
      );
    }
  }

  /**
   * Import artifacts from directory
   */
  async importArtifacts(inputDir: string): Promise<void> {
    debug(`Importing artifacts from ${inputDir}`);
    
    try {
      // Import build artifacts
      const contractsDir = path.join(inputDir, "contracts");
      if (await this.directoryExists(contractsDir)) {
        const files = await this.getArtifactFiles(contractsDir);
        for (const file of files) {
          if (file.endsWith(".json")) {
            const content = await fs.readFile(path.join(contractsDir, file), "utf-8");
            const artifact: BuildArtifact = JSON.parse(content);
            await this.saveBuildArtifact(artifact);
          }
        }
      }
      
      // Import deployment artifacts
      const deploymentsDir = path.join(inputDir, "deployments");
      if (await this.directoryExists(deploymentsDir)) {
        const networks = await this.getDirectories(deploymentsDir);
        for (const networkName of networks) {
          const networkDir = path.join(deploymentsDir, networkName);
          const files = await this.getArtifactFiles(networkDir);
          for (const file of files) {
            if (file.endsWith(".json")) {
              const content = await fs.readFile(path.join(networkDir, file), "utf-8");
              const artifact: DeploymentArtifact = JSON.parse(content);
              await this.saveDeploymentArtifact(artifact);
            }
          }
        }
      }
      
      debug("Artifacts imported successfully");
    } catch (error) {
      throw new HardhatPluginError(
        "@neo-solidity/hardhat-solc-neo",
        `Failed to import artifacts: ${error}`
      );
    }
  }

  /**
   * Validate artifact integrity
   */
  async validateArtifact(artifact: BuildArtifact): Promise<ArtifactValidationResult> {
    const result: ArtifactValidationResult = {
      valid: true,
      errors: [],
      warnings: [],
      integrity: {
        checksumValid: true,
        signatureValid: true,
        timestampValid: true
      }
    };

    // Validate required fields
    if (!artifact.contractName) {
      result.errors.push("Missing contract name");
      result.valid = false;
    }

    if (!artifact.sourceName) {
      result.errors.push("Missing source name");
      result.valid = false;
    }

    if (!artifact.contract) {
      result.errors.push("Missing contract data");
      result.valid = false;
    } else {
      // Validate Neo-specific fields
      if (!artifact.contract.neo) {
        result.errors.push("Missing Neo-specific compilation output");
        result.valid = false;
      } else {
        if (!artifact.contract.neo.nef) {
          result.errors.push("Missing NEF data");
          result.valid = false;
        }
        
        if (!artifact.contract.neo.manifest) {
          result.errors.push("Missing manifest data");
          result.valid = false;
        }
      }
    }

    return result;
  }

  /**
   * Compare two artifacts
   */
  async compareArtifacts(artifact1: BuildArtifact, artifact2: BuildArtifact): Promise<ArtifactComparison> {
    const differences: string[] = [];
    const breakingChanges: string[] = [];
    const nonBreakingChanges: string[] = [];

    // Compare basic properties
    if (artifact1.contractName !== artifact2.contractName) {
      differences.push('Contract name differs');
    }

    if (artifact1.sourceName !== artifact2.sourceName) {
      differences.push('Source name differs');
    }

    // Compare bytecode
    if (artifact1.bytecode !== artifact2.bytecode) {
      differences.push('Bytecode differs');
      breakingChanges.push('Bytecode change detected');
    }

    // Compare ABI
    const abiComparison = this.compareABI(artifact1.abi, artifact2.abi);
    differences.push(...abiComparison.differences);
    breakingChanges.push(...abiComparison.breakingChanges);
    nonBreakingChanges.push(...abiComparison.nonBreakingChanges);

    // Compare Neo-specific data
    if (artifact1.neo && artifact2.neo) {
      if (artifact1.neo.nef !== artifact2.neo.nef) {
        differences.push('NEF differs');
        breakingChanges.push('NEF change detected');
      }

      if (JSON.stringify(artifact1.neo.manifest) !== JSON.stringify(artifact2.neo.manifest)) {
        differences.push('Manifest differs');
        const manifestComparison = this.compareManifests(artifact1.neo.manifest, artifact2.neo.manifest);
        breakingChanges.push(...manifestComparison.breaking);
        nonBreakingChanges.push(...manifestComparison.nonBreaking);
      }
    }

    const identical = differences.length === 0;
    const upgradeable = breakingChanges.length === 0;

    return {
      identical,
      differences,
      compatibility: {
        upgradeable,
        breakingChanges,
        nonBreakingChanges
      }
    };
  }

  /**
   * Search artifacts by criteria
   */
  async searchArtifacts(criteria: ArtifactSearchCriteria): Promise<BuildArtifact[]> {
    const allArtifacts = await this.getAllBuildArtifacts();
    
    return allArtifacts.filter(artifact => {
      if (criteria.contractName) {
        if (typeof criteria.contractName === "string") {
          if (!artifact.contractName.includes(criteria.contractName)) {
            return false;
          }
        } else {
          if (!criteria.contractName.test(artifact.contractName)) {
            return false;
          }
        }
      }
      
      // Add more filtering logic as needed
      return true;
    });
  }

  /**
   * Get artifact statistics
   */
  async getStatistics(): Promise<ArtifactStatistics> {
    const buildArtifacts = await this.getAllBuildArtifacts();
    const allNetworks = await this.getDirectories(this.config.deploymentDir);
    
    let totalDeployments = 0;
    const networkDistribution: { [key: string]: number } = {};
    
    for (const network of allNetworks) {
      const deployments = await this.getNetworkDeployments(network);
      totalDeployments += deployments.length;
      networkDistribution[network] = deployments.length;
    }

    return {
      totalBuildArtifacts: buildArtifacts.length,
      totalDeploymentArtifacts: totalDeployments,
      storageUsage: {
        total: 0, // Would calculate actual storage usage
        buildArtifacts: 0,
        deploymentArtifacts: 0,
        buildInfo: 0,
        cache: 0
      },
      compilerVersions: {},
      networkDistribution,
      recentActivity: {
        buildsLastWeek: 0,
        deploymentsLastWeek: 0,
        verificationsLastWeek: 0
      }
    };
  }

  // Helper methods

  private async getArtifactFiles(dir: string): Promise<string[]> {
    try {
      return await fs.readdir(dir);
    } catch (error) {
      if ((error as any).code === "ENOENT") {
        return [];
      }
      throw error;
    }
  }

  private async getDirectories(dir: string): Promise<string[]> {
    try {
      const entries = await fs.readdir(dir, { withFileTypes: true });
      return entries
        .filter(entry => entry.isDirectory())
        .map(entry => entry.name);
    } catch (error) {
      if ((error as any).code === "ENOENT") {
        return [];
      }
      throw error;
    }
  }

  private async directoryExists(dir: string): Promise<boolean> {
    try {
      const stat = await fs.stat(dir);
      return stat.isDirectory();
    } catch {
      return false;
    }
  }

  private async removeDirectory(dir: string): Promise<void> {
    try {
      await fs.rm(dir, { recursive: true, force: true });
    } catch (error) {
      if ((error as any).code !== "ENOENT") {
        throw error;
      }
    }
  }

  /**
   * Compare ABI interfaces
   */
  private compareABI(abi1: any[], abi2: any[]): {
    differences: string[];
    breakingChanges: string[];
    nonBreakingChanges: string[];
  } {
    const differences: string[] = [];
    const breakingChanges: string[] = [];
    const nonBreakingChanges: string[] = [];

    // Create maps for easier comparison
    const functions1 = new Map(abi1.filter(item => item.type === 'function').map(item => [item.name, item]));
    const functions2 = new Map(abi2.filter(item => item.type === 'function').map(item => [item.name, item]));
    
    const events1 = new Map(abi1.filter(item => item.type === 'event').map(item => [item.name, item]));
    const events2 = new Map(abi2.filter(item => item.type === 'event').map(item => [item.name, item]));

    // Check for removed functions (breaking)
    for (const [name, func] of functions1) {
      if (!functions2.has(name)) {
        differences.push(`Function '${name}' removed`);
        breakingChanges.push(`Function '${name}' removed`);
      } else {
        // Compare function signatures
        const func2 = functions2.get(name)!;
        const sigDiff = this.compareFunctionSignatures(func, func2);
        if (sigDiff.length > 0) {
          differences.push(`Function '${name}' signature changed: ${sigDiff.join(', ')}`);
          breakingChanges.push(`Function '${name}' signature changed`);
        }
      }
    }

    // Check for added functions (non-breaking)
    for (const [name] of functions2) {
      if (!functions1.has(name)) {
        differences.push(`Function '${name}' added`);
        nonBreakingChanges.push(`Function '${name}' added`);
      }
    }

    // Check for removed events (potentially breaking)
    for (const [name] of events1) {
      if (!events2.has(name)) {
        differences.push(`Event '${name}' removed`);
        breakingChanges.push(`Event '${name}' removed`);
      }
    }

    // Check for added events (non-breaking)
    for (const [name] of events2) {
      if (!events1.has(name)) {
        differences.push(`Event '${name}' added`);
        nonBreakingChanges.push(`Event '${name}' added`);
      }
    }

    return { differences, breakingChanges, nonBreakingChanges };
  }

  /**
   * Compare function signatures
   */
  private compareFunctionSignatures(func1: any, func2: any): string[] {
    const differences: string[] = [];

    // Compare inputs
    if (JSON.stringify(func1.inputs) !== JSON.stringify(func2.inputs)) {
      differences.push('inputs changed');
    }

    // Compare outputs
    if (JSON.stringify(func1.outputs) !== JSON.stringify(func2.outputs)) {
      differences.push('outputs changed');
    }

    // Compare state mutability
    if (func1.stateMutability !== func2.stateMutability) {
      differences.push(`state mutability changed from '${func1.stateMutability}' to '${func2.stateMutability}'`);
    }

    return differences;
  }

  /**
   * Compare Neo manifests
   */
  private compareManifests(manifest1: any, manifest2: any): {
    breaking: string[];
    nonBreaking: string[];
  } {
    const breaking: string[] = [];
    const nonBreaking: string[] = [];

    // Compare basic properties
    if (manifest1.name !== manifest2.name) {
      breaking.push('Contract name changed');
    }

    if (manifest1.abi?.hash !== manifest2.abi?.hash) {
      breaking.push('ABI hash changed');
    }

    // Compare permissions
    if (JSON.stringify(manifest1.permissions) !== JSON.stringify(manifest2.permissions)) {
      const oldPerms = manifest1.permissions?.length || 0;
      const newPerms = manifest2.permissions?.length || 0;
      
      if (newPerms > oldPerms) {
        nonBreaking.push('Permissions added');
      } else if (newPerms < oldPerms) {
        breaking.push('Permissions removed');
      } else {
        breaking.push('Permissions modified');
      }
    }

    // Compare trusts
    if (JSON.stringify(manifest1.trusts) !== JSON.stringify(manifest2.trusts)) {
      nonBreaking.push('Trusts modified');
    }

    // Compare groups
    if (JSON.stringify(manifest1.groups) !== JSON.stringify(manifest2.groups)) {
      nonBreaking.push('Groups modified');
    }

    return { breaking, nonBreaking };
  }
}