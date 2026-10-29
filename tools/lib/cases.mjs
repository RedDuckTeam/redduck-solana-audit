// Shared helpers for benchmark cases: frontmatter parsing, case discovery, paths.

import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

export const ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
export const BENCH = path.join(ROOT, 'benchmark');
export const PLUGIN = path.join(ROOT, 'plugins', 'redduck-solana-audit');
export const CANARY = fs.readFileSync(path.join(ROOT, 'tools', 'canary.txt'), 'utf8').trim();

// Flat YAML frontmatter: `key: value` lines between the first two `---` lines.
export function parseFrontmatter(text) {
  const m = text.match(/^---\r?\n([\s\S]*?)\r?\n---\r?\n?([\s\S]*)$/);
  if (!m) return { data: {}, body: text };
  const data = {};
  for (const line of m[1].split(/\r?\n/)) {
    const kv = line.match(/^([A-Za-z0-9_]+):\s*(.*?)\s*(#.*)?$/);
    if (kv) data[kv[1]] = kv[2].replace(/^["']|["']$/g, '');
  }
  return { data, body: m[2] };
}

// Markdown sections keyed by their `## Heading` text.
export function sections(body) {
  const out = {};
  let current = null;
  for (const line of body.split(/\r?\n/)) {
    const h = line.match(/^##\s+(.+?)\s*$/);
    if (h) {
      current = h[1].toLowerCase();
      out[current] = [];
    } else if (current) out[current].push(line);
  }
  for (const k of Object.keys(out)) out[k] = out[k].join('\n').trim();
  return out;
}

// Every case directory under benchmark/cases with its CASE.md frontmatter and rubric.
export function listCases({ role } = {}) {
  const dir = path.join(BENCH, 'cases');
  if (!fs.existsSync(dir)) return [];
  const cases = [];
  for (const id of fs.readdirSync(dir).sort()) {
    const caseFile = path.join(dir, id, 'CASE.md');
    if (!fs.existsSync(caseFile)) continue;
    const { data } = parseFrontmatter(fs.readFileSync(caseFile, 'utf8'));
    if (data.id !== id) throw new Error(`${caseFile}: frontmatter id "${data.id}" does not match folder "${id}"`);
    if (role && data.role !== role) continue;
    const rubricFile = path.join(dir, id, 'rubric.md');
    const rubric = fs.existsSync(rubricFile) ? parseRubric(fs.readFileSync(rubricFile, 'utf8')) : null;
    cases.push({ ...data, dir: path.join(dir, id), rubric });
  }
  return cases;
}

// rubric.md: `## Mechanism`, `## Locations` (bullets `path:start-end`), `## Judge criteria`.
export function parseRubric(text) {
  const { body } = parseFrontmatter(text);
  const s = sections(body);
  const locations = (s.locations || '')
    .split(/\r?\n/)
    .map((l) => l.match(/^[-*]\s*`?([^`\s:]+):(\d+)(?:-(\d+))?`?/))
    .filter(Boolean)
    .map((m) => ({ file: m[1], start: Number(m[2]), end: Number(m[3] || m[2]) }));
  return { mechanism: s.mechanism || '', locations, criteria: s['judge criteria'] || '' };
}

// The ```json block of benchmark/LEDGER.md, or an empty ledger when the file does not exist yet.
export function readLedger() {
  const file = path.join(BENCH, 'LEDGER.md');
  if (!fs.existsSync(file)) return { cases: [] };
  const m = fs.readFileSync(file, 'utf8').match(/```json\s*([\s\S]*?)```/);
  return m ? JSON.parse(m[1]) : { cases: [] };
}

// Eval ids for a case: `<id>a` and `<id>b`; `vulnerable` names the vulnerable side.
export function evalSides(c) {
  const vuln = c.vulnerable === 'b' ? 'b' : 'a';
  const fixed = vuln === 'a' ? 'b' : 'a';
  const sides = [{ evalId: `${c.id}${vuln}`, side: 'vulnerable' }];
  if (c.fixed !== 'none') sides.push({ evalId: `${c.id}${fixed}`, side: 'fixed' });
  return sides;
}

export function copyDir(src, dst, skip = new Set(['target', 'node_modules', '.anchor'])) {
  fs.mkdirSync(dst, { recursive: true });
  for (const entry of fs.readdirSync(src, { withFileTypes: true })) {
    if (skip.has(entry.name)) continue;
    const s = path.join(src, entry.name);
    const d = path.join(dst, entry.name);
    if (entry.isDirectory()) copyDir(s, d, skip);
    else fs.copyFileSync(s, d);
  }
}
