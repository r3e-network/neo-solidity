import { wallet } from "@cityofzion/neon-js";

export const DEFAULT_ADDRESS_VERSION = 0x35;

export function strip0x(value: string): string {
  return value.startsWith("0x") ? value.slice(2) : value;
}

export function isHexString(value: string): boolean {
  const hex = strip0x(value);
  return hex.length > 0 && hex.length % 2 === 0 && /^[0-9a-fA-F]+$/.test(hex);
}

export function decodeNeoBytes(value: unknown): Buffer {
  if (value == null) return Buffer.alloc(0);

  if (Buffer.isBuffer(value)) return value;
  if (value instanceof Uint8Array) return Buffer.from(value);

  if (typeof value === "string") {
    const trimmed = value.trim();
    if (!trimmed) return Buffer.alloc(0);

    if (trimmed.startsWith("0x")) {
      return Buffer.from(strip0x(trimmed), "hex");
    }

    // neo-cli often returns hex strings; neo-go commonly returns base64 for ByteString/ByteArray.
    if (isHexString(trimmed)) {
      return Buffer.from(trimmed, "hex");
    }

    return Buffer.from(trimmed, "base64");
  }

  if (Array.isArray(value) && value.every((item) => typeof item === "number")) {
    return Buffer.from(value);
  }

  throw new Error(`Unsupported ByteString/ByteArray value: ${typeof value}`);
}

export function normalizeNeoHash160(value: string): string {
  const trimmed = value.trim();

  if (wallet.isAddress(trimmed)) {
    return "0x" + wallet.getScriptHashFromAddress(trimmed);
  }

  const hex = strip0x(trimmed);
  if (!/^[0-9a-fA-F]{40}$/.test(hex)) {
    throw new Error(`Expected 20-byte hex (Hash160), got: ${value}`);
  }

  return "0x" + hex.toLowerCase();
}

/**
 * Converts a Solidity/EVM-style address (big-endian `0x` 20-byte hex) to Neo script hash
 * (`0x`-prefixed, little-endian 20-byte hex).
 *
 * Accepts base58 Neo N3 addresses too.
 */
export function evmAddressToNeoHash160(address: string): string {
  const trimmed = address.trim();

  if (wallet.isAddress(trimmed)) {
    return "0x" + wallet.getScriptHashFromAddress(trimmed);
  }

  const hex = strip0x(trimmed);
  if (!/^[0-9a-fA-F]{40}$/.test(hex)) {
    throw new Error(`Expected address as base58 or 0x + 40 hex chars, got: ${address}`);
  }

  const bytesBe = Buffer.from(hex, "hex");
  const bytesLe = Buffer.from(bytesBe).reverse();
  return "0x" + bytesLe.toString("hex");
}

/**
 * Converts a Neo UInt160 (little-endian `0x` hex, or base58 address) to a Solidity/EVM-style
 * big-endian `0x` address.
 */
export function neoHash160ToEvmAddress(value: string): string {
  const trimmed = value.trim();

  if (wallet.isAddress(trimmed)) {
    return neoHash160ToEvmAddress("0x" + wallet.getScriptHashFromAddress(trimmed));
  }

  const hexLe = strip0x(trimmed);
  if (!/^[0-9a-fA-F]{40}$/.test(hexLe)) {
    throw new Error(`Expected Hash160 as base58 or 0x + 40 hex chars, got: ${value}`);
  }

  const bytesLe = Buffer.from(hexLe, "hex");
  const bytesBe = Buffer.from(bytesLe).reverse();
  return "0x" + bytesBe.toString("hex");
}

export function neoBytesToEvmAddress(bytesValue: unknown): string {
  const bytes = decodeNeoBytes(bytesValue);
  if (bytes.length !== 20) {
    throw new Error(`Expected 20 bytes for address, got ${bytes.length}`);
  }
  return "0x" + Buffer.from(bytes).reverse().toString("hex");
}

export function stackItemArrayValue(item: any): any[] {
  if (!item) return [];
  if (Array.isArray(item)) return item;
  if (typeof item === "object" && item.type === "Array" && Array.isArray(item.value)) return item.value;
  if (typeof item === "object" && item.type === "Struct" && Array.isArray(item.value)) return item.value;
  return [];
}

export function parseNeoBoolean(value: unknown): boolean {
  if (typeof value === "boolean") return value;
  if (typeof value === "number") return value !== 0;
  if (typeof value === "bigint") return value !== 0n;
  if (typeof value === "string") {
    const trimmed = value.trim().toLowerCase();
    if (trimmed === "true" || trimmed === "1") return true;
    if (trimmed === "false" || trimmed === "0") return false;
  }
  return Boolean(value);
}

export function parseNeoInteger(value: unknown): bigint {
  if (typeof value === "bigint") return value;
  if (typeof value === "number") return BigInt(Math.trunc(value));
  if (typeof value === "string") {
    const trimmed = value.trim();
    if (!trimmed) return 0n;
    return BigInt(trimmed);
  }
  return 0n;
}
