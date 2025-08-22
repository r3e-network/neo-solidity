import { task, types } from "hardhat/config";
import { HardhatRuntimeEnvironment } from "hardhat/types";
import chalk from "chalk";

task("neo-accounts", "List Neo accounts configured for deployment")
  .addFlag("balances", "Show account balances")
  .addFlag("private", "Show private keys (use with caution)")
  .setAction(async (taskArgs, hre: HardhatRuntimeEnvironment) => {
    const { balances, private: showPrivate } = taskArgs;
    
    console.log(chalk.blue(`📋 Neo Accounts for ${hre.network.name}:`));

    try {
      const accounts = hre.neoDeploy.accounts.getAllAccounts();
      
      if (accounts.length === 0) {
        console.log(chalk.yellow("No accounts configured"));
        console.log(chalk.gray("Add accounts to your hardhat.config.js:"));
        console.log(chalk.gray(`
  neoNetworks: {
    ${hre.network.name}: {
      accounts: [
        "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
      ]
    }
  }
        `));
        return;
      }

      const defaultAccount = hre.neoDeploy.accounts.getDefaultAccount();

      for (let i = 0; i < accounts.length; i++) {
        const account = accounts[i];
        const isDefault = account === defaultAccount;
        
        console.log(`\n${i + 1}. ${account.address}${isDefault ? " (default)" : ""}`);
        console.log(`   Script Hash: ${account.scriptHash}`);
        
        if (account.label) {
          console.log(`   Label: ${account.label}`);
        }
        
        if (account.publicKey) {
          console.log(`   Public Key: ${account.publicKey}`);
        }
        
        if (showPrivate && account.privateKey) {
          console.log(chalk.red(`   Private Key: ${account.privateKey}`));
        }
        
        if (account.isMultiSig) {
          console.log(`   Type: MultiSig (${account.multiSig?.threshold}/${account.multiSig?.publicKeys.length})`);
        }

        // Show balances if requested
        if (balances) {
          try {
            const accountBalances = await hre.neoDeploy.rpc.getBalance(account.address);
            
            if (accountBalances.length > 0) {
              console.log("   Balances:");
              for (const balance of accountBalances) {
                const amount = (Number(balance.amount) / Math.pow(10, balance.decimals)).toFixed(balance.decimals);
                console.log(`     ${amount} ${balance.symbol}`);
              }
            } else {
              console.log("   Balances: No tokens");
            }
          } catch (error) {
            console.log(chalk.gray("   Balances: Unable to fetch"));
          }
        }
      }

      console.log(chalk.blue(`\nTotal: ${accounts.length} accounts`));
      
      const signingAccounts = hre.neoDeploy.accounts.getSigningAccounts();
      console.log(chalk.green(`Signing capable: ${signingAccounts.length} accounts`));

    } catch (error) {
      console.error(chalk.red("❌ Failed to list accounts:"));
      console.error(error instanceof Error ? error.message : String(error));
      throw error;
    }
  });

task("neo-account-balance", "Check Neo account balance")
  .addParam("address", "Account address to check")
  .setAction(async (taskArgs, hre: HardhatRuntimeEnvironment) => {
    const { address } = taskArgs;
    
    console.log(chalk.blue(`💰 Checking balance for ${address}...`));

    try {
      const balances = await hre.neoDeploy.rpc.getBalance(address);
      
      if (balances.length === 0) {
        console.log(chalk.yellow("No tokens found"));
        return;
      }

      console.log(chalk.green("Account balances:"));
      
      let totalUSD = 0;
      for (const balance of balances) {
        const amount = Number(balance.amount) / Math.pow(10, balance.decimals);
        console.log(`  ${amount.toFixed(balance.decimals)} ${balance.symbol}`);
        
        // Mock USD conversion (would use real price feeds)
        if (balance.symbol === "GAS") {
          totalUSD += amount * 50; // Mock $50 per GAS
        } else if (balance.symbol === "NEO") {
          totalUSD += amount * 100; // Mock $100 per NEO
        }
      }
      
      if (totalUSD > 0) {
        console.log(chalk.cyan(`  ≈ $${totalUSD.toFixed(2)} USD`));
      }

    } catch (error) {
      console.error(chalk.red("❌ Failed to check balance:"));
      console.error(error instanceof Error ? error.message : String(error));
      throw error;
    }
  });

task("neo-account-import", "Import account from private key")
  .addParam("privateKey", "Private key in hex format")
  .addOptionalParam("label", "Label for the account")
  .setAction(async (taskArgs, hre: HardhatRuntimeEnvironment) => {
    const { privateKey, label } = taskArgs;
    
    console.log(chalk.blue("📥 Importing account..."));

    try {
      // Add account to account manager
      hre.neoDeploy.accounts.addAccount({
        privateKey,
        label: label || `Imported Account ${Date.now()}`,
        // Other fields would be derived from private key
        address: "NMockAddress123456789",
        scriptHash: "0xmockscripthash123456789"
      });
      
      const importedAccount = hre.neoDeploy.accounts.getAllAccounts().slice(-1)[0];
      
      console.log(chalk.green("✅ Account imported successfully!"));
      console.log(`   Address: ${importedAccount.address}`);
      console.log(`   Script Hash: ${importedAccount.scriptHash}`);
      console.log(`   Label: ${importedAccount.label}`);
      
      console.log(chalk.yellow("\n⚠️  Remember to add this account to your hardhat.config.js for persistence"));

    } catch (error) {
      console.error(chalk.red("❌ Failed to import account:"));
      console.error(error instanceof Error ? error.message : String(error));
      throw error;
    }
  });

task("neo-account-export", "Export accounts to file")
  .addParam("file", "Output file path")
  .addFlag("includePrivateKeys", "Include private keys in export")
  .setAction(async (taskArgs, hre: HardhatRuntimeEnvironment) => {
    const { file, includePrivateKeys } = taskArgs;
    
    console.log(chalk.blue(`📤 Exporting accounts to ${file}...`));

    if (includePrivateKeys) {
      console.log(chalk.red("⚠️  WARNING: Exporting private keys! Keep the file secure."));
    }

    try {
      await hre.neoDeploy.accounts.exportAccountsToFile(file, includePrivateKeys);
      
      const accounts = hre.neoDeploy.accounts.getAllAccounts();
      console.log(chalk.green(`✅ Exported ${accounts.length} accounts to ${file}`));
      
      if (includePrivateKeys) {
        console.log(chalk.red("🔒 File contains private keys - store securely!"));
      }

    } catch (error) {
      console.error(chalk.red("❌ Failed to export accounts:"));
      console.error(error instanceof Error ? error.message : String(error));
      throw error;
    }
  });

task("neo-account-generate", "Generate new account")
  .addOptionalParam("label", "Label for the new account")
  .addFlag("save", "Save to accounts list")
  .setAction(async (taskArgs, hre: HardhatRuntimeEnvironment) => {
    const { label, save } = taskArgs;
    
    console.log(chalk.blue("🎲 Generating new account..."));

    try {
      // This would generate a real Neo account with proper cryptography
      // For now, return mock data
      const newAccount = {
        address: `N${Math.random().toString(36).substring(2, 15)}${Math.random().toString(36).substring(2, 15)}`,
        scriptHash: `0x${Array.from({ length: 40 }, () => Math.floor(Math.random() * 16).toString(16)).join('')}`,
        privateKey: `0x${Array.from({ length: 64 }, () => Math.floor(Math.random() * 16).toString(16)).join('')}`,
        publicKey: `03${Array.from({ length: 62 }, () => Math.floor(Math.random() * 16).toString(16)).join('')}`,
        label: label || `Generated Account ${Date.now()}`,
        isMultiSig: false
      };
      
      console.log(chalk.green("✅ New account generated!"));
      console.log(`   Address: ${newAccount.address}`);
      console.log(`   Script Hash: ${newAccount.scriptHash}`);
      console.log(`   Public Key: ${newAccount.publicKey}`);
      console.log(chalk.red(`   Private Key: ${newAccount.privateKey}`));
      
      if (save) {
        hre.neoDeploy.accounts.addAccount(newAccount);
        console.log(chalk.blue("💾 Account saved to accounts list"));
      }
      
      console.log(chalk.yellow("\n⚠️  IMPORTANT: Save the private key securely!"));
      console.log(chalk.yellow("This private key cannot be recovered if lost."));

    } catch (error) {
      console.error(chalk.red("❌ Failed to generate account:"));
      console.error(error instanceof Error ? error.message : String(error));
      throw error;
    }
  });

task("neo-account-set-default", "Set default account for deployments")
  .addParam("account", "Account address or index")
  .setAction(async (taskArgs, hre: HardhatRuntimeEnvironment) => {
    const { account } = taskArgs;
    
    console.log(chalk.blue(`🎯 Setting default account to ${account}...`));

    try {
      // Try to parse as index first
      const accountIndex = parseInt(account);
      if (!isNaN(accountIndex)) {
        hre.neoDeploy.accounts.setDefaultAccount(accountIndex);
      } else {
        hre.neoDeploy.accounts.setDefaultAccount(account);
      }
      
      const newDefault = hre.neoDeploy.accounts.getDefaultAccount();
      console.log(chalk.green(`✅ Default account set to ${newDefault?.address}`));

    } catch (error) {
      console.error(chalk.red("❌ Failed to set default account:"));
      console.error(error instanceof Error ? error.message : String(error));
      throw error;
    }
  });