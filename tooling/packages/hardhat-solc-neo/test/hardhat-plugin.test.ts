import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest';
import { HardhatRuntimeEnvironment } from 'hardhat/types';
import { NeoSolidityCompiler } from '../src/compiler';
import { ArtifactManager } from '../src/artifacts';
import { DebugManager } from '../src/debug';
import { GasProfiler } from '../src/profiler';
import { DeploymentManager } from '../src/deployment';
import { NetworkManager } from '../src/network';
import { SourceMapGenerator } from '../src/sourcemap';
import { OptimizationEngine } from '../src/optimizer';
import { NeoHardhatConfig } from '@neo-solidity/types';
import * as fs from 'fs-extra';
import * as path from 'path';

// Mock filesystem
vi.mock('fs-extra');

describe('Hardhat Neo-Solidity Plugin', () => {
  let mockHRE: Partial<HardhatRuntimeEnvironment>;
  let mockConfig: NeoHardhatConfig;
  let tempDir: string;

  beforeEach(async () => {
    tempDir = '/tmp/test-project';
    
    mockConfig = {
      solidity: {
        version: '0.8.20',
        settings: {
          optimizer: {
            enabled: true,
            runs: 200
          },
          outputSelection: {
            '*': {
              '*': ['abi', 'evm.bytecode', 'evm.deployedBytecode']
            }
          },
          neo: {
            generateNef: true,
            generateManifest: true,
            optimizeGas: true,
            debugInfo: true
          }
        }
      },
      networks: {
        hardhat: {
          magic: 860833102,
          addressVersion: 53,
          rpc: {
            url: 'http://127.0.0.1:10332'
          },
          accounts: [],
          gasLimit: '9007199254740991',
          gasPrice: '0',
          blockGasLimit: '9007199254740991'
        }
      },
      paths: {
        sources: './contracts',
        artifacts: './artifacts',
        cache: './cache',
        tests: './test'
      },
      neo: {
        rpcUrl: 'http://127.0.0.1:10332',
        privateKey: '',
        addressVersion: 53,
        magic: 860833102,
        gasLimit: '9007199254740991',
        gasPrice: '0'
      }
    };

    mockHRE = {
      config: {
        neoSolc: mockConfig,
        paths: {
          sources: path.join(tempDir, 'contracts'),
          artifacts: path.join(tempDir, 'artifacts'),
          cache: path.join(tempDir, 'cache'),
          tests: path.join(tempDir, 'test')
        }
      } as any,
      network: {
        name: 'hardhat'
      }
    };

    // Setup filesystem mocks
    vi.mocked(fs.pathExists).mockResolvedValue(true);
    vi.mocked(fs.ensureDir).mockResolvedValue(undefined);
    vi.mocked(fs.readFile).mockResolvedValue('mock file content');
    vi.mocked(fs.writeFile).mockResolvedValue(undefined);
    vi.mocked(fs.readJson).mockResolvedValue({});
    vi.mocked(fs.writeJson).mockResolvedValue(undefined);
  });

  afterEach(() => {
    vi.clearAllMocks();
  });

  describe('NeoSolidityCompiler', () => {
    it('should initialize with correct configuration', () => {
      const compiler = new NeoSolidityCompiler(mockConfig, mockHRE.config!.paths);
      
      expect(compiler).toBeDefined();
    });

    it('should compile a simple contract', async () => {
      const compiler = new NeoSolidityCompiler(mockConfig, mockHRE.config!.paths);
      
      const sourceCode = `
        // SPDX-License-Identifier: MIT
        pragma solidity ^0.8.20;
        
        contract SimpleStorage {
            uint256 public value;
            
            function setValue(uint256 _value) public {
                value = _value;
            }
        }
      `;

      // Mock successful compilation
      vi.mocked(fs.readFile).mockResolvedValueOnce(sourceCode);
      
      const result = await compiler.compile(['SimpleStorage.sol']);
      
      expect(result).toBeDefined();
      expect(result.success).toBe(true);
      expect(result.contracts).toBeDefined();
    });

    it('should handle compilation errors gracefully', async () => {
      const compiler = new NeoSolidityCompiler(mockConfig, mockHRE.config!.paths);
      
      const invalidSourceCode = `
        pragma solidity ^0.8.20;
        contract Invalid {
          invalid syntax here
        }
      `;

      vi.mocked(fs.readFile).mockResolvedValueOnce(invalidSourceCode);
      
      const result = await compiler.compile(['Invalid.sol']);
      
      expect(result.success).toBe(false);
      expect(result.errors.length).toBeGreaterThan(0);
    });

    it('should generate Neo-specific artifacts', async () => {
      const compiler = new NeoSolidityCompiler(mockConfig, mockHRE.config!.paths);
      
      const sourceCode = `
        pragma solidity ^0.8.20;
        contract Test {}
      `;

      vi.mocked(fs.readFile).mockResolvedValueOnce(sourceCode);
      
      const result = await compiler.compile(['Test.sol']);
      
      if (result.success && result.contracts['Test.sol']?.['Test']) {
        const contract = result.contracts['Test.sol']['Test'];
        expect(contract.neo).toBeDefined();
        expect(contract.neo.nef).toBeDefined();
        expect(contract.neo.manifest).toBeDefined();
      }
    });
  });

  describe('ArtifactManager', () => {
    let artifactManager: ArtifactManager;

    beforeEach(() => {
      artifactManager = new ArtifactManager(path.join(tempDir, 'artifacts'));
    });

    it('should save and retrieve build artifacts', async () => {
      const artifact = {
        _format: 'hh-sol-artifact-1',
        contractName: 'TestContract',
        sourceName: 'TestContract.sol',
        abi: [],
        bytecode: '0x608060405234801561001057600080fd5b50',
        deployedBytecode: '0x608060405234801561001057600080fd5b50',
        linkReferences: {},
        deployedLinkReferences: {},
        contract: {
          abi: [],
          evm: {
            bytecode: { object: '0x608060405234801561001057600080fd5b50' },
            deployedBytecode: { object: '0x608060405234801561001057600080fd5b50' }
          },
          neo: {
            nef: 'mock-nef-data',
            manifest: 'mock-manifest-data'
          }
        }
      };

      await artifactManager.saveBuildArtifact(artifact);
      
      expect(fs.writeFile).toHaveBeenCalled();
      
      vi.mocked(fs.readFile).mockResolvedValueOnce(JSON.stringify(artifact));
      
      const retrieved = await artifactManager.getBuildArtifact('TestContract');
      
      expect(retrieved).toEqual(artifact);
    });

    it('should handle missing artifacts gracefully', async () => {
      vi.mocked(fs.readFile).mockRejectedValueOnce({ code: 'ENOENT' });
      
      const result = await artifactManager.getBuildArtifact('NonExistentContract');
      
      expect(result).toBeNull();
    });

    it('should validate artifact integrity', async () => {
      const validArtifact = {
        _format: 'hh-sol-artifact-1',
        contractName: 'TestContract',
        sourceName: 'TestContract.sol',
        abi: [],
        bytecode: '0x608060405234801561001057600080fd5b50',
        deployedBytecode: '0x608060405234801561001057600080fd5b50',
        linkReferences: {},
        deployedLinkReferences: {},
        contract: {
          abi: [],
          evm: {
            bytecode: { object: '0x608060405234801561001057600080fd5b50' },
            deployedBytecode: { object: '0x608060405234801561001057600080fd5b50' }
          },
          neo: {
            nef: 'mock-nef-data',
            manifest: 'mock-manifest-data'
          }
        }
      };

      const validation = await artifactManager.validateArtifact(validArtifact);
      
      expect(validation.valid).toBe(true);
      expect(validation.errors.length).toBe(0);
    });

    it('should detect invalid artifacts', async () => {
      const invalidArtifact = {
        // Missing required fields
        contractName: '',
        sourceName: '',
        contract: null
      } as any;

      const validation = await artifactManager.validateArtifact(invalidArtifact);
      
      expect(validation.valid).toBe(false);
      expect(validation.errors.length).toBeGreaterThan(0);
    });
  });

  describe('DeploymentManager', () => {
    let deploymentManager: DeploymentManager;

    beforeEach(() => {
      deploymentManager = new DeploymentManager(mockConfig, mockHRE.network!);
    });

    it('should deploy a contract successfully', async () => {
      const mockArtifact = {
        abi: [
          {
            type: 'constructor',
            inputs: []
          }
        ],
        bytecode: '0x608060405234801561001057600080fd5b50'
      };

      vi.mocked(fs.readJson).mockResolvedValueOnce(mockArtifact);
      vi.mocked(fs.pathExists).mockResolvedValueOnce(true);

      // Mock ethers contract deployment
      const mockReceipt = {
        transactionHash: '0xabc123',
        blockNumber: 12345,
        blockHash: '0xdef456',
        gasUsed: { toString: () => '21000' },
        gasPrice: { toString: () => '20000000000' }
      };

      const result = await deploymentManager.deployContract('TestContract', [], {
        gasLimit: '100000',
        skipDryRun: true
      });

      expect(result.success).toBe(true);
      expect(result.contractName).toBe('TestContract');
      expect(result.transactionHash).toBeDefined();
    });

    it('should estimate deployment costs', async () => {
      const mockArtifact = {
        abi: [],
        bytecode: '0x608060405234801561001057600080fd5b50'
      };

      vi.mocked(fs.readJson).mockResolvedValueOnce(mockArtifact);

      const estimate = await deploymentManager.estimateDeploymentCost('TestContract');

      expect(estimate.gasEstimate).toBeDefined();
      expect(estimate.costInWei).toBeDefined();
      expect(estimate.costInEth).toBeDefined();
    });

    it('should generate deployment reports', async () => {
      vi.mocked(fs.readdir).mockResolvedValueOnce(['TestContract.json']);
      vi.mocked(fs.readJson).mockResolvedValueOnce({
        contractName: 'TestContract',
        gasUsed: '21000',
        gasPrice: '20000000000'
      });

      const report = await deploymentManager.generateDeploymentReport();

      expect(report.totalDeployments).toBe(1);
      expect(report.totalGasUsed).toBeDefined();
      expect(report.totalCost).toBeDefined();
    });
  });

  describe('NetworkManager', () => {
    let networkManager: NetworkManager;

    beforeEach(() => {
      networkManager = new NetworkManager(mockConfig.networks);
    });

    it('should manage network configurations', () => {
      const networks = networkManager.listNetworks();
      
      expect(networks).toContain('hardhat');
      
      const hardhatConfig = networkManager.getNetwork('hardhat');
      expect(hardhatConfig).toBeDefined();
      expect(hardhatConfig?.magic).toBe(860833102);
    });

    it('should validate network configurations', () => {
      expect(() => {
        networkManager.addNetwork('invalid', {
          magic: 0,
          addressVersion: 0,
          rpc: { url: '' },
          accounts: [],
          gasLimit: '0',
          gasPrice: '0',
          blockGasLimit: '0'
        });
      }).toThrow();
    });

    it('should get network status', async () => {
      const status = await networkManager.getNetworkStatus('hardhat');
      
      expect(status).toBeDefined();
      expect(typeof status.connected).toBe('boolean');
      expect(typeof status.blockNumber).toBe('number');
      expect(typeof status.chainId).toBe('number');
    });

    it('should perform health checks', async () => {
      const health = await networkManager.healthCheck('hardhat');
      
      expect(health).toBeDefined();
      expect(typeof health.healthy).toBe('boolean');
      expect(Array.isArray(health.checks)).toBe(true);
    });
  });

  describe('GasProfiler', () => {
    let gasProfiler: GasProfiler;

    beforeEach(() => {
      gasProfiler = new GasProfiler(mockConfig);
    });

    it('should start and stop profiling sessions', async () => {
      await gasProfiler.startProfiling();
      
      const profile = await gasProfiler.stopProfiling();
      
      expect(profile).toBeDefined();
      expect(profile.id).toBeDefined();
      expect(profile.startTime).toBeDefined();
      expect(profile.endTime).toBeDefined();
      expect(profile.duration).toBeGreaterThan(0);
    });

    it('should profile individual transactions', async () => {
      const mockTxHash = '0x1234567890abcdef';
      
      // Mock provider responses
      const mockTx = {
        hash: mockTxHash,
        from: '0xabc123',
        to: '0xdef456',
        value: { toString: () => '0' },
        gasPrice: { toString: () => '20000000000' },
        gasLimit: { toString: () => '21000' }
      };

      const mockReceipt = {
        transactionHash: mockTxHash,
        blockNumber: 12345,
        gasUsed: { toString: () => '21000' },
        status: 1,
        logs: []
      };

      const profile = await gasProfiler.profileTransaction(mockTxHash);
      
      expect(profile).toBeDefined();
      expect(profile.hash).toBe(mockTxHash);
      expect(profile.gasUsed).toBeDefined();
    });

    it('should generate optimization suggestions', async () => {
      await gasProfiler.startProfiling();
      const profile = await gasProfiler.stopProfiling();
      
      expect(profile.optimizations).toBeDefined();
      expect(Array.isArray(profile.optimizations)).toBe(true);
    });
  });

  describe('DebugManager', () => {
    let debugManager: DebugManager;

    beforeEach(() => {
      debugManager = new DebugManager(mockConfig);
    });

    it('should start debug sessions', async () => {
      const mockTxHash = '0x1234567890abcdef';
      
      // Mock transaction data
      const mockTx = {
        hash: mockTxHash,
        to: '0xcontract123',
        from: '0xuser456'
      };

      const mockReceipt = {
        transactionHash: mockTxHash,
        contractAddress: '0xcontract123'
      };

      const session = await debugManager.startDebugSession(mockTxHash);
      
      expect(session).toBeDefined();
      expect(session.id).toBeDefined();
      expect(session.transactionHash).toBe(mockTxHash);
    });

    it('should manage breakpoints', async () => {
      const breakpoint = await debugManager.setBreakpoint('TestContract.sol', 10);
      
      expect(breakpoint).toBeDefined();
      expect(breakpoint.source).toBe('TestContract.sol');
      expect(breakpoint.line).toBe(10);
      expect(breakpoint.enabled).toBe(true);
      
      const removed = await debugManager.removeBreakpoint(breakpoint.id);
      expect(removed).toBe(true);
    });

    it('should evaluate expressions', async () => {
      const mockTxHash = '0x1234567890abcdef';
      const session = await debugManager.startDebugSession(mockTxHash);
      
      const result = await debugManager.evaluateExpression(session.id, 'stack[0]');
      
      expect(result).toBeDefined();
    });
  });

  describe('Integration Tests', () => {
    it('should compile, deploy, and verify a complete workflow', async () => {
      const compiler = new NeoSolidityCompiler(mockConfig, mockHRE.config!.paths);
      const artifactManager = new ArtifactManager(path.join(tempDir, 'artifacts'));
      const deploymentManager = new DeploymentManager(mockConfig, mockHRE.network!);

      const sourceCode = `
        pragma solidity ^0.8.20;
        contract Integration {
            uint256 public value;
            function setValue(uint256 _value) public { value = _value; }
        }
      `;

      // Mock file system
      vi.mocked(fs.readFile).mockResolvedValueOnce(sourceCode);
      
      // Step 1: Compile
      const compileResult = await compiler.compile(['Integration.sol']);
      expect(compileResult.success).toBe(true);

      // Step 2: Save artifacts
      if (compileResult.success && compileResult.contracts['Integration.sol']?.['Integration']) {
        const artifact = {
          _format: 'hh-sol-artifact-1',
          contractName: 'Integration',
          sourceName: 'Integration.sol',
          abi: compileResult.contracts['Integration.sol']['Integration'].abi,
          bytecode: compileResult.contracts['Integration.sol']['Integration'].evm.bytecode.object,
          deployedBytecode: compileResult.contracts['Integration.sol']['Integration'].evm.deployedBytecode.object,
          linkReferences: {},
          deployedLinkReferences: {},
          contract: compileResult.contracts['Integration.sol']['Integration']
        };

        await artifactManager.saveBuildArtifact(artifact);
        expect(fs.writeFile).toHaveBeenCalled();
      }

      // Step 3: Deploy (mocked)
      vi.mocked(fs.readJson).mockResolvedValueOnce({
        abi: [],
        bytecode: '0x608060405234801561001057600080fd5b50'
      });
      vi.mocked(fs.pathExists).mockResolvedValueOnce(true);

      const deployResult = await deploymentManager.deployContract('Integration', [], {
        skipDryRun: true
      });

      expect(deployResult.success).toBe(true);
      expect(deployResult.contractName).toBe('Integration');
    });

    it('should handle error scenarios gracefully', async () => {
      const compiler = new NeoSolidityCompiler(mockConfig, mockHRE.config!.paths);

      // Test with non-existent file
      vi.mocked(fs.readFile).mockRejectedValueOnce({ code: 'ENOENT' });
      
      const result = await compiler.compile(['NonExistent.sol']);
      
      expect(result.success).toBe(false);
      expect(result.errors.length).toBeGreaterThan(0);
    });
  });
});