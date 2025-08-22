import { NeoAccount } from "@neo-solidity/types";
import { HardhatPluginError } from "hardhat/plugins";
import Debug from "debug";

const debug = Debug("hardhat:neo-deployer:accounts");

/**
 * Account manager for Neo deployments
 */
export class AccountManager {
  private accounts: NeoAccount[];
  private defaultAccountIndex = 0;

  constructor(accountsConfig: (string | NeoAccount)[]) {
    this.accounts = this.processAccountsConfig(accountsConfig);
  }

  /**
   * Get account by address
   */
  getAccount(address: string): NeoAccount | undefined {
    return this.accounts.find(account => account.address === address);
  }

  /**
   * Get account by index
   */
  getAccountByIndex(index: number): NeoAccount | undefined {
    return this.accounts[index];
  }

  /**
   * Get default account
   */
  getDefaultAccount(): NeoAccount | undefined {
    return this.accounts[this.defaultAccountIndex];
  }

  /**
   * Get all accounts
   */
  getAllAccounts(): NeoAccount[] {
    return [...this.accounts];
  }

  /**
   * Set default account
   */
  setDefaultAccount(addressOrIndex: string | number): void {
    if (typeof addressOrIndex === "number") {
      if (addressOrIndex < 0 || addressOrIndex >= this.accounts.length) {
        throw new Error(`Account index ${addressOrIndex} out of range`);
      }
      this.defaultAccountIndex = addressOrIndex;
    } else {
      const index = this.accounts.findIndex(account => account.address === addressOrIndex);
      if (index === -1) {
        throw new Error(`Account with address ${addressOrIndex} not found`);
      }
      this.defaultAccountIndex = index;
    }
  }

  /**
   * Add account
   */
  addAccount(account: NeoAccount | string): void {
    const processedAccount = this.processAccountConfig(account);
    this.accounts.push(processedAccount);
  }

  /**
   * Remove account
   */
  removeAccount(address: string): boolean {
    const index = this.accounts.findIndex(account => account.address === address);
    if (index === -1) {
      return false;
    }
    
    this.accounts.splice(index, 1);
    
    // Adjust default account index if necessary
    if (this.defaultAccountIndex >= this.accounts.length) {
      this.defaultAccountIndex = Math.max(0, this.accounts.length - 1);
    }
    
    return true;
  }

  /**
   * Get account count
   */
  getAccountCount(): number {
    return this.accounts.length;
  }

  /**
   * Check if account exists
   */
  hasAccount(address: string): boolean {
    return this.accounts.some(account => account.address === address);
  }

  /**
   * Get accounts with private keys (for signing)
   */
  getSigningAccounts(): NeoAccount[] {
    return this.accounts.filter(account => account.privateKey);
  }

  /**
   * Get default signer
   */
  getDefaultSigner(): NeoAccount | undefined {
    const defaultAccount = this.getDefaultAccount();
    return defaultAccount?.privateKey ? defaultAccount : undefined;
  }

  /**
   * Validate account configuration
   */
  validateAccount(account: NeoAccount): boolean {
    // Basic validation
    if (!account.address || !account.scriptHash) {
      return false;
    }

    // Validate address format (simplified)
    if (!account.address.startsWith('N') && !account.address.startsWith('A')) {
      return false;
    }

    // Validate script hash format
    if (!account.scriptHash.startsWith('0x') || account.scriptHash.length !== 42) {
      return false;
    }

    // If private key is provided, validate it
    if (account.privateKey) {
      if (account.privateKey.length !== 64 && !account.privateKey.startsWith('0x')) {
        return false;
      }
    }

    return true;
  }

  /**
   * Import accounts from file
   */
  async importAccountsFromFile(filePath: string): Promise<void> {
    try {
      const fs = await import('fs/promises');
      const content = await fs.readFile(filePath, 'utf-8');
      const accountsData = JSON.parse(content);
      
      if (!Array.isArray(accountsData)) {
        throw new Error("Accounts file must contain an array of accounts");
      }
      
      for (const accountData of accountsData) {
        const account = this.processAccountConfig(accountData);
        this.accounts.push(account);
      }
      
      debug(`Imported ${accountsData.length} accounts from ${filePath}`);
    } catch (error) {
      throw new HardhatPluginError(
        "@neo-solidity/hardhat-neo-deployer",
        `Failed to import accounts: ${error}`
      );
    }
  }

  /**
   * Export accounts to file
   */
  async exportAccountsToFile(filePath: string, includePrivateKeys = false): Promise<void> {
    try {
      const fs = await import('fs/promises');
      
      const exportData = this.accounts.map(account => {
        const exported: any = {
          address: account.address,
          scriptHash: account.scriptHash,
          publicKey: account.publicKey,
          label: account.label,
          isMultiSig: account.isMultiSig
        };
        
        if (includePrivateKeys && account.privateKey) {
          exported.privateKey = account.privateKey;
        }
        
        if (account.multiSig) {
          exported.multiSig = account.multiSig;
        }
        
        return exported;
      });
      
      await fs.writeFile(filePath, JSON.stringify(exportData, null, 2));
      debug(`Exported ${this.accounts.length} accounts to ${filePath}`);
    } catch (error) {
      throw new HardhatPluginError(
        "@neo-solidity/hardhat-neo-deployer",
        `Failed to export accounts: ${error}`
      );
    }
  }

  // Private methods

  private processAccountsConfig(accountsConfig: (string | NeoAccount)[]): NeoAccount[] {
    return accountsConfig.map(config => this.processAccountConfig(config));
  }

  private processAccountConfig(config: string | NeoAccount): NeoAccount {
    if (typeof config === "string") {
      // Assume it's a private key in hex format
      return this.createAccountFromPrivateKey(config);
    } else {
      // Validate and return account object
      if (!this.validateAccount(config)) {
        throw new Error(`Invalid account configuration: ${JSON.stringify(config)}`);
      }
      return config;
    }
  }

  private createAccountFromPrivateKey(privateKey: string): NeoAccount {
    // This would derive address and script hash from private key
    // For now, return a mock account
    const normalizedPrivateKey = privateKey.startsWith('0x') ? privateKey : `0x${privateKey}`;
    
    return {
      address: this.generateMockAddress(),
      scriptHash: this.generateMockScriptHash(),
      privateKey: normalizedPrivateKey,
      publicKey: this.derivePublicKey(normalizedPrivateKey),
      label: `Account ${this.accounts.length + 1}`,
      isMultiSig: false
    };
  }

  private derivePublicKey(privateKey: string): string {
    // This would derive the public key from private key using Neo cryptography
    // For now, return a mock public key
    return "03" + "a".repeat(62);
  }

  private generateMockAddress(): string {
    return "N" + Math.random().toString(36).substring(2, 15) + Math.random().toString(36).substring(2, 15);
  }

  private generateMockScriptHash(): string {
    return "0x" + Array.from({ length: 40 }, () => Math.floor(Math.random() * 16).toString(16)).join('');
  }
}