import { task, types } from "hardhat/config";
import { HardhatRuntimeEnvironment } from "hardhat/types";
import chalk from "chalk";

task("neo-deploy", "Deploy contracts to Neo blockchain")
  .addParam("contract", "Contract name to deploy")
  .addOptionalParam("args", "Constructor arguments (JSON array)", "[]")
  .addOptionalParam("from", "Account to deploy from")
  .addOptionalParam("gasLimit", "Gas limit for deployment")
  .addOptionalParam("verify", "Verify contract after deployment", false, types.boolean)
  .setAction(async (taskArgs, hre: HardhatRuntimeEnvironment) => {
    const { contract, args, from, gasLimit, verify } = taskArgs;
    
    console.log(chalk.blue(`🚀 Deploying ${contract} to ${hre.network.name}...`));

    try {
      // Parse constructor arguments
      let constructorArgs: any[] = [];
      if (args !== "[]") {
        try {
          constructorArgs = JSON.parse(args);
        } catch (error) {
          throw new Error(`Invalid constructor arguments JSON: ${error}`);
        }
      }

      // Set deployment account if specified
      if (from) {
        hre.neoDeploy.accounts.setDefaultAccount(from);
      }

      // Deploy contract
      const deployment = await hre.neoDeploy.deployer.deploy(contract, constructorArgs, {
        gasLimit,
        from
      });

      console.log(chalk.green("✅ Deployment successful!"));
      console.log(chalk.blue("📋 Deployment Details:"));
      console.log(`   Contract: ${contract}`);
      console.log(`   Address: ${deployment.address}`);
      console.log(`   Script Hash: ${deployment.scriptHash}`);
      console.log(`   Transaction Hash: ${deployment.transactionHash}`);
      console.log(`   Block Number: ${deployment.blockNumber}`);
      console.log(`   Gas Used: ${deployment.gasUsed.toString()}`);

      // Verify contract if requested
      if (verify) {
        console.log(chalk.yellow("🔍 Verifying contract..."));
        
        try {
          await hre.run("neo-verify", {
            contract,
            address: deployment.address,
            constructorArgs: JSON.stringify(constructorArgs)
          });
        } catch (verifyError) {
          console.log(chalk.yellow(`⚠️  Verification failed: ${verifyError}`));
          console.log(chalk.gray("You can verify manually later using: npx hardhat neo-verify"));
        }
      }

      // Show contract interaction example
      console.log(chalk.blue("\n💡 Contract Interaction:"));
      console.log(chalk.gray(`   const contract = await hre.neoDeploy.deployer.getContract("${contract}", "${deployment.address}");`));
      console.log(chalk.gray("   // Call contract methods..."));

    } catch (error) {
      console.error(chalk.red("❌ Deployment failed:"));
      console.error(error instanceof Error ? error.message : String(error));
      throw error;
    }
  });

task("neo-deploy-batch", "Deploy multiple contracts in batch")
  .addParam("config", "Deployment configuration file path")
  .addFlag("verify", "Verify all contracts after deployment")
  .setAction(async (taskArgs, hre: HardhatRuntimeEnvironment) => {
    const { config, verify } = taskArgs;
    
    console.log(chalk.blue(`📦 Batch deploying from ${config}...`));

    try {
      // Read deployment configuration
      const fs = await import('fs/promises');
      const configContent = await fs.readFile(config, 'utf-8');
      const deploymentConfig = JSON.parse(configContent);

      if (!Array.isArray(deploymentConfig.contracts)) {
        throw new Error("Configuration must contain a 'contracts' array");
      }

      // Deploy contracts
      const deployments = await hre.neoDeploy.deployer.deployBatch(
        deploymentConfig.contracts
      );

      console.log(chalk.green(`✅ Successfully deployed ${deployments.length} contracts!`));

      // Print summary
      console.log(chalk.blue("\n📋 Deployment Summary:"));
      for (const deployment of deployments) {
        console.log(`   ${deployment.contract}: ${deployment.address}`);
      }

      // Verify all contracts if requested
      if (verify) {
        console.log(chalk.yellow("\n🔍 Verifying contracts..."));
        
        for (const deployment of deployments) {
          try {
            // Find the original config for constructor args
            const contractConfig = deploymentConfig.contracts.find(
              (c: any) => c.name === deployment.contract
            );
            
            await hre.run("neo-verify", {
              contract: deployment.contract,
              address: deployment.address,
              constructorArgs: JSON.stringify(contractConfig?.args || [])
            });
          } catch (verifyError) {
            console.log(chalk.yellow(`⚠️  Verification failed for ${deployment.contract}: ${verifyError}`));
          }
        }
      }

      // Save deployment summary
      const summaryPath = `deployments/${hre.network.name}/summary.json`;
      const fs2 = await import('fs/promises');
      await fs2.mkdir(`deployments/${hre.network.name}`, { recursive: true });
      
      const summary = {
        network: hre.network.name,
        timestamp: new Date().toISOString(),
        deployments: deployments.map(d => ({
          contract: d.contract,
          address: d.address,
          scriptHash: d.scriptHash,
          transactionHash: d.transactionHash,
          blockNumber: d.blockNumber,
          gasUsed: d.gasUsed.toString()
        }))
      };
      
      await fs2.writeFile(summaryPath, JSON.stringify(summary, null, 2));
      console.log(chalk.blue(`📄 Deployment summary saved to ${summaryPath}`));

    } catch (error) {
      console.error(chalk.red("❌ Batch deployment failed:"));
      console.error(error instanceof Error ? error.message : String(error));
      throw error;
    }
  });

task("neo-deploy-estimate", "Estimate deployment gas costs")
  .addParam("contract", "Contract name to estimate")
  .addOptionalParam("args", "Constructor arguments (JSON array)", "[]")
  .setAction(async (taskArgs, hre: HardhatRuntimeEnvironment) => {
    const { contract, args } = taskArgs;
    
    console.log(chalk.blue(`⛽ Estimating deployment gas for ${contract}...`));

    try {
      // Parse constructor arguments
      let constructorArgs: any[] = [];
      if (args !== "[]") {
        try {
          constructorArgs = JSON.parse(args);
        } catch (error) {
          throw new Error(`Invalid constructor arguments JSON: ${error}`);
        }
      }

      // Estimate gas
      const estimate = await hre.neoDeploy.deployer.estimateDeploymentGas(
        contract,
        constructorArgs
      );

      console.log(chalk.green("📊 Gas Estimation:"));
      console.log(`   System Fee: ${estimate.systemFee} GAS`);
      console.log(`   Network Fee: ${estimate.networkFee} GAS`);
      
      const totalGas = (BigInt(estimate.systemFee) + BigInt(estimate.networkFee)).toString();
      console.log(`   Total: ${totalGas} GAS`);

      // Convert to approximate USD (would need real price feed)
      const gasPrice = 50; // Mock $50 per GAS
      const estimatedUSD = (Number(totalGas) / 1e8 * gasPrice).toFixed(2);
      console.log(`   Estimated Cost: ~$${estimatedUSD} USD`);

    } catch (error) {
      console.error(chalk.red("❌ Gas estimation failed:"));
      console.error(error instanceof Error ? error.message : String(error));
      throw error;
    }
  });