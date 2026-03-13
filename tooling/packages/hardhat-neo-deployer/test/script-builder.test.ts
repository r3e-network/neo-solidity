import { describe, expect, it } from "vitest";
import { sc, u } from "@cityofzion/neon-js";
import { createContractCallScript } from "../src/script-builder";

describe("script-builder", () => {
  it("matches neon-js for contract calls without arguments", () => {
    const expected = sc.createScript({
      scriptHash: "fffdc93764dbaddd97c48f252a53ea4643faa3fd",
      operation: "ping",
      args: [],
    });

    expect(
      createContractCallScript({
        scriptHash: "fffdc93764dbaddd97c48f252a53ea4643faa3fd",
        operation: "ping",
        args: [],
      })
    ).toBe(expected);
  });

  it("matches neon-js for mixed contract parameters", () => {
    const args = [
      { type: "String", value: "hello" },
      { type: "Integer", value: "42" },
      { type: "Boolean", value: true },
      { type: "ByteArray", value: u.HexString.fromHex("01020304", true) },
      { type: "Array", value: [{ type: "String", value: "nested" }] },
    ];

    const expected = sc.createScript({
      scriptHash: "d2a4cff31913016155e38e474a2c06d08be276cf",
      operation: "mixed",
      args,
    });

    expect(
      createContractCallScript({
        scriptHash: "d2a4cff31913016155e38e474a2c06d08be276cf",
        operation: "mixed",
        args,
      })
    ).toBe(expected);
  });
});
