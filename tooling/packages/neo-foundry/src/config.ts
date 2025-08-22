import { promises as fs } from "fs";
import path from "path";
import { NeoSolidityConfig, NeoNetworkConfig } from "@neo-solidity/types";

/**
 * Neo-Foundry configuration
 */
export interface NeoFoundryConfig {
  /** Profile configurations */
  profile: {
    [profileName: string]: {
      /** Source directory */
      src: string;
      /** Test directory */
      test: string;
      /** Script directory */
      script: string;
      /** Output directory */
      out: string;
      /** Library directories */
      libs: string[];
      /** Remappings */
      remappings: string[];
      /** Neo-Solidity compiler settings */
      neoSolc: NeoSolidityConfig;
      /** Neo networks configuration */
      networks: {
        [networkName: string]: NeoNetworkConfig & {
          /** RPC URL */
          url: string;
          /** Private keys for deployment */
          privateKeys?: string[];
          /** Mnemonic for account derivation */
          mnemonic?: {
            phrase: string;
            derivationPath: string;
          };
        };
      };
      /** Build settings */
      build: {
        /** Enable incremental compilation */
        incremental: boolean;
        /** Build cache directory */
        cacheDir: string;
        /** Enable size optimization */
        sizeOptimization: boolean;
      };
      /** Testing settings */
      testing: {
        /** Fork testing network */
        forkNetwork?: string;
        /** Fork block number */
        forkBlockNumber?: number;
        /** Gas reporting */
        gasReports: boolean;
        /** Coverage reporting */
        coverage: boolean;
      };
    };
  };
  /** Default profile */
  defaultProfile: string;
}

/**
 * Default configuration
 */
export const DEFAULT_NEO_FOUNDRY_CONFIG: NeoFoundryConfig = {
  profile: {
    default: {
      src: "src",
      test: "test",
      script: "script",
      out: "out",
      libs: ["lib"],
      remappings: [],
      neoSolc: {
        version: "latest",
        optimizer: {
          enabled: true,
          runs: 200
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
          gasCostModel: "hybrid",
          storageOptimization: true,
          eventOptimization: true
        }
      },
      networks: {
        mainnet: {
          name: "Neo MainNet",
          url: "https://mainnet1.neo.coz.io:443",
          rpcUrls: ["https://mainnet1.neo.coz.io:443"],
          magic: 860833102,
          addressVersion: 0x35,
          nativeTokens: {
            gas: {
              name: "GasToken",
              symbol: "GAS",
              hash: "0xd2a4cff31913016155e38e474a2c06d08be276cf",
              decimals: 8
            },
            neo: {
              name: "NeoToken",
              symbol: "NEO",
              hash: "0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5",
              decimals: 0
            }
          }
        },
        testnet: {
          name: "Neo TestNet",
          url: "https://testnet1.neo.coz.io:443",
          rpcUrls: ["https://testnet1.neo.coz.io:443"],
          magic: 894710606,
          addressVersion: 0x35,
          nativeTokens: {
            gas: {
              name: "GasToken",
              symbol: "GAS",
              hash: "0xd2a4cff31913016155e38e474a2c06d08be276cf",
              decimals: 8
            },
            neo: {
              name: "NeoToken",
              symbol: "NEO",
              hash: "0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5",
              decimals: 0
            }
          },
          testnet: true
        }
      },
      build: {
        incremental: true,
        cacheDir: "cache",
        sizeOptimization: true
      },
      testing: {
        gasReports: true,
        coverage: true
      }
    }
  },
  defaultProfile: "default"
};

/**
 * Configuration manager
 */
export class ConfigManager {
  private config: NeoFoundryConfig;
  private configPath: string;

  constructor(configPath = "foundry.toml") {
    this.configPath = configPath;
    this.config = DEFAULT_NEO_FOUNDRY_CONFIG;
  }

  /**
   * Load configuration from file
   */
  async loadConfig(): Promise<NeoFoundryConfig> {
    try {
      const configExists = await this.fileExists(this.configPath);
      if (!configExists) {
        // Create default config file
        await this.saveConfig();
        return this.config;
      }

      const content = await fs.readFile(this.configPath, "utf-8");
      
      // Parse TOML-like format (simplified - would use proper TOML parser)
      const parsed = this.parseSimpleToml(content);
      
      // Merge with defaults
      this.config = this.mergeConfig(DEFAULT_NEO_FOUNDRY_CONFIG, parsed);
      
      return this.config;
    } catch (error) {
      console.warn(`Failed to load config from ${this.configPath}, using defaults:`, error);
      return this.config;
    }
  }

  /**
   * Save configuration to file
   */
  async saveConfig(): Promise<void> {
    const tomlContent = this.configToToml(this.config);
    await fs.writeFile(this.configPath, tomlContent);
  }

  /**
   * Get configuration
   */
  getConfig(): NeoFoundryConfig {
    return this.config;
  }

  /**
   * Get profile configuration
   */
  getProfile(profileName?: string): NeoFoundryConfig["profile"][string] {
    const profile = profileName || this.config.defaultProfile;
    return this.config.profile[profile];
  }

  /**
   * Set profile configuration
   */
  setProfile(profileName: string, profileConfig: Partial<NeoFoundryConfig["profile"][string]>): void {
    this.config.profile[profileName] = {
      ...this.config.profile[profileName] || this.config.profile[this.config.defaultProfile],
      ...profileConfig
    };
  }

  /**
   * Get network configuration
   */
  getNetwork(networkName: string, profileName?: string): NeoFoundryConfig["profile"][string]["networks"][string] | undefined {
    const profile = this.getProfile(profileName);
    return profile.networks[networkName];
  }

  /**
   * Initialize project with default configuration
   */
  async initProject(projectPath = "."): Promise<void> {
    const configPath = path.join(projectPath, "foundry.toml");
    
    // Create directories
    const profile = this.getProfile();
    await fs.mkdir(path.join(projectPath, profile.src), { recursive: true });
    await fs.mkdir(path.join(projectPath, profile.test), { recursive: true });
    await fs.mkdir(path.join(projectPath, profile.script), { recursive: true });
    await fs.mkdir(path.join(projectPath, profile.out), { recursive: true });
    
    // Create sample contract
    const sampleContract = `// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

contract Counter {
    uint256 public number;

    function setNumber(uint256 newNumber) public {
        number = newNumber;
    }

    function increment() public {
        number++;
    }
}`;

    await fs.writeFile(path.join(projectPath, profile.src, "Counter.sol"), sampleContract);

    // Create sample test
    const sampleTest = `// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "forge-std/Test.sol";
import "../src/Counter.sol";

contract CounterTest is Test {
    Counter public counter;

    function setUp() public {
        counter = new Counter();
        counter.setNumber(0);
    }

    function test_Increment() public {
        counter.increment();
        assertEq(counter.number(), 1);
    }

    function testFuzz_SetNumber(uint256 x) public {
        counter.setNumber(x);
        assertEq(counter.number(), x);
    }
}`;

    await fs.writeFile(path.join(projectPath, profile.test, "Counter.t.sol"), sampleTest);

    // Save configuration
    const configManager = new ConfigManager(configPath);
    await configManager.saveConfig();
  }

  // Private methods

  private async fileExists(filePath: string): Promise<boolean> {
    try {
      await fs.access(filePath);
      return true;
    } catch {
      return false;
    }
  }

  private parseSimpleToml(content: string): Partial<NeoFoundryConfig> {
    // Simplified TOML parser - in production, use a proper TOML library
    const lines = content.split('\n');
    const result: any = {};
    
    let currentSection: string[] = [];
    
    for (const line of lines) {
      const trimmed = line.trim();
      if (!trimmed || trimmed.startsWith('#')) continue;
      
      if (trimmed.startsWith('[') && trimmed.endsWith(']')) {
        currentSection = trimmed.slice(1, -1).split('.');
      } else if (trimmed.includes('=')) {
        const [key, value] = trimmed.split('=', 2);
        this.setNestedValue(result, [...currentSection, key.trim()], this.parseValue(value.trim()));
      }
    }
    
    return result;
  }

  private parseValue(value: string): any {
    // Remove quotes
    if ((value.startsWith('"') && value.endsWith('"')) || 
        (value.startsWith("'") && value.endsWith("'"))) {
      return value.slice(1, -1);
    }
    
    // Parse arrays
    if (value.startsWith('[') && value.endsWith(']')) {
      return value.slice(1, -1).split(',').map(v => this.parseValue(v.trim()));
    }
    
    // Parse booleans
    if (value === 'true') return true;
    if (value === 'false') return false;
    
    // Parse numbers
    const num = Number(value);
    if (!isNaN(num)) return num;
    
    return value;
  }

  private setNestedValue(obj: any, path: string[], value: any): void {
    let current = obj;
    for (let i = 0; i < path.length - 1; i++) {
      if (!current[path[i]]) current[path[i]] = {};
      current = current[path[i]];
    }
    current[path[path.length - 1]] = value;
  }

  private configToToml(config: NeoFoundryConfig): string {
    // Convert config to TOML format
    let toml = `# Neo-Foundry Configuration\n\n`;
    toml += `default_profile = "${config.defaultProfile}"\n\n`;
    
    for (const [profileName, profile] of Object.entries(config.profile)) {
      toml += `[profile.${profileName}]\n`;
      toml += `src = "${profile.src}"\n`;
      toml += `test = "${profile.test}"\n`;
      toml += `script = "${profile.script}"\n`;
      toml += `out = "${profile.out}"\n`;
      toml += `libs = [${profile.libs.map(lib => `"${lib}"`).join(', ')}]\n`;
      
      if (profile.remappings.length > 0) {
        toml += `remappings = [${profile.remappings.map(r => `"${r}"`).join(', ')}]\n`;
      }
      
      toml += `\n[profile.${profileName}.build]\n`;
      toml += `incremental = ${profile.build.incremental}\n`;
      toml += `cache_dir = "${profile.build.cacheDir}"\n`;
      toml += `size_optimization = ${profile.build.sizeOptimization}\n`;
      
      toml += `\n[profile.${profileName}.testing]\n`;
      toml += `gas_reports = ${profile.testing.gasReports}\n`;
      toml += `coverage = ${profile.testing.coverage}\n`;
      
      if (profile.testing.forkNetwork) {
        toml += `fork_network = "${profile.testing.forkNetwork}"\n`;
      }
      
      toml += `\n`;
    }
    
    return toml;
  }

  private mergeConfig(base: NeoFoundryConfig, override: Partial<NeoFoundryConfig>): NeoFoundryConfig {
    // Deep merge configuration objects
    return {
      ...base,
      ...override,
      profile: {
        ...base.profile,
        ...override.profile
      }
    };
  }
}