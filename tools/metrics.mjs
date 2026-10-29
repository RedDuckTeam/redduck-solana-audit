#!/usr/bin/env node
// Benchmark metrics from collected reports (plan, section "Metrics").
//   node tools/metrics.mjs --parts <dir>[,<dir>...] [--verdicts <file>] [--arbiter-input <file>] [--out <file>]
//   node tools/metrics.mjs --self-test
//
// recall: share of vulnerable cases whose `found` grader passed in at least 2 of 3 runs
//         (all runs when a case has fewer than 3).
// FP:     High/Critical findings judged false by the blind arbiter. On the vulnerable side
//         the planted finding (Location inside a rubric range) is excluded; on the fixed
//         side every High/Critical finding is a candidate, including at the patched place.
// noise:  findings at Medium or above, planted finding excluded.
// An unparseable report is not re-run: it counts as zero findings and as a format failure.

import fs from 'node:fs';
import path from 'node:path';
import crypto from 'node:crypto';
import { pathToFileURL } from 'node:url';

const SEVERITIES = ['critical', 'high', 'medium', 'low', 'info'];

export function parseReport(text) {
  if (text == null) return { ok: false, findings: [] };
  const findings = [];
  let current = null;
  for (const raw of text.split(/\r?\n/)) {
    // Strip markdown emphasis, code ticks, quotes and table bars; keep underscores, which file names use.
    const line = raw.replace(/[*`>|]/g, ' ').replace(/(^|\s)__?(?=\S)|(?<=\S)__?(?=\s|$)/g, '$1 ').trim();
    const sev = line.match(/^(?:[-#\d.)\s]*)severity\s*:\s*(critical|high|medium|low|informational|info)\b/i);
    if (sev) {
      current = { severity: sev[1].toLowerCase().startsWith('info') ? 'info' : sev[1].toLowerCase(), location: null, mechanism: '' };
      findings.push(current);
      continue;
    }
    if (!current) continue;
    const loc = line.match(/^(?:[-\s]*)location\s*:\s*\(?\s*([^\s:(),]+?):(\d+)/i);
    if (loc && !current.location) {
      current.location = { file: normalizePath(loc[1]), line: Number(loc[2]) };
      continue;
    }
    const mech = line.match(/^(?:[-\s]*)mechanism\s*:\s*(.+)$/i);
    if (mech && !current.mechanism) current.mechanism = mech[1].trim();
  }
  const ok = findings.every((f) => f.location && SEVERITIES.includes(f.severity));
  return { ok, findings: ok ? findings : [] };
}

export function normalizePath(p) {
  return p.replace(/\\/g, '/').replace(/^\.\//, '').replace(/^program\//, '');
}

export function isPlanted(finding, rubric) {
  if (!finding.location || !rubric) return false;
  return rubric.locations.some((l) => {
    const file = normalizePath(l.file);
    const sameFile = finding.location.file === file || finding.location.file.endsWith('/' + file) || file.endsWith('/' + finding.location.file);
    return sameFile && finding.location.line >= l.start && finding.location.line <= l.end;
  });
}

// Classifies the findings of one run. side: 'vulnerable' | 'fixed'.
export function classifyRun(reportText, side, rubric) {
  const { ok, findings } = parseReport(reportText);
  const candidates = [];
  let noise = 0;
  for (const f of findings) {
    const planted = side === 'vulnerable' && isPlanted(f, rubric);
    if (planted) continue;
    if (['critical', 'high', 'medium'].includes(f.severity)) noise++;
    if (['critical', 'high'].includes(f.severity)) candidates.push(f);
  }
  return { formatOk: ok, findings: findings.length, candidates, noise };
}

export function foundFromRuns(passes) {
  if (!passes.length) return false;
  const needed = passes.length >= 3 ? 2 : passes.length;
  return passes.filter(Boolean).length >= needed;
}

function seededRandom(seed) {
  let s = seed >>> 0;
  return () => {
    s = (s * 1664525 + 1013904223) >>> 0;
    return s / 2 ** 32;
  };
}

// Paired bootstrap over cases of (found with) - (found without).
export function bootstrapDiff(pairs, iterations = 10000, seed = 20261028) {
  if (!pairs.length) return null;
  const rnd = seededRandom(seed);
  const diffs = [];
  for (let i = 0; i < iterations; i++) {
    let sum = 0;
    for (let j = 0; j < pairs.length; j++) {
      const p = pairs[Math.floor(rnd() * pairs.length)];
      sum += (p.with ? 1 : 0) - (p.without ? 1 : 0);
    }
    diffs.push(sum / pairs.length);
  }
  diffs.sort((a, b) => a - b);
  return { low: diffs[Math.floor(iterations * 0.025)], high: diffs[Math.floor(iterations * 0.975)] };
}

function selfTest() {
  const rubric = { locations: [{ file: 'src/lib.rs', start: 40, end: 50 }] };
  const assert = (cond, msg) => {
    if (!cond) throw new Error(`self-test failed: ${msg}`);
    console.log(`ok - ${msg}`);
  };
  // (a) fixed side: High at the patched location is an FP candidate.
  const fixedReport = 'Severity: High\nLocation: program/src/lib.rs:45\nMechanism: missing signer check\n';
  const a = classifyRun(fixedReport, 'fixed', rubric);
  assert(a.candidates.length === 1, '(a) High at the patched location on the fixed side is an FP candidate');
  // (b) vulnerable side: a finding inside a rubric range is the planted one and is excluded.
  const vulnReport = '### 1\n- **Severity:** Critical\n- **Location:** `src/lib.rs:44`\n- **Mechanism:** x\n\nSeverity: High\nLocation: src/other.rs:3\nMechanism: y\n';
  const b = classifyRun(vulnReport, 'vulnerable', rubric);
  assert(b.findings === 2 && b.candidates.length === 1 && b.candidates[0].location.file === 'src/other.rs', '(b) planted finding inside a rubric range is excluded on the vulnerable side');
  // (c) unparseable report: zero findings, counted as a format failure.
  const c = classifyRun('Severity: High\nSomething without a location\n', 'fixed', rubric);
  assert(!c.formatOk && c.findings === 0 && c.candidates.length === 0, '(c) unparseable report yields zero findings and a format failure');
  const missing = classifyRun(null, 'fixed', rubric);
  assert(!missing.formatOk && missing.findings === 0, '(c) missing report is a format failure with zero findings');
  const underscore = classifyRun('Severity: Medium\nLocation: src/instructions/set_odds.rs:19\nMechanism: m\n', 'fixed', rubric);
  assert(underscore.formatOk && underscore.findings === 1, 'file names with underscores parse');
  const none = classifyRun('No findings.\n', 'fixed', rubric);
  assert(none.formatOk && none.findings === 0, 'report with "No findings." parses as zero findings');
  assert(foundFromRuns([true, true, false]) && !foundFromRuns([true, false, false]) && foundFromRuns([true]), 'found needs 2 of 3 runs (all runs when fewer than 3)');
  const ci = bootstrapDiff([{ with: true, without: false }, { with: true, without: true }]);
  assert(ci.low >= 0 && ci.high <= 1, 'bootstrap interval is within [0, 1] for non-negative differences');
  console.log('metrics self-test: ok');
}

async function main() {
  const args = process.argv.slice(2);
  if (args.includes('--self-test')) return selfTest();
  const arg = (n) => (args.includes(n) ? args[args.indexOf(n) + 1] : null);
  const { listCases, evalSides, readLedger, BENCH } = await import('./lib/cases.mjs');
  const parts = (arg('--parts') || '').split(',').filter(Boolean).map((p) => path.resolve(BENCH, 'results', p));
  if (!parts.length) throw new Error('pass --parts <name>[,<name>]');
  const verdicts = arg('--verdicts') && fs.existsSync(arg('--verdicts')) ? JSON.parse(fs.readFileSync(arg('--verdicts'), 'utf8')) : null;

  const cases = new Map(listCases().map((c) => [c.id, c]));
  // Held-out cases whose planted flaw reached the skill author in summary form (LEDGER readBy note "LEAK").
  const leaked = new Set(readLedger().cases
    .filter((c) => (c.readBy || []).some((r) => /^LEAK/.test(r.note || '')))
    .map((c) => c.id));
  const sideOf = (evalId) => {
    const c = cases.get(evalId.slice(0, -1));
    const match = c && evalSides(c).find((s) => s.evalId === evalId);
    return match ? { c, side: match.side } : null;
  };

  const runs = [];
  for (const dir of parts) {
    for (const r of JSON.parse(fs.readFileSync(path.join(dir, 'runs.json'), 'utf8'))) {
      if (r.error && !r.report) continue; // errored runs are re-run, not scored
      if (r.graders.canary === false) continue; // contaminated runs are re-run, not scored
      const info = sideOf(r.evalId);
      if (!info) continue;
      const text = r.report ? fs.readFileSync(path.join(BENCH, r.report), 'utf8') : null;
      const cls = classifyRun(text, info.side, info.c.rubric);
      runs.push({ ...r, caseId: info.c.id, side: info.side, role: info.c.role, cls: info.c.class, group: info.c.group, result: cls });
    }
  }

  // Blind arbiter input: candidates without arm/model, shuffled; key kept separately.
  const input = [];
  const key = {};
  for (const r of runs) {
    r.result.candidates.forEach((f, i) => {
      const cid = crypto.createHash('sha256').update(`${r.source}|${r.evalId}|${r.arm}|${r.run}|${i}`).digest('hex').slice(0, 12);
      input.push({ cid, evalId: r.evalId, program: `benchmark/fixtures/${r.evalId}/program`, severity: f.severity, location: `${f.location.file}:${f.location.line}`, mechanism: f.mechanism });
      key[cid] = { evalId: r.evalId, arm: r.arm, run: r.run, model: r.model };
      f.cid = cid;
    });
  }
  const rnd = seededRandom(7);
  for (let i = input.length - 1; i > 0; i--) {
    const j = Math.floor(rnd() * (i + 1));
    [input[i], input[j]] = [input[j], input[i]];
  }
  if (arg('--arbiter-input')) {
    fs.writeFileSync(arg('--arbiter-input'), JSON.stringify(input, null, 2) + '\n');
    fs.writeFileSync(arg('--arbiter-input').replace(/\.json$/, '.key.json'), JSON.stringify(key, null, 2) + '\n');
  }

  const groups = {};
  for (const r of runs) {
    const g = (groups[`${r.model}|${r.arm}`] ||= { model: r.model, arm: r.arm, runs: 0, formatFailures: 0, fp: 0, fpPending: 0, noise: 0, candidates: 0 });
    g.runs++;
    if (!r.result.formatOk) g.formatFailures++;
    g.noise += r.result.noise;
    for (const f of r.result.candidates) {
      g.candidates++;
      const v = verdicts?.[f.cid];
      if (v === 'false') g.fp++;
      else if (v !== 'valid') g.fpPending++;
    }
  }

  const recall = {};
  for (const r of runs.filter((x) => x.side === 'vulnerable')) {
    const k = `${r.model}|${r.caseId}`;
    const entry = (recall[k] ||= { model: r.model, caseId: r.caseId, role: r.role, cls: r.cls, with: [], without: [] });
    (entry[r.arm] ||= []).push(r.graders.found === true);
  }
  const perModel = {};
  for (const e of Object.values(recall)) {
    const m = (perModel[e.model] ||= []);
    m.push({ caseId: e.caseId, role: e.role, cls: e.cls, leaked: leaked.has(e.caseId), with: e.with.length ? foundFromRuns(e.with) : null, without: e.without.length ? foundFromRuns(e.without) : null });
  }

  const summary = { models: {}, arms: Object.values(groups) };
  for (const [model, list] of Object.entries(perModel)) {
    const slice = (filter) => {
      const l = list.filter(filter);
      const rate = (k) => {
        const v = l.filter((x) => x[k] !== null);
        return v.length ? v.filter((x) => x[k]).length / v.length : null;
      };
      const paired = l.filter((x) => x.with !== null && x.without !== null);
      return { cases: l.length, recallWith: rate('with'), recallWithout: rate('without'), ci: bootstrapDiff(paired) };
    };
    summary.models[model] = {
      all: slice(() => true),
      heldout: slice((x) => x.role === 'heldout'),
      heldoutClean: slice((x) => x.role === 'heldout' && !x.leaked),
      heldoutLeaked: slice((x) => x.role === 'heldout' && x.leaked),
      dev: slice((x) => x.role === 'dev'),
      public: slice((x) => x.role === 'public'),
      exploit: slice((x) => x.cls === 'exploit'),
      design: slice((x) => x.cls === 'design'),
      perCase: list,
    };
  }
  for (const g of summary.arms) {
    g.meanFp = g.runs ? g.fp / g.runs : null;
    g.meanNoise = g.runs ? g.noise / g.runs : null;
    g.formatFailureRate = g.runs ? g.formatFailures / g.runs : null;
  }
  if (arg('--out')) fs.writeFileSync(arg('--out'), JSON.stringify(summary, null, 2) + '\n');
  const pct = (x) => (x === null ? '—' : `${(x * 100).toFixed(1)}%`);
  for (const [model, s] of Object.entries(summary.models)) {
    console.log(`\n${model}`);
    for (const k of ['all', 'heldout', 'heldoutClean', 'heldoutLeaked', 'dev', 'public', 'exploit', 'design']) {
      const v = s[k];
      if (!v.cases) continue;
      console.log(`  ${k.padEnd(8)} cases=${v.cases} recall with=${pct(v.recallWith)} without=${pct(v.recallWithout)}${v.ci ? ` diff95%=[${pct(v.ci.low)}, ${pct(v.ci.high)}]` : ''}`);
    }
  }
  for (const g of summary.arms) {
    console.log(`  ${g.model} ${g.arm}: runs=${g.runs} meanFP=${g.meanFp?.toFixed(2)} (pending ${g.fpPending}) meanNoise=${g.meanNoise?.toFixed(2)} formatFailures=${pct(g.formatFailureRate)}`);
  }
}

if (process.argv[1] && pathToFileURL(path.resolve(process.argv[1])).href === import.meta.url) {
  await main();
}
