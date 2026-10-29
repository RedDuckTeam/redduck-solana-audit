#!/usr/bin/env node
// Prints a random case id from c01..c24 that is not used yet.
//   node tools/pick-id.mjs [--prefix c|p] [--max 24]

import fs from 'node:fs';
import path from 'node:path';
import crypto from 'node:crypto';
import { BENCH } from './lib/cases.mjs';

const args = process.argv.slice(2);
const prefix = args.includes('--prefix') ? args[args.indexOf('--prefix') + 1] : 'c';
const max = Number(args.includes('--max') ? args[args.indexOf('--max') + 1] : 24);
const used = new Set(fs.existsSync(path.join(BENCH, 'cases')) ? fs.readdirSync(path.join(BENCH, 'cases')) : []);
const free = [];
for (let i = 1; i <= max; i++) {
  const id = `${prefix}${String(i).padStart(2, '0')}`;
  if (!used.has(id)) free.push(id);
}
if (!free.length) throw new Error('no free ids');
console.log(free[crypto.randomInt(free.length)]);
