// Re-export types for convenience
export * from "@neo-solidity/types";

/**
 * Neo-Foundry specific types
 */

/**
 * Test execution options
 */
export interface TestOptions {
  pattern?: string;
  verbose?: boolean;
  gasReport?: boolean;
  coverage?: boolean;
  forkUrl?: string;
  forkBlockNumber?: number;
  profile?: string;
}

/**
 * Build options
 */
export interface BuildOptions {
  force?: boolean;
  watch?: boolean;
  profile?: string;
  quiet?: boolean;
}

/**
 * Forge script options
 */
export interface ScriptOptions {
  broadcast?: boolean;
  verify?: boolean;
  resume?: boolean;
  slow?: boolean;
  legacy?: boolean;
  rpcUrl?: string;
  privateKey?: string;
  mnemonic?: string;
  interactive?: boolean;
}

/**
 * Cast options
 */
export interface CastOptions {
  rpcUrl?: string;
  privateKey?: string;
  from?: string;
  gasLimit?: string;
  gasPrice?: string;
  value?: string;
  blockTag?: string | number;
}

/**
 * Anvil options
 */
export interface AnvilOptions {
  port?: number;
  chainId?: number;
  accounts?: number;
  balance?: string;
  gasLimit?: string;
  gasPrice?: string;
  blockTime?: number;
  fork?: string;
  forkBlockNumber?: number;
  quiet?: boolean;
}

/**
 * Dependency configuration
 */
export interface DependencyConfig {
  name: string;
  url: string;
  tag?: string;
  branch?: string;
  commit?: string;
  version?: string;
}

/**
 * Remapping configuration
 */
export interface RemappingConfig {
  from: string;
  to: string;
}

/**
 * Gas report entry
 */
export interface GasReportEntry {
  contract: string;
  method: string;
  min: number;
  max: number;
  avg: number;
  calls: number;
}

/**
 * Coverage report entry
 */
export interface CoverageReportEntry {
  file: string;
  lines: {
    total: number;
    covered: number;
    percentage: number;
  };
  functions: {
    total: number;
    covered: number;
    percentage: number;
  };
  branches: {
    total: number;
    covered: number;
    percentage: number;
  };
}

/**
 * Test result
 */
export interface TestResult {
  name: string;
  status: "passed" | "failed" | "skipped";
  duration: number;
  gasUsed?: number;
  error?: string;
  logs?: string[];
}

/**
 * Test suite result
 */
export interface TestSuiteResult {
  name: string;
  tests: TestResult[];
  duration: number;
  passed: number;
  failed: number;
  skipped: number;
}

/**
 * Build result
 */
export interface BuildResult {
  success: boolean;
  duration: number;
  contracts: number;
  warnings: number;
  errors: string[];
  artifacts: string[];
}

/**
 * Deployment script result
 */
export interface ScriptResult {
  success: boolean;
  transactions: Array<{
    hash: string;
    to?: string;
    value?: string;
    gasUsed: string;
  }>;
  contracts: Array<{
    name: string;
    address: string;
  }>;
  duration: number;
}

/**
 * Forge configuration validation result
 */
export interface ConfigValidationResult {
  valid: boolean;
  errors: string[];
  warnings: string[];
}

/**
 * Project initialization options
 */
export interface ProjectInitOptions {
  template?: string;
  force?: boolean;
  vcs?: boolean;
  offline?: boolean;
  quiet?: boolean;
}