import { describe, expect, it } from "vitest";
import { tx, wallet } from "@cityofzion/neon-js";
import {
  createNeoSigner,
  createNeoTransaction,
  createWitnessFromSignature,
  signTransaction,
} from "../src/transaction-primitives";
import { createAccountFromPrivateKey } from "../src/account-primitives";

describe("transaction-primitives", () => {
  const signer = createAccountFromPrivateKey(
    "7d128a6d096f0c14c3a25a2b0c41cf79661bfcb4a8cc95aaaea28bde4d732344",
    0x35,
    "Fixture"
  );

  it("serializes signers like neon-js", () => {
    const expected = new tx.Signer({
      account: signer.scriptHash,
      scopes: tx.WitnessScope.CalledByEntry,
    }).serialize();

    expect(
      createNeoSigner({
        account: signer.scriptHash,
        scopes: "CalledByEntry",
      }).serialize()
    ).toBe(expected);
  });

  it("builds signature witnesses like neon-js", () => {
    const expected = tx.Witness.fromSignature("11".repeat(64), signer.publicKey).serialize();
    expect(createWitnessFromSignature("11".repeat(64), signer.publicKey).serialize()).toBe(expected);
  });

  it("serializes and signs simple invocation transactions like neon-js", () => {
    const script = "0c017811c01f0c0962616c616e63654f660c14fda3fa4346ea532a258fc497ddaddb6437c9fdff41627d5b52";
    const nonce = 123456789;

    const neonTx = new tx.Transaction({
      script,
      nonce,
      signers: [
        new tx.Signer({
          account: signer.scriptHash,
          scopes: tx.WitnessScope.CalledByEntry,
        }),
      ],
      validUntilBlock: 123,
      systemFee: "1000",
      networkFee: "2000",
    });
    neonTx.sign(signer as any, 860833102);

    const localTx = createNeoTransaction({
      scriptHex: script,
      nonce,
      signers: [{ account: signer.scriptHash, scopes: "CalledByEntry" }],
      validUntilBlock: 123,
      systemFee: "1000",
      networkFee: "2000",
    });
    signTransaction(localTx, signer, 860833102);

    expect(localTx.serialize(false)).toBe(neonTx.serialize(false));
    expect(localTx.witnesses[0].verificationScript).toBe(neonTx.witnesses[0].verificationScript.toBigEndian());
    expect(localTx.witnesses[0].invocationScript).toMatch(/^0c40[0-9a-f]{128}$/);
    expect(
      wallet.verify(
        localTx.getMessageForSigning(860833102),
        localTx.witnesses[0].invocationScript.slice(4),
        signer.publicKey
      )
    ).toBe(true);
  });
});
