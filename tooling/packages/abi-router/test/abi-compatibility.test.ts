import { describe, expect, it } from "vitest";
import { id } from "ethers";
import { NeoABICompatibilityLayer } from "../src/abi-compatibility";

describe("abi-compatibility overload handling", () => {
  const overloadedAbi = [
    {
      type: "function" as const,
      name: "foo",
      inputs: [{ name: "value", type: "uint256" }],
      outputs: [{ name: "", type: "uint256" }],
      stateMutability: "view" as const,
    },
    {
      type: "function" as const,
      name: "foo",
      inputs: [
        { name: "value", type: "uint256" },
        { name: "extra", type: "uint256" },
      ],
      outputs: [{ name: "", type: "uint256" }],
      stateMutability: "view" as const,
    },
    {
      type: "function" as const,
      name: "bar",
      inputs: [{ name: "value", type: "uint256" }],
      outputs: [{ name: "", type: "uint256" }],
      stateMutability: "view" as const,
    },
  ];

  it("encodes overloaded functions by signature", () => {
    const layer = new NeoABICompatibilityLayer();
    layer.registerABI(overloadedAbi);

    const encoded = layer.encodeFunction("foo(uint256,uint256)", [5, 6]);

    expect(encoded.startsWith(id("foo(uint256,uint256)").slice(0, 10))).toBe(true);
  });

  it("rejects ambiguous bare overload names", () => {
    const layer = new NeoABICompatibilityLayer();
    layer.registerABI(overloadedAbi);

    expect(() => layer.encodeFunction("foo", [5])).toThrow(/overloaded|ambiguous/i);
  });

  it("keeps bare-name resolution for non-overloaded functions", () => {
    const layer = new NeoABICompatibilityLayer();
    layer.registerABI(overloadedAbi);

    const encoded = layer.encodeFunction("bar", [7]);

    expect(encoded.startsWith(id("bar(uint256)").slice(0, 10))).toBe(true);
  });
});
