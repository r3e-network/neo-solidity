#!/usr/bin/env node
'use strict';

const fs = require('fs');
const path = require('path');
const { spawnSync } = require('child_process');

const ROOT = path.resolve(__dirname, '..');
const TARGETS_PATH = path.join(ROOT, 'docs/data/famous-contracts-targets.json');
const REPORT_MD_PATH = path.join(ROOT, 'docs/solidity/famous-contracts-neo-audit.md');
const REPORT_JSON_PATH = path.join(ROOT, 'docs/data/famous-contracts-audit-results.json');
const AUDIT_DIR = process.env.NEO_FAMOUS_AUDIT_DIR || '/tmp/neo-famous-contracts-audit';
const BUILD_DIR = process.env.NEO_FAMOUS_BUILD_DIR || '/tmp/neo-famous-contracts-build';

const NPM_PACKAGES = [
  '@openzeppelin/contracts@5.4.0',
  '@openzeppelin/contracts-upgradeable@5.4.0',
  '@aave/core-v3@1.19.3',
  '@safe-global/safe-contracts@1.4.1-2',
  '@chainlink/contracts@1.5.0',
  '@uniswap/v4-core@1.0.2',
  '@uniswap/v4-periphery@1.0.3',
  '@uniswap/v3-core@1.0.1',
  '@uniswap/v3-periphery@1.4.4'
];

const REQUIRED_PACKAGE_PROBES = [
  '@openzeppelin/contracts/package.json',
  '@openzeppelin/contracts-upgradeable/package.json',
  '@aave/core-v3/package.json',
  '@safe-global/safe-contracts/package.json',
  '@chainlink/contracts/package.json',
  '@uniswap/v4-core/package.json',
  '@uniswap/v4-periphery/package.json'
];

const OPENZEPPELIN_ALIAS_CANDIDATES = ['4.7.3', '4.8.3', '4.9.6', '5.0.2', '5.1.0'];

function runCommand(cmd, args, options = {}) {
  const result = spawnSync(cmd, args, {
    encoding: 'utf8',
    maxBuffer: 50 * 1024 * 1024,
    ...options
  });

  if (result.error) {
    throw result.error;
  }

  return result;
}

function ensureNeoSolc() {
  const releaseBin = path.join(ROOT, 'target/release/neo-solc');
  if (fs.existsSync(releaseBin)) {
    return releaseBin;
  }

  console.log('[audit] Building neo-solc release binary...');
  const build = runCommand('cargo', ['build', '--release', '--bin', 'neo-solc'], { cwd: ROOT });
  if (build.status !== 0) {
    throw new Error(`failed to build neo-solc:\n${build.stdout}\n${build.stderr}`);
  }

  if (!fs.existsSync(releaseBin)) {
    throw new Error('neo-solc binary missing after build');
  }

  return releaseBin;
}

function ensureAuditWorkspace() {
  fs.mkdirSync(AUDIT_DIR, { recursive: true });
  const nodeModulesDir = path.join(AUDIT_DIR, 'node_modules');

  const hasAllPackages = REQUIRED_PACKAGE_PROBES.every((probe) =>
    fs.existsSync(path.join(nodeModulesDir, probe))
  );

  if (hasAllPackages) {
    return;
  }

  console.log('[audit] Installing famous protocol npm dependencies...');
  if (!fs.existsSync(path.join(AUDIT_DIR, 'package.json'))) {
    const init = runCommand('npm', ['init', '-y'], { cwd: AUDIT_DIR });
    if (init.status !== 0) {
      throw new Error(`npm init failed:\n${init.stdout}\n${init.stderr}`);
    }
  }

  const install = runCommand('npm', ['install', '--silent', ...NPM_PACKAGES], { cwd: AUDIT_DIR });
  if (install.status !== 0) {
    throw new Error(`npm install failed:\n${install.stdout}\n${install.stderr}`);
  }
}

function ensureOpenZeppelinVersionAliases() {
  const ozRoot = path.join(AUDIT_DIR, 'node_modules', '@openzeppelin');
  if (!fs.existsSync(ozRoot)) {
    return;
  }

  for (const version of OPENZEPPELIN_ALIAS_CANDIDATES) {
    const contractsVersionDir = path.join(ozRoot, `contracts-${version}`);
    const contractsAlias = path.join(ozRoot, `contracts@${version}`);
    if (fs.existsSync(contractsVersionDir)) {
      try {
        fs.rmSync(contractsAlias, { recursive: true, force: true });
      } catch {
        // noop
      }
      fs.symlinkSync(`contracts-${version}`, contractsAlias);
    }

    const upgradeableVersionDir = path.join(ozRoot, `contracts-upgradeable-${version}`);
    const upgradeableAlias = path.join(ozRoot, `contracts-upgradeable@${version}`);
    if (fs.existsSync(upgradeableVersionDir)) {
      try {
        fs.rmSync(upgradeableAlias, { recursive: true, force: true });
      } catch {
        // noop
      }
      fs.symlinkSync(`contracts-upgradeable-${version}`, upgradeableAlias);
    }
  }
}

function parseDiagnostics(stderr) {
  const diagnostics = [];
  for (const rawLine of stderr.split(/\r?\n/)) {
    const line = rawLine.trim();
    if (!line) {
      continue;
    }

    if (line.startsWith('{') && line.endsWith('}')) {
      try {
        const parsed = JSON.parse(line);
        diagnostics.push({
          severity: parsed.severity || 'error',
          code: parsed.code || 'UNKNOWN',
          message: parsed.message || parsed.formattedMessage || line
        });
        continue;
      } catch {
        // fall through
      }
    }

    const severity = /warning/i.test(line) ? 'warning' : 'error';
    diagnostics.push({ severity, code: 'RAW', message: line });
  }

  return diagnostics;
}

function normalizeMessage(msg) {
  return msg.replace(/\s+/g, ' ').trim();
}

function classifyBlocker(message) {
  const lower = message.toLowerCase();

  if (lower.includes('inline assembly is not supported')) {
    return {
      tag: 'assembly',
      fix: '需要移除 `assembly`，改用 `NativeCalls.sol` / `Syscalls.sol` / 高级 Solidity 语义重写'
    };
  }

  if (lower.includes("unsupported low-level evm call 'delegatecall'")) {
    return {
      tag: 'delegatecall',
      fix: '需要改为显式 `Syscalls.contractCall` 跨合约调用，并按 Neo 存储隔离模型重构状态共享'
    };
  }

  if (lower.includes('unsupported low-level evm call')) {
    return {
      tag: 'low_level_call',
      fix: '需要将低层 `call/staticcall/callcode` 迁移到受控 `Syscalls.contractCall` + 明确 ABI 序列化'
    };
  }

  if (lower.includes('unsupported type') && lower.includes('mapping(')) {
    return {
      tag: 'named_mapping',
      fix: '需要编译器补齐命名 `mapping(address key => T)` 语法 lowering，或改写为 `mapping(address => T)`'
    };
  }

  if (lower.includes('import cycle detected')) {
    return {
      tag: 'import_cycle',
      fix: '需要编译器导入解析支持循环依赖图，或对上游源码做解环拆分'
    };
  }

  if (lower.includes('overloaded function') && lower.includes('not supported')) {
    return {
      tag: 'abi_overload',
      fix: '需要避免同参数个数的重载（重命名公开方法），或扩展 Neo ABI 到签名级调度'
    };
  }

  if (lower.includes('error resolving imports') || lower.includes('cannot resolve import')) {
    return {
      tag: 'import_resolution',
      fix: '需要补齐依赖库和 include path；若为工具链导入限制，需扩展导入解析能力'
    };
  }

  if (lower.includes('unknown identifier')) {
    return {
      tag: 'name_resolution',
      fix: '需要补齐对应语义 lowering（命名解析/继承展平），或对源码做 Neo 兼容重写'
    };
  }

  if (lower.includes('pragma') && lower.includes('solidity')) {
    return {
      tag: 'solidity_version',
      fix: '需要将源码迁移到 Solidity 0.8.x 范围并处理破坏性变更'
    };
  }

  if (lower.includes('function call options (`{...}`) are not supported (value)')) {
    return {
      tag: 'value_call_options',
      fix: '需要把 `{value: ...}` 风格调用改成显式 NEP-17 转账（`NativeCalls.gasTransfer` / `NativeCalls.neoTransfer`）并使用 `onNEP17Payment` 回调'
    };
  }

  if (lower.includes('abi.encodewithselector is only supported for neo contract calls')) {
    return {
      tag: 'abi_encode_selector',
      fix: '需要改用 `Syscalls.contractCall` / `Syscalls.contractCallWithFlags` / `NativeCalls.*`，不要依赖原生 EVM calldata 字节流'
    };
  }

  if (lower.includes('inheritance linearization failed')) {
    return {
      tag: 'inheritance_linearization',
      fix: '需要调整继承层次（或扩展编译器的 C3 线性化兼容），避免多重继承顺序冲突'
    };
  }

  if (lower.includes('modifier/constructor argument mismatch')) {
    return {
      tag: 'ctor_modifier_mismatch',
      fix: '需要修复构造器/修饰器参数传递路径，或扩展编译器对复杂构造器链的 lowering'
    };
  }

  if (lower.includes('uses unsupported type')) {
    return {
      tag: 'unsupported_param_type',
      fix: '需要扩展接口/结构体参数类型 lowering（复杂参数序列化），或先重构为基础类型边界'
    };
  }

  if (lower.includes('duplicate state variable')) {
    return {
      tag: 'duplicate_state_var',
      fix: '需要修复状态变量命名冲突解析（编译器语义分析）或在源码层拆分冲突字段'
    };
  }

  return {
    tag: 'other',
    fix: '需要扩展 neo-solidity 对该语义的 IR lowering，或用 Neo 等价模式重写该模块'
  };
}

function sanitizeName(input) {
  return input.replace(/[^a-zA-Z0-9_.-]+/g, '_');
}

function compileTarget(neoSolc, target, index, total) {
  const isRepoSource = target.source === 'repo';
  const sourcePath = isRepoSource
    ? path.join(ROOT, target.path)
    : path.join(AUDIT_DIR, 'node_modules', target.path);

  const progress = `[${String(index + 1).padStart(2, '0')}/${total}]`;

  if (!fs.existsSync(sourcePath)) {
    console.log(`${progress} ${target.project} / ${target.contract}: missing source`);
    return {
      ...target,
      sourcePath,
      status: 'missing',
      exitCode: null,
      diagnostics: [],
      mainIssue: 'source file not found',
      blockerTag: 'missing_source',
      neoRequirement: '需要修正目标路径或更换可公开获取的官方源码入口'
    };
  }

  const prefix = path.join(BUILD_DIR, sanitizeName(`${index}_${target.project}_${target.contract}`));
  const args = [
    sourcePath,
    '-I',
    path.join(ROOT, 'devpack'),
    '-I',
    path.join(AUDIT_DIR, 'node_modules'),
    '--json-errors',
    '--json-warnings',
    '-o',
    prefix
  ];

  const result = runCommand(neoSolc, args, { cwd: ROOT });
  const diagnostics = parseDiagnostics(result.stderr || '');
  const errors = diagnostics.filter((d) => d.severity !== 'warning');
  const status = result.status === 0 ? 'pass' : 'fail';

  let mainIssue = '';
  let blockerTag = 'none';
  let neoRequirement = '可直接编译为 NeoVM（如需生产使用，仍需做 manifest 权限最小化和业务安全审计）';

  if (status !== 'pass') {
    const primary = errors[0] || diagnostics[0];
    mainIssue = primary ? normalizeMessage(primary.message) : `compiler exited with ${result.status}`;
    const classified = classifyBlocker(mainIssue);
    blockerTag = classified.tag;
    neoRequirement = classified.fix;
  }

  console.log(`${progress} ${target.project} / ${target.contract}: ${status}`);

  return {
    ...target,
    sourcePath,
    status,
    exitCode: result.status,
    diagnostics,
    mainIssue,
    blockerTag,
    neoRequirement
  };
}

function topBlockers(results) {
  const counter = new Map();
  for (const row of results) {
    if (row.status !== 'fail') {
      continue;
    }
    const key = row.blockerTag || 'other';
    counter.set(key, (counter.get(key) || 0) + 1);
  }
  return [...counter.entries()].sort((a, b) => b[1] - a[1]);
}

function short(text, max = 140) {
  if (!text) {
    return '-';
  }
  return text.length <= max ? text : `${text.slice(0, max - 1)}…`;
}

function generateMarkdown(neoSolcVersion, results) {
  const total = results.length;
  const pass = results.filter((r) => r.status === 'pass').length;
  const fail = results.filter((r) => r.status === 'fail').length;
  const missing = results.filter((r) => r.status === 'missing').length;

  const blockers = topBlockers(results);
  const date = new Date().toISOString();

  const lines = [];
  lines.push('# Famous Solidity Contracts on NeoVM: Compatibility Audit');
  lines.push('');
  lines.push(`- Generated at (UTC): \`${date}\``);
  lines.push(`- Compiler: \`${neoSolcVersion.trim()}\``);
  lines.push(`- Target contracts: \`${total}\``);
  lines.push(`- Compile success: \`${pass}\``);
  lines.push(`- Compile failed: \`${fail}\``);
  if (missing > 0) {
    lines.push(`- Missing source entries: \`${missing}\``);
  }
  lines.push('');
  lines.push('## What "Need XXX to Implement" Means');
  lines.push('');
  lines.push('- This report marks each failing contract with the **primary unsupported point** in current `neo-solidity`.');
  lines.push('- The "Need on Neo" column states what is required to make that pattern work:');
  lines.push('  1) compiler capability expansion, and/or');
  lines.push('  2) Solidity source refactor to Neo-native patterns (`Runtime`, `Syscalls`, `NativeCalls`, `onNEP17Payment`, etc.).');
  lines.push('');
  lines.push('## Top Blockers');
  lines.push('');

  if (blockers.length === 0) {
    lines.push('- No blockers in this run.');
  } else {
    for (const [tag, count] of blockers) {
      lines.push(`- \`${tag}\`: ${count}`);
    }
  }

  lines.push('');
  lines.push('## Per-Contract Results');
  lines.push('');
  lines.push('| # | Project | Contract | Result | Primary Unsupported Point | Need on Neo | Source |');
  lines.push('|---:|---|---|---|---|---|---|');

  results.forEach((row, idx) => {
    const resultMark = row.status === 'pass' ? '✅ pass' : row.status === 'missing' ? '⚪ missing' : '❌ fail';
    const sourceRel =
      row.source === 'npm'
        ? `node_modules/${row.path}`
        : row.sourcePath.startsWith(ROOT)
          ? path.relative(ROOT, row.sourcePath)
          : row.sourcePath;

    lines.push(
      `| ${idx + 1} | ${row.project} | ${row.contract} | ${resultMark} | ${short(row.mainIssue)} | ${short(
        row.neoRequirement,
        160
      )} | \`${sourceRel}\` |`
    );
  });

  lines.push('');
  lines.push('## Notes');
  lines.push('');
  lines.push('- A **pass** means the contract compiled through `neo-solc` in this environment.');
  lines.push('- A **fail** does not mean the contract is impossible on Neo; it means current source + current compiler need refactor or feature work.');
  lines.push('- Use this as a migration backlog: prioritize high-value blockers (`delegatecall`, import cycles, ABI overload collisions, named mapping syntax).');

  return lines.join('\n');
}

function main() {
  fs.mkdirSync(BUILD_DIR, { recursive: true });

  const neoSolc = ensureNeoSolc();
  ensureAuditWorkspace();
  ensureOpenZeppelinVersionAliases();

  const versionResult = runCommand(neoSolc, ['--version'], { cwd: ROOT });
  const neoSolcVersion = versionResult.status === 0 ? versionResult.stdout : 'neo-solc (version unavailable)';

  const targets = JSON.parse(fs.readFileSync(TARGETS_PATH, 'utf8'));
  const results = [];

  console.log(`[audit] Starting compilation of ${targets.length} famous contracts...`);

  for (let i = 0; i < targets.length; i++) {
    results.push(compileTarget(neoSolc, targets[i], i, targets.length));
  }

  const reportJson = {
    generatedAt: new Date().toISOString(),
    compiler: neoSolcVersion.trim(),
    root: ROOT,
    auditDir: AUDIT_DIR,
    buildDir: BUILD_DIR,
    totals: {
      total: results.length,
      pass: results.filter((r) => r.status === 'pass').length,
      fail: results.filter((r) => r.status === 'fail').length,
      missing: results.filter((r) => r.status === 'missing').length
    },
    blockerStats: Object.fromEntries(topBlockers(results)),
    results
  };

  fs.writeFileSync(REPORT_JSON_PATH, JSON.stringify(reportJson, null, 2));
  fs.writeFileSync(REPORT_MD_PATH, generateMarkdown(neoSolcVersion, results));

  console.log(`[audit] Wrote JSON: ${path.relative(ROOT, REPORT_JSON_PATH)}`);
  console.log(`[audit] Wrote Markdown: ${path.relative(ROOT, REPORT_MD_PATH)}`);
}

main();
