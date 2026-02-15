#!/usr/bin/env node
'use strict';

const fs = require('fs');
const os = require('os');
const path = require('path');
const { spawnSync } = require('child_process');

const ROOT = path.resolve(__dirname, '..');
const TARGETS_PATH = path.join(ROOT, 'docs/data/famous-contracts-targets.json');
const AUDIT_RESULTS_PATH = path.join(ROOT, 'docs/data/famous-contracts-audit-results.json');
const AUDIT_DIR = process.env.NEO_FAMOUS_AUDIT_DIR || '/tmp/neo-famous-contracts-audit';

function run(cmd, args, options = {}) {
  const res = spawnSync(cmd, args, {
    encoding: 'utf8',
    maxBuffer: 50 * 1024 * 1024,
    ...options
  });
  if (res.error) {
    throw res.error;
  }
  return res;
}

function packageFromTargetPath(targetPath) {
  const parts = targetPath.split('/');
  if (targetPath.startsWith('@')) {
    return parts.slice(0, 2).join('/');
  }
  return parts[0];
}

function relativePathInPackage(targetPath) {
  const pkg = packageFromTargetPath(targetPath);
  return targetPath.slice(pkg.length + 1);
}

function readJson(filePath) {
  return JSON.parse(fs.readFileSync(filePath, 'utf8'));
}

function packAndExtractPackage(pkgName, version) {
  const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'neo-famous-verify-'));
  const spec = `${pkgName}@${version}`;
  const pack = run('npm', ['pack', '--silent', spec], { cwd: tempDir });
  if (pack.status !== 0) {
    throw new Error(`npm pack failed for ${spec}:\n${pack.stdout}\n${pack.stderr}`);
  }
  const tarball = pack.stdout
    .trim()
    .split(/\r?\n/)
    .filter(Boolean)
    .pop();
  if (!tarball) {
    throw new Error(`npm pack did not produce tarball name for ${spec}`);
  }

  const tarPath = path.join(tempDir, tarball);
  const extract = run('tar', ['-xzf', tarPath], { cwd: tempDir });
  if (extract.status !== 0) {
    throw new Error(`tar extraction failed for ${spec}:\n${extract.stdout}\n${extract.stderr}`);
  }

  const extractedRoot = path.join(tempDir, 'package');
  return { tempDir, extractedRoot, spec };
}

function main() {
  const targets = readJson(TARGETS_PATH);
  const audit = readJson(AUDIT_RESULTS_PATH);

  const npmTargets = targets.filter((target) => target.source === 'npm');
  const repoTargets = targets.filter((target) => target.source === 'repo');
  const packageTargets = new Map();

  for (const target of npmTargets) {
    const pkg = packageFromTargetPath(target.path);
    if (!packageTargets.has(pkg)) {
      packageTargets.set(pkg, []);
    }
    packageTargets.get(pkg).push(target);
  }

  const errors = [];
  const mismatchDetails = [];
  let comparedFiles = 0;
  const tempDirs = [];

  try {
    for (const [pkgName, pkgFiles] of packageTargets.entries()) {
      const installedRoot = path.join(AUDIT_DIR, 'node_modules', pkgName);
      const installedPackageJson = path.join(installedRoot, 'package.json');
      if (!fs.existsSync(installedPackageJson)) {
        errors.push(`missing installed package: ${pkgName} (${installedPackageJson})`);
        continue;
      }

      const { version } = readJson(installedPackageJson);
      const { tempDir, extractedRoot, spec } = packAndExtractPackage(pkgName, version);
      tempDirs.push(tempDir);

      for (const target of pkgFiles) {
        const rel = relativePathInPackage(target.path);
        const installedFile = path.join(installedRoot, rel);
        const officialFile = path.join(extractedRoot, rel);

        if (!fs.existsSync(installedFile)) {
          mismatchDetails.push({
            type: 'missing_installed_file',
            package: spec,
            target: target.contract,
            file: rel
          });
          continue;
        }

        if (!fs.existsSync(officialFile)) {
          mismatchDetails.push({
            type: 'missing_official_file',
            package: spec,
            target: target.contract,
            file: rel
          });
          continue;
        }

        const installedBytes = fs.readFileSync(installedFile);
        const officialBytes = fs.readFileSync(officialFile);
        comparedFiles += 1;
        if (!installedBytes.equals(officialBytes)) {
          mismatchDetails.push({
            type: 'content_mismatch',
            package: spec,
            target: target.contract,
            file: rel,
            installedBytes: installedBytes.length,
            officialBytes: officialBytes.length
          });
        }
      }
    }
  } finally {
    for (const tempDir of tempDirs) {
      try {
        fs.rmSync(tempDir, { recursive: true, force: true });
      } catch {
        // ignore cleanup failures
      }
    }
  }

  for (const target of repoTargets) {
    const sourcePath = path.join(ROOT, target.path);
    if (!fs.existsSync(sourcePath)) {
      errors.push(`missing repo source target: ${target.path}`);
      continue;
    }
    const content = fs.readFileSync(sourcePath, 'utf8');
    if (!/\b(contract|interface|library)\b/.test(content)) {
      errors.push(`repo target does not look like complete Solidity source: ${target.path}`);
    }
  }

  if (audit?.totals?.pass !== audit?.totals?.total) {
    errors.push(
      `audit not fully passing: pass=${audit?.totals?.pass}, total=${audit?.totals?.total}`
    );
  }

  if (mismatchDetails.length > 0) {
    errors.push(`found ${mismatchDetails.length} npm source mismatches against official tarballs`);
  }

  if (errors.length > 0) {
    console.error('[verify] FAILED');
    for (const err of errors) {
      console.error(`- ${err}`);
    }
    if (mismatchDetails.length > 0) {
      console.error('[verify] mismatch details:');
      for (const item of mismatchDetails) {
        console.error(`- ${JSON.stringify(item)}`);
      }
    }
    process.exit(1);
  }

  console.log('[verify] OK');
  console.log(`- npm targets compared: ${comparedFiles}`);
  console.log(`- repo targets checked: ${repoTargets.length}`);
  console.log(`- audit totals: ${audit.totals.pass}/${audit.totals.total} pass`);
}

main();
