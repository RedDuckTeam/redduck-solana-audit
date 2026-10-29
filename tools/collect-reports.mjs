#!/usr/bin/env node
// Copies each run's report out of the --keep-temp directories into
// benchmark/results/<part>/ and records one row per run in runs.json.
//   node tools/collect-reports.mjs --json <eval-output.json> --part <name> --model <id> [--arm-as <arm>] [--traces] [--keep]
// --traces also copies each run's trace.jsonl to results/<part>/traces/ (gitignored).
// --arm-as without relabels a stub-plugin run (whose only arm is reported as "with")
// as the no-skill arm.
// The kept directory of a run is dirname(dirname(tracePath)) and the report is
// <keptDir>/home/cwd/solana-audit-report.md (RUNNING.md, Harness facts h).
// Kept directories are deleted after copying unless --keep is given.

import fs from 'node:fs';
import path from 'node:path';
import { BENCH } from './lib/cases.mjs';
import { REPORT_FILE } from './lib/config.mjs';

const args = process.argv.slice(2);
const arg = (name) => (args.includes(name) ? args[args.indexOf(name) + 1] : null);
const jsonFile = arg('--json');
const part = arg('--part');
const model = arg('--model');
if (!jsonFile || !part || !model) throw new Error('usage: --json <file> --part <name> --model <id> [--keep]');

const result = JSON.parse(fs.readFileSync(jsonFile, 'utf8'));
const partDir = path.join(BENCH, 'results', part);
const runsFile = path.join(partDir, 'runs.json');
const rows = fs.existsSync(runsFile) ? JSON.parse(fs.readFileSync(runsFile, 'utf8')) : [];

let copied = 0;
let missing = 0;
const armAs = arg('--arm-as');
for (const c of result.cases || []) {
  for (const rawArm of Object.keys(c.arms || {})) {
    const arm = armAs || rawArm;
    c.arms[rawArm].forEach((run) => {
      const prior = rows.filter((r) => r.evalId === c.name && r.arm === arm).length;
      const runId = `${arm}-${prior + 1}`;
      const target = path.join(partDir, 'reports', c.name, `${runId}.md`);
      let report = null;
      if (run.tracePath) {
        const keptDir = path.dirname(path.dirname(run.tracePath));
        const src = path.join(keptDir, 'home', 'cwd', REPORT_FILE);
        if (fs.existsSync(src)) {
          fs.mkdirSync(path.dirname(target), { recursive: true });
          fs.copyFileSync(src, target);
          report = path.relative(BENCH, target).split(path.sep).join('/');
          copied++;
        } else missing++;
        if (args.includes('--traces') && fs.existsSync(run.tracePath)) {
          const traceTarget = path.join(partDir, 'traces', c.name, `${runId}.trace.jsonl`);
          fs.mkdirSync(path.dirname(traceTarget), { recursive: true });
          fs.copyFileSync(run.tracePath, traceTarget);
        }
        if (!args.includes('--keep')) fs.rmSync(keptDir, { recursive: true, force: true });
      } else missing++;
      rows.push({
        evalId: c.name,
        arm,
        run: runId,
        model,
        error: run.error ?? null,
        score: run.score,
        turns: run.turns,
        durationSeconds: run.durationSeconds,
        costUsd: run.costUsd,
        graders: Object.fromEntries((run.graders || []).map((g) => [g.name, g.passed])),
        report,
        source: path.basename(jsonFile),
      });
    });
  }
}
fs.mkdirSync(partDir, { recursive: true });
fs.writeFileSync(runsFile, JSON.stringify(rows, null, 2) + '\n');
console.log(`collect-reports: ${copied} report(s) copied, ${missing} run(s) without a report, ${rows.length} row(s) in ${path.relative(BENCH, runsFile)}`);
