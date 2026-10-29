#!/usr/bin/env node
// Quiet regression gate: prints nothing when every check passes, prints the
// failing command's output otherwise. Used as the single feedback command.

import { spawnSync } from 'node:child_process';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const checks = [
  ['claude', ['plugin', 'validate', '--strict', path.join(ROOT, 'plugins', 'redduck-solana-audit')]],
  ['claude', ['plugin', 'validate', '--strict', ROOT]],
  [process.execPath, [path.join(ROOT, 'tools', 'verify.mjs'), '--regression']],
];

let failed = false;
for (const [cmd, args] of checks) {
  const result = spawnSync(cmd, args, { encoding: 'utf8', shell: process.platform === 'win32' && cmd === 'claude' });
  if (result.status !== 0) {
    failed = true;
    console.log(`FAILED: ${[path.basename(cmd), ...args].join(' ')}`);
    console.log((result.stdout || '') + (result.stderr || '') + (result.error ? String(result.error) : ''));
  }
}
process.exit(failed ? 1 : 0);
