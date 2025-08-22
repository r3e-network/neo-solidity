import { extendConfig, extendEnvironment, task } from "hardhat/config";
import { lazyObject } from "hardhat/plugins";
import { HardhatConfig, HardhatUserConfig, HardhatRuntimeEnvironment } from "hardhat/types";
import { NeoNetworkConfig, NeoAccount } from "@neo-solidity/types";

import "./tasks/deploy";
import "./tasks/deploy-verify";
import "./tasks/account";
import { NeoDeployer } from "./deployer";
import { NeoRpcClient } from "./rpc-client";
import { AccountManager } from "./account-manager";

// Configuration extension
declare module "hardhat/types/config" {
  export interface HardhatUserConfig {
    neoNetworks?: {
      [networkName: string]: Partial<NeoNetworkConfig> & {
        accounts?: (string | NeoAccount)[];
      };
    };
  }

  export interface HardhatConfig {
    neoNetworks: {
      [networkName: string]: NeoNetworkConfig & {
        accounts: (string | NeoAccount)[];
      };
    };
  }
}

// Runtime environment extension
declare module "hardhat/types/runtime" {
  export interface HardhatRuntimeEnvironment {
    neoDeploy: {
      deployer: NeoDeployer;
      rpc: NeoRpcClient;
      accounts: AccountManager;
    };
  }
}

// Default network configurations
const DEFAULT_NEO_NETWORKS = {
  mainnet: {
    name: 'Neo MainNet',
    rpcUrls: [
      'https://mainnet1.neo.coz.io:443',
      'https://mainnet2.neo.coz.io:443',
      'https://n3seed1.ngd.network:10332'
    ],
    magic: 860833102,
    addressVersion: 0x35,
    nativeTokens: {
      gas: {
        name: 'GasToken',
        symbol: 'GAS',
        hash: '0xd2a4cff31913016155e38e474a2c06d08be276cf',
        decimals: 8
      },
      neo: {
        name: 'NeoToken',
        symbol: 'NEO',
        hash: '0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5',
        decimals: 0
      }
    },
    accounts: []
  },
  testnet: {
    name: 'Neo TestNet',
    rpcUrls: [
      'https://testnet1.neo.coz.io:443',
      'https://testnet2.neo.coz.io:443'
    ],
    magic: 894710606,
    addressVersion: 0x35,
    nativeTokens: {
      gas: {
        name: 'GasToken',
        symbol: 'GAS',
        hash: '0xd2a4cff31913016155e38e474a2c06d08be276cf',
        decimals: 8
      },
      neo: {
        name: 'NeoToken',
        symbol: 'NEO',
        hash: '0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5',
        decimals: 0
      }
    },
    testnet: true,
    accounts: []
  },
  private: {
    name: 'Neo Private',
    rpcUrls: ['http://localhost:40332'],
    magic: 12345,
    addressVersion: 0x35,
    nativeTokens: {
      gas: {
        name: 'GasToken',
        symbol: 'GAS',
        hash: '0xd2a4cff31913016155e38e474a2c06d08be276cf',
        decimals: 8
      },
      neo: {
        name: 'NeoToken',
        symbol: 'NEO',
        hash: '0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5',
        decimals: 0
      }
    },
    testnet: true,
    accounts: []
  }
};

// Extend Hardhat configuration
extendConfig(
  (config: HardhatConfig, userConfig: Readonly<HardhatUserConfig>) => {
    // Merge default networks with user configuration
    config.neoNetworks = {} as any;
    
    // Start with default networks
    for (const [networkName, defaultConfig] of Object.entries(DEFAULT_NEO_NETWORKS)) {
      config.neoNetworks[networkName] = {
        ...defaultConfig,
        accounts: []
      };
    }
    
    // Override with user configuration
    if (userConfig.neoNetworks) {
      for (const [networkName, userNetworkConfig] of Object.entries(userConfig.neoNetworks)) {
        config.neoNetworks[networkName] = {
          ...config.neoNetworks[networkName] || {},
          ...userNetworkConfig,
          accounts: userNetworkConfig.accounts || []
        } as any;
      }
    }
  }
);

// Extend Hardhat runtime environment
extendEnvironment((hre: HardhatRuntimeEnvironment) => {
  hre.neoDeploy = lazyObject(() => {
    const networkConfig = hre.config.neoNetworks[hre.network.name];
    
    if (!networkConfig) {
      throw new Error(`Neo network configuration not found for "${hre.network.name}"`);
    }
    
    const rpc = new NeoRpcClient(networkConfig);
    const accounts = new AccountManager(networkConfig.accounts);
    const deployer = new NeoDeployer(rpc, accounts, hre.artifacts);
    
    return {
      deployer,
      rpc,
      accounts
    };
  });
});

export * from "./deployer";
export * from "./rpc-client"; 
export * from "./account-manager";
export * from "./types";