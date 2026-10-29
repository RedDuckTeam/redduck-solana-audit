#!/usr/bin/env node
// Prints random base58 program ids for declare_id! in benchmark programs.
//   node tools/new-program-id.mjs [count]

import crypto from 'node:crypto';

const ALPHABET = '123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz';

function base58(bytes) {
  let n = BigInt('0x' + Buffer.from(bytes).toString('hex'));
  let out = '';
  while (n > 0n) {
    out = ALPHABET[Number(n % 58n)] + out;
    n /= 58n;
  }
  for (const b of bytes) {
    if (b !== 0) break;
    out = '1' + out;
  }
  return out;
}

const count = Number(process.argv[2] || 1);
for (let i = 0; i < count; i++) console.log(base58(crypto.randomBytes(32)));
