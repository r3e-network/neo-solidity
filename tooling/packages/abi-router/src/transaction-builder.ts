import { NeoRpcProvider } from "@neo-solidity/types";
import { NeoDeploymentArtifacts } from "./types.js";
import Debug from "debug";

const debug = Debug("neo-solidity:transaction-builder");

/**
 * Transaction builder for Neo N3.
 *
 * Read-only calls (`invokefunction`, `invokescript`) can be performed without signing.
 * Sending transactions and deploying contracts requires:
 * - signer/account management
 * - witness scope selection
 * - fee estimation (`invokescript` + `calculatenetworkfee`)
 * - raw transaction serialization + `sendrawtransaction`
 *
 * These flows are implemented in `@neo-solidity/hardhat-neo-deployer`.
 */
export class TransactionBuilder {
  constructor(private readonly rpcProvider: NeoRpcProvider) {
    void this.rpcProvider;
  }

  async sendTransaction(
    contractAddress: string,
    methodName: string,
    args: any[] = [],
    options: {
      gasLimit?: string;
      gasPrice?: string;
      value?: string;
    } = {}
  ): Promise<{ hash: string }> {
    debug(`sendTransaction requested for ${methodName} on ${contractAddress}`);
    void args;
    void options;

    throw new Error(
      "TransactionBuilder.sendTransaction is not implemented. Use @neo-solidity/hardhat-neo-deployer " +
        "(Hardhat) or sign a Neo N3 transaction with @cityofzion/neon-js and call NeoRpcProvider.sendRawTransaction."
    );
  }

  async deployContract(
    artifacts: NeoDeploymentArtifacts,
    constructorArgs: any[] = [],
    options: {
      gasLimit?: string;
      gasPrice?: string;
      value?: string;
    } = {}
  ): Promise<{ address: string; hash: string }> {
    debug("deployContract requested");
    void artifacts;
    void constructorArgs;
    void options;

    throw new Error(
      "TransactionBuilder.deployContract is not implemented. Use @neo-solidity/hardhat-neo-deployer (Hardhat) " +
        "or neo-solc + native Neo tooling to deploy."
    );
  }
}
