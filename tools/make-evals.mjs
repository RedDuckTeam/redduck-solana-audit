#!/usr/bin/env node
// Generates eval cases from benchmark/cases/<id>/{CASE.md,rubric.md}.
//   node tools/make-evals.mjs --role dev|public|heldout [--role ...] [--out <pluginDir>] [--clean]
// --out defaults to the shipped plugin; the ceiling probe passes benchmark/stub-plugin.
// --clean removes the generated cases of the given roles instead of writing them.
// Every case gets the same prompt, limits and tools; only the staged program differs.

import fs from 'node:fs';
import path from 'node:path';
import { ROOT, PLUGIN, CANARY, listCases, evalSides } from './lib/cases.mjs';
import { MAX_TURNS, TIMEOUT_SECONDS, ALLOWED_TOOLS, REPORT_FILE, GROUPS } from './lib/config.mjs';

const args = process.argv.slice(2);
const roles = args.flatMap((a, i) => (a === '--role' ? [args[i + 1]] : []));
const out = args.includes('--out') ? path.resolve(ROOT, args[args.indexOf('--out') + 1]) : PLUGIN;
const clean = args.includes('--clean');
if (!roles.length) throw new Error('pass at least one --role dev|public|heldout');

const prompt = fs.readFileSync(path.join(ROOT, 'tools', 'templates', 'prompt.md'), 'utf8');
const stageTemplate = fs.readFileSync(path.join(ROOT, 'tools', 'templates', 'stage.sh'), 'utf8');

const write = (file, text) => {
  fs.mkdirSync(path.dirname(file), { recursive: true });
  fs.writeFileSync(file, text.replace(/\r\n/g, '\n'));
};

function foundCriteria(c) {
  const r = c.rubric;
  const locations = r.locations.map((l) => `- ${l.file} lines ${l.start}-${l.end}`).join('\n');
  return [
    'You are grading a security audit report of a Solana program.',
    '',
    `The program contains this flaw: ${r.mechanism}`,
    '',
    'Acceptable places for the finding (paths are relative to the program folder; a report may prefix them with "program/"):',
    locations,
    '',
    'PASS if the report contains a finding that identifies this flaw (the same root cause, not just a nearby symptom or a generic best-practice remark) and ties it to one of the acceptable places, either by a line inside a listed range or by naming the function, instruction or account struct at that place.',
    'The severity of the finding and the section it appears in do not matter.',
    'FAIL otherwise.',
    r.criteria ? `\nAdditional guidance: ${r.criteria}` : '',
    '',
    `<!-- ${CANARY} -->`,
    '',
  ].join('\n');
}

let written = 0;
let removed = 0;
for (const c of listCases()) {
  if (!roles.includes(c.role)) continue;
  if (!GROUPS.includes(c.group)) throw new Error(`${c.id}: group "${c.group}" is not one of ${GROUPS.join(', ')}`);
  for (const { evalId, side } of evalSides(c)) {
    const dir = path.join(out, 'evals', evalId);
    if (clean) {
      if (fs.existsSync(dir)) {
        fs.rmSync(dir, { recursive: true, force: true });
        removed++;
      }
      continue;
    }
    if (side === 'vulnerable' && (!c.rubric || !c.rubric.mechanism || !c.rubric.locations.length)) {
      throw new Error(`${c.id}: rubric.md needs a Mechanism and at least one Location`);
    }
    fs.rmSync(dir, { recursive: true, force: true });
    write(path.join(dir, 'case.yaml'), [
      'schema_version: "1.1"',
      `name: ${evalId}`,
      `tags: [${c.group}, ${c.role}, ${side === 'vulnerable' ? 'vuln' : 'fixed'}]`,
      'context:',
      '  scaffold_script: stage.sh',
      'execution:',
      `  max_turns: ${MAX_TURNS}`,
      `  timeout_seconds: ${TIMEOUT_SECONDS}`,
      `  allowed_tools: [${ALLOWED_TOOLS.join(', ')}]`,
      '',
    ].join('\n'));
    write(path.join(dir, 'prompt.md'), prompt);
    write(path.join(dir, 'stage.sh'), stageTemplate.replace('__CASE_ID__', evalId));
    write(path.join(dir, 'graders', 'report-exists.md'),
      `---\ntype: file_exists\npath: ${REPORT_FILE}\narm: both\n---\n`);
    write(path.join(dir, 'graders', 'canary.md'),
      `---\ntype: regex\ntarget: trace\npattern: ${CANARY}\nmatch: not_contains\narm: both\n---\n`);
    write(path.join(dir, 'graders', 'skill.md'),
      `---\ntype: tool_used\ntool: Skill\narm: with-only\n---\n`);
    if (side === 'vulnerable') {
      write(path.join(dir, 'graders', 'found.md'),
        `---\ntype: llm\nfocus: { source: file, path: ${REPORT_FILE} }\n---\n${foundCriteria(c)}`);
    }
    written++;
  }
}
console.log(clean ? `make-evals: removed ${removed} case(s)` : `make-evals: wrote ${written} case(s) to ${path.relative(ROOT, out) || '.'}`);
