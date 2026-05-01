import { sourcemapTypes, profilerTypes } from '@neo-devpack-solidity/types';
import { EventEmitter } from 'events';
import { HardhatPluginError } from "hardhat/plugins";

type DebugSession = sourcemapTypes.DebugSession;
type DebugState = sourcemapTypes.DebugState;
type Breakpoint = sourcemapTypes.Breakpoint;
type StackFrame = sourcemapTypes.StackFrame;
type Variable = sourcemapTypes.Variable;
type TraceStep = profilerTypes.TraceStep;
type MemoryState = sourcemapTypes.MemoryState;
type StorageState = sourcemapTypes.StorageState;

const notSupported = (feature: string): never => {
  throw new HardhatPluginError(
    '@neo-devpack-solidity/hardhat-solc-neo',
    `Neo debugger placeholder: ${feature} is not implemented yet.`
  );
};

/**
 * Placeholder implementation for the Neo debug manager.
 * The original implementation was just a collection of stubs, so we keep the
 * public API but surface a consistent Hardhat error to make the status clear.
 */
export class DebugManager extends EventEmitter {
  constructor(_config: unknown) {
    super();
    void _config;
  }

  async startDebugSession(_transactionHash: string): Promise<DebugSession> {
    return notSupported('startDebugSession');
  }

  async stepOver(_sessionId: string): Promise<DebugState> {
    return notSupported('stepOver');
  }

  async stepInto(_sessionId: string): Promise<DebugState> {
    return notSupported('stepInto');
  }

  async stepOut(_sessionId: string): Promise<DebugState> {
    return notSupported('stepOut');
  }

  async continue(_sessionId: string): Promise<DebugState> {
    return notSupported('continue');
  }

  async setBreakpoint(
    _source: string,
    _line: number,
    _column?: number,
    _condition?: string
  ): Promise<Breakpoint> {
    return notSupported('setBreakpoint');
  }

  async removeBreakpoint(_breakpointId: string): Promise<boolean> {
    return notSupported('removeBreakpoint');
  }

  async getStackTrace(_sessionId: string): Promise<StackFrame[]> {
    return notSupported('getStackTrace');
  }

  async getLocalVariables(_sessionId: string): Promise<Variable[]> {
    return notSupported('getLocalVariables');
  }

  async getMemory(_sessionId: string): Promise<MemoryState> {
    return notSupported('getMemory');
  }

  async getStorage(_sessionId: string): Promise<StorageState> {
    return notSupported('getStorage');
  }

  async evaluateExpression(_sessionId: string, _expression: string): Promise<unknown> {
    return notSupported('evaluateExpression');
  }

  async analyzeGasUsage(_sessionId: string): Promise<{
    totalGas: string;
    gasPerOperation: Array<{ op: string; gas: string; percentage: number }>;
    recommendations: string[];
  }> {
    return notSupported('analyzeGasUsage');
  }

  async generateExecutionReport(_sessionId: string): Promise<{
    executionTime: number;
    operationCounts: { [op: string]: number };
    memoryUsage: { peak: number; average: number };
    storageAccess: { reads: number; writes: number };
    callGraph: Array<{ name: string }>;
  }> {
    return notSupported('generateExecutionReport');
  }

  async exportDebugSession(_sessionId: string, _format: 'json' | 'csv'): Promise<string> {
    return notSupported('exportDebugSession');
  }

  async getDebugTrace(_transactionHash: string): Promise<TraceStep[]> {
    return notSupported('getDebugTrace');
  }
}
