// Completeness checks for the finished tree, run by `node tools/verify.mjs --all`.
// V3 skill structure, V5 copy check, V12 eval consistency, V13 counts.
// V5 needs the source texts the skill was written from; pass their folders in
// VERIFY_SOURCES (path-delimiter separated). They are not part of this repository.

import fs from 'node:fs';
import path from 'node:path';

const HEADINGS = ['Rule', 'Why Solana forces it', 'Code signals', 'Auditor questions', 'How to confirm', 'Not a bug if', 'Real-world cases', 'Severity'];
const CHECKLIST = ['arithmetic', 'duplicate-accounts', 'unsafe-and-panics', 'reload-after-cpi', 'token-2022', 'closing', 'seed-collisions', 'init-frontrun', 'realloc', 'authority-transfer', 'signers-bumps-cpi'];

function walk(dir, out = []) {
  if (!fs.existsSync(dir)) return out;
  if (fs.statSync(dir).isFile()) {
    out.push(dir);
    return out;
  }
  for (const e of fs.readdirSync(dir, { withFileTypes: true })) {
    if (['target', 'node_modules', '.git', 'vendor', 'results'].includes(e.name)) continue;
    const p = path.join(dir, e.name);
    if (e.isDirectory()) walk(p, out);
    else out.push(p);
  }
  return out;
}

const words = (text) => text.toLowerCase().replace(/[`*_#>|[\]()]/g, ' ').split(/\s+/).filter(Boolean);

export async function runAll({ ROOT, PLUGIN, SKILL, BENCH, PATTERN_SLUGS }) {
  const failures = [];
  const fail = (check, msg) => failures.push(`[${check}] ${msg}`);
  const read = (p) => fs.readFileSync(p, 'utf8');
  const rel = (p) => path.relative(ROOT, p).split(path.sep).join('/');

  // V3: skill structure.
  const skillText = read(path.join(SKILL, 'SKILL.md'));
  for (const slug of PATTERN_SLUGS) {
    const file = path.join(SKILL, 'references', 'patterns', `${slug}.md`);
    if (!fs.existsSync(file)) {
      fail('V3', `pattern file missing: ${slug}.md`);
      continue;
    }
    const text = read(file);
    for (const h of HEADINGS) if (!new RegExp(`^## ${h}\\s*$`, 'm').test(text)) fail('V3', `${slug}.md lacks heading "## ${h}"`);
    if (!skillText.includes(`**${slug}.**`)) fail('V3', `SKILL.md has no one-liner for ${slug}`);
  }
  for (const file of walk(path.join(SKILL, 'references'))) {
    const lines = read(file).split(/\r?\n/);
    if (lines.length > 100 && !lines.slice(0, 20).some((l) => /^## Contents/.test(l))) fail('V3', `${rel(file)} has ${lines.length} lines and no "## Contents" near the top`);
  }
  const checklist = read(path.join(SKILL, 'references', 'baseline-checklist.md'));
  for (const item of CHECKLIST) if (!new RegExp(`^## \\d+\\. ${item}\\s*$`, 'm').test(checklist)) fail('V3', `baseline-checklist.md lacks item ${item}`);
  const fp = read(path.join(SKILL, 'references', 'false-positives.md'));
  if ((fp.match(/^\d+\. \*\*/gm) || []).length < 8) fail('V3', 'false-positives.md has fewer than 8 numbered entries');
  if (!/spot/i.test(fp)) fail('V3', 'false-positives.md lacks the spot-price internal oracle warning');
  const rf = read(path.join(SKILL, 'references', 'report-format.md'));
  for (const k of ['Severity:', 'Location:', 'Mechanism:', '## Coverage table', '## Design findings']) if (!rf.includes(k)) fail('V3', `report-format.md lacks ${k}`);
  for (const f of [path.join(PLUGIN, 'agents', 'finding-verifier.md'), path.join(SKILL, 'references', 'verification.md')]) if (!fs.existsSync(f)) fail('V3', `${rel(f)} missing`);
  const et = read(path.join(SKILL, 'references', 'exploit-tests.md'));
  for (const s of ['## Anchor', '## Native and Pinocchio', '## Windows and WSL', '## No toolchain']) if (!et.includes(s)) fail('V3', `exploit-tests.md lacks ${s}`);
  if (!skillText.includes('redduck-solana-audit:finding-verifier') || !skillText.includes('references/verification.md')) fail('V3', 'SKILL.md step 7 must name the verifier agent and the verification.md fallback');

  // V5: no long verbatim runs from the source texts.
  const sources = (process.env.VERIFY_SOURCES || '').split(path.delimiter).filter(Boolean);
  if (!sources.length) fail('V5', 'VERIFY_SOURCES not set; cannot run the copy check');
  else {
    const N = 12;
    const grams = new Set();
    for (const src of sources) {
      for (const f of walk(src).filter((p) => /\.(md|txt)$/i.test(p))) {
        const w = words(read(f));
        for (let i = 0; i + N <= w.length; i++) grams.add(w.slice(i, i + N).join(' '));
      }
    }
    const targets = [path.join(SKILL, 'SKILL.md'), ...walk(path.join(SKILL, 'references'))];
    for (const t of targets) {
      const w = words(read(t));
      for (let i = 0; i + N <= w.length; i++) {
        if (grams.has(w.slice(i, i + N).join(' '))) {
          fail('V5', `${rel(t)} repeats 12+ words from a source: "${w.slice(i, i + N).join(' ')}"`);
          break;
        }
      }
    }
  }

  // V12: every eval case uses the same prompt, limits and tools, and one group tag.
  const evalsDir = path.join(PLUGIN, 'evals');
  const caseDirs = fs.existsSync(evalsDir) ? fs.readdirSync(evalsDir).filter((d) => fs.existsSync(path.join(evalsDir, d, 'case.yaml'))) : [];
  const template = read(path.join(ROOT, 'tools', 'templates', 'prompt.md')).replace(/\r\n/g, '\n');
  let execBlock = null;
  for (const d of caseDirs) {
    const prompt = read(path.join(evalsDir, d, 'prompt.md')).replace(/\r\n/g, '\n');
    if (prompt !== template) fail('V12', `${d}/prompt.md differs from tools/templates/prompt.md`);
    const yaml = read(path.join(evalsDir, d, 'case.yaml'));
    const exec = yaml.slice(yaml.indexOf('execution:'));
    if (execBlock === null) execBlock = exec;
    else if (exec !== execBlock) fail('V12', `${d}/case.yaml execution block differs from the others`);
    const tags = (yaml.match(/^tags:\s*\[(.*)\]/m) || [])[1] || '';
    if (tags.split(',').map((t) => t.trim()).filter((t) => /^g\d$/.test(t)).length !== 1) fail('V12', `${d}/case.yaml must have exactly one group tag`);
  }

  // V13: counts and documents.
  const cases = fs.existsSync(path.join(BENCH, 'cases')) ? fs.readdirSync(path.join(BENCH, 'cases')) : [];
  // 16 own pairs: the planned 24 were reduced after classifier stops (plan changelog, 2026-10-28).
  const own = cases.filter((c) => /^c\d{2}$/.test(c));
  if (own.length !== 16) fail('V13', `expected 16 own cases, found ${own.length}`);
  for (const c of cases) for (const f of ['CASE.md', 'rubric.md']) if (!fs.existsSync(path.join(BENCH, 'cases', c, f))) fail('V13', `${c} lacks ${f}`);
  const sourcesMd = path.join(BENCH, 'public', 'SOURCES.md');
  if (!fs.existsSync(sourcesMd) || !/[0-9a-f]{40}/.test(read(sourcesMd))) fail('V13', 'benchmark/public/SOURCES.md missing or without pinned commit shas');
  const running = read(path.join(BENCH, 'RUNNING.md'));
  for (const h of ['## Harness facts', '## Reproducing the full comparison', '## Hand-off']) if (!running.includes(h)) fail('V13', `RUNNING.md lacks "${h}"`);
  if (/claude plugin eval/.test(running) && running.split('\n').some((l) => /claude plugin eval /.test(l) && !/--no-publish/.test(l))) fail('V13', 'a claude plugin eval command in RUNNING.md lacks --no-publish');
  const readme = read(path.join(ROOT, 'README.md'));
  for (const h of ['## What it checks', '## Install', '## Where it has been checked', '## Benchmark', '## Limits', '## License']) if (!readme.includes(h)) fail('V13', `README.md lacks "${h}"`);
  if (!/academy\.redduck\.io/.test(readme)) fail('V13', 'README.md lacks the course link');
  if (!/MIT License/.test(read(path.join(ROOT, 'LICENSE')))) fail('V13', 'LICENSE is not MIT');
  if (!/## 0\.1\.0/.test(read(path.join(ROOT, 'CHANGELOG.md')))) fail('V13', 'CHANGELOG.md lacks 0.1.0');
  const manifest = JSON.parse(read(path.join(PLUGIN, '.claude-plugin', 'plugin.json')));
  if (manifest.version !== '0.1.0') fail('V13', `plugin.json version is ${manifest.version}`);
  for (const d of ['awesome-solana-ai-pr.md', 'solana-dev-skill-pr.md', 'anthropic-directory-submission.md', 'tob-skills-curated-pr.md', 'awesome-lists.md']) {
    if (!fs.existsSync(path.join(ROOT, 'drafts', d))) fail('V13', `drafts/${d} missing`);
  }
  const log = fs.existsSync(path.join(BENCH, 'SMOKE-LOG.md')) ? read(path.join(BENCH, 'SMOKE-LOG.md')) : '';
  for (const line of log.split('\n').filter((l) => /claude plugin eval /.test(l))) if (!/--no-publish/.test(line)) fail('P-8', 'a logged eval command lacks --no-publish');
  const total = log.split('\n').map((l) => l.split('|').map((x) => x.trim())).filter((c) => /^\d+$/.test(c[1] || '')).map((c) => Number(c[c.length - 2]) || 0).pop() || 0;
  if (total > 200) fail('P-7', `eval-run budget exceeded: ${total} > 200`);

  return failures;
}
