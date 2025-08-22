require("@nomicfoundation/hardhat-toolbox");
require("@r3e-network/hardhat-solc-neo");
require("@r3e-network/hardhat-neo-deployer");

/**
 * Hardhat configuration for Neo N3 Devpack
 * Author: Jimmy <jimmy@r3e.network>
 */

module.exports = {
  solidity: {
    version: "0.8.19",
    settings: {
      optimizer: {
        enabled: true,
        runs: 200
      },
      neo: {
        target: "3.0",
        optimization: 3,
        gasModel: "neo",
        devpack: {
          enabled: true,
          syscalls: "all",
          nativeContracts: "all",
          nepStandards: ["NEP-17", "NEP-11", "NEP-24"],
          libraries: ["Neo", "Storage", "Runtime"]
        }
      }
    }
  },
  
  networks: {
    neo_local: {
      url: "http://localhost:20332",
      accounts: {
        mnemonic: "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"
      },
      gas: "auto",
      gasPrice: "auto"
    },
    
    neo_testnet: {
      url: "http://seed1t5.neo.org:20332",
      accounts: process.env.PRIVATE_KEY ? [process.env.PRIVATE_KEY] : [],
      gas: "auto",
      gasPrice: "auto",
      timeout: 60000
    },
    
    neo_mainnet: {
      url: "http://seed1.neo.org:10332",
      accounts: process.env.MAINNET_PRIVATE_KEY ? [process.env.MAINNET_PRIVATE_KEY] : [],
      gas: "auto",
      gasPrice: "auto",
      timeout: 120000
    }
  },
  
  paths: {
    sources: "./contracts",
    tests: "./test",
    cache: "./cache",
    artifacts: "./artifacts"
  },
  
  mocha: {
    timeout: 40000
  },
  
  gasReporter: {
    enabled: process.env.REPORT_GAS !== undefined,
    currency: "USD",
    gasPrice: 21,
    coinmarketcap: process.env.COINMARKETCAP_API_KEY
  },
  
  contractSizer: {
    alphaSort: true,
    disambiguatePaths: false,
    runOnCompile: true,
    strict: true
  },
  
  docgen: {
    path: './docs',
    clear: true,
    runOnCompile: false,
    pages: (item, file) => {
      return file.absolutePath.startsWith(paths.sources);
    }
  },
  
  // Neo-specific configuration
  neo: {
    networks: {
      testnet: {
        rpcUrl: "http://seed1t5.neo.org:20332",
        networkMagic: 894710606,
        addressVersion: 53
      },
      mainnet: {
        rpcUrl: "http://seed1.neo.org:10332", 
        networkMagic: 860833102,
        addressVersion: 53
      }
    },
    
    deployment: {
      gasLimit: 100000000, // 1 GAS
      from: process.env.DEPLOYER_ADDRESS,
      confirmations: 1
    },
    
    verification: {
      enabled: true,
      apiKey: process.env.NEO_SCAN_API_KEY,
      apiUrl: "https://dora.coz.io/api"
    }
  }
};