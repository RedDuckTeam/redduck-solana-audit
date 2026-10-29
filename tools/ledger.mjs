#!/usr/bin/env node
// Maintains benchmark/LEDGER.md: one entry per case with sha256 of every file, the
// author, the role (dev | heldout | public | retired), the reviewer verdict and the
// sessions that read or ran the case. Isolation is process-based and not enforced;
// the "read by" list is self-reported.
//   node tools/ledger.mjs add <id> --author <who>        record or re-record hashes
//   node tools/ledger.mjs review <id> pass|fail --by <who> [--note <text>]
//   node tools/ledger.mjs read <id> --by <who> [--note <text>]
//   node tools/ledger.mjs rehash <id> --by <who> --note <why the files changed>
//   node tools/ledger.mjs role <id> <role> --by <who> --note <why>
//   node tools/ledger.mjs show <id>                       prints id, role, review only

import fs from 'node:fs';
import path from 'node:path';
import crypto from 'node:crypto';
import { ROOT, BENCH, CANARY, listCases, evalSides, readLedger } from './lib/cases.mjs';

const FILE = path.join(BENCH, 'LEDGER.md');
const [cmd, id, ...rest] = process.argv.slice(2);
const opt = (name) => (rest.includes(name) ? rest[rest.indexOf(name) + 1] : null);
const now = () => new Date().toISOString();

function save(ledger) {
  ledger.cases.sort((a, b) => a.id.localeCompare(b.id));
  const text = [
    '# Case ledger',
    '',
    `Canary: ${CANARY}`,
    '',
    'One entry per benchmark case: sha256 of every file, author, role, reviewer verdict and the sessions that read or ran it.',
    'Isolation between the skill author and held-out cases is process-isolated, not enforced; the read-by list is self-reported.',
    'Maintained by `node tools/ledger.mjs`; `node tools/verify.mjs` recomputes the hashes.',
    '',
    '```json',
    JSON.stringify(ledger, null, 2),
    '```',
    '',
  ].join('\n');
  fs.writeFileSync(FILE, text);
}

function walkFiles(dir, out = []) {
  if (!fs.existsSync(dir)) return out;
  for (const e of fs.readdirSync(dir, { withFileTypes: true })) {
    if (['target', 'node_modules', '.anchor'].includes(e.name)) continue;
    const p = path.join(dir, e.name);
    if (e.isDirectory()) walkFiles(p, out);
    else out.push(p);
  }
  return out;
}

function hashes(c) {
  const dirs = [c.dir];
  if (c.crate && !c.source_vulnerable) {
    dirs.push(path.join(BENCH, 'workspaces', 'vulnerable', 'programs', c.crate));
    if (c.fixed !== 'none') dirs.push(path.join(BENCH, 'workspaces', 'fixed', 'programs', c.crate));
  }
  for (const { evalId } of evalSides(c)) dirs.push(path.join(BENCH, 'fixtures', evalId));
  const files = {};
  for (const f of dirs.flatMap((d) => walkFiles(d)).sort()) {
    files[path.relative(ROOT, f).split(path.sep).join('/')] = crypto.createHash('sha256').update(fs.readFileSync(f)).digest('hex');
  }
  return files;
}

const ledger = readLedger();
const entry = () => ledger.cases.find((c) => c.id === id);
const caseInfo = () => {
  const c = listCases().find((x) => x.id === id);
  if (!c) throw new Error(`no case ${id} in benchmark/cases`);
  return c;
};

switch (cmd) {
  case 'add': {
    const c = caseInfo();
    const e = entry() || { id, history: [] };
    Object.assign(e, { role: c.role, author: opt('--author') || c.author, review: e.review || 'missing', readBy: e.readBy || [], files: hashes(c) });
    e.history.push({ at: now(), event: 'add', by: opt('--author') || c.author });
    if (!entry()) ledger.cases.push(e);
    break;
  }
  case 'review': {
    const e = entry();
    if (!e) throw new Error(`${id} not in ledger`);
    const verdict = rest[0];
    if (!['pass', 'fail'].includes(verdict)) throw new Error('verdict must be pass or fail');
    e.review = verdict;
    e.history.push({ at: now(), event: `review-${verdict}`, by: opt('--by'), note: opt('--note') || undefined });
    break;
  }
  case 'read': {
    const e = entry();
    if (!e) throw new Error(`${id} not in ledger`);
    e.readBy.push({ at: now(), by: opt('--by'), note: opt('--note') || undefined });
    break;
  }
  case 'rehash': {
    const e = entry();
    if (!e) throw new Error(`${id} not in ledger`);
    if (!opt('--note')) throw new Error('rehash needs --note explaining the change');
    e.files = hashes(caseInfo());
    e.history.push({ at: now(), event: 'rehash', by: opt('--by'), note: opt('--note') });
    break;
  }
  case 'role': {
    const e = entry();
    if (!e) throw new Error(`${id} not in ledger`);
    e.role = rest[0];
    e.history.push({ at: now(), event: `role-${rest[0]}`, by: opt('--by'), note: opt('--note') });
    break;
  }
  case 'show': {
    const e = entry();
    console.log(e ? `${e.id}: role=${e.role} review=${e.review}` : `${id}: not in ledger`);
    process.exit(0);
  }
  default:
    throw new Error('commands: add | review | read | rehash | role | show');
}
save(ledger);
console.log(`ledger: ${cmd} ${id} ok`);
