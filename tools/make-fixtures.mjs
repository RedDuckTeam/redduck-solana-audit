#!/usr/bin/env node
// Builds benchmark/fixtures/<evalId>/program from the program sources of each case.
// Own cases read benchmark/workspaces/{vulnerable,fixed}/programs/<crate>; public
// cases name their sources in CASE.md (`source_vulnerable`, optional `source_fixed`).
// Only program files are copied: never CASE.md, rubric.md, TASK.md, SOLUTION.md or tests.
//   node tools/make-fixtures.mjs [--case <id>]

import fs from 'node:fs';
import path from 'node:path';
import { BENCH, ROOT, listCases, evalSides, copyDir } from './lib/cases.mjs';

const only = process.argv.includes('--case') ? process.argv[process.argv.indexOf('--case') + 1] : null;
const FORBIDDEN = /^(CASE\.md|rubric\.md|TASK\.md|SOLUTION\.md|tests?)$/i;

let count = 0;
for (const c of listCases()) {
  if (only && c.id !== only) continue;
  for (const { evalId, side } of evalSides(c)) {
    const explicit = side === 'vulnerable' ? c.source_vulnerable : c.source_fixed;
    const src = explicit
      ? path.join(ROOT, explicit)
      : path.join(BENCH, 'workspaces', side, 'programs', c.crate);
    if (!fs.existsSync(src)) throw new Error(`${c.id}: ${side} source missing at ${path.relative(ROOT, src)}`);
    const dst = path.join(BENCH, 'fixtures', evalId, 'program');
    fs.rmSync(path.join(BENCH, 'fixtures', evalId), { recursive: true, force: true });
    copyDir(src, dst, new Set(['target', 'node_modules', '.anchor', 'tests', 'test', 'TASK.md', 'SOLUTION.md', 'CASE.md', 'rubric.md']));
    for (const name of fs.readdirSync(dst)) {
      if (FORBIDDEN.test(name)) throw new Error(`${evalId}: forbidden file ${name} reached the fixture`);
    }
    // A crate description can state the flaw (the template's programs do), so drop it.
    const manifest = path.join(dst, 'Cargo.toml');
    if (fs.existsSync(manifest)) {
      fs.writeFileSync(manifest, fs.readFileSync(manifest, 'utf8').replace(/^description\s*=.*\r?\n/m, ''));
    }
    count++;
  }
}
console.log(`make-fixtures: ${count} fixture(s) written`);
