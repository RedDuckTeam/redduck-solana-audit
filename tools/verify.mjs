#!/usr/bin/env node
// Static checks for the repository. Plan: .omc/plans/solana-audit-skill.md, V-steps.
//   --regression  invariants that must hold at every stage (default)
//   --all         regression + completeness checks for the finished tree
// Exit code 0 = all checks pass, 1 = at least one failure.

import fs from 'node:fs';
import path from 'node:path';
import crypto from 'node:crypto';
import { fileURLToPath, pathToFileURL } from 'node:url';

const ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const PLUGIN = path.join(ROOT, 'plugins', 'redduck-solana-audit');
const SKILL = path.join(PLUGIN, 'skills', 'auditing-solana-programs');
const BENCH = path.join(ROOT, 'benchmark');
const SKIP_DIRS = new Set(['vendor', 'target', 'node_modules', 'logs', 'results', '.git']);

const mode = process.argv.includes('--all') ? 'all' : 'regression';
const failures = [];
const fail = (check, msg) => failures.push(`[${check}] ${msg}`);

const rel = (p) => path.relative(ROOT, p).split(path.sep).join('/');
const exists = (p) => fs.existsSync(p);
const read = (p) => fs.readFileSync(p, 'utf8');

function walk(dir, out = []) {
  if (!exists(dir)) return out;
  for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
    if (entry.isDirectory()) {
      if (SKIP_DIRS.has(entry.name)) continue;
      out.push(path.join(dir, entry.name) + path.sep);
      walk(path.join(dir, entry.name), out);
    } else {
      out.push(path.join(dir, entry.name));
    }
  }
  return out;
}
const filesOnly = (list) => list.filter((p) => !p.endsWith(path.sep));
const isText = (p) => /\.(md|rs|ts|js|mjs|json|toml|yaml|yml|sh|txt)$/i.test(p) || path.basename(p) === '.gitignore';

// Names of internal source documents that must never appear in the published repository.
// Built from parts so this file does not itself contain them.
const INTERNAL_NAMES = [
  ['FEEDBACK', '_'],
  ['company', 'MindSet'],
  ['transcripts', 'WARPR'],
  ['mark', '-ii'],
  ['ANALYSIS', '_ACADEMY'],
  ['Patterns for ', 'Academy'],
].map((parts) => parts.join(''));

const PATTERN_SLUGS = [
  'root-of-trust', 'identity-rejection', 'untrusted-executor', 'role-inversion',
  'lazy-loading', 'pay-as-you-act', 'divide-and-conquer', 'complexity-asymmetry',
  'commit-execution-split', 'action-as-data', 'virtual-execution', 'one-way-state',
  'boundary-of-consent', 'internal-oracle', 'defensive-computation', 'fail-fast',
  'execution-layer-shift', 'dependency-independency',
];
const slugRegex = PATTERN_SLUGS.map((s) => s.split('-').join('[-_ ]?')).join('|');
const STOP_LIST = new RegExp(`(vuln|(?<!de)bug|exploit|insecure|unsafe_|todo|fixme|missing.?check|${slugRegex})`, 'i');

const ALLOWED_FRONTMATTER = new Set(['name', 'description', 'license', 'compatibility', 'metadata']);

// Minimal frontmatter reader: top-level keys, inline values, and > or | block scalars.
function readFrontmatter(file) {
  const text = read(file);
  if (!text.startsWith('---\n') && !text.startsWith('---\r\n')) return null;
  const end = text.indexOf('\n---', 3);
  if (end === -1) return null;
  const lines = text.slice(text.indexOf('\n') + 1, end).split(/\r?\n/);
  const result = {};
  let current = null;
  for (const line of lines) {
    const m = line.match(/^([A-Za-z0-9_-]+):\s*(.*)$/);
    if (m) {
      current = m[1];
      const value = m[2].trim();
      result[current] = value === '>' || value === '|' || value === '' ? '' : value.replace(/^["']|["']$/g, '');
    } else if (current && /^\s+/.test(line)) {
      result[current] = (result[current] ? result[current] + ' ' : '') + line.trim();
    }
  }
  const body = text.slice(end + 4).replace(/^\r?\n/, '');
  return { keys: result, body };
}

function checkFrontmatter() {
  const file = path.join(SKILL, 'SKILL.md');
  if (!exists(file)) return fail('V2', `${rel(file)} missing`);
  const fm = readFrontmatter(file);
  if (!fm) return fail('V2', 'SKILL.md has no frontmatter starting on line 1');
  for (const key of Object.keys(fm.keys)) {
    if (!ALLOWED_FRONTMATTER.has(key)) fail('V2', `frontmatter key not portable: ${key}`);
  }
  const { name, description } = fm.keys;
  if (name !== path.basename(SKILL)) fail('V2', `name "${name}" does not match folder "${path.basename(SKILL)}"`);
  if (!/^[a-z0-9]+(-[a-z0-9]+)*$/.test(name || '') || (name || '').length > 64) fail('V2', `invalid name "${name}"`);
  if (!description) fail('V2', 'description is empty');
  else if (description.length > 1024) fail('V2', `description is ${description.length} chars (> 1024)`);
  const bodyLines = fm.body.split(/\r?\n/).length;
  if (bodyLines > 500) fail('V3', `SKILL.md body is ${bodyLines} lines (> 500)`);
}

function checkLeaks() {
  const files = filesOnly(walk(ROOT)).filter(isText).filter((p) => p !== fileURLToPath(import.meta.url));
  for (const file of files) {
    const text = read(file);
    for (const name of INTERNAL_NAMES) {
      if (text.includes(name)) fail('V4', `${rel(file)} mentions an internal document name`);
    }
  }
  const cyrillicScope = [PLUGIN, path.join(ROOT, 'examples'), path.join(ROOT, 'README.md')];
  for (const scope of cyrillicScope) {
    const list = scope.endsWith('.md') ? (exists(scope) ? [scope] : []) : filesOnly(walk(scope));
    for (const file of list.filter(isText)) {
      if (/[Ѐ-ӿ]/.test(read(file))) fail('V4', `${rel(file)} contains Cyrillic text`);
    }
  }
}

function checkStopList() {
  const roots = [path.join(BENCH, 'fixtures')];
  for (const side of ['vulnerable', 'fixed']) roots.push(path.join(BENCH, 'workspaces', side, 'programs'));
  for (const root of roots) {
    for (const entry of walk(root)) {
      const relative = path.relative(root, entry);
      if (STOP_LIST.test(relative)) fail('V6', `path hints at the answer: ${rel(entry)}`);
      if (entry.endsWith(path.sep) || !/\.(rs|ts|toml|md)$/i.test(entry)) continue;
      const hit = read(entry).match(STOP_LIST);
      if (hit) fail('V6', `${rel(entry)} contains stop-list word "${hit[0]}"`);
    }
  }
}

function checkShippedFolder() {
  const entries = walk(PLUGIN);
  const stageTemplate = path.join(ROOT, 'tools', 'templates', 'stage.sh');
  for (const entry of entries) {
    const r = rel(entry);
    const base = path.basename(entry.replace(/[\\/]$/, ''));
    if (entry.endsWith(path.sep) && (base === 'hooks' || base === 'bin')) fail('V7', `forbidden directory in plugin: ${r}`);
    if (base === 'hooks.json' || base === '.mcp.json') fail('V7', `forbidden file in plugin: ${r}`);
    if (!entry.endsWith(path.sep) && /\.(sh|mjs|js|cjs|ts|py)$/i.test(entry)) {
      const isStage = /\/evals\/[^/]+\/stage\.sh$/.test(r);
      if (!isStage) fail('V7', `script in shipped plugin: ${r}`);
      else if (exists(stageTemplate)) {
        const expected = read(stageTemplate).replace(/__CASE_ID__/g, '').replace(/\s+/g, ' ');
        const actual = read(entry).replace(/CASE_ID="[^"]*"/, 'CASE_ID=""').replace(/\s+/g, ' ');
        if (expected !== actual) fail('V7', `${r} differs from tools/templates/stage.sh`);
      }
    }
  }
  for (const dir of ['skills', 'agents']) {
    for (const file of filesOnly(walk(path.join(PLUGIN, dir))).filter(isText)) {
      if (/\bcurl |\bwget |fetch\(/.test(read(file))) fail('V7', `${rel(file)} contains a network call`);
    }
  }
}

function sha256(file) {
  return crypto.createHash('sha256').update(fs.readFileSync(file)).digest('hex');
}

function readLedger() {
  const file = path.join(BENCH, 'LEDGER.md');
  if (!exists(file)) return null;
  const m = read(file).match(/```json\s*([\s\S]*?)```/);
  if (!m) {
    fail('V11', 'LEDGER.md has no ```json block');
    return null;
  }
  try {
    return JSON.parse(m[1]);
  } catch (e) {
    fail('V11', `LEDGER.md json block does not parse: ${e.message}`);
    return null;
  }
}

function checkLedger() {
  const ledger = readLedger();
  if (!ledger) return;
  for (const c of ledger.cases || []) {
    for (const [file, hash] of Object.entries(c.files || {})) {
      const abs = path.join(ROOT, file);
      if (!exists(abs)) fail('V11', `${c.id}: ledger file missing ${file}`);
      else if (sha256(abs) !== hash) fail('V11', `${c.id}: hash mismatch for ${file} (record the edit in the ledger)`);
    }
  }
}

function checkCanary() {
  const canaryFile = path.join(ROOT, 'tools', 'canary.txt');
  if (!exists(canaryFile)) return fail('V14', 'tools/canary.txt missing');
  const canary = read(canaryFile).trim();
  if (!/^[A-Za-z0-9]+$/.test(canary)) fail('V14', 'canary must be plain alphanumeric');
  const answerFiles = [];
  for (const dir of fs.existsSync(path.join(BENCH, 'cases')) ? fs.readdirSync(path.join(BENCH, 'cases')) : []) {
    for (const name of ['CASE.md', 'rubric.md']) {
      const f = path.join(BENCH, 'cases', dir, name);
      if (exists(f)) answerFiles.push(f);
    }
  }
  if (exists(path.join(BENCH, 'LEDGER.md'))) answerFiles.push(path.join(BENCH, 'LEDGER.md'));
  for (const f of filesOnly(walk(path.join(PLUGIN, 'evals')))) {
    // Graders that carry the answer; file_exists and tool_used graders carry none.
    if (/[\\/]graders[\\/](found|canary)\.md$/.test(f)) answerFiles.push(f);
  }
  for (const f of answerFiles) {
    if (!read(f).includes(canary)) fail('V14', `${rel(f)} lacks the canary`);
  }
}

checkFrontmatter();
checkLeaks();
checkStopList();
checkShippedFolder();
checkLedger();
checkCanary();

if (mode === 'all') {
  const extra = path.join(ROOT, 'tools', 'verify-all.mjs');
  if (!exists(extra)) fail('ALL', 'tools/verify-all.mjs missing');
  else {
    const { runAll } = await import(pathToFileURL(extra).href);
    for (const f of await runAll({ ROOT, PLUGIN, SKILL, BENCH, PATTERN_SLUGS })) failures.push(f);
  }
}

if (failures.length) {
  console.log(`verify (${mode}): ${failures.length} failure(s)`);
  for (const f of failures) console.log(`  ${f}`);
  process.exit(1);
}
console.log(`verify (${mode}): ok`);
