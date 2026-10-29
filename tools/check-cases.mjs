#!/usr/bin/env node
// Static check of benchmark pairs: `cargo check` of each program in both workspaces
// (programs are never built or executed, by the user's decision) plus the case
// reviewer's verdict from benchmark/LEDGER.md. Prints one line per case and nothing
// else, so held-out cases can be checked without showing their content; full cargo
// output goes to benchmark/logs/.
//   node tools/check-cases.mjs [--role dev|heldout|public|all] [--no-cargo]

import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { spawnSync } from 'node:child_process';
import { BENCH, listCases, readLedger } from './lib/cases.mjs';

const args = process.argv.slice(2);
const role = args.includes('--role') ? args[args.indexOf('--role') + 1] : 'all';
const useCargo = !args.includes('--no-cargo') && spawnSync('cargo', ['--version'], { encoding: 'utf8' }).status === 0;
const targetDir = path.join(os.tmpdir(), 'redduck-solana-audit-target');
const logs = path.join(BENCH, 'logs');
fs.mkdirSync(logs, { recursive: true });

const ledger = readLedger();
const reviewOf = (id) => ledger.cases?.find((c) => c.id === id)?.review || 'missing';

function cargoCheck(side, crate, id) {
  const ws = path.join(BENCH, 'workspaces', side);
  const r = spawnSync('cargo', ['check', '--quiet', '-p', crate], {
    cwd: ws,
    encoding: 'utf8',
    env: { ...process.env, CARGO_TARGET_DIR: targetDir },
  });
  fs.writeFileSync(path.join(logs, `check-${id}-${side}.log`), `${r.stdout || ''}${r.stderr || ''}`);
  return r.status === 0;
}

let bad = 0;
for (const c of listCases()) {
  if (role !== 'all' && c.role !== role) continue;
  let check = 'check-skipped';
  if (useCargo && c.crate && !c.source_vulnerable) {
    const ok = cargoCheck('vulnerable', c.crate, c.id) && (c.fixed === 'none' || cargoCheck('fixed', c.crate, c.id));
    check = ok ? 'check-ok' : 'check-fail';
  }
  const review = `review-${reviewOf(c.id)}`;
  if (check === 'check-fail' || review !== 'review-pass') bad++;
  console.log(`${c.id}: ${check} / ${review}`);
}
process.exit(bad ? 1 : 0);
