import { describe, it, expect, beforeAll, afterAll, beforeEach, afterEach } from 'vitest';
import { NeoSolidityCLI } from '@neo-solidity/cli-tools';
import { ProjectScaffolder } from '@neo-solidity/templates';
import { NeoForge } from '@neo-solidity/neo-foundry';
import { NeoCast } from '@neo-solidity/neo-foundry';
import { NeoABICompatibilityLayer } from '@neo-solidity/abi-router';
import * as fs from 'fs-extra';
import * as path from 'path';
import * as os from 'os';

describe('Integration Test Scenarios', () => {
  let tempDir: string;
  let cli: NeoSolidityCLI;
  let scaffolder: ProjectScaffolder;
  let forge: NeoForge;
  let cast: NeoCast;
  
  beforeAll(async () => {
    // Create temporary directory for test projects
    tempDir = await fs.mkdtemp(path.join(os.tmpdir(), 'neo-solidity-test-'));
    
    // Initialize components
    cli = new NeoSolidityCLI({
      defaults: {},
      profiles: { test: { verbose: true } },
      plugins: [],
      aliases: {}
    });
    
    scaffolder = new ProjectScaffolder();
    
    const forgeConfig = {
      src: 'src',
      out: 'out',
      libs: ['lib'],
      remappings: [],
      auto_detect_solc: true,
      solc_version: '0.8.20',
      optimizer: true,
      optimizer_runs: 200,
      via_ir: false,
      verbosity: 1,
      evm_version: 'paris'
    };
    
    forge = new NeoForge(forgeConfig, tempDir);
    cast = new NeoCast('http://127.0.0.1:10332');
  });

  afterAll(async () => {
    // Clean up temporary directory
    await fs.remove(tempDir);
  });

  describe('ERC20 Token Project Workflow', () => {
    let projectPath: string;

    beforeEach(async () => {
      projectPath = path.join(tempDir, 'erc20-project');
      await fs.ensureDir(projectPath);
    });

    afterEach(async () => {
      await fs.remove(projectPath);
    });

    it('should create, compile, test, and deploy an ERC20 token', async () => {
      // Step 1: Scaffold ERC20 project
      const scaffoldResult = await scaffolder.scaffold({
        template: 'erc20',
        name: 'MyToken',
        directory: projectPath,
        context: {
          tokenName: 'MyToken',
          tokenSymbol: 'MTK',
          totalSupply: '1000000',
          author: 'Test Developer',
          license: 'MIT'
        },
        dryRun: false,
        interactive: false,
        gitInit: true,
        install: false // Skip npm install for tests
      });

      expect(scaffoldResult.success).toBe(true);
      expect(scaffoldResult.filesCreated.length).toBeGreaterThan(0);
      expect(await fs.pathExists(path.join(projectPath, 'package.json'))).toBe(true);
      expect(await fs.pathExists(path.join(projectPath, 'contracts'))).toBe(true);

      // Step 2: Compile contracts using Forge
      process.chdir(projectPath);
      
      const buildResult = await forge.build({
        root: projectPath,
        force: true
      });

      expect(buildResult.success).toBe(true);
      expect(buildResult.contracts).toBeDefined();
      expect(Object.keys(buildResult.contracts).length).toBeGreaterThan(0);

      // Step 3: Run tests
      const testResult = await forge.test({
        root: projectPath,
        verbosity: 1
      });

      expect(testResult.success).toBe(true);
      expect(testResult.summary.passed).toBeGreaterThan(0);

      // Step 4: Generate gas report
      const gasTestResult = await forge.test({
        root: projectPath,
        gas_report: true
      });

      expect(gasTestResult.gas_report).toBeDefined();
      expect(gasTestResult.gas_report?.contracts).toBeDefined();

      // Step 5: Test contract verification
      const contractName = 'MyToken';
      const contractPath = path.join(projectPath, 'contracts', `${contractName}.sol`);
      
      if (await fs.pathExists(contractPath)) {
        const verifyResult = await forge.verify(
          '0x1234567890123456789012345678901234567890', // Mock address
          contractPath
        );
        
        // Verification might fail in test environment, but should not throw
        expect(typeof verifyResult).toBe('boolean');
      }
    });

    it('should handle complex deployment scenarios', async () => {
      // Create a project with dependencies
      const scaffoldResult = await scaffolder.scaffold({
        template: 'erc20',
        name: 'ComplexToken',
        directory: projectPath,
        context: {
          tokenName: 'ComplexToken',
          tokenSymbol: 'CTK',
          mintable: true,
          burnable: true,
          pausable: true,
          ownable: true
        }
      });

      expect(scaffoldResult.success).toBe(true);

      process.chdir(projectPath);

      // Build with optimization
      const buildResult = await forge.build({
        root: projectPath,
        optimizer: true,
        optimizer_runs: 1000,
        sizes: true
      });

      expect(buildResult.success).toBe(true);

      // Test deployment cost estimation
      const estimateScript = `
        // SPDX-License-Identifier: MIT
        pragma solidity ^0.8.20;
        
        import "forge-std/Script.sol";
        import "../contracts/ComplexToken.sol";
        
        contract DeployScript is Script {
          function run() external {
            vm.startBroadcast();
            
            ComplexToken token = new ComplexToken();
            
            vm.stopBroadcast();
          }
        }
      `;

      const scriptPath = path.join(projectPath, 'script', 'Deploy.s.sol');
      await fs.ensureDir(path.dirname(scriptPath));
      await fs.writeFile(scriptPath, estimateScript);

      // Run deployment script (dry run)
      const scriptResult = await forge.script(scriptPath, {
        rpc_url: 'http://127.0.0.1:10332'
      });

      expect(scriptResult.success).toBe(true);
      expect(scriptResult.transactions.length).toBeGreaterThanOrEqual(0);
    });
  });

  describe('DeFi Protocol Integration', () => {
    let projectPath: string;

    beforeEach(async () => {
      projectPath = path.join(tempDir, 'defi-protocol');
      await fs.ensureDir(projectPath);
    });

    afterEach(async () => {
      await fs.remove(projectPath);
    });

    it('should create and test a complete DeFi protocol', async () => {
      // Create custom DeFi project
      const scaffoldResult = await scaffolder.scaffold({
        template: 'basic',
        name: 'DeFiProtocol',
        directory: projectPath,
        context: {
          description: 'A comprehensive DeFi protocol with lending and borrowing',
          framework: 'hardhat'
        }
      });

      expect(scaffoldResult.success).toBe(true);

      // Create multiple contract files
      const contracts = [
        {
          name: 'LendingPool',
          content: `
            // SPDX-License-Identifier: MIT
            pragma solidity ^0.8.20;
            
            contract LendingPool {
              mapping(address => uint256) public deposits;
              mapping(address => uint256) public borrowed;
              
              function deposit() external payable {
                deposits[msg.sender] += msg.value;
              }
              
              function borrow(uint256 amount) external {
                require(deposits[msg.sender] >= amount * 2, "Insufficient collateral");
                borrowed[msg.sender] += amount;
              }
              
              function repay() external payable {
                require(borrowed[msg.sender] >= msg.value, "Overpayment");
                borrowed[msg.sender] -= msg.value;
              }
            }
          `
        },
        {
          name: 'PriceOracle',
          content: `
            // SPDX-License-Identifier: MIT
            pragma solidity ^0.8.20;
            
            contract PriceOracle {
              mapping(address => uint256) public prices;
              address public owner;
              
              constructor() {
                owner = msg.sender;
              }
              
              function updatePrice(address token, uint256 price) external {
                require(msg.sender == owner, "Only owner");
                prices[token] = price;
              }
              
              function getPrice(address token) external view returns (uint256) {
                return prices[token];
              }
            }
          `
        }
      ];

      const contractsDir = path.join(projectPath, 'contracts');
      await fs.ensureDir(contractsDir);

      for (const contract of contracts) {
        await fs.writeFile(
          path.join(contractsDir, `${contract.name}.sol`),
          contract.content
        );
      }

      process.chdir(projectPath);

      // Compile all contracts
      const buildResult = await forge.build({
        root: projectPath
      });

      expect(buildResult.success).toBe(true);
      expect(Object.keys(buildResult.contracts).length).toBeGreaterThanOrEqual(contracts.length);

      // Create comprehensive test suite
      const testContent = `
        // SPDX-License-Identifier: MIT
        pragma solidity ^0.8.20;
        
        import "forge-std/Test.sol";
        import "../contracts/LendingPool.sol";
        import "../contracts/PriceOracle.sol";
        
        contract DeFiProtocolTest is Test {
          LendingPool pool;
          PriceOracle oracle;
          address user1;
          address user2;
          
          function setUp() public {
            pool = new LendingPool();
            oracle = new PriceOracle();
            user1 = address(0x1);
            user2 = address(0x2);
            
            vm.deal(user1, 10 ether);
            vm.deal(user2, 10 ether);
          }
          
          function testDeposit() public {
            vm.prank(user1);
            pool.deposit{value: 1 ether}();
            
            assertEq(pool.deposits(user1), 1 ether);
          }
          
          function testBorrow() public {
            vm.prank(user1);
            pool.deposit{value: 2 ether}();
            
            vm.prank(user1);
            pool.borrow(1 ether);
            
            assertEq(pool.borrowed(user1), 1 ether);
          }
          
          function testPriceOracle() public {
            address token = address(0x123);
            uint256 price = 1000;
            
            oracle.updatePrice(token, price);
            
            assertEq(oracle.getPrice(token), price);
          }
          
          function testFailInsufficientCollateral() public {
            vm.prank(user1);
            pool.deposit{value: 1 ether}();
            
            vm.prank(user1);
            pool.borrow(2 ether); // Should fail
          }
        }
      `;

      const testPath = path.join(projectPath, 'test', 'DeFiProtocolTest.t.sol');
      await fs.ensureDir(path.dirname(testPath));
      await fs.writeFile(testPath, testContent);

      // Run comprehensive tests
      const testResult = await forge.test({
        root: projectPath,
        verbosity: 2,
        gas_report: true
      });

      expect(testResult.success).toBe(true);
      expect(testResult.summary.passed).toBeGreaterThan(0);
      expect(testResult.summary.failed).toBe(0);

      // Test coverage
      const coverageResult = await forge.test({
        root: projectPath,
        coverage: true
      });

      expect(coverageResult.coverage).toBeDefined();
      expect(coverageResult.coverage?.summary).toBeDefined();
    });
  });

  describe('ABI Compatibility Testing', () => {
    let abiLayer: NeoABICompatibilityLayer;

    beforeEach(() => {
      abiLayer = new NeoABICompatibilityLayer();
    });

    it('should handle ethers.js integration', async () => {
      // Test contract ABI
      const contractABI = [
        {
          type: 'function',
          name: 'transfer',
          inputs: [
            { name: 'to', type: 'address' },
            { name: 'amount', type: 'uint256' }
          ],
          outputs: [{ name: 'success', type: 'bool' }],
          stateMutability: 'nonpayable'
        },
        {
          type: 'event',
          name: 'Transfer',
          inputs: [
            { name: 'from', type: 'address', indexed: true },
            { name: 'to', type: 'address', indexed: true },
            { name: 'value', type: 'uint256', indexed: false }
          ],
          anonymous: false
        }
      ];

      // Register ABI
      for (const entry of contractABI) {
        if (entry.type === 'function') {
          abiLayer.registry.addFunction(entry);
        } else if (entry.type === 'event') {
          abiLayer.registry.addEvent(entry);
        }
      }

      // Test ethers.js conversion
      const ethersInterface = abiLayer.toEthersInterface();
      expect(ethersInterface).toBeDefined();

      // Test function encoding
      const encoded = abiLayer.encodeFunction('transfer', [
        '0x1234567890123456789012345678901234567890',
        '1000000000000000000' // 1 ether in wei
      ]);
      
      expect(encoded).toBeDefined();
      expect(encoded.startsWith('0x')).toBe(true);

      // Test function decoding
      const decoded = abiLayer.decodeFunction('transfer', encoded);
      expect(decoded).toHaveLength(2);
      expect(decoded[0]).toBe('0x1234567890123456789012345678901234567890');
    });

    it('should handle web3.js integration', async () => {
      const contractABI = [
        {
          type: 'function',
          name: 'balanceOf',
          inputs: [{ name: 'owner', type: 'address' }],
          outputs: [{ name: 'balance', type: 'uint256' }],
          stateMutability: 'view'
        }
      ];

      for (const entry of contractABI) {
        abiLayer.registry.addFunction(entry);
      }

      // Test web3.js conversion
      const web3ABI = abiLayer.toWeb3ABI();
      expect(Array.isArray(web3ABI)).toBe(true);
      expect(web3ABI.length).toBeGreaterThan(0);

      const functionABI = web3ABI.find(item => item.type === 'function' && item.name === 'balanceOf');
      expect(functionABI).toBeDefined();
      expect(functionABI?.constant).toBe(true);
    });

    it('should convert between Ethereum and Neo types', () => {
      // Test type conversions
      const neoValue = abiLayer.convertToNeoVM('address', '0x1234567890123456789012345678901234567890');
      expect(neoValue.type).toBe('Hash160');
      expect(neoValue.value).toBeDefined();

      const ethValue = abiLayer.convertFromNeoVM('address', neoValue);
      expect(ethValue).toBe('0x1234567890123456789012345678901234567890');

      // Test array conversion
      const arrayValue = abiLayer.convertToNeoVM('uint256[]', ['100', '200', '300']);
      expect(arrayValue.type).toBe('Array');
      expect(arrayValue.value).toHaveLength(3);

      const convertedArray = abiLayer.convertFromNeoVM('uint256[]', arrayValue);
      expect(Array.isArray(convertedArray)).toBe(true);
      expect(convertedArray).toHaveLength(3);
    });

    it('should validate input and output types', () => {
      const contractABI = [
        {
          type: 'function',
          name: 'testFunction',
          inputs: [
            { name: 'addr', type: 'address' },
            { name: 'amount', type: 'uint256' },
            { name: 'flag', type: 'bool' }
          ],
          outputs: [{ name: 'result', type: 'bool' }],
          stateMutability: 'nonpayable'
        }
      ];

      abiLayer.registry.addFunction(contractABI[0]);

      // Valid inputs
      const validInputs = [
        '0x1234567890123456789012345678901234567890',
        '1000',
        true
      ];
      
      const isValid = abiLayer.validateInput('testFunction', validInputs);
      expect(isValid).toBe(true);

      // Invalid inputs
      const invalidInputs = ['invalid_address', '1000', true];
      const isInvalid = abiLayer.validateInput('testFunction', invalidInputs);
      expect(isInvalid).toBe(false);
    });
  });

  describe('CLI Tool Integration', () => {
    it('should handle complete CLI workflows', async () => {
      // Register CLI commands
      cli.register({
        name: 'init',
        description: 'Initialize a new project',
        options: [
          {
            name: 'template',
            description: 'Project template to use',
            type: 'string',
            required: true
          },
          {
            name: 'name',
            description: 'Project name',
            type: 'string',
            required: true
          }
        ],
        action: async (args) => {
          const result = await scaffolder.scaffold({
            template: args.template,
            name: args.name,
            directory: path.join(tempDir, args.name)
          });
          
          if (result.success) {
            console.log('Project created successfully');
          } else {
            throw new Error('Project creation failed');
          }
        },
        examples: [
          {
            command: 'init --template erc20 --name MyToken',
            description: 'Create a new ERC20 token project'
          }
        ]
      });

      // Test CLI execution
      const result = await cli.execute(['node', 'cli', 'init', '--template', 'erc20', '--name', 'CLITestToken']);
      
      expect(result.success).toBe(true);
      expect(await fs.pathExists(path.join(tempDir, 'CLITestToken'))).toBe(true);
    });

    it('should handle error scenarios gracefully', async () => {
      cli.register({
        name: 'failing-command',
        description: 'A command that fails',
        options: [],
        action: async () => {
          throw new Error('Intentional failure');
        }
      });

      const result = await cli.execute(['node', 'cli', 'failing-command']);
      
      expect(result.success).toBe(false);
      expect(result.error).toBeDefined();
      expect(result.error?.message).toContain('Intentional failure');
    });
  });

  describe('End-to-End Workflow', () => {
    it('should complete a full development lifecycle', async () => {
      const projectName = 'E2EProject';
      const projectPath = path.join(tempDir, projectName);

      // 1. Scaffold project
      const scaffoldResult = await scaffolder.scaffold({
        template: 'basic',
        name: projectName,
        directory: projectPath,
        context: {
          author: 'Integration Test',
          description: 'End-to-end test project'
        }
      });

      expect(scaffoldResult.success).toBe(true);

      // 2. Create a custom contract
      const contractContent = `
        // SPDX-License-Identifier: MIT
        pragma solidity ^0.8.20;
        
        contract E2EContract {
          uint256 private value;
          address public owner;
          
          event ValueChanged(uint256 indexed oldValue, uint256 indexed newValue);
          
          constructor(uint256 _initialValue) {
            value = _initialValue;
            owner = msg.sender;
          }
          
          function getValue() external view returns (uint256) {
            return value;
          }
          
          function setValue(uint256 _newValue) external {
            require(msg.sender == owner, "Only owner can set value");
            uint256 oldValue = value;
            value = _newValue;
            emit ValueChanged(oldValue, _newValue);
          }
          
          function transferOwnership(address newOwner) external {
            require(msg.sender == owner, "Only owner can transfer ownership");
            require(newOwner != address(0), "New owner cannot be zero address");
            owner = newOwner;
          }
        }
      `;

      await fs.writeFile(
        path.join(projectPath, 'contracts', 'E2EContract.sol'),
        contractContent
      );

      process.chdir(projectPath);

      // 3. Compile
      const buildResult = await forge.build({
        root: projectPath
      });

      expect(buildResult.success).toBe(true);

      // 4. Create comprehensive tests
      const testContent = `
        // SPDX-License-Identifier: MIT
        pragma solidity ^0.8.20;
        
        import "forge-std/Test.sol";
        import "../contracts/E2EContract.sol";
        
        contract E2EContractTest is Test {
          E2EContract contract_;
          address owner;
          address user;
          
          function setUp() public {
            owner = address(this);
            user = address(0x1);
            contract_ = new E2EContract(100);
          }
          
          function testInitialState() public {
            assertEq(contract_.getValue(), 100);
            assertEq(contract_.owner(), owner);
          }
          
          function testSetValue() public {
            contract_.setValue(200);
            assertEq(contract_.getValue(), 200);
          }
          
          function testSetValueEmitsEvent() public {
            vm.expectEmit(true, true, false, false);
            emit E2EContract.ValueChanged(100, 200);
            contract_.setValue(200);
          }
          
          function testFailSetValueUnauthorized() public {
            vm.prank(user);
            contract_.setValue(200); // Should fail
          }
          
          function testTransferOwnership() public {
            contract_.transferOwnership(user);
            assertEq(contract_.owner(), user);
          }
          
          function testFailTransferToZeroAddress() public {
            contract_.transferOwnership(address(0)); // Should fail
          }
        }
      `;

      await fs.writeFile(
        path.join(projectPath, 'test', 'E2EContract.t.sol'),
        testContent
      );

      // 5. Run tests with coverage and gas reporting
      const testResult = await forge.test({
        root: projectPath,
        verbosity: 2,
        gas_report: true,
        coverage: true
      });

      expect(testResult.success).toBe(true);
      expect(testResult.summary.passed).toBeGreaterThan(0);
      expect(testResult.summary.failed).toBe(0);

      // 6. Generate deployment script
      const deployScript = `
        // SPDX-License-Identifier: MIT
        pragma solidity ^0.8.20;
        
        import "forge-std/Script.sol";
        import "../contracts/E2EContract.sol";
        
        contract DeployE2E is Script {
          function run() external {
            vm.startBroadcast();
            
            E2EContract contract_ = new E2EContract(42);
            
            console.log("E2EContract deployed at:", address(contract_));
            
            vm.stopBroadcast();
          }
        }
      `;

      const scriptPath = path.join(projectPath, 'script', 'Deploy.s.sol');
      await fs.ensureDir(path.dirname(scriptPath));
      await fs.writeFile(scriptPath, deployScript);

      // 7. Run deployment script (simulation)
      const scriptResult = await forge.script(scriptPath, {
        rpc_url: 'http://127.0.0.1:10332'
      });

      expect(scriptResult.success).toBe(true);

      // 8. Flatten for verification
      const flattenResult = await forge.flatten('contracts/E2EContract.sol');
      
      expect(flattenResult.success).toBe(true);
      expect(flattenResult.flattened_source).toBeDefined();
      expect(flattenResult.flattened_source.length).toBeGreaterThan(0);

      console.log('✅ End-to-end workflow completed successfully');
    });

    it('should handle complex multi-contract scenarios', async () => {
      const projectName = 'MultiContractProject';
      const projectPath = path.join(tempDir, projectName);

      // Create project structure
      await fs.ensureDir(path.join(projectPath, 'contracts'));

      // Create multiple interdependent contracts
      const contracts = [
        {
          name: 'Storage',
          content: `
            // SPDX-License-Identifier: MIT
            pragma solidity ^0.8.20;
            
            contract Storage {
              mapping(bytes32 => uint256) private data;
              
              function set(bytes32 key, uint256 value) external {
                data[key] = value;
              }
              
              function get(bytes32 key) external view returns (uint256) {
                return data[key];
              }
            }
          `
        },
        {
          name: 'Calculator',
          content: `
            // SPDX-License-Identifier: MIT
            pragma solidity ^0.8.20;
            
            import "./Storage.sol";
            
            contract Calculator {
              Storage private storageContract;
              
              constructor(address _storage) {
                storageContract = Storage(_storage);
              }
              
              function addAndStore(bytes32 key, uint256 a, uint256 b) external {
                uint256 result = a + b;
                storageContract.set(key, result);
              }
              
              function getStoredValue(bytes32 key) external view returns (uint256) {
                return storageContract.get(key);
              }
            }
          `
        }
      ];

      for (const contract of contracts) {
        await fs.writeFile(
          path.join(projectPath, 'contracts', `${contract.name}.sol`),
          contract.content
        );
      }

      process.chdir(projectPath);

      // Build all contracts
      const buildResult = await forge.build({
        root: projectPath
      });

      expect(buildResult.success).toBe(true);
      expect(Object.keys(buildResult.contracts)).toContain('contracts/Storage.sol');
      expect(Object.keys(buildResult.contracts)).toContain('contracts/Calculator.sol');

      // Test dependency handling
      const multiTest = `
        // SPDX-License-Identifier: MIT
        pragma solidity ^0.8.20;
        
        import "forge-std/Test.sol";
        import "../contracts/Storage.sol";
        import "../contracts/Calculator.sol";
        
        contract MultiContractTest is Test {
          Storage storage_;
          Calculator calculator;
          
          function setUp() public {
            storage_ = new Storage();
            calculator = new Calculator(address(storage_));
          }
          
          function testIntegration() public {
            bytes32 key = keccak256("test");
            
            calculator.addAndStore(key, 10, 20);
            
            uint256 result = calculator.getStoredValue(key);
            assertEq(result, 30);
            
            uint256 directResult = storage_.get(key);
            assertEq(directResult, 30);
          }
        }
      `;

      await fs.ensureDir(path.join(projectPath, 'test'));
      await fs.writeFile(
        path.join(projectPath, 'test', 'MultiContract.t.sol'),
        multiTest
      );

      const testResult = await forge.test({
        root: projectPath
      });

      expect(testResult.success).toBe(true);
      expect(testResult.summary.passed).toBeGreaterThan(0);

      console.log('✅ Multi-contract scenario completed successfully');
    });
  });
});