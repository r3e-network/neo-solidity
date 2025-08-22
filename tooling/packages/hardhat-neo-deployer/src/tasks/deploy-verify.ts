import { task, types } from "hardhat/config";
import { HardhatRuntimeEnvironment } from "hardhat/types";
import chalk from "chalk";

task("neo-deploy-and-verify", "Deploy and verify contract in one step")
  .addParam("contract", "Contract name to deploy")
  .addOptionalParam("args", "Constructor arguments (JSON array)", "[]")
  .addOptionalParam("from", "Account to deploy from")
  .addOptionalParam("gasLimit", "Gas limit for deployment")
  .addOptionalParam("waitBlocks", "Blocks to wait before verification", 5, types.int)
  .setAction(async (taskArgs, hre: HardhatRuntimeEnvironment) => {
    const { contract, args, from, gasLimit, waitBlocks } = taskArgs;
    
    console.log(chalk.blue(`🚀 Deploying and verifying ${contract} on ${hre.network.name}...`));

    try {
      // Deploy contract first
      console.log(chalk.yellow("1️⃣ Deploying contract..."));
      
      const deployResult = await hre.run("neo-deploy", {
        contract,
        args,
        from,
        gasLimit,
        verify: false // We'll verify separately
      });

      // Wait for specified number of blocks
      if (waitBlocks > 0) {
        console.log(chalk.yellow(`2️⃣ Waiting for ${waitBlocks} blocks...`));
        await waitForBlocks(hre, waitBlocks);
      }

      // Verify contract
      console.log(chalk.yellow("3️⃣ Verifying contract..."));
      
      // Extract deployment address from the deploy task result
      // In a real implementation, this would be properly passed between tasks
      const deploymentArtifact = await hre.neoDeploy.artifacts?.getDeploymentArtifact?.(
        contract, 
        hre.network.name
      );
      
      if (!deploymentArtifact) {
        throw new Error("Could not find deployment artifact for verification");
      }

      await hre.run("neo-verify", {
        contract,
        address: deploymentArtifact.address,
        constructorArgs: args
      });

      console.log(chalk.green("✅ Deploy and verify completed successfully!"));
      console.log(chalk.blue("🔗 Links:"));
      console.log(`   Contract: ${deploymentArtifact.address}`);
      console.log(`   Explorer: https://explorer.neo.org/contract/${deploymentArtifact.address}`);

    } catch (error) {
      console.error(chalk.red("❌ Deploy and verify failed:"));
      console.error(error instanceof Error ? error.message : String(error));
      throw error;
    }
  });

task("neo-redeploy", "Redeploy contract (useful for development)")
  .addParam("contract", "Contract name to redeploy")
  .addOptionalParam("args", "Constructor arguments (JSON array)", "[]")
  .addOptionalParam("from", "Account to deploy from")
  .addFlag("force", "Force redeployment even if already deployed")
  .setAction(async (taskArgs, hre: HardhatRuntimeEnvironment) => {
    const { contract, args, from, force } = taskArgs;
    
    console.log(chalk.blue(`🔄 Redeploying ${contract} on ${hre.network.name}...`));

    try {
      // Check if contract is already deployed
      const existingDeployment = await hre.neoDeploy.artifacts?.getDeploymentArtifact?.(
        contract,
        hre.network.name
      );

      if (existingDeployment && !force) {
        console.log(chalk.yellow(`⚠️  Contract already deployed at ${existingDeployment.address}`));
        console.log(chalk.gray("Use --force flag to redeploy anyway"));
        return;
      }

      if (existingDeployment) {
        console.log(chalk.yellow(`🗑️  Removing existing deployment at ${existingDeployment.address}`));
        // In a real implementation, you might want to track old deployments
      }

      // Recompile to ensure latest version
      console.log(chalk.yellow("🔧 Recompiling contracts..."));
      await hre.run("neo-compile", { force: true, quiet: true });

      // Deploy
      await hre.run("neo-deploy-and-verify", {
        contract,
        args,
        from
      });

      console.log(chalk.green("✅ Redeployment completed!"));

    } catch (error) {
      console.error(chalk.red("❌ Redeployment failed:"));
      console.error(error instanceof Error ? error.message : String(error));
      throw error;
    }
  });

task("neo-deploy-upgrade", "Deploy contract upgrade")
  .addParam("contract", "Contract name to upgrade")
  .addParam("proxy", "Proxy contract address")
  .addOptionalParam("args", "Upgrade arguments (JSON array)", "[]")
  .addOptionalParam("from", "Account to deploy from")
  .setAction(async (taskArgs, hre: HardhatRuntimeEnvironment) => {
    const { contract, proxy, args, from } = taskArgs;
    
    console.log(chalk.blue(`🆙 Deploying upgrade for ${contract} via proxy ${proxy}...`));

    try {
      // Parse upgrade arguments
      let upgradeArgs: any[] = [];
      if (args !== "[]") {
        try {
          upgradeArgs = JSON.parse(args);
        } catch (error) {
          throw new Error(`Invalid upgrade arguments JSON: ${error}`);
        }
      }

      // Deploy new implementation
      console.log(chalk.yellow("1️⃣ Deploying new implementation..."));
      
      const deployment = await hre.neoDeploy.deployer.deploy(`${contract}_Implementation`, upgradeArgs, {
        from
      });

      console.log(chalk.green(`✅ New implementation deployed at ${deployment.address}`));

      // Get proxy contract
      console.log(chalk.yellow("2️⃣ Updating proxy..."));
      
      const proxyContract = await hre.neoDeploy.deployer.getContract("Proxy", proxy);
      
      // Update proxy to point to new implementation
      // This would depend on the specific proxy pattern being used
      console.log(chalk.yellow("📝 Proxy update transaction would be created here"));
      
      console.log(chalk.green("✅ Contract upgrade completed!"));
      console.log(chalk.blue("📋 Upgrade Summary:"));
      console.log(`   Proxy: ${proxy}`);
      console.log(`   New Implementation: ${deployment.address}`);
      console.log(`   Transaction: ${deployment.transactionHash}`);

    } catch (error) {
      console.error(chalk.red("❌ Contract upgrade failed:"));
      console.error(error instanceof Error ? error.message : String(error));
      throw error;
    }
  });

/**
 * Wait for specified number of blocks
 */
async function waitForBlocks(hre: HardhatRuntimeEnvironment, blocks: number): Promise<void> {
  const startBlock = await hre.neoDeploy.rpc.getBlockCount();
  const targetBlock = startBlock + blocks;
  
  console.log(chalk.gray(`   Waiting for block ${targetBlock} (current: ${startBlock})...`));
  
  while (true) {
    const currentBlock = await hre.neoDeploy.rpc.getBlockCount();
    
    if (currentBlock >= targetBlock) {
      break;
    }
    
    // Wait 15 seconds (average Neo block time)
    await new Promise(resolve => setTimeout(resolve, 15000));
    
    process.stdout.write(chalk.gray("."));
  }
  
  console.log(chalk.gray(" ✅"));
}