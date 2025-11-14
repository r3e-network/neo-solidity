import { EventEmitter } from "events";
import {
  NeoHardhatConfig,
  DeploymentOptions,
  DeploymentResult
} from "@neo-solidity/types";

/**
 * Minimal deployment manager placeholder.
 * Deployment is intentionally unsupported in this plugin because the Neo RPC/transaction
 * flow has not been wired up yet. All methods surface a consistent error so callers do
 * not assume deployment succeeded.
 */
export class DeploymentManager extends EventEmitter {
  constructor(private readonly config: NeoHardhatConfig, private readonly network: any) {
    super();
    // Retain references to config/network for potential future use.
    void this.config;
    void this.network;
  }

  async deployContract(
    _contractName: string,
    _constructorArgs: any[] = [],
    _options: DeploymentOptions = {}
  ): Promise<DeploymentResult> {
    this.unsupported("deployContract");
  }

  async deployMultiple(): Promise<DeploymentResult[]> {
    this.unsupported("deployMultiple");
  }

  async verifyContract(): Promise<boolean> {
    this.unsupported("verifyContract");
  }

  async upgradeContract(): Promise<DeploymentResult> {
    this.unsupported("upgradeContract");
  }

  async getDeployment(): Promise<DeploymentResult | undefined> {
    this.unsupported("getDeployment");
  }

  async getAllDeployments(): Promise<DeploymentResult[]> {
    this.unsupported("getAllDeployments");
  }

  async estimateDeploymentCost(): Promise<{
    gasEstimate: string;
    costInWei: string;
    costInEth: string;
  }> {
    this.unsupported("estimateDeploymentCost");
  }

  async batchDeploy(): Promise<DeploymentResult[]> {
    this.unsupported("batchDeploy");
  }

  async generateDeploymentReport(): Promise<{
    totalDeployments: number;
    totalGasUsed: string;
    totalCost: string;
    deploymentsByStatus: { [status: string]: number };
    topGasConsumers: Array<{ contract: string; gasUsed: string }>;
  }> {
    this.unsupported("generateDeploymentReport");
  }

  private unsupported(feature: string): never {
    throw new Error(
      `${feature} is not available because Neo deployments are not implemented in ` +
        "@neo-solidity/hardhat-solc-neo. Please deploy using native Neo tooling."
    );
  }
}
