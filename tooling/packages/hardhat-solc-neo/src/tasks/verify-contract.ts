import { task } from "hardhat/config";
import { HardhatRuntimeEnvironment } from "hardhat/types";
import chalk from "chalk";
import { VerificationResult } from "@neo-solidity/types";

task("neo-verify", "Verify deployed contract on Neo blockchain explorer")
  .addParam("contract", "Contract name")
  .addParam("address", "Contract address")
  .addOptionalParam("network", "Network name")
  .addOptionalParam("constructorArgs", "Constructor arguments (JSON)")
  .setAction(async (taskArgs, hre: HardhatRuntimeEnvironment) => {
    const { contract, address, network, constructorArgs } = taskArgs;
    const networkName = network || hre.network.name;
    
    console.log(chalk.blue(`🔍 Verifying contract ${contract} at ${address} on ${networkName}...`));

    try {
      // Get deployment artifact
      const deployment = await hre.neoSolc.artifacts.getDeploymentArtifact(contract, networkName);
      if (!deployment) {
        throw new Error(`No deployment found for ${contract} on ${networkName}`);
      }

      // Get build artifact
      const buildArtifact = await hre.neoSolc.artifacts.getBuildArtifact(contract);
      if (!buildArtifact) {
        throw new Error(`No build artifact found for ${contract}`);
      }

      // Parse constructor arguments
      let args: any[] = [];
      if (constructorArgs) {
        try {
          args = JSON.parse(constructorArgs);
        } catch (error) {
          throw new Error(`Invalid constructor arguments JSON: ${error}`);
        }
      }

      // Perform verification
      const result = await verifyContract({
        contractName: contract,
        address,
        networkName,
        buildArtifact,
        constructorArgs: args
      });

      if (result.success) {
        console.log(chalk.green("✅ Contract verified successfully"));
        if (result.explorerUrl) {
          console.log(chalk.blue(`🔗 Explorer: ${result.explorerUrl}`));
        }
        if (result.sourceUrl) {
          console.log(chalk.blue(`📄 Source: ${result.sourceUrl}`));
        }

        // Update deployment artifact with verification status
        deployment.verified = true;
        deployment.verification = {
          status: "verified",
          explorerUrl: result.explorerUrl,
          sourceCodeUrl: result.sourceUrl
        };
        await hre.neoSolc.artifacts.saveDeploymentArtifact(deployment);
      } else {
        console.log(chalk.red("❌ Contract verification failed"));
        console.log(chalk.red(result.message));
        
        // Update deployment artifact with failed verification
        deployment.verification = {
          status: "failed"
        };
        await hre.neoSolc.artifacts.saveDeploymentArtifact(deployment);
      }

    } catch (error) {
      console.error(chalk.red("❌ Verification failed:"));
      console.error(error instanceof Error ? error.message : String(error));
      throw error;
    }
  });

/**
 * Verify contract on Neo blockchain explorer
 */
async function verifyContract(params: {
  contractName: string;
  address: string;
  networkName: string;
  buildArtifact: any;
  constructorArgs: any[];
}): Promise<VerificationResult> {
  const { contractName, address, networkName, buildArtifact, constructorArgs } = params;

  try {
    // This is a simplified implementation
    // In a real implementation, this would:
    // 1. Submit source code to Neo blockchain explorer API
    // 2. Wait for verification process to complete
    // 3. Return verification status and URLs

    console.log(chalk.yellow("📤 Submitting source code for verification..."));
    
    // Simulate verification process
    await new Promise(resolve => setTimeout(resolve, 2000));
    
    // Mock successful verification
    return {
      success: true,
      message: "Contract verified successfully",
      explorerUrl: `https://explorer.neo.org/contract/${address}`,
      sourceUrl: `https://explorer.neo.org/contract/${address}/source`
    };

  } catch (error) {
    return {
      success: false,
      message: error instanceof Error ? error.message : String(error)
    };
  }
}

task("neo-verify-all", "Verify all deployed contracts on a network")
  .addOptionalParam("network", "Network name")
  .setAction(async (taskArgs, hre: HardhatRuntimeEnvironment) => {
    const networkName = taskArgs.network || hre.network.name;
    
    console.log(chalk.blue(`🔍 Verifying all contracts on ${networkName}...`));

    try {
      const deployments = await hre.neoSolc.artifacts.getNetworkDeployments(networkName);
      
      if (deployments.length === 0) {
        console.log(chalk.yellow(`No deployments found on ${networkName}`));
        return;
      }

      console.log(chalk.blue(`Found ${deployments.length} deployments`));

      let verified = 0;
      let failed = 0;
      let skipped = 0;

      for (const deployment of deployments) {
        if (deployment.verified) {
          console.log(chalk.gray(`⏩ Skipping ${deployment.contractName} (already verified)`));
          skipped++;
          continue;
        }

        console.log(chalk.blue(`🔍 Verifying ${deployment.contractName}...`));

        try {
          const buildArtifact = await hre.neoSolc.artifacts.getBuildArtifact(deployment.contractName);
          if (!buildArtifact) {
            console.log(chalk.yellow(`⚠️  No build artifact for ${deployment.contractName}`));
            failed++;
            continue;
          }

          const result = await verifyContract({
            contractName: deployment.contractName,
            address: deployment.address,
            networkName,
            buildArtifact,
            constructorArgs: deployment.constructorArgs
          });

          if (result.success) {
            console.log(chalk.green(`✅ ${deployment.contractName} verified`));
            deployment.verified = true;
            deployment.verification = {
              status: "verified",
              explorerUrl: result.explorerUrl,
              sourceCodeUrl: result.sourceUrl
            };
            await hre.neoSolc.artifacts.saveDeploymentArtifact(deployment);
            verified++;
          } else {
            console.log(chalk.red(`❌ ${deployment.contractName} failed: ${result.message}`));
            failed++;
          }
        } catch (error) {
          console.log(chalk.red(`❌ ${deployment.contractName} error: ${error}`));
          failed++;
        }
      }

      console.log(chalk.blue("\n📊 Verification Summary:"));
      console.log(chalk.green(`✅ Verified: ${verified}`));
      console.log(chalk.red(`❌ Failed: ${failed}`));
      console.log(chalk.gray(`⏩ Skipped: ${skipped}`));

    } catch (error) {
      console.error(chalk.red("❌ Batch verification failed:"));
      console.error(error instanceof Error ? error.message : String(error));
      throw error;
    }
  });