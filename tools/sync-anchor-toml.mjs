#!/usr/bin/env node
// Rewrites [programs.localnet] in both workspace Anchor.toml files from the
// declare_id! of every program crate. Prints counts only, so it can run over
// held-out cases without showing their content.
//   node tools/sync-anchor-toml.mjs

import fs from 'node:fs';
import path from 'node:path';
import { BENCH } from './lib/cases.mjs';

for (const side of ['vulnerable', 'fixed']) {
  const ws = path.join(BENCH, 'workspaces', side);
  const programs = path.join(ws, 'programs');
  const entries = [];
  let missing = 0;
  for (const crate of fs.existsSync(programs) ? fs.readdirSync(programs).sort() : []) {
    const lib = path.join(programs, crate, 'src', 'lib.rs');
    const id = fs.existsSync(lib) ? (fs.readFileSync(lib, 'utf8').match(/declare_id!\(\s*"([1-9A-HJ-NP-Za-km-z]{32,44})"\s*\)/) || [])[1] : null;
    if (id) entries.push(`${crate} = "${id}"`);
    else missing++;
  }
  const file = path.join(ws, 'Anchor.toml');
  const text = fs.readFileSync(file, 'utf8');
  const section = `[programs.localnet]\n${entries.join('\n')}${entries.length ? '\n' : ''}`;
  const next = text.replace(/\[programs\.localnet\]\n(?:[^\[\n][^\n]*\n|\n)*?(?=\n\[|\[|$)/, section);
  fs.writeFileSync(file, next);
  console.log(`${side}: ${entries.length} program(s) listed, ${missing} without declare_id!`);
}
