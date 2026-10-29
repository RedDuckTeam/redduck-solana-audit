# Case ledger

Canary: RDSAF81C11F051B132899533

One entry per benchmark case: sha256 of every file, author, role, reviewer verdict and the sessions that read or ran it.
Isolation between the skill author and held-out cases is process-isolated, not enforced; the read-by list is self-reported.
Maintained by `node tools/ledger.mjs`; `node tools/verify.mjs` recomputes the hashes.

```json
{
  "cases": [
    {
      "id": "c01",
      "history": [
        {
          "at": "2026-10-28T18:43:17.732Z",
          "event": "add",
          "by": "case-author"
        },
        {
          "at": "2026-10-28T19:43:21.247Z",
          "event": "rehash",
          "by": "executor",
          "note": "author fixes round 1 after review-batchA"
        },
        {
          "at": "2026-10-28T19:51:40.137Z",
          "event": "review-fail",
          "by": "case-reviewer",
          "note": "re-review round 1: see logs/review-batchA.md"
        },
        {
          "at": "2026-10-28T20:03:04.535Z",
          "event": "rehash",
          "by": "executor",
          "note": "author fixes round 2"
        },
        {
          "at": "2026-10-28T20:06:08.753Z",
          "event": "review-pass",
          "by": "case-reviewer",
          "note": "re-review round 2"
        }
      ],
      "role": "heldout",
      "author": "case-author",
      "review": "pass",
      "readBy": [
        {
          "at": "2026-10-28T19:43:21.848Z",
          "by": "executor",
          "note": "LEAK: case author's fix report named the planted defect in one line; executor saw that summary, not the code"
        }
      ],
      "files": {
        "benchmark/cases/c01/CASE.md": "70a833cc250d290271613538e513359fcbb55c3d2c7d03aa2598d8f2ce78e5c4",
        "benchmark/cases/c01/rubric.md": "2f6f1f08825e4243a41b6cc721685995592a7e062bb89504c3a4b8b3be0cea45",
        "benchmark/fixtures/c01a/program/Cargo.toml": "f87c9904a6b8676793c85e77789dc63fc83dda885746d0809f19cabd8343b5d0",
        "benchmark/fixtures/c01a/program/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/fixtures/c01a/program/src/errors.rs": "315da6ade55cb5c32390c28b274a76c89cc39f55ea08bdd9413a9d2900b3b9bf",
        "benchmark/fixtures/c01a/program/src/events.rs": "8b9635f62f81929f118d623b4c9fccf581f604db91c85373fcc780198216fafc",
        "benchmark/fixtures/c01a/program/src/instructions/add_liquidity.rs": "dedddce7a54642a46a0839d61acb94eb4473344ef78b3ccbcbd734e2fa6e62ad",
        "benchmark/fixtures/c01a/program/src/instructions/borrow.rs": "d347bd82e6c363a2671454f6b9d4e514f797140907c991e8515e33de2d6329bf",
        "benchmark/fixtures/c01a/program/src/instructions/collateral.rs": "97a3c6376d0d595d378bf4912e4fa64d870b84e0cb31525b802a877f7e805bfa",
        "benchmark/fixtures/c01a/program/src/instructions/init.rs": "365f49a4112d57f765ca2acd97f9ab47731801180ed67985b55491f15968a752",
        "benchmark/fixtures/c01a/program/src/instructions/mod.rs": "dd0074f9d3af349f15a939ccabb6f568b7507b6f2c0aa5944e57ceab21ec8400",
        "benchmark/fixtures/c01a/program/src/instructions/open.rs": "7db8530d30ee41cd9707c15b79bdc91e3615354959bd0d1784d4f3d5318b4b11",
        "benchmark/fixtures/c01a/program/src/instructions/repay.rs": "3747587c87337873b6d9503db5472094a5bdbe353917dbf3afcbd7cf30b57075",
        "benchmark/fixtures/c01a/program/src/instructions/swap.rs": "02f34773b97314340f469155d6977f9d72d5fc18835ae82d06cb703e704ff7c6",
        "benchmark/fixtures/c01a/program/src/lib.rs": "ecaf337fc1a5acf708274511d8f60a98883371b80b0f7ddc657f99a0d60feabb",
        "benchmark/fixtures/c01a/program/src/state.rs": "598faf4b36241daf2fa40142f6892c871873437d5d32c4290cb5b1d3ac3fb7b7",
        "benchmark/fixtures/c01b/program/Cargo.toml": "f87c9904a6b8676793c85e77789dc63fc83dda885746d0809f19cabd8343b5d0",
        "benchmark/fixtures/c01b/program/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/fixtures/c01b/program/src/errors.rs": "315da6ade55cb5c32390c28b274a76c89cc39f55ea08bdd9413a9d2900b3b9bf",
        "benchmark/fixtures/c01b/program/src/events.rs": "8b9635f62f81929f118d623b4c9fccf581f604db91c85373fcc780198216fafc",
        "benchmark/fixtures/c01b/program/src/instructions/add_liquidity.rs": "dedddce7a54642a46a0839d61acb94eb4473344ef78b3ccbcbd734e2fa6e62ad",
        "benchmark/fixtures/c01b/program/src/instructions/borrow.rs": "41a28b783aa9a4358aac266e62e27e991e1e61cf0d7c0213968deca8e3ba9f35",
        "benchmark/fixtures/c01b/program/src/instructions/collateral.rs": "97a3c6376d0d595d378bf4912e4fa64d870b84e0cb31525b802a877f7e805bfa",
        "benchmark/fixtures/c01b/program/src/instructions/init.rs": "365f49a4112d57f765ca2acd97f9ab47731801180ed67985b55491f15968a752",
        "benchmark/fixtures/c01b/program/src/instructions/mod.rs": "dd0074f9d3af349f15a939ccabb6f568b7507b6f2c0aa5944e57ceab21ec8400",
        "benchmark/fixtures/c01b/program/src/instructions/open.rs": "7db8530d30ee41cd9707c15b79bdc91e3615354959bd0d1784d4f3d5318b4b11",
        "benchmark/fixtures/c01b/program/src/instructions/repay.rs": "3747587c87337873b6d9503db5472094a5bdbe353917dbf3afcbd7cf30b57075",
        "benchmark/fixtures/c01b/program/src/instructions/swap.rs": "02f34773b97314340f469155d6977f9d72d5fc18835ae82d06cb703e704ff7c6",
        "benchmark/fixtures/c01b/program/src/lib.rs": "ecaf337fc1a5acf708274511d8f60a98883371b80b0f7ddc657f99a0d60feabb",
        "benchmark/fixtures/c01b/program/src/state.rs": "598faf4b36241daf2fa40142f6892c871873437d5d32c4290cb5b1d3ac3fb7b7",
        "benchmark/workspaces/fixed/programs/lending_desk/Cargo.toml": "f87c9904a6b8676793c85e77789dc63fc83dda885746d0809f19cabd8343b5d0",
        "benchmark/workspaces/fixed/programs/lending_desk/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/workspaces/fixed/programs/lending_desk/src/errors.rs": "315da6ade55cb5c32390c28b274a76c89cc39f55ea08bdd9413a9d2900b3b9bf",
        "benchmark/workspaces/fixed/programs/lending_desk/src/events.rs": "8b9635f62f81929f118d623b4c9fccf581f604db91c85373fcc780198216fafc",
        "benchmark/workspaces/fixed/programs/lending_desk/src/instructions/add_liquidity.rs": "dedddce7a54642a46a0839d61acb94eb4473344ef78b3ccbcbd734e2fa6e62ad",
        "benchmark/workspaces/fixed/programs/lending_desk/src/instructions/borrow.rs": "d347bd82e6c363a2671454f6b9d4e514f797140907c991e8515e33de2d6329bf",
        "benchmark/workspaces/fixed/programs/lending_desk/src/instructions/collateral.rs": "97a3c6376d0d595d378bf4912e4fa64d870b84e0cb31525b802a877f7e805bfa",
        "benchmark/workspaces/fixed/programs/lending_desk/src/instructions/init.rs": "365f49a4112d57f765ca2acd97f9ab47731801180ed67985b55491f15968a752",
        "benchmark/workspaces/fixed/programs/lending_desk/src/instructions/mod.rs": "dd0074f9d3af349f15a939ccabb6f568b7507b6f2c0aa5944e57ceab21ec8400",
        "benchmark/workspaces/fixed/programs/lending_desk/src/instructions/open.rs": "7db8530d30ee41cd9707c15b79bdc91e3615354959bd0d1784d4f3d5318b4b11",
        "benchmark/workspaces/fixed/programs/lending_desk/src/instructions/repay.rs": "3747587c87337873b6d9503db5472094a5bdbe353917dbf3afcbd7cf30b57075",
        "benchmark/workspaces/fixed/programs/lending_desk/src/instructions/swap.rs": "02f34773b97314340f469155d6977f9d72d5fc18835ae82d06cb703e704ff7c6",
        "benchmark/workspaces/fixed/programs/lending_desk/src/lib.rs": "ecaf337fc1a5acf708274511d8f60a98883371b80b0f7ddc657f99a0d60feabb",
        "benchmark/workspaces/fixed/programs/lending_desk/src/state.rs": "598faf4b36241daf2fa40142f6892c871873437d5d32c4290cb5b1d3ac3fb7b7",
        "benchmark/workspaces/vulnerable/programs/lending_desk/Cargo.toml": "f87c9904a6b8676793c85e77789dc63fc83dda885746d0809f19cabd8343b5d0",
        "benchmark/workspaces/vulnerable/programs/lending_desk/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/workspaces/vulnerable/programs/lending_desk/src/errors.rs": "315da6ade55cb5c32390c28b274a76c89cc39f55ea08bdd9413a9d2900b3b9bf",
        "benchmark/workspaces/vulnerable/programs/lending_desk/src/events.rs": "8b9635f62f81929f118d623b4c9fccf581f604db91c85373fcc780198216fafc",
        "benchmark/workspaces/vulnerable/programs/lending_desk/src/instructions/add_liquidity.rs": "dedddce7a54642a46a0839d61acb94eb4473344ef78b3ccbcbd734e2fa6e62ad",
        "benchmark/workspaces/vulnerable/programs/lending_desk/src/instructions/borrow.rs": "41a28b783aa9a4358aac266e62e27e991e1e61cf0d7c0213968deca8e3ba9f35",
        "benchmark/workspaces/vulnerable/programs/lending_desk/src/instructions/collateral.rs": "97a3c6376d0d595d378bf4912e4fa64d870b84e0cb31525b802a877f7e805bfa",
        "benchmark/workspaces/vulnerable/programs/lending_desk/src/instructions/init.rs": "365f49a4112d57f765ca2acd97f9ab47731801180ed67985b55491f15968a752",
        "benchmark/workspaces/vulnerable/programs/lending_desk/src/instructions/mod.rs": "dd0074f9d3af349f15a939ccabb6f568b7507b6f2c0aa5944e57ceab21ec8400",
        "benchmark/workspaces/vulnerable/programs/lending_desk/src/instructions/open.rs": "7db8530d30ee41cd9707c15b79bdc91e3615354959bd0d1784d4f3d5318b4b11",
        "benchmark/workspaces/vulnerable/programs/lending_desk/src/instructions/repay.rs": "3747587c87337873b6d9503db5472094a5bdbe353917dbf3afcbd7cf30b57075",
        "benchmark/workspaces/vulnerable/programs/lending_desk/src/instructions/swap.rs": "02f34773b97314340f469155d6977f9d72d5fc18835ae82d06cb703e704ff7c6",
        "benchmark/workspaces/vulnerable/programs/lending_desk/src/lib.rs": "ecaf337fc1a5acf708274511d8f60a98883371b80b0f7ddc657f99a0d60feabb",
        "benchmark/workspaces/vulnerable/programs/lending_desk/src/state.rs": "598faf4b36241daf2fa40142f6892c871873437d5d32c4290cb5b1d3ac3fb7b7"
      }
    },
    {
      "id": "c02",
      "history": [
        {
          "at": "2026-10-28T18:43:17.814Z",
          "event": "add",
          "by": "case-author"
        },
        {
          "at": "2026-10-28T19:43:21.332Z",
          "event": "rehash",
          "by": "executor",
          "note": "author fixes round 1 after review-batchA"
        },
        {
          "at": "2026-10-28T19:51:39.994Z",
          "event": "review-pass",
          "by": "case-reviewer",
          "note": "re-review round 1"
        }
      ],
      "role": "heldout",
      "author": "case-author",
      "review": "pass",
      "readBy": [
        {
          "at": "2026-10-28T19:43:21.932Z",
          "by": "executor",
          "note": "LEAK: case author's fix report named the planted defect in one line; executor saw that summary, not the code"
        }
      ],
      "files": {
        "benchmark/cases/c02/CASE.md": "96c32e3476392ec240717080ce9aebbc20fd8ec0b3356a4985ed9ba756c6bdab",
        "benchmark/cases/c02/rubric.md": "b0b044918798ab5ae942a95405aa48cd0466e37eb94a1e64b857d0e303b89d1b",
        "benchmark/fixtures/c02a/program/Cargo.toml": "f2e6934dc361754ae3f6d7f692eeb02cdf9fc5db7a3510a1d4732ae6010df33c",
        "benchmark/fixtures/c02a/program/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/fixtures/c02a/program/src/errors.rs": "c1d0032684b10f3980738a98c434bfb3f8faedf0b2c243f01ab37bf847cc8983",
        "benchmark/fixtures/c02a/program/src/events.rs": "bbe1095762c92e846add48fd5052d0e7c7df355b5b161f5b6617dcf9a5eb4c82",
        "benchmark/fixtures/c02a/program/src/instructions/claim.rs": "be9dd45b85cd8451f37c8126e313f49b37f383aa4c359b97d356c7eba4adf521",
        "benchmark/fixtures/c02a/program/src/instructions/close.rs": "42105e0fd4352bcdc059ad274c1211b29faa211f337ef30f72e607655e109abf",
        "benchmark/fixtures/c02a/program/src/instructions/deposit.rs": "587b9df2874fbf7ad5e62a60f22cb1df35611e7e8e713958717dc9f306e790df",
        "benchmark/fixtures/c02a/program/src/instructions/distribute.rs": "e2618492e075dfa135ed7d023c4fc494977ddbab54e83674cad35e9827fcd8be",
        "benchmark/fixtures/c02a/program/src/instructions/init.rs": "0e4cae45e49c96fc53ef0616d2de65e4437e0ff4d56980e4aeeb1f11a2b7da55",
        "benchmark/fixtures/c02a/program/src/instructions/mod.rs": "37216fb748887b8951d4cacc52831d8dfb49fc42de6d9289df2d78c040ea36ea",
        "benchmark/fixtures/c02a/program/src/instructions/open.rs": "455d293a43ca53eb486ad18da55b3824146cea3c3f4d05532208dc0c31c0f854",
        "benchmark/fixtures/c02a/program/src/instructions/withdraw.rs": "ac00e0561a17a8cb264351f0bae7b9f8f8f4107b8ce1bacb4d1f306f40ca6e64",
        "benchmark/fixtures/c02a/program/src/lib.rs": "1f82827f464dc287111437acbf41a5711c0b62d1e28fbb77604dad29577bf45d",
        "benchmark/fixtures/c02a/program/src/state.rs": "903409ed9d95cf83e744dae647d365ca4678f4736267489328c6f5a7cd752ac7",
        "benchmark/fixtures/c02b/program/Cargo.toml": "f2e6934dc361754ae3f6d7f692eeb02cdf9fc5db7a3510a1d4732ae6010df33c",
        "benchmark/fixtures/c02b/program/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/fixtures/c02b/program/src/errors.rs": "c1d0032684b10f3980738a98c434bfb3f8faedf0b2c243f01ab37bf847cc8983",
        "benchmark/fixtures/c02b/program/src/events.rs": "bbe1095762c92e846add48fd5052d0e7c7df355b5b161f5b6617dcf9a5eb4c82",
        "benchmark/fixtures/c02b/program/src/instructions/claim.rs": "be9dd45b85cd8451f37c8126e313f49b37f383aa4c359b97d356c7eba4adf521",
        "benchmark/fixtures/c02b/program/src/instructions/close.rs": "42105e0fd4352bcdc059ad274c1211b29faa211f337ef30f72e607655e109abf",
        "benchmark/fixtures/c02b/program/src/instructions/deposit.rs": "4c4c1c471f97900b7e6fe4ff86717b1f0c6e4d1c07781321c99bffe0804dfef1",
        "benchmark/fixtures/c02b/program/src/instructions/distribute.rs": "e2618492e075dfa135ed7d023c4fc494977ddbab54e83674cad35e9827fcd8be",
        "benchmark/fixtures/c02b/program/src/instructions/init.rs": "0e4cae45e49c96fc53ef0616d2de65e4437e0ff4d56980e4aeeb1f11a2b7da55",
        "benchmark/fixtures/c02b/program/src/instructions/mod.rs": "37216fb748887b8951d4cacc52831d8dfb49fc42de6d9289df2d78c040ea36ea",
        "benchmark/fixtures/c02b/program/src/instructions/open.rs": "455d293a43ca53eb486ad18da55b3824146cea3c3f4d05532208dc0c31c0f854",
        "benchmark/fixtures/c02b/program/src/instructions/withdraw.rs": "ac00e0561a17a8cb264351f0bae7b9f8f8f4107b8ce1bacb4d1f306f40ca6e64",
        "benchmark/fixtures/c02b/program/src/lib.rs": "1f82827f464dc287111437acbf41a5711c0b62d1e28fbb77604dad29577bf45d",
        "benchmark/fixtures/c02b/program/src/state.rs": "903409ed9d95cf83e744dae647d365ca4678f4736267489328c6f5a7cd752ac7",
        "benchmark/workspaces/fixed/programs/commission_pool/Cargo.toml": "f2e6934dc361754ae3f6d7f692eeb02cdf9fc5db7a3510a1d4732ae6010df33c",
        "benchmark/workspaces/fixed/programs/commission_pool/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/workspaces/fixed/programs/commission_pool/src/errors.rs": "c1d0032684b10f3980738a98c434bfb3f8faedf0b2c243f01ab37bf847cc8983",
        "benchmark/workspaces/fixed/programs/commission_pool/src/events.rs": "bbe1095762c92e846add48fd5052d0e7c7df355b5b161f5b6617dcf9a5eb4c82",
        "benchmark/workspaces/fixed/programs/commission_pool/src/instructions/claim.rs": "be9dd45b85cd8451f37c8126e313f49b37f383aa4c359b97d356c7eba4adf521",
        "benchmark/workspaces/fixed/programs/commission_pool/src/instructions/close.rs": "42105e0fd4352bcdc059ad274c1211b29faa211f337ef30f72e607655e109abf",
        "benchmark/workspaces/fixed/programs/commission_pool/src/instructions/deposit.rs": "4c4c1c471f97900b7e6fe4ff86717b1f0c6e4d1c07781321c99bffe0804dfef1",
        "benchmark/workspaces/fixed/programs/commission_pool/src/instructions/distribute.rs": "e2618492e075dfa135ed7d023c4fc494977ddbab54e83674cad35e9827fcd8be",
        "benchmark/workspaces/fixed/programs/commission_pool/src/instructions/init.rs": "0e4cae45e49c96fc53ef0616d2de65e4437e0ff4d56980e4aeeb1f11a2b7da55",
        "benchmark/workspaces/fixed/programs/commission_pool/src/instructions/mod.rs": "37216fb748887b8951d4cacc52831d8dfb49fc42de6d9289df2d78c040ea36ea",
        "benchmark/workspaces/fixed/programs/commission_pool/src/instructions/open.rs": "455d293a43ca53eb486ad18da55b3824146cea3c3f4d05532208dc0c31c0f854",
        "benchmark/workspaces/fixed/programs/commission_pool/src/instructions/withdraw.rs": "ac00e0561a17a8cb264351f0bae7b9f8f8f4107b8ce1bacb4d1f306f40ca6e64",
        "benchmark/workspaces/fixed/programs/commission_pool/src/lib.rs": "1f82827f464dc287111437acbf41a5711c0b62d1e28fbb77604dad29577bf45d",
        "benchmark/workspaces/fixed/programs/commission_pool/src/state.rs": "903409ed9d95cf83e744dae647d365ca4678f4736267489328c6f5a7cd752ac7",
        "benchmark/workspaces/vulnerable/programs/commission_pool/Cargo.toml": "f2e6934dc361754ae3f6d7f692eeb02cdf9fc5db7a3510a1d4732ae6010df33c",
        "benchmark/workspaces/vulnerable/programs/commission_pool/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/workspaces/vulnerable/programs/commission_pool/src/errors.rs": "c1d0032684b10f3980738a98c434bfb3f8faedf0b2c243f01ab37bf847cc8983",
        "benchmark/workspaces/vulnerable/programs/commission_pool/src/events.rs": "bbe1095762c92e846add48fd5052d0e7c7df355b5b161f5b6617dcf9a5eb4c82",
        "benchmark/workspaces/vulnerable/programs/commission_pool/src/instructions/claim.rs": "be9dd45b85cd8451f37c8126e313f49b37f383aa4c359b97d356c7eba4adf521",
        "benchmark/workspaces/vulnerable/programs/commission_pool/src/instructions/close.rs": "42105e0fd4352bcdc059ad274c1211b29faa211f337ef30f72e607655e109abf",
        "benchmark/workspaces/vulnerable/programs/commission_pool/src/instructions/deposit.rs": "587b9df2874fbf7ad5e62a60f22cb1df35611e7e8e713958717dc9f306e790df",
        "benchmark/workspaces/vulnerable/programs/commission_pool/src/instructions/distribute.rs": "e2618492e075dfa135ed7d023c4fc494977ddbab54e83674cad35e9827fcd8be",
        "benchmark/workspaces/vulnerable/programs/commission_pool/src/instructions/init.rs": "0e4cae45e49c96fc53ef0616d2de65e4437e0ff4d56980e4aeeb1f11a2b7da55",
        "benchmark/workspaces/vulnerable/programs/commission_pool/src/instructions/mod.rs": "37216fb748887b8951d4cacc52831d8dfb49fc42de6d9289df2d78c040ea36ea",
        "benchmark/workspaces/vulnerable/programs/commission_pool/src/instructions/open.rs": "455d293a43ca53eb486ad18da55b3824146cea3c3f4d05532208dc0c31c0f854",
        "benchmark/workspaces/vulnerable/programs/commission_pool/src/instructions/withdraw.rs": "ac00e0561a17a8cb264351f0bae7b9f8f8f4107b8ce1bacb4d1f306f40ca6e64",
        "benchmark/workspaces/vulnerable/programs/commission_pool/src/lib.rs": "1f82827f464dc287111437acbf41a5711c0b62d1e28fbb77604dad29577bf45d",
        "benchmark/workspaces/vulnerable/programs/commission_pool/src/state.rs": "903409ed9d95cf83e744dae647d365ca4678f4736267489328c6f5a7cd752ac7"
      }
    },
    {
      "id": "c03",
      "history": [
        {
          "at": "2026-10-28T18:43:17.896Z",
          "event": "add",
          "by": "case-author"
        },
        {
          "at": "2026-10-28T19:43:21.420Z",
          "event": "rehash",
          "by": "executor",
          "note": "author fixes round 1 after review-batchA"
        },
        {
          "at": "2026-10-28T19:51:40.205Z",
          "event": "review-fail",
          "by": "case-reviewer",
          "note": "re-review round 1: see logs/review-batchA.md"
        },
        {
          "at": "2026-10-28T20:03:04.625Z",
          "event": "rehash",
          "by": "executor",
          "note": "author fixes round 2"
        },
        {
          "at": "2026-10-28T20:06:08.830Z",
          "event": "review-pass",
          "by": "case-reviewer",
          "note": "re-review round 2"
        }
      ],
      "role": "dev",
      "author": "case-author",
      "review": "pass",
      "readBy": [],
      "files": {
        "benchmark/cases/c03/CASE.md": "fc72d39924c0619dc1327cc37bcdff909bacf5627f270d266c170f0303d606e4",
        "benchmark/cases/c03/rubric.md": "8afd71f1c17ddfb7aa06d3e3490a0f310f640f9a68efedcfc9d98e43cb99b741",
        "benchmark/fixtures/c03a/program/Cargo.toml": "ea262ef5f2c5f61c14b7decfa8ad1467554a83c5723468112d32c58d2dd5ca81",
        "benchmark/fixtures/c03a/program/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/fixtures/c03a/program/src/errors.rs": "c9cf252f2078bdf5e66e4171f12c337e4def9c143b3165cc87b83099453cd956",
        "benchmark/fixtures/c03a/program/src/events.rs": "5efc1d9faea111a4a9ffe30d1aeaca7b3c97bc3ee7047760f6cefd21c40562a9",
        "benchmark/fixtures/c03a/program/src/instructions/accrue.rs": "ceac5f21166687ae6384be76680a87f0ed26b7b4be7dfdb3008886dc3ad51c7d",
        "benchmark/fixtures/c03a/program/src/instructions/close.rs": "0ce50cb89385f796c195a08f9b488d6c7c98772f34d4c06a31e0d01862eb4a31",
        "benchmark/fixtures/c03a/program/src/instructions/deposit.rs": "2833917a4ef516264e3da5bb73c3304f37c73fdbd1d7285500ab40474398d4ba",
        "benchmark/fixtures/c03a/program/src/instructions/initialize.rs": "0fa088fd6ed5200f8280a7b3c0c96aae851535738a4f51671084d12b9bdfbba1",
        "benchmark/fixtures/c03a/program/src/instructions/mod.rs": "f9ccad48eced79c18dfa3a6c04dd3ed046bd862952245ae12279598775bd8d4a",
        "benchmark/fixtures/c03a/program/src/instructions/open.rs": "d4195001f61df260946d25f7947978952b2dc9beaf7eaee8c45daab79821bddc",
        "benchmark/fixtures/c03a/program/src/instructions/redeem.rs": "f4ec6dc80a0b2c3a2110ec6bebe5501abd8810d47c13bf6bc6c57ecdb929f114",
        "benchmark/fixtures/c03a/program/src/instructions/withdraw.rs": "5d82b3604ed5903e303935d82940e1c197f451b58bdbbbbc3af014d569b6750c",
        "benchmark/fixtures/c03a/program/src/lib.rs": "7e8d7afa760474a29c0b79de23a8e19b265ca6cd39ea3ac58bdf18bfea54929d",
        "benchmark/fixtures/c03a/program/src/state.rs": "529da45854c57e60411af35c95f37cf58b4d057935879ceb4c54ea7cff8b4832",
        "benchmark/fixtures/c03b/program/Cargo.toml": "ea262ef5f2c5f61c14b7decfa8ad1467554a83c5723468112d32c58d2dd5ca81",
        "benchmark/fixtures/c03b/program/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/fixtures/c03b/program/src/errors.rs": "c9cf252f2078bdf5e66e4171f12c337e4def9c143b3165cc87b83099453cd956",
        "benchmark/fixtures/c03b/program/src/events.rs": "5efc1d9faea111a4a9ffe30d1aeaca7b3c97bc3ee7047760f6cefd21c40562a9",
        "benchmark/fixtures/c03b/program/src/instructions/accrue.rs": "ceac5f21166687ae6384be76680a87f0ed26b7b4be7dfdb3008886dc3ad51c7d",
        "benchmark/fixtures/c03b/program/src/instructions/close.rs": "0ce50cb89385f796c195a08f9b488d6c7c98772f34d4c06a31e0d01862eb4a31",
        "benchmark/fixtures/c03b/program/src/instructions/deposit.rs": "2833917a4ef516264e3da5bb73c3304f37c73fdbd1d7285500ab40474398d4ba",
        "benchmark/fixtures/c03b/program/src/instructions/initialize.rs": "0fa088fd6ed5200f8280a7b3c0c96aae851535738a4f51671084d12b9bdfbba1",
        "benchmark/fixtures/c03b/program/src/instructions/mod.rs": "f9ccad48eced79c18dfa3a6c04dd3ed046bd862952245ae12279598775bd8d4a",
        "benchmark/fixtures/c03b/program/src/instructions/open.rs": "d4195001f61df260946d25f7947978952b2dc9beaf7eaee8c45daab79821bddc",
        "benchmark/fixtures/c03b/program/src/instructions/redeem.rs": "f4ec6dc80a0b2c3a2110ec6bebe5501abd8810d47c13bf6bc6c57ecdb929f114",
        "benchmark/fixtures/c03b/program/src/instructions/withdraw.rs": "7099f2b8172556a586552bbfc07b0233a2cc350ae9ed491e2c913d0004984ec9",
        "benchmark/fixtures/c03b/program/src/lib.rs": "7e8d7afa760474a29c0b79de23a8e19b265ca6cd39ea3ac58bdf18bfea54929d",
        "benchmark/fixtures/c03b/program/src/state.rs": "529da45854c57e60411af35c95f37cf58b4d057935879ceb4c54ea7cff8b4832",
        "benchmark/workspaces/fixed/programs/note_market/Cargo.toml": "ea262ef5f2c5f61c14b7decfa8ad1467554a83c5723468112d32c58d2dd5ca81",
        "benchmark/workspaces/fixed/programs/note_market/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/workspaces/fixed/programs/note_market/src/errors.rs": "c9cf252f2078bdf5e66e4171f12c337e4def9c143b3165cc87b83099453cd956",
        "benchmark/workspaces/fixed/programs/note_market/src/events.rs": "5efc1d9faea111a4a9ffe30d1aeaca7b3c97bc3ee7047760f6cefd21c40562a9",
        "benchmark/workspaces/fixed/programs/note_market/src/instructions/accrue.rs": "ceac5f21166687ae6384be76680a87f0ed26b7b4be7dfdb3008886dc3ad51c7d",
        "benchmark/workspaces/fixed/programs/note_market/src/instructions/close.rs": "0ce50cb89385f796c195a08f9b488d6c7c98772f34d4c06a31e0d01862eb4a31",
        "benchmark/workspaces/fixed/programs/note_market/src/instructions/deposit.rs": "2833917a4ef516264e3da5bb73c3304f37c73fdbd1d7285500ab40474398d4ba",
        "benchmark/workspaces/fixed/programs/note_market/src/instructions/initialize.rs": "0fa088fd6ed5200f8280a7b3c0c96aae851535738a4f51671084d12b9bdfbba1",
        "benchmark/workspaces/fixed/programs/note_market/src/instructions/mod.rs": "f9ccad48eced79c18dfa3a6c04dd3ed046bd862952245ae12279598775bd8d4a",
        "benchmark/workspaces/fixed/programs/note_market/src/instructions/open.rs": "d4195001f61df260946d25f7947978952b2dc9beaf7eaee8c45daab79821bddc",
        "benchmark/workspaces/fixed/programs/note_market/src/instructions/redeem.rs": "f4ec6dc80a0b2c3a2110ec6bebe5501abd8810d47c13bf6bc6c57ecdb929f114",
        "benchmark/workspaces/fixed/programs/note_market/src/instructions/withdraw.rs": "5d82b3604ed5903e303935d82940e1c197f451b58bdbbbbc3af014d569b6750c",
        "benchmark/workspaces/fixed/programs/note_market/src/lib.rs": "7e8d7afa760474a29c0b79de23a8e19b265ca6cd39ea3ac58bdf18bfea54929d",
        "benchmark/workspaces/fixed/programs/note_market/src/state.rs": "529da45854c57e60411af35c95f37cf58b4d057935879ceb4c54ea7cff8b4832",
        "benchmark/workspaces/vulnerable/programs/note_market/Cargo.toml": "ea262ef5f2c5f61c14b7decfa8ad1467554a83c5723468112d32c58d2dd5ca81",
        "benchmark/workspaces/vulnerable/programs/note_market/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/workspaces/vulnerable/programs/note_market/src/errors.rs": "c9cf252f2078bdf5e66e4171f12c337e4def9c143b3165cc87b83099453cd956",
        "benchmark/workspaces/vulnerable/programs/note_market/src/events.rs": "5efc1d9faea111a4a9ffe30d1aeaca7b3c97bc3ee7047760f6cefd21c40562a9",
        "benchmark/workspaces/vulnerable/programs/note_market/src/instructions/accrue.rs": "ceac5f21166687ae6384be76680a87f0ed26b7b4be7dfdb3008886dc3ad51c7d",
        "benchmark/workspaces/vulnerable/programs/note_market/src/instructions/close.rs": "0ce50cb89385f796c195a08f9b488d6c7c98772f34d4c06a31e0d01862eb4a31",
        "benchmark/workspaces/vulnerable/programs/note_market/src/instructions/deposit.rs": "2833917a4ef516264e3da5bb73c3304f37c73fdbd1d7285500ab40474398d4ba",
        "benchmark/workspaces/vulnerable/programs/note_market/src/instructions/initialize.rs": "0fa088fd6ed5200f8280a7b3c0c96aae851535738a4f51671084d12b9bdfbba1",
        "benchmark/workspaces/vulnerable/programs/note_market/src/instructions/mod.rs": "f9ccad48eced79c18dfa3a6c04dd3ed046bd862952245ae12279598775bd8d4a",
        "benchmark/workspaces/vulnerable/programs/note_market/src/instructions/open.rs": "d4195001f61df260946d25f7947978952b2dc9beaf7eaee8c45daab79821bddc",
        "benchmark/workspaces/vulnerable/programs/note_market/src/instructions/redeem.rs": "f4ec6dc80a0b2c3a2110ec6bebe5501abd8810d47c13bf6bc6c57ecdb929f114",
        "benchmark/workspaces/vulnerable/programs/note_market/src/instructions/withdraw.rs": "7099f2b8172556a586552bbfc07b0233a2cc350ae9ed491e2c913d0004984ec9",
        "benchmark/workspaces/vulnerable/programs/note_market/src/lib.rs": "7e8d7afa760474a29c0b79de23a8e19b265ca6cd39ea3ac58bdf18bfea54929d",
        "benchmark/workspaces/vulnerable/programs/note_market/src/state.rs": "529da45854c57e60411af35c95f37cf58b4d057935879ceb4c54ea7cff8b4832"
      }
    },
    {
      "id": "c04",
      "history": [
        {
          "at": "2026-10-28T18:43:17.978Z",
          "event": "add",
          "by": "case-author"
        },
        {
          "at": "2026-10-28T19:43:21.507Z",
          "event": "rehash",
          "by": "executor",
          "note": "author fixes round 1 after review-batchA"
        },
        {
          "at": "2026-10-28T19:51:40.277Z",
          "event": "review-fail",
          "by": "case-reviewer",
          "note": "re-review round 1: see logs/review-batchA.md"
        },
        {
          "at": "2026-10-28T20:03:04.711Z",
          "event": "rehash",
          "by": "executor",
          "note": "author fixes round 2"
        },
        {
          "at": "2026-10-28T20:06:08.901Z",
          "event": "review-pass",
          "by": "case-reviewer",
          "note": "re-review round 2"
        }
      ],
      "role": "heldout",
      "author": "case-author",
      "review": "pass",
      "readBy": [
        {
          "at": "2026-10-28T19:43:22.007Z",
          "by": "executor",
          "note": "LEAK: case author's fix report named the planted defect in one line; executor saw that summary, not the code"
        }
      ],
      "files": {
        "benchmark/cases/c04/CASE.md": "2bd41a2bba1372b1d50b03fa29567d9929b4b1c1c80b9213e6894c3a7d97f8ec",
        "benchmark/cases/c04/rubric.md": "9407aee5ea32c848863a6db5137cf191e5d90d9cbe7b0d8710009540e2f5b520",
        "benchmark/fixtures/c04a/program/Cargo.toml": "a2a2e86648aa87a14670fb74e60dc0376f6fb6271d7c024601a095ab08c9d5fe",
        "benchmark/fixtures/c04a/program/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/fixtures/c04a/program/src/errors.rs": "24edf13d4357a9f941e95e5bb7482a2b7be8e14962447e4b4390c553d822cfdd",
        "benchmark/fixtures/c04a/program/src/events.rs": "1138493ec48850283202e791d960bfdcc47ea80f4e57be94ea01befb903dbde5",
        "benchmark/fixtures/c04a/program/src/instructions/buy.rs": "0ceea7f94646a886ae962436a6e04c5f8d5dd7725dd0b0027aa6948366187e32",
        "benchmark/fixtures/c04a/program/src/instructions/buy_bundle.rs": "e754419c7582533a6b064d7ac302f77abeb93faccc2ec0c15d00bd35601761d1",
        "benchmark/fixtures/c04a/program/src/instructions/claim.rs": "a666af2b0e7c9397a8b18e3ed916da44a624e8b86a09bcf42df32a001ee7eed7",
        "benchmark/fixtures/c04a/program/src/instructions/initialize.rs": "637a7379dd532317ea4f9acb2b3c353158e5a315115e7ac7ebb45bbc18b8fe6a",
        "benchmark/fixtures/c04a/program/src/instructions/mod.rs": "2e4635f81ba091cd02a863b8c4062441d33d49c8139db43b25a14db21fa7727e",
        "benchmark/fixtures/c04a/program/src/instructions/open.rs": "9930dbceb72fb28f96337cee70fb09773a4c73b6f223234893f93ec61048e129",
        "benchmark/fixtures/c04a/program/src/instructions/withdraw.rs": "d4a1e048a85d63b80211d6833ea8ed0e77897f1befc015be2d5f3256641ae1a9",
        "benchmark/fixtures/c04a/program/src/lib.rs": "54665c399b498ad4bd2ef979f4c19f887cde6f21f641aa93ae5a3a594f5731c2",
        "benchmark/fixtures/c04a/program/src/state.rs": "3147580e1a2117cad15b210e065a560a2d6f1c9b837c5f6f04e1830bd10c71a9",
        "benchmark/fixtures/c04b/program/Cargo.toml": "a2a2e86648aa87a14670fb74e60dc0376f6fb6271d7c024601a095ab08c9d5fe",
        "benchmark/fixtures/c04b/program/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/fixtures/c04b/program/src/errors.rs": "24edf13d4357a9f941e95e5bb7482a2b7be8e14962447e4b4390c553d822cfdd",
        "benchmark/fixtures/c04b/program/src/events.rs": "1138493ec48850283202e791d960bfdcc47ea80f4e57be94ea01befb903dbde5",
        "benchmark/fixtures/c04b/program/src/instructions/buy.rs": "0ceea7f94646a886ae962436a6e04c5f8d5dd7725dd0b0027aa6948366187e32",
        "benchmark/fixtures/c04b/program/src/instructions/buy_bundle.rs": "340389de60fee5f9af43675d24035693bcf6d3d60be5070de28cf3207d0e3ae2",
        "benchmark/fixtures/c04b/program/src/instructions/claim.rs": "a666af2b0e7c9397a8b18e3ed916da44a624e8b86a09bcf42df32a001ee7eed7",
        "benchmark/fixtures/c04b/program/src/instructions/initialize.rs": "637a7379dd532317ea4f9acb2b3c353158e5a315115e7ac7ebb45bbc18b8fe6a",
        "benchmark/fixtures/c04b/program/src/instructions/mod.rs": "2e4635f81ba091cd02a863b8c4062441d33d49c8139db43b25a14db21fa7727e",
        "benchmark/fixtures/c04b/program/src/instructions/open.rs": "9930dbceb72fb28f96337cee70fb09773a4c73b6f223234893f93ec61048e129",
        "benchmark/fixtures/c04b/program/src/instructions/withdraw.rs": "d4a1e048a85d63b80211d6833ea8ed0e77897f1befc015be2d5f3256641ae1a9",
        "benchmark/fixtures/c04b/program/src/lib.rs": "54665c399b498ad4bd2ef979f4c19f887cde6f21f641aa93ae5a3a594f5731c2",
        "benchmark/fixtures/c04b/program/src/state.rs": "3147580e1a2117cad15b210e065a560a2d6f1c9b837c5f6f04e1830bd10c71a9",
        "benchmark/workspaces/fixed/programs/launchpad/Cargo.toml": "a2a2e86648aa87a14670fb74e60dc0376f6fb6271d7c024601a095ab08c9d5fe",
        "benchmark/workspaces/fixed/programs/launchpad/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/workspaces/fixed/programs/launchpad/src/errors.rs": "24edf13d4357a9f941e95e5bb7482a2b7be8e14962447e4b4390c553d822cfdd",
        "benchmark/workspaces/fixed/programs/launchpad/src/events.rs": "1138493ec48850283202e791d960bfdcc47ea80f4e57be94ea01befb903dbde5",
        "benchmark/workspaces/fixed/programs/launchpad/src/instructions/buy.rs": "0ceea7f94646a886ae962436a6e04c5f8d5dd7725dd0b0027aa6948366187e32",
        "benchmark/workspaces/fixed/programs/launchpad/src/instructions/buy_bundle.rs": "340389de60fee5f9af43675d24035693bcf6d3d60be5070de28cf3207d0e3ae2",
        "benchmark/workspaces/fixed/programs/launchpad/src/instructions/claim.rs": "a666af2b0e7c9397a8b18e3ed916da44a624e8b86a09bcf42df32a001ee7eed7",
        "benchmark/workspaces/fixed/programs/launchpad/src/instructions/initialize.rs": "637a7379dd532317ea4f9acb2b3c353158e5a315115e7ac7ebb45bbc18b8fe6a",
        "benchmark/workspaces/fixed/programs/launchpad/src/instructions/mod.rs": "2e4635f81ba091cd02a863b8c4062441d33d49c8139db43b25a14db21fa7727e",
        "benchmark/workspaces/fixed/programs/launchpad/src/instructions/open.rs": "9930dbceb72fb28f96337cee70fb09773a4c73b6f223234893f93ec61048e129",
        "benchmark/workspaces/fixed/programs/launchpad/src/instructions/withdraw.rs": "d4a1e048a85d63b80211d6833ea8ed0e77897f1befc015be2d5f3256641ae1a9",
        "benchmark/workspaces/fixed/programs/launchpad/src/lib.rs": "54665c399b498ad4bd2ef979f4c19f887cde6f21f641aa93ae5a3a594f5731c2",
        "benchmark/workspaces/fixed/programs/launchpad/src/state.rs": "3147580e1a2117cad15b210e065a560a2d6f1c9b837c5f6f04e1830bd10c71a9",
        "benchmark/workspaces/vulnerable/programs/launchpad/Cargo.toml": "a2a2e86648aa87a14670fb74e60dc0376f6fb6271d7c024601a095ab08c9d5fe",
        "benchmark/workspaces/vulnerable/programs/launchpad/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/workspaces/vulnerable/programs/launchpad/src/errors.rs": "24edf13d4357a9f941e95e5bb7482a2b7be8e14962447e4b4390c553d822cfdd",
        "benchmark/workspaces/vulnerable/programs/launchpad/src/events.rs": "1138493ec48850283202e791d960bfdcc47ea80f4e57be94ea01befb903dbde5",
        "benchmark/workspaces/vulnerable/programs/launchpad/src/instructions/buy.rs": "0ceea7f94646a886ae962436a6e04c5f8d5dd7725dd0b0027aa6948366187e32",
        "benchmark/workspaces/vulnerable/programs/launchpad/src/instructions/buy_bundle.rs": "e754419c7582533a6b064d7ac302f77abeb93faccc2ec0c15d00bd35601761d1",
        "benchmark/workspaces/vulnerable/programs/launchpad/src/instructions/claim.rs": "a666af2b0e7c9397a8b18e3ed916da44a624e8b86a09bcf42df32a001ee7eed7",
        "benchmark/workspaces/vulnerable/programs/launchpad/src/instructions/initialize.rs": "637a7379dd532317ea4f9acb2b3c353158e5a315115e7ac7ebb45bbc18b8fe6a",
        "benchmark/workspaces/vulnerable/programs/launchpad/src/instructions/mod.rs": "2e4635f81ba091cd02a863b8c4062441d33d49c8139db43b25a14db21fa7727e",
        "benchmark/workspaces/vulnerable/programs/launchpad/src/instructions/open.rs": "9930dbceb72fb28f96337cee70fb09773a4c73b6f223234893f93ec61048e129",
        "benchmark/workspaces/vulnerable/programs/launchpad/src/instructions/withdraw.rs": "d4a1e048a85d63b80211d6833ea8ed0e77897f1befc015be2d5f3256641ae1a9",
        "benchmark/workspaces/vulnerable/programs/launchpad/src/lib.rs": "54665c399b498ad4bd2ef979f4c19f887cde6f21f641aa93ae5a3a594f5731c2",
        "benchmark/workspaces/vulnerable/programs/launchpad/src/state.rs": "3147580e1a2117cad15b210e065a560a2d6f1c9b837c5f6f04e1830bd10c71a9"
      }
    },
    {
      "id": "c05",
      "history": [
        {
          "at": "2026-10-28T18:43:18.071Z",
          "event": "add",
          "by": "case-author"
        },
        {
          "at": "2026-10-28T19:43:21.598Z",
          "event": "rehash",
          "by": "executor",
          "note": "author fixes round 1 after review-batchA"
        },
        {
          "at": "2026-10-28T19:51:40.067Z",
          "event": "review-pass",
          "by": "case-reviewer",
          "note": "re-review round 1"
        }
      ],
      "role": "heldout",
      "author": "case-author",
      "review": "pass",
      "readBy": [
        {
          "at": "2026-10-28T19:43:22.085Z",
          "by": "executor",
          "note": "LEAK: case author's fix report named the planted defect in one line; executor saw that summary, not the code"
        }
      ],
      "files": {
        "benchmark/cases/c05/CASE.md": "355362b3bc535d9080c7f51a53a08fc519f5d111e7c8dbc6dc28599a091433e6",
        "benchmark/cases/c05/rubric.md": "040a43fb04d15842a7cd94143f919e670939a9a2eec72dfb130ca3b0d1c175c0",
        "benchmark/fixtures/c05a/program/Cargo.toml": "df3ba2a6e65ff2211dccc35dec2b0bbc7521959cdb2d664404749e4a0f75bb97",
        "benchmark/fixtures/c05a/program/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/fixtures/c05a/program/src/errors.rs": "284852381ba5d872fa61030196bbc08b84f4ba528ce4e1324df23bd4356aa24e",
        "benchmark/fixtures/c05a/program/src/events.rs": "f82720a19dbbeb0e7bb54ace775db76e53d8704df062bcb325a3e0ee93e64c3f",
        "benchmark/fixtures/c05a/program/src/instructions/close.rs": "4587ae35aa7e080c6098d20647f6a0c12ec813a5a1e891098b17f67e44faf67a",
        "benchmark/fixtures/c05a/program/src/instructions/initialize.rs": "ad34f3af4cf95177e601278697b1e93af5de9e82c04acd1382cd8756c3573b33",
        "benchmark/fixtures/c05a/program/src/instructions/load.rs": "cbb92084b11944f27c4a4e7c2e07fee4f395f1d77609539dfbce038f6dd651bc",
        "benchmark/fixtures/c05a/program/src/instructions/merge.rs": "85c3bf69765ab7a0c8e5a388a70e475cecfa555361316efb4da390275370e100",
        "benchmark/fixtures/c05a/program/src/instructions/mod.rs": "f551708a4f7bceeba6577b1792a4af9169e4f319326cd38d06ada41d2f80d042",
        "benchmark/fixtures/c05a/program/src/instructions/open.rs": "abe2f52c32e01afd0af3a8b089b8994efeac67cdf8ce5abe378d6b97d25749b2",
        "benchmark/fixtures/c05a/program/src/instructions/redeem.rs": "6102994108111463a1180c3d39e3dd7de2b5af88ad1922809026aa5035310443",
        "benchmark/fixtures/c05a/program/src/instructions/transfer.rs": "041fe119af0b0a364223eda81a731992b2b8d737a1a15cd38efdf4ebe75c06ab",
        "benchmark/fixtures/c05a/program/src/lib.rs": "91f4ff6197b917ba9b2aa51c9be5b5a87edb27e3b1a87cd198ba9045adc26643",
        "benchmark/fixtures/c05a/program/src/state.rs": "26e352a03ffb45ab8a9f712bbfad9bbcb0b51fac6c80ba072a6754fd41b92853",
        "benchmark/fixtures/c05b/program/Cargo.toml": "df3ba2a6e65ff2211dccc35dec2b0bbc7521959cdb2d664404749e4a0f75bb97",
        "benchmark/fixtures/c05b/program/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/fixtures/c05b/program/src/errors.rs": "284852381ba5d872fa61030196bbc08b84f4ba528ce4e1324df23bd4356aa24e",
        "benchmark/fixtures/c05b/program/src/events.rs": "f82720a19dbbeb0e7bb54ace775db76e53d8704df062bcb325a3e0ee93e64c3f",
        "benchmark/fixtures/c05b/program/src/instructions/close.rs": "4587ae35aa7e080c6098d20647f6a0c12ec813a5a1e891098b17f67e44faf67a",
        "benchmark/fixtures/c05b/program/src/instructions/initialize.rs": "ad34f3af4cf95177e601278697b1e93af5de9e82c04acd1382cd8756c3573b33",
        "benchmark/fixtures/c05b/program/src/instructions/load.rs": "cbb92084b11944f27c4a4e7c2e07fee4f395f1d77609539dfbce038f6dd651bc",
        "benchmark/fixtures/c05b/program/src/instructions/merge.rs": "85c3bf69765ab7a0c8e5a388a70e475cecfa555361316efb4da390275370e100",
        "benchmark/fixtures/c05b/program/src/instructions/mod.rs": "f551708a4f7bceeba6577b1792a4af9169e4f319326cd38d06ada41d2f80d042",
        "benchmark/fixtures/c05b/program/src/instructions/open.rs": "abe2f52c32e01afd0af3a8b089b8994efeac67cdf8ce5abe378d6b97d25749b2",
        "benchmark/fixtures/c05b/program/src/instructions/redeem.rs": "6102994108111463a1180c3d39e3dd7de2b5af88ad1922809026aa5035310443",
        "benchmark/fixtures/c05b/program/src/instructions/transfer.rs": "a80a6f2cd5ab677f02eb77485306006fb5e43563cd0a124785af06293a43d4ed",
        "benchmark/fixtures/c05b/program/src/lib.rs": "91f4ff6197b917ba9b2aa51c9be5b5a87edb27e3b1a87cd198ba9045adc26643",
        "benchmark/fixtures/c05b/program/src/state.rs": "26e352a03ffb45ab8a9f712bbfad9bbcb0b51fac6c80ba072a6754fd41b92853",
        "benchmark/workspaces/fixed/programs/gift_card/Cargo.toml": "df3ba2a6e65ff2211dccc35dec2b0bbc7521959cdb2d664404749e4a0f75bb97",
        "benchmark/workspaces/fixed/programs/gift_card/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/workspaces/fixed/programs/gift_card/src/errors.rs": "284852381ba5d872fa61030196bbc08b84f4ba528ce4e1324df23bd4356aa24e",
        "benchmark/workspaces/fixed/programs/gift_card/src/events.rs": "f82720a19dbbeb0e7bb54ace775db76e53d8704df062bcb325a3e0ee93e64c3f",
        "benchmark/workspaces/fixed/programs/gift_card/src/instructions/close.rs": "4587ae35aa7e080c6098d20647f6a0c12ec813a5a1e891098b17f67e44faf67a",
        "benchmark/workspaces/fixed/programs/gift_card/src/instructions/initialize.rs": "ad34f3af4cf95177e601278697b1e93af5de9e82c04acd1382cd8756c3573b33",
        "benchmark/workspaces/fixed/programs/gift_card/src/instructions/load.rs": "cbb92084b11944f27c4a4e7c2e07fee4f395f1d77609539dfbce038f6dd651bc",
        "benchmark/workspaces/fixed/programs/gift_card/src/instructions/merge.rs": "85c3bf69765ab7a0c8e5a388a70e475cecfa555361316efb4da390275370e100",
        "benchmark/workspaces/fixed/programs/gift_card/src/instructions/mod.rs": "f551708a4f7bceeba6577b1792a4af9169e4f319326cd38d06ada41d2f80d042",
        "benchmark/workspaces/fixed/programs/gift_card/src/instructions/open.rs": "abe2f52c32e01afd0af3a8b089b8994efeac67cdf8ce5abe378d6b97d25749b2",
        "benchmark/workspaces/fixed/programs/gift_card/src/instructions/redeem.rs": "6102994108111463a1180c3d39e3dd7de2b5af88ad1922809026aa5035310443",
        "benchmark/workspaces/fixed/programs/gift_card/src/instructions/transfer.rs": "041fe119af0b0a364223eda81a731992b2b8d737a1a15cd38efdf4ebe75c06ab",
        "benchmark/workspaces/fixed/programs/gift_card/src/lib.rs": "91f4ff6197b917ba9b2aa51c9be5b5a87edb27e3b1a87cd198ba9045adc26643",
        "benchmark/workspaces/fixed/programs/gift_card/src/state.rs": "26e352a03ffb45ab8a9f712bbfad9bbcb0b51fac6c80ba072a6754fd41b92853",
        "benchmark/workspaces/vulnerable/programs/gift_card/Cargo.toml": "df3ba2a6e65ff2211dccc35dec2b0bbc7521959cdb2d664404749e4a0f75bb97",
        "benchmark/workspaces/vulnerable/programs/gift_card/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/workspaces/vulnerable/programs/gift_card/src/errors.rs": "284852381ba5d872fa61030196bbc08b84f4ba528ce4e1324df23bd4356aa24e",
        "benchmark/workspaces/vulnerable/programs/gift_card/src/events.rs": "f82720a19dbbeb0e7bb54ace775db76e53d8704df062bcb325a3e0ee93e64c3f",
        "benchmark/workspaces/vulnerable/programs/gift_card/src/instructions/close.rs": "4587ae35aa7e080c6098d20647f6a0c12ec813a5a1e891098b17f67e44faf67a",
        "benchmark/workspaces/vulnerable/programs/gift_card/src/instructions/initialize.rs": "ad34f3af4cf95177e601278697b1e93af5de9e82c04acd1382cd8756c3573b33",
        "benchmark/workspaces/vulnerable/programs/gift_card/src/instructions/load.rs": "cbb92084b11944f27c4a4e7c2e07fee4f395f1d77609539dfbce038f6dd651bc",
        "benchmark/workspaces/vulnerable/programs/gift_card/src/instructions/merge.rs": "85c3bf69765ab7a0c8e5a388a70e475cecfa555361316efb4da390275370e100",
        "benchmark/workspaces/vulnerable/programs/gift_card/src/instructions/mod.rs": "f551708a4f7bceeba6577b1792a4af9169e4f319326cd38d06ada41d2f80d042",
        "benchmark/workspaces/vulnerable/programs/gift_card/src/instructions/open.rs": "abe2f52c32e01afd0af3a8b089b8994efeac67cdf8ce5abe378d6b97d25749b2",
        "benchmark/workspaces/vulnerable/programs/gift_card/src/instructions/redeem.rs": "6102994108111463a1180c3d39e3dd7de2b5af88ad1922809026aa5035310443",
        "benchmark/workspaces/vulnerable/programs/gift_card/src/instructions/transfer.rs": "a80a6f2cd5ab677f02eb77485306006fb5e43563cd0a124785af06293a43d4ed",
        "benchmark/workspaces/vulnerable/programs/gift_card/src/lib.rs": "91f4ff6197b917ba9b2aa51c9be5b5a87edb27e3b1a87cd198ba9045adc26643",
        "benchmark/workspaces/vulnerable/programs/gift_card/src/state.rs": "26e352a03ffb45ab8a9f712bbfad9bbcb0b51fac6c80ba072a6754fd41b92853"
      }
    },
    {
      "id": "c06",
      "history": [
        {
          "at": "2026-10-28T18:43:18.151Z",
          "event": "add",
          "by": "case-author"
        },
        {
          "at": "2026-10-28T19:43:21.688Z",
          "event": "rehash",
          "by": "executor",
          "note": "author fixes round 1 after review-batchA"
        },
        {
          "at": "2026-10-28T19:51:40.350Z",
          "event": "review-fail",
          "by": "case-reviewer",
          "note": "re-review round 1: see logs/review-batchA.md"
        },
        {
          "at": "2026-10-28T20:03:04.795Z",
          "event": "rehash",
          "by": "executor",
          "note": "author fixes round 2"
        },
        {
          "at": "2026-10-28T20:06:08.974Z",
          "event": "review-pass",
          "by": "case-reviewer",
          "note": "re-review round 2"
        }
      ],
      "role": "dev",
      "author": "case-author",
      "review": "pass",
      "readBy": [],
      "files": {
        "benchmark/cases/c06/CASE.md": "bea87633e22421b0b97a564cec448df1788fb4afc31ce325249cf36dc9725bcd",
        "benchmark/cases/c06/rubric.md": "bdd229a626852de9e14170721fcbe34c3eb23a7f6b743f3a760374e5843a76e2",
        "benchmark/fixtures/c06a/program/Cargo.toml": "7d9c8ea63acd5783bae893e73d5db9116737e44e7e44e844826c175c7350aff6",
        "benchmark/fixtures/c06a/program/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/fixtures/c06a/program/src/errors.rs": "660a98bcdb2ee18496ae55442d889dc3bb198ee383535af77812da935dcc1940",
        "benchmark/fixtures/c06a/program/src/events.rs": "4a680c14e6d8d9f1797f113041477b724cfe0387f877b9d2c096345552231981",
        "benchmark/fixtures/c06a/program/src/instructions/cancel.rs": "9216b4f28cab5378e33f3dfebe0d1fe03e6efa4d8df83da217dcecf7be92ac2c",
        "benchmark/fixtures/c06a/program/src/instructions/fund.rs": "c6ba453cc852c2da1bc490d88bef13f2a9cfb107a97c2fd919114a6654eb397d",
        "benchmark/fixtures/c06a/program/src/instructions/init.rs": "a95411826c33ad39b94d907832ae81f2ef20fac3d56069853aa42656d53c869c",
        "benchmark/fixtures/c06a/program/src/instructions/mod.rs": "4168ff29f9cbac48532cb373526e94b3066fa845e8a409e0f4283f3279cf3196",
        "benchmark/fixtures/c06a/program/src/instructions/open.rs": "0129761c039f0743351a0c88892459afac7f7aebd83f1bf08813f3b784573b90",
        "benchmark/fixtures/c06a/program/src/instructions/reveal.rs": "9ecfacbfae87cdb1a6940afd912a33fba3aa32c490ae3afda0e6da8a4f4be3a3",
        "benchmark/fixtures/c06a/program/src/instructions/set_odds.rs": "1479695f199b3f6fc3fbe334e6e7edda536deee49e8d5c044abdbe2479e22d7b",
        "benchmark/fixtures/c06a/program/src/instructions/withdraw.rs": "98699aeba31ceec364877fee8d85035573ee6b5b9daa07520da66218da9a2ea8",
        "benchmark/fixtures/c06a/program/src/lib.rs": "36c021cd1b93c84bceff38990053b66895b5f32ef0c4d335c77e8a2f2aed9589",
        "benchmark/fixtures/c06a/program/src/state.rs": "f2461618a4650c154ff39856acf0e72918780027a5edb9d653a74e71cc91a014",
        "benchmark/fixtures/c06b/program/Cargo.toml": "7d9c8ea63acd5783bae893e73d5db9116737e44e7e44e844826c175c7350aff6",
        "benchmark/fixtures/c06b/program/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/fixtures/c06b/program/src/errors.rs": "f985a9f6109fd8a9fb034978d7fd33e74d189f0e224f6f35d1bdca9c867380d5",
        "benchmark/fixtures/c06b/program/src/events.rs": "4a680c14e6d8d9f1797f113041477b724cfe0387f877b9d2c096345552231981",
        "benchmark/fixtures/c06b/program/src/instructions/cancel.rs": "e7489aa0aeec298985e63c0f05ab173ae1d895cd7a5ccdfd5c537f58c5e214bc",
        "benchmark/fixtures/c06b/program/src/instructions/fund.rs": "c6ba453cc852c2da1bc490d88bef13f2a9cfb107a97c2fd919114a6654eb397d",
        "benchmark/fixtures/c06b/program/src/instructions/init.rs": "a95411826c33ad39b94d907832ae81f2ef20fac3d56069853aa42656d53c869c",
        "benchmark/fixtures/c06b/program/src/instructions/mod.rs": "4168ff29f9cbac48532cb373526e94b3066fa845e8a409e0f4283f3279cf3196",
        "benchmark/fixtures/c06b/program/src/instructions/open.rs": "0129761c039f0743351a0c88892459afac7f7aebd83f1bf08813f3b784573b90",
        "benchmark/fixtures/c06b/program/src/instructions/reveal.rs": "9ecfacbfae87cdb1a6940afd912a33fba3aa32c490ae3afda0e6da8a4f4be3a3",
        "benchmark/fixtures/c06b/program/src/instructions/set_odds.rs": "1479695f199b3f6fc3fbe334e6e7edda536deee49e8d5c044abdbe2479e22d7b",
        "benchmark/fixtures/c06b/program/src/instructions/withdraw.rs": "98699aeba31ceec364877fee8d85035573ee6b5b9daa07520da66218da9a2ea8",
        "benchmark/fixtures/c06b/program/src/lib.rs": "36c021cd1b93c84bceff38990053b66895b5f32ef0c4d335c77e8a2f2aed9589",
        "benchmark/fixtures/c06b/program/src/state.rs": "f2461618a4650c154ff39856acf0e72918780027a5edb9d653a74e71cc91a014",
        "benchmark/workspaces/fixed/programs/loot_crate/Cargo.toml": "7d9c8ea63acd5783bae893e73d5db9116737e44e7e44e844826c175c7350aff6",
        "benchmark/workspaces/fixed/programs/loot_crate/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/workspaces/fixed/programs/loot_crate/src/errors.rs": "f985a9f6109fd8a9fb034978d7fd33e74d189f0e224f6f35d1bdca9c867380d5",
        "benchmark/workspaces/fixed/programs/loot_crate/src/events.rs": "4a680c14e6d8d9f1797f113041477b724cfe0387f877b9d2c096345552231981",
        "benchmark/workspaces/fixed/programs/loot_crate/src/instructions/cancel.rs": "e7489aa0aeec298985e63c0f05ab173ae1d895cd7a5ccdfd5c537f58c5e214bc",
        "benchmark/workspaces/fixed/programs/loot_crate/src/instructions/fund.rs": "c6ba453cc852c2da1bc490d88bef13f2a9cfb107a97c2fd919114a6654eb397d",
        "benchmark/workspaces/fixed/programs/loot_crate/src/instructions/init.rs": "a95411826c33ad39b94d907832ae81f2ef20fac3d56069853aa42656d53c869c",
        "benchmark/workspaces/fixed/programs/loot_crate/src/instructions/mod.rs": "4168ff29f9cbac48532cb373526e94b3066fa845e8a409e0f4283f3279cf3196",
        "benchmark/workspaces/fixed/programs/loot_crate/src/instructions/open.rs": "0129761c039f0743351a0c88892459afac7f7aebd83f1bf08813f3b784573b90",
        "benchmark/workspaces/fixed/programs/loot_crate/src/instructions/reveal.rs": "9ecfacbfae87cdb1a6940afd912a33fba3aa32c490ae3afda0e6da8a4f4be3a3",
        "benchmark/workspaces/fixed/programs/loot_crate/src/instructions/set_odds.rs": "1479695f199b3f6fc3fbe334e6e7edda536deee49e8d5c044abdbe2479e22d7b",
        "benchmark/workspaces/fixed/programs/loot_crate/src/instructions/withdraw.rs": "98699aeba31ceec364877fee8d85035573ee6b5b9daa07520da66218da9a2ea8",
        "benchmark/workspaces/fixed/programs/loot_crate/src/lib.rs": "36c021cd1b93c84bceff38990053b66895b5f32ef0c4d335c77e8a2f2aed9589",
        "benchmark/workspaces/fixed/programs/loot_crate/src/state.rs": "f2461618a4650c154ff39856acf0e72918780027a5edb9d653a74e71cc91a014",
        "benchmark/workspaces/vulnerable/programs/loot_crate/Cargo.toml": "7d9c8ea63acd5783bae893e73d5db9116737e44e7e44e844826c175c7350aff6",
        "benchmark/workspaces/vulnerable/programs/loot_crate/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/workspaces/vulnerable/programs/loot_crate/src/errors.rs": "660a98bcdb2ee18496ae55442d889dc3bb198ee383535af77812da935dcc1940",
        "benchmark/workspaces/vulnerable/programs/loot_crate/src/events.rs": "4a680c14e6d8d9f1797f113041477b724cfe0387f877b9d2c096345552231981",
        "benchmark/workspaces/vulnerable/programs/loot_crate/src/instructions/cancel.rs": "9216b4f28cab5378e33f3dfebe0d1fe03e6efa4d8df83da217dcecf7be92ac2c",
        "benchmark/workspaces/vulnerable/programs/loot_crate/src/instructions/fund.rs": "c6ba453cc852c2da1bc490d88bef13f2a9cfb107a97c2fd919114a6654eb397d",
        "benchmark/workspaces/vulnerable/programs/loot_crate/src/instructions/init.rs": "a95411826c33ad39b94d907832ae81f2ef20fac3d56069853aa42656d53c869c",
        "benchmark/workspaces/vulnerable/programs/loot_crate/src/instructions/mod.rs": "4168ff29f9cbac48532cb373526e94b3066fa845e8a409e0f4283f3279cf3196",
        "benchmark/workspaces/vulnerable/programs/loot_crate/src/instructions/open.rs": "0129761c039f0743351a0c88892459afac7f7aebd83f1bf08813f3b784573b90",
        "benchmark/workspaces/vulnerable/programs/loot_crate/src/instructions/reveal.rs": "9ecfacbfae87cdb1a6940afd912a33fba3aa32c490ae3afda0e6da8a4f4be3a3",
        "benchmark/workspaces/vulnerable/programs/loot_crate/src/instructions/set_odds.rs": "1479695f199b3f6fc3fbe334e6e7edda536deee49e8d5c044abdbe2479e22d7b",
        "benchmark/workspaces/vulnerable/programs/loot_crate/src/instructions/withdraw.rs": "98699aeba31ceec364877fee8d85035573ee6b5b9daa07520da66218da9a2ea8",
        "benchmark/workspaces/vulnerable/programs/loot_crate/src/lib.rs": "36c021cd1b93c84bceff38990053b66895b5f32ef0c4d335c77e8a2f2aed9589",
        "benchmark/workspaces/vulnerable/programs/loot_crate/src/state.rs": "f2461618a4650c154ff39856acf0e72918780027a5edb9d653a74e71cc91a014"
      }
    },
    {
      "id": "c07",
      "history": [
        {
          "at": "2026-10-28T18:43:18.232Z",
          "event": "add",
          "by": "case-author"
        },
        {
          "at": "2026-10-28T19:43:21.775Z",
          "event": "rehash",
          "by": "executor",
          "note": "author fixes round 1 after review-batchA"
        },
        {
          "at": "2026-10-28T19:51:40.435Z",
          "event": "review-fail",
          "by": "case-reviewer",
          "note": "re-review round 1: see logs/review-batchA.md"
        },
        {
          "at": "2026-10-28T20:03:04.881Z",
          "event": "rehash",
          "by": "executor",
          "note": "author fixes round 2"
        },
        {
          "at": "2026-10-28T20:06:09.048Z",
          "event": "review-pass",
          "by": "case-reviewer",
          "note": "re-review round 2"
        }
      ],
      "role": "heldout",
      "author": "case-author",
      "review": "pass",
      "readBy": [
        {
          "at": "2026-10-28T19:43:22.158Z",
          "by": "executor",
          "note": "LEAK: case author's fix report named the planted defect in one line; executor saw that summary, not the code"
        }
      ],
      "files": {
        "benchmark/cases/c07/CASE.md": "d3b061ce1a415e1b9bd695b4548ffadd74083b915def6d0145a2557f83ca2c3b",
        "benchmark/cases/c07/rubric.md": "96e6533c2a5708d4127f86e73a3ea5b006cc4116e25f88dedfced1e1d9f47867",
        "benchmark/fixtures/c07a/program/ARCHITECTURE.md": "7a4cfad8945808471837d7ebee7ca91ff9101b07371888573759c6a02361edb8",
        "benchmark/fixtures/c07a/program/Cargo.toml": "9862154974303c529ab2cefcbe27b807ffef2976af58f3f0d283f95c8e5d19b5",
        "benchmark/fixtures/c07a/program/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/fixtures/c07a/program/src/errors.rs": "f1b9cc01d8f54dfe276a52c7fcb4c785182ec710e410124933e1343aac8c69c1",
        "benchmark/fixtures/c07a/program/src/events.rs": "5650a96acb6d708ab659d99528450d9cc42b96d3554ee80191a52159fc8939b8",
        "benchmark/fixtures/c07a/program/src/instructions/deposit.rs": "7d02e63afc9812231ccaecb11e7823f5b737af6b970cb427d4e4f39162e039b8",
        "benchmark/fixtures/c07a/program/src/instructions/enter.rs": "30fea19f6945f1e5141c5043cb89abd322fd16642721b4a789f03516b6f7e0c0",
        "benchmark/fixtures/c07a/program/src/instructions/expire.rs": "41e8727955ec8f26e71580cf9855b945227c2aec308726d9169fc5199dfc9b7b",
        "benchmark/fixtures/c07a/program/src/instructions/init.rs": "b3d3919d9b429216f299b0808581f79e38ade39a25627ba2d0dc2b0649c8e497",
        "benchmark/fixtures/c07a/program/src/instructions/mod.rs": "2d4a319640f6baf9c17424eeb97ae78464896acb9192bedbb8b1f51624a63a7e",
        "benchmark/fixtures/c07a/program/src/instructions/open.rs": "e1456e99a732695aefd803700fa002267cded9a98fef36d963c539c018ae24c9",
        "benchmark/fixtures/c07a/program/src/instructions/settle.rs": "82697a2735ee2fadbe3275ebda1392b3ecc266f6c611875d175319f9403665fd",
        "benchmark/fixtures/c07a/program/src/instructions/withdraw.rs": "7c5518715a1e8188f3cd183c6a5c4f1a9a439ad4baf124baf500d67dfba6bede",
        "benchmark/fixtures/c07a/program/src/lib.rs": "13ef9119ab448cdde3f7a42f6d4f8af6dab6f562d444e3ef46545b393d2c0d58",
        "benchmark/fixtures/c07a/program/src/state.rs": "e1657b2bb6260ddd8f9211143d72bb821057a3dcb0f61e8d0f93d1469ee9dd66",
        "benchmark/fixtures/c07a/program/src/token_ops.rs": "2011f2d3dabda03dea739a5e7aa4dc8e2009e4c4e3254c955a8a5290ee0164ca",
        "benchmark/fixtures/c07b/program/ARCHITECTURE.md": "a033ac7182baacdeab801f95f707469440027e05a106324e3a61f3486b9e044e",
        "benchmark/fixtures/c07b/program/Cargo.toml": "9862154974303c529ab2cefcbe27b807ffef2976af58f3f0d283f95c8e5d19b5",
        "benchmark/fixtures/c07b/program/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/fixtures/c07b/program/src/errors.rs": "f1b9cc01d8f54dfe276a52c7fcb4c785182ec710e410124933e1343aac8c69c1",
        "benchmark/fixtures/c07b/program/src/events.rs": "5650a96acb6d708ab659d99528450d9cc42b96d3554ee80191a52159fc8939b8",
        "benchmark/fixtures/c07b/program/src/instructions/deposit.rs": "7d02e63afc9812231ccaecb11e7823f5b737af6b970cb427d4e4f39162e039b8",
        "benchmark/fixtures/c07b/program/src/instructions/enter.rs": "05fdc6abc4a30158d8734bf6fc9a7ceaea8dabd1aacec42301ffed531cb1b1ea",
        "benchmark/fixtures/c07b/program/src/instructions/expire.rs": "db233638fa8df34aa0e9521d564501524d94c1f0498ff02b3d485bc3702dd5a0",
        "benchmark/fixtures/c07b/program/src/instructions/init.rs": "b3d3919d9b429216f299b0808581f79e38ade39a25627ba2d0dc2b0649c8e497",
        "benchmark/fixtures/c07b/program/src/instructions/mod.rs": "2d4a319640f6baf9c17424eeb97ae78464896acb9192bedbb8b1f51624a63a7e",
        "benchmark/fixtures/c07b/program/src/instructions/open.rs": "e1456e99a732695aefd803700fa002267cded9a98fef36d963c539c018ae24c9",
        "benchmark/fixtures/c07b/program/src/instructions/settle.rs": "3dcb5138eff5c804de29bd9b5d9700d2a670a462205787f54b9a9d9d04e9fd58",
        "benchmark/fixtures/c07b/program/src/instructions/withdraw.rs": "8a59197d521409f9a1303b9f3a9077b9d7b8fd5dc24c7b4c8206a3735215dd64",
        "benchmark/fixtures/c07b/program/src/lib.rs": "13ef9119ab448cdde3f7a42f6d4f8af6dab6f562d444e3ef46545b393d2c0d58",
        "benchmark/fixtures/c07b/program/src/state.rs": "e1657b2bb6260ddd8f9211143d72bb821057a3dcb0f61e8d0f93d1469ee9dd66",
        "benchmark/fixtures/c07b/program/src/token_ops.rs": "2011f2d3dabda03dea739a5e7aa4dc8e2009e4c4e3254c955a8a5290ee0164ca",
        "benchmark/workspaces/fixed/programs/wager_arena/ARCHITECTURE.md": "7a4cfad8945808471837d7ebee7ca91ff9101b07371888573759c6a02361edb8",
        "benchmark/workspaces/fixed/programs/wager_arena/Cargo.toml": "9862154974303c529ab2cefcbe27b807ffef2976af58f3f0d283f95c8e5d19b5",
        "benchmark/workspaces/fixed/programs/wager_arena/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/workspaces/fixed/programs/wager_arena/src/errors.rs": "f1b9cc01d8f54dfe276a52c7fcb4c785182ec710e410124933e1343aac8c69c1",
        "benchmark/workspaces/fixed/programs/wager_arena/src/events.rs": "5650a96acb6d708ab659d99528450d9cc42b96d3554ee80191a52159fc8939b8",
        "benchmark/workspaces/fixed/programs/wager_arena/src/instructions/deposit.rs": "7d02e63afc9812231ccaecb11e7823f5b737af6b970cb427d4e4f39162e039b8",
        "benchmark/workspaces/fixed/programs/wager_arena/src/instructions/enter.rs": "30fea19f6945f1e5141c5043cb89abd322fd16642721b4a789f03516b6f7e0c0",
        "benchmark/workspaces/fixed/programs/wager_arena/src/instructions/expire.rs": "41e8727955ec8f26e71580cf9855b945227c2aec308726d9169fc5199dfc9b7b",
        "benchmark/workspaces/fixed/programs/wager_arena/src/instructions/init.rs": "b3d3919d9b429216f299b0808581f79e38ade39a25627ba2d0dc2b0649c8e497",
        "benchmark/workspaces/fixed/programs/wager_arena/src/instructions/mod.rs": "2d4a319640f6baf9c17424eeb97ae78464896acb9192bedbb8b1f51624a63a7e",
        "benchmark/workspaces/fixed/programs/wager_arena/src/instructions/open.rs": "e1456e99a732695aefd803700fa002267cded9a98fef36d963c539c018ae24c9",
        "benchmark/workspaces/fixed/programs/wager_arena/src/instructions/settle.rs": "82697a2735ee2fadbe3275ebda1392b3ecc266f6c611875d175319f9403665fd",
        "benchmark/workspaces/fixed/programs/wager_arena/src/instructions/withdraw.rs": "7c5518715a1e8188f3cd183c6a5c4f1a9a439ad4baf124baf500d67dfba6bede",
        "benchmark/workspaces/fixed/programs/wager_arena/src/lib.rs": "13ef9119ab448cdde3f7a42f6d4f8af6dab6f562d444e3ef46545b393d2c0d58",
        "benchmark/workspaces/fixed/programs/wager_arena/src/state.rs": "e1657b2bb6260ddd8f9211143d72bb821057a3dcb0f61e8d0f93d1469ee9dd66",
        "benchmark/workspaces/fixed/programs/wager_arena/src/token_ops.rs": "2011f2d3dabda03dea739a5e7aa4dc8e2009e4c4e3254c955a8a5290ee0164ca",
        "benchmark/workspaces/vulnerable/programs/wager_arena/ARCHITECTURE.md": "a033ac7182baacdeab801f95f707469440027e05a106324e3a61f3486b9e044e",
        "benchmark/workspaces/vulnerable/programs/wager_arena/Cargo.toml": "9862154974303c529ab2cefcbe27b807ffef2976af58f3f0d283f95c8e5d19b5",
        "benchmark/workspaces/vulnerable/programs/wager_arena/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/workspaces/vulnerable/programs/wager_arena/src/errors.rs": "f1b9cc01d8f54dfe276a52c7fcb4c785182ec710e410124933e1343aac8c69c1",
        "benchmark/workspaces/vulnerable/programs/wager_arena/src/events.rs": "5650a96acb6d708ab659d99528450d9cc42b96d3554ee80191a52159fc8939b8",
        "benchmark/workspaces/vulnerable/programs/wager_arena/src/instructions/deposit.rs": "7d02e63afc9812231ccaecb11e7823f5b737af6b970cb427d4e4f39162e039b8",
        "benchmark/workspaces/vulnerable/programs/wager_arena/src/instructions/enter.rs": "05fdc6abc4a30158d8734bf6fc9a7ceaea8dabd1aacec42301ffed531cb1b1ea",
        "benchmark/workspaces/vulnerable/programs/wager_arena/src/instructions/expire.rs": "db233638fa8df34aa0e9521d564501524d94c1f0498ff02b3d485bc3702dd5a0",
        "benchmark/workspaces/vulnerable/programs/wager_arena/src/instructions/init.rs": "b3d3919d9b429216f299b0808581f79e38ade39a25627ba2d0dc2b0649c8e497",
        "benchmark/workspaces/vulnerable/programs/wager_arena/src/instructions/mod.rs": "2d4a319640f6baf9c17424eeb97ae78464896acb9192bedbb8b1f51624a63a7e",
        "benchmark/workspaces/vulnerable/programs/wager_arena/src/instructions/open.rs": "e1456e99a732695aefd803700fa002267cded9a98fef36d963c539c018ae24c9",
        "benchmark/workspaces/vulnerable/programs/wager_arena/src/instructions/settle.rs": "3dcb5138eff5c804de29bd9b5d9700d2a670a462205787f54b9a9d9d04e9fd58",
        "benchmark/workspaces/vulnerable/programs/wager_arena/src/instructions/withdraw.rs": "8a59197d521409f9a1303b9f3a9077b9d7b8fd5dc24c7b4c8206a3735215dd64",
        "benchmark/workspaces/vulnerable/programs/wager_arena/src/lib.rs": "13ef9119ab448cdde3f7a42f6d4f8af6dab6f562d444e3ef46545b393d2c0d58",
        "benchmark/workspaces/vulnerable/programs/wager_arena/src/state.rs": "e1657b2bb6260ddd8f9211143d72bb821057a3dcb0f61e8d0f93d1469ee9dd66",
        "benchmark/workspaces/vulnerable/programs/wager_arena/src/token_ops.rs": "2011f2d3dabda03dea739a5e7aa4dc8e2009e4c4e3254c955a8a5290ee0164ca"
      }
    },
    {
      "id": "c08",
      "history": [
        {
          "at": "2026-10-28T19:21:36.886Z",
          "event": "add",
          "by": "case-author"
        },
        {
          "at": "2026-10-28T19:36:07.531Z",
          "event": "review-fail",
          "by": "case-reviewer",
          "note": "review-batchD: see logs/review-batchD.md"
        },
        {
          "at": "2026-10-28T19:38:16.800Z",
          "event": "rehash",
          "by": "executor",
          "note": "author fixes after review-batchD"
        },
        {
          "at": "2026-10-28T19:40:04.020Z",
          "event": "review-pass",
          "by": "case-reviewer",
          "note": "re-review after author fixes (review-batchD)"
        }
      ],
      "role": "heldout",
      "author": "case-author",
      "review": "pass",
      "readBy": [],
      "files": {
        "benchmark/cases/c08/CASE.md": "20d1de1d5d0d01da01c31a647c2030dcd1300247725713b8eee8e6f6903c293f",
        "benchmark/cases/c08/rubric.md": "b3c852be21aac61a238b86e75a7e7acb88f1eb509294d47d476d0efd24d844d1",
        "benchmark/fixtures/c08a/program/Cargo.toml": "18a8cfc59ec436fc3d3d172ca2dfdf653ee746272c39b4f7f4ff5db4ca18a458",
        "benchmark/fixtures/c08a/program/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/fixtures/c08a/program/src/errors.rs": "db314a6e435c8a39baa343c061f01a284c09cb5be468be8be5a8dcd0e5a4f598",
        "benchmark/fixtures/c08a/program/src/events.rs": "f5f3bbc07bbd1a65f55352ee8809fe647ce6c3e074f6deeaddeeac33cbab2275",
        "benchmark/fixtures/c08a/program/src/instructions/approval.rs": "e1779392474cba0c144b620bacfbee9aad5b4cbdface262bf85ea2e5519f1b66",
        "benchmark/fixtures/c08a/program/src/instructions/council.rs": "972df5e75acd0fc7008edc740775f235bc1bc7a31bdbc98b2420ffd59ee128de",
        "benchmark/fixtures/c08a/program/src/instructions/execute.rs": "3f56d6784f0593654bbca1d378ae690b6dbfc24340b85b49aba0ff1f4b6d6602",
        "benchmark/fixtures/c08a/program/src/instructions/mod.rs": "f0b9662f6b35b10466a7c707028729ec96b7c4b019ed1c7991aa55e42c618d1a",
        "benchmark/fixtures/c08a/program/src/instructions/timelock.rs": "6d9a34ea7a93d79bd4501050e3d3396a3ae41c3adb576917dbe21625c5796381",
        "benchmark/fixtures/c08a/program/src/lib.rs": "e01a278617ab83899a75d0978ad8275a9c695778e02d256f09cab6e3d2ca840d",
        "benchmark/fixtures/c08a/program/src/state.rs": "8b640589802a7d352c6e367b669ae3d36c44a2fb98fa185123a32c78e6e0bec2",
        "benchmark/fixtures/c08b/program/Cargo.toml": "18a8cfc59ec436fc3d3d172ca2dfdf653ee746272c39b4f7f4ff5db4ca18a458",
        "benchmark/fixtures/c08b/program/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/fixtures/c08b/program/src/errors.rs": "db314a6e435c8a39baa343c061f01a284c09cb5be468be8be5a8dcd0e5a4f598",
        "benchmark/fixtures/c08b/program/src/events.rs": "f5f3bbc07bbd1a65f55352ee8809fe647ce6c3e074f6deeaddeeac33cbab2275",
        "benchmark/fixtures/c08b/program/src/instructions/approval.rs": "e1779392474cba0c144b620bacfbee9aad5b4cbdface262bf85ea2e5519f1b66",
        "benchmark/fixtures/c08b/program/src/instructions/council.rs": "972df5e75acd0fc7008edc740775f235bc1bc7a31bdbc98b2420ffd59ee128de",
        "benchmark/fixtures/c08b/program/src/instructions/execute.rs": "d8bae8a2050094e31f2ff2e23a54572e0a9625e7d34ff7b9ab37595c35e4fee1",
        "benchmark/fixtures/c08b/program/src/instructions/mod.rs": "f0b9662f6b35b10466a7c707028729ec96b7c4b019ed1c7991aa55e42c618d1a",
        "benchmark/fixtures/c08b/program/src/instructions/timelock.rs": "6d9a34ea7a93d79bd4501050e3d3396a3ae41c3adb576917dbe21625c5796381",
        "benchmark/fixtures/c08b/program/src/lib.rs": "e01a278617ab83899a75d0978ad8275a9c695778e02d256f09cab6e3d2ca840d",
        "benchmark/fixtures/c08b/program/src/state.rs": "8b640589802a7d352c6e367b669ae3d36c44a2fb98fa185123a32c78e6e0bec2",
        "benchmark/workspaces/fixed/programs/grants_council/Cargo.toml": "18a8cfc59ec436fc3d3d172ca2dfdf653ee746272c39b4f7f4ff5db4ca18a458",
        "benchmark/workspaces/fixed/programs/grants_council/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/workspaces/fixed/programs/grants_council/src/errors.rs": "db314a6e435c8a39baa343c061f01a284c09cb5be468be8be5a8dcd0e5a4f598",
        "benchmark/workspaces/fixed/programs/grants_council/src/events.rs": "f5f3bbc07bbd1a65f55352ee8809fe647ce6c3e074f6deeaddeeac33cbab2275",
        "benchmark/workspaces/fixed/programs/grants_council/src/instructions/approval.rs": "e1779392474cba0c144b620bacfbee9aad5b4cbdface262bf85ea2e5519f1b66",
        "benchmark/workspaces/fixed/programs/grants_council/src/instructions/council.rs": "972df5e75acd0fc7008edc740775f235bc1bc7a31bdbc98b2420ffd59ee128de",
        "benchmark/workspaces/fixed/programs/grants_council/src/instructions/execute.rs": "3f56d6784f0593654bbca1d378ae690b6dbfc24340b85b49aba0ff1f4b6d6602",
        "benchmark/workspaces/fixed/programs/grants_council/src/instructions/mod.rs": "f0b9662f6b35b10466a7c707028729ec96b7c4b019ed1c7991aa55e42c618d1a",
        "benchmark/workspaces/fixed/programs/grants_council/src/instructions/timelock.rs": "6d9a34ea7a93d79bd4501050e3d3396a3ae41c3adb576917dbe21625c5796381",
        "benchmark/workspaces/fixed/programs/grants_council/src/lib.rs": "e01a278617ab83899a75d0978ad8275a9c695778e02d256f09cab6e3d2ca840d",
        "benchmark/workspaces/fixed/programs/grants_council/src/state.rs": "8b640589802a7d352c6e367b669ae3d36c44a2fb98fa185123a32c78e6e0bec2",
        "benchmark/workspaces/vulnerable/programs/grants_council/Cargo.toml": "18a8cfc59ec436fc3d3d172ca2dfdf653ee746272c39b4f7f4ff5db4ca18a458",
        "benchmark/workspaces/vulnerable/programs/grants_council/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/workspaces/vulnerable/programs/grants_council/src/errors.rs": "db314a6e435c8a39baa343c061f01a284c09cb5be468be8be5a8dcd0e5a4f598",
        "benchmark/workspaces/vulnerable/programs/grants_council/src/events.rs": "f5f3bbc07bbd1a65f55352ee8809fe647ce6c3e074f6deeaddeeac33cbab2275",
        "benchmark/workspaces/vulnerable/programs/grants_council/src/instructions/approval.rs": "e1779392474cba0c144b620bacfbee9aad5b4cbdface262bf85ea2e5519f1b66",
        "benchmark/workspaces/vulnerable/programs/grants_council/src/instructions/council.rs": "972df5e75acd0fc7008edc740775f235bc1bc7a31bdbc98b2420ffd59ee128de",
        "benchmark/workspaces/vulnerable/programs/grants_council/src/instructions/execute.rs": "d8bae8a2050094e31f2ff2e23a54572e0a9625e7d34ff7b9ab37595c35e4fee1",
        "benchmark/workspaces/vulnerable/programs/grants_council/src/instructions/mod.rs": "f0b9662f6b35b10466a7c707028729ec96b7c4b019ed1c7991aa55e42c618d1a",
        "benchmark/workspaces/vulnerable/programs/grants_council/src/instructions/timelock.rs": "6d9a34ea7a93d79bd4501050e3d3396a3ae41c3adb576917dbe21625c5796381",
        "benchmark/workspaces/vulnerable/programs/grants_council/src/lib.rs": "e01a278617ab83899a75d0978ad8275a9c695778e02d256f09cab6e3d2ca840d",
        "benchmark/workspaces/vulnerable/programs/grants_council/src/state.rs": "8b640589802a7d352c6e367b669ae3d36c44a2fb98fa185123a32c78e6e0bec2"
      }
    },
    {
      "id": "c09",
      "history": [
        {
          "at": "2026-10-28T19:21:36.970Z",
          "event": "add",
          "by": "case-author"
        },
        {
          "at": "2026-10-28T19:36:07.606Z",
          "event": "review-fail",
          "by": "case-reviewer",
          "note": "review-batchD: see logs/review-batchD.md"
        },
        {
          "at": "2026-10-28T19:38:16.892Z",
          "event": "rehash",
          "by": "executor",
          "note": "author fixes after review-batchD"
        },
        {
          "at": "2026-10-28T19:40:04.111Z",
          "event": "review-pass",
          "by": "case-reviewer",
          "note": "re-review after author fixes (review-batchD)"
        }
      ],
      "role": "dev",
      "author": "case-author",
      "review": "pass",
      "readBy": [],
      "files": {
        "benchmark/cases/c09/CASE.md": "321111e2baad6c1f0f43139c2c789b7c99ab8ea5804407ddaa7ff6bfc975f93f",
        "benchmark/cases/c09/rubric.md": "8a0ee249552e338f8df3a2e5e76812c3f6102ab054263f694e012b6799d6e5aa",
        "benchmark/fixtures/c09a/program/Cargo.toml": "1c96cf46ab42f27dafe36d8441533394d3eaeffda344fd200e29d582271d977b",
        "benchmark/fixtures/c09a/program/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/fixtures/c09a/program/src/errors.rs": "ad269318fe093d25531f00e24cbf641cc082b471eb88db0d1539ff69314c6574",
        "benchmark/fixtures/c09a/program/src/events.rs": "ec840e1439416adc394f39212d100d518852956cc30b13442e51b8fa5f7af549",
        "benchmark/fixtures/c09a/program/src/instructions/exit.rs": "3a2ae757be2c1cfda8177a9b6f85755e3c133d05c2e183d5decb58a9ec8a6e86",
        "benchmark/fixtures/c09a/program/src/instructions/mod.rs": "6a30aac182fae1bd1ba29a1ba1ab2e8c250231c197f938d1f1694ad6e5220e54",
        "benchmark/fixtures/c09a/program/src/instructions/position.rs": "7ecf21758cc47aa398f90e56cad0da27481cf6237c1a194983efd1a8c0f1596a",
        "benchmark/fixtures/c09a/program/src/instructions/season.rs": "bd3feec64be5961781dc84d963d81cbfd53f8e76d9848ec8014c6cce7c2f7c66",
        "benchmark/fixtures/c09a/program/src/lib.rs": "8415a6a58d98a7df5f03e371762526b4362c0696caf010f7f65aa03186d268d4",
        "benchmark/fixtures/c09a/program/src/math.rs": "6a3a8d65e99932dda8c584c5033e0af470b68e59a8443f300affd77a18cd0203",
        "benchmark/fixtures/c09a/program/src/state.rs": "f63fabcee1aa4b4e2114bab6c6fdfcef5bb3da658e244e9c7759fb30359d002b",
        "benchmark/fixtures/c09b/program/Cargo.toml": "1c96cf46ab42f27dafe36d8441533394d3eaeffda344fd200e29d582271d977b",
        "benchmark/fixtures/c09b/program/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/fixtures/c09b/program/src/errors.rs": "ad269318fe093d25531f00e24cbf641cc082b471eb88db0d1539ff69314c6574",
        "benchmark/fixtures/c09b/program/src/events.rs": "ec840e1439416adc394f39212d100d518852956cc30b13442e51b8fa5f7af549",
        "benchmark/fixtures/c09b/program/src/instructions/exit.rs": "3a2ae757be2c1cfda8177a9b6f85755e3c133d05c2e183d5decb58a9ec8a6e86",
        "benchmark/fixtures/c09b/program/src/instructions/mod.rs": "6a30aac182fae1bd1ba29a1ba1ab2e8c250231c197f938d1f1694ad6e5220e54",
        "benchmark/fixtures/c09b/program/src/instructions/position.rs": "c113507befdc71a0ce61858e073c83266d0b5084b09ed86130a66005e8dab56b",
        "benchmark/fixtures/c09b/program/src/instructions/season.rs": "bd3feec64be5961781dc84d963d81cbfd53f8e76d9848ec8014c6cce7c2f7c66",
        "benchmark/fixtures/c09b/program/src/lib.rs": "8415a6a58d98a7df5f03e371762526b4362c0696caf010f7f65aa03186d268d4",
        "benchmark/fixtures/c09b/program/src/math.rs": "6a3a8d65e99932dda8c584c5033e0af470b68e59a8443f300affd77a18cd0203",
        "benchmark/fixtures/c09b/program/src/state.rs": "f63fabcee1aa4b4e2114bab6c6fdfcef5bb3da658e244e9c7759fb30359d002b",
        "benchmark/workspaces/fixed/programs/season_locker/Cargo.toml": "1c96cf46ab42f27dafe36d8441533394d3eaeffda344fd200e29d582271d977b",
        "benchmark/workspaces/fixed/programs/season_locker/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/workspaces/fixed/programs/season_locker/src/errors.rs": "ad269318fe093d25531f00e24cbf641cc082b471eb88db0d1539ff69314c6574",
        "benchmark/workspaces/fixed/programs/season_locker/src/events.rs": "ec840e1439416adc394f39212d100d518852956cc30b13442e51b8fa5f7af549",
        "benchmark/workspaces/fixed/programs/season_locker/src/instructions/exit.rs": "3a2ae757be2c1cfda8177a9b6f85755e3c133d05c2e183d5decb58a9ec8a6e86",
        "benchmark/workspaces/fixed/programs/season_locker/src/instructions/mod.rs": "6a30aac182fae1bd1ba29a1ba1ab2e8c250231c197f938d1f1694ad6e5220e54",
        "benchmark/workspaces/fixed/programs/season_locker/src/instructions/position.rs": "7ecf21758cc47aa398f90e56cad0da27481cf6237c1a194983efd1a8c0f1596a",
        "benchmark/workspaces/fixed/programs/season_locker/src/instructions/season.rs": "bd3feec64be5961781dc84d963d81cbfd53f8e76d9848ec8014c6cce7c2f7c66",
        "benchmark/workspaces/fixed/programs/season_locker/src/lib.rs": "8415a6a58d98a7df5f03e371762526b4362c0696caf010f7f65aa03186d268d4",
        "benchmark/workspaces/fixed/programs/season_locker/src/math.rs": "6a3a8d65e99932dda8c584c5033e0af470b68e59a8443f300affd77a18cd0203",
        "benchmark/workspaces/fixed/programs/season_locker/src/state.rs": "f63fabcee1aa4b4e2114bab6c6fdfcef5bb3da658e244e9c7759fb30359d002b",
        "benchmark/workspaces/vulnerable/programs/season_locker/Cargo.toml": "1c96cf46ab42f27dafe36d8441533394d3eaeffda344fd200e29d582271d977b",
        "benchmark/workspaces/vulnerable/programs/season_locker/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/workspaces/vulnerable/programs/season_locker/src/errors.rs": "ad269318fe093d25531f00e24cbf641cc082b471eb88db0d1539ff69314c6574",
        "benchmark/workspaces/vulnerable/programs/season_locker/src/events.rs": "ec840e1439416adc394f39212d100d518852956cc30b13442e51b8fa5f7af549",
        "benchmark/workspaces/vulnerable/programs/season_locker/src/instructions/exit.rs": "3a2ae757be2c1cfda8177a9b6f85755e3c133d05c2e183d5decb58a9ec8a6e86",
        "benchmark/workspaces/vulnerable/programs/season_locker/src/instructions/mod.rs": "6a30aac182fae1bd1ba29a1ba1ab2e8c250231c197f938d1f1694ad6e5220e54",
        "benchmark/workspaces/vulnerable/programs/season_locker/src/instructions/position.rs": "c113507befdc71a0ce61858e073c83266d0b5084b09ed86130a66005e8dab56b",
        "benchmark/workspaces/vulnerable/programs/season_locker/src/instructions/season.rs": "bd3feec64be5961781dc84d963d81cbfd53f8e76d9848ec8014c6cce7c2f7c66",
        "benchmark/workspaces/vulnerable/programs/season_locker/src/lib.rs": "8415a6a58d98a7df5f03e371762526b4362c0696caf010f7f65aa03186d268d4",
        "benchmark/workspaces/vulnerable/programs/season_locker/src/math.rs": "6a3a8d65e99932dda8c584c5033e0af470b68e59a8443f300affd77a18cd0203",
        "benchmark/workspaces/vulnerable/programs/season_locker/src/state.rs": "f63fabcee1aa4b4e2114bab6c6fdfcef5bb3da658e244e9c7759fb30359d002b"
      }
    },
    {
      "id": "c10",
      "history": [
        {
          "at": "2026-10-28T19:21:37.061Z",
          "event": "add",
          "by": "case-author"
        },
        {
          "at": "2026-10-28T19:36:07.692Z",
          "event": "review-fail",
          "by": "case-reviewer",
          "note": "review-batchD: see logs/review-batchD.md"
        },
        {
          "at": "2026-10-28T19:38:16.978Z",
          "event": "rehash",
          "by": "executor",
          "note": "author fixes after review-batchD"
        },
        {
          "at": "2026-10-28T19:40:04.184Z",
          "event": "review-pass",
          "by": "case-reviewer",
          "note": "re-review after author fixes (review-batchD)"
        }
      ],
      "role": "heldout",
      "author": "case-author",
      "review": "pass",
      "readBy": [],
      "files": {
        "benchmark/cases/c10/CASE.md": "bd85343a3128ed46469867834caa655076830ce9bbed896531f552f3fb421b81",
        "benchmark/cases/c10/rubric.md": "9f4522b371a2729781fb0dd31722b08763dc4053b1dff28ca7c5bf276eaa975f",
        "benchmark/fixtures/c10a/program/ARCHITECTURE.md": "b6e5cd583c7fd78e0d0ea37b5f53b7fa49774c6011cc7f64b5a93b697559a7cc",
        "benchmark/fixtures/c10a/program/Cargo.toml": "cc0eb3a1dd3b5121622553c6f35b8ed156acff781fcde41d1192eb9f9d46d835",
        "benchmark/fixtures/c10a/program/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/fixtures/c10a/program/src/errors.rs": "fcd92c50b8cd851b8ad6134b0d701edb54cd552e8fb6a79f9b1053bfb0fc1931",
        "benchmark/fixtures/c10a/program/src/events.rs": "d7216983b6b8e4746a363f55fbdba6f1c81dae495569a9d00bc019e496fdcb15",
        "benchmark/fixtures/c10a/program/src/instructions/admin.rs": "78076117341f700fc6cbf2141bef2c89a5ae312149fa2df8a8b2ca1dcce55d8a",
        "benchmark/fixtures/c10a/program/src/instructions/convert.rs": "c4c8b4274f7086db3e650dbce0652ae0272c4190e09cd0f11c7e82538ae042dd",
        "benchmark/fixtures/c10a/program/src/instructions/liquidity.rs": "8c1709de6e3a49b21a27a05d2ef3964a3a2d3bd9de5bfd31a8a09dffa5cde464",
        "benchmark/fixtures/c10a/program/src/instructions/mod.rs": "f5d321f1e3f06ee1817a6fc62e0248314d3881e61aa7fabfc441995da5436072",
        "benchmark/fixtures/c10a/program/src/instructions/rate.rs": "f00834863022f21716f85ab547f8474fcfac658c19a820a667303254937242f9",
        "benchmark/fixtures/c10a/program/src/lib.rs": "1616193d0d800c17e79a71fb166d628fce58d7e7af445c7861b9b9f007f82855",
        "benchmark/fixtures/c10a/program/src/quote.rs": "30d4f158d972b20953a2ce03451a27e4ee6476d23b116aabaf2c4ac60a5457b0",
        "benchmark/fixtures/c10a/program/src/state.rs": "a75f4bd5005a8a2631b8101ab1b50013639f5ff885613ede40d43c03515f2ce6",
        "benchmark/fixtures/c10b/program/ARCHITECTURE.md": "03d2f553933c820cca5a4a00a980d44de1341a6a0ce5e41e5121d8431999e9fc",
        "benchmark/fixtures/c10b/program/Cargo.toml": "cc0eb3a1dd3b5121622553c6f35b8ed156acff781fcde41d1192eb9f9d46d835",
        "benchmark/fixtures/c10b/program/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/fixtures/c10b/program/src/errors.rs": "6726a0a82acb5f95fe0e9b034be1a0236e15d3225b0526191596c3b9c0d488c8",
        "benchmark/fixtures/c10b/program/src/events.rs": "d7216983b6b8e4746a363f55fbdba6f1c81dae495569a9d00bc019e496fdcb15",
        "benchmark/fixtures/c10b/program/src/instructions/admin.rs": "78076117341f700fc6cbf2141bef2c89a5ae312149fa2df8a8b2ca1dcce55d8a",
        "benchmark/fixtures/c10b/program/src/instructions/convert.rs": "c4c8b4274f7086db3e650dbce0652ae0272c4190e09cd0f11c7e82538ae042dd",
        "benchmark/fixtures/c10b/program/src/instructions/liquidity.rs": "8c1709de6e3a49b21a27a05d2ef3964a3a2d3bd9de5bfd31a8a09dffa5cde464",
        "benchmark/fixtures/c10b/program/src/instructions/mod.rs": "f5d321f1e3f06ee1817a6fc62e0248314d3881e61aa7fabfc441995da5436072",
        "benchmark/fixtures/c10b/program/src/instructions/rate.rs": "c0e98ab70056dbbef980b9c6dc0a84549afb2ab33db4051bf3c5047b56dda7ba",
        "benchmark/fixtures/c10b/program/src/lib.rs": "1616193d0d800c17e79a71fb166d628fce58d7e7af445c7861b9b9f007f82855",
        "benchmark/fixtures/c10b/program/src/quote.rs": "30d4f158d972b20953a2ce03451a27e4ee6476d23b116aabaf2c4ac60a5457b0",
        "benchmark/fixtures/c10b/program/src/state.rs": "a75f4bd5005a8a2631b8101ab1b50013639f5ff885613ede40d43c03515f2ce6",
        "benchmark/workspaces/fixed/programs/fx_corridor/ARCHITECTURE.md": "03d2f553933c820cca5a4a00a980d44de1341a6a0ce5e41e5121d8431999e9fc",
        "benchmark/workspaces/fixed/programs/fx_corridor/Cargo.toml": "cc0eb3a1dd3b5121622553c6f35b8ed156acff781fcde41d1192eb9f9d46d835",
        "benchmark/workspaces/fixed/programs/fx_corridor/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/workspaces/fixed/programs/fx_corridor/src/errors.rs": "6726a0a82acb5f95fe0e9b034be1a0236e15d3225b0526191596c3b9c0d488c8",
        "benchmark/workspaces/fixed/programs/fx_corridor/src/events.rs": "d7216983b6b8e4746a363f55fbdba6f1c81dae495569a9d00bc019e496fdcb15",
        "benchmark/workspaces/fixed/programs/fx_corridor/src/instructions/admin.rs": "78076117341f700fc6cbf2141bef2c89a5ae312149fa2df8a8b2ca1dcce55d8a",
        "benchmark/workspaces/fixed/programs/fx_corridor/src/instructions/convert.rs": "c4c8b4274f7086db3e650dbce0652ae0272c4190e09cd0f11c7e82538ae042dd",
        "benchmark/workspaces/fixed/programs/fx_corridor/src/instructions/liquidity.rs": "8c1709de6e3a49b21a27a05d2ef3964a3a2d3bd9de5bfd31a8a09dffa5cde464",
        "benchmark/workspaces/fixed/programs/fx_corridor/src/instructions/mod.rs": "f5d321f1e3f06ee1817a6fc62e0248314d3881e61aa7fabfc441995da5436072",
        "benchmark/workspaces/fixed/programs/fx_corridor/src/instructions/rate.rs": "c0e98ab70056dbbef980b9c6dc0a84549afb2ab33db4051bf3c5047b56dda7ba",
        "benchmark/workspaces/fixed/programs/fx_corridor/src/lib.rs": "1616193d0d800c17e79a71fb166d628fce58d7e7af445c7861b9b9f007f82855",
        "benchmark/workspaces/fixed/programs/fx_corridor/src/quote.rs": "30d4f158d972b20953a2ce03451a27e4ee6476d23b116aabaf2c4ac60a5457b0",
        "benchmark/workspaces/fixed/programs/fx_corridor/src/state.rs": "a75f4bd5005a8a2631b8101ab1b50013639f5ff885613ede40d43c03515f2ce6",
        "benchmark/workspaces/vulnerable/programs/fx_corridor/ARCHITECTURE.md": "b6e5cd583c7fd78e0d0ea37b5f53b7fa49774c6011cc7f64b5a93b697559a7cc",
        "benchmark/workspaces/vulnerable/programs/fx_corridor/Cargo.toml": "cc0eb3a1dd3b5121622553c6f35b8ed156acff781fcde41d1192eb9f9d46d835",
        "benchmark/workspaces/vulnerable/programs/fx_corridor/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/workspaces/vulnerable/programs/fx_corridor/src/errors.rs": "fcd92c50b8cd851b8ad6134b0d701edb54cd552e8fb6a79f9b1053bfb0fc1931",
        "benchmark/workspaces/vulnerable/programs/fx_corridor/src/events.rs": "d7216983b6b8e4746a363f55fbdba6f1c81dae495569a9d00bc019e496fdcb15",
        "benchmark/workspaces/vulnerable/programs/fx_corridor/src/instructions/admin.rs": "78076117341f700fc6cbf2141bef2c89a5ae312149fa2df8a8b2ca1dcce55d8a",
        "benchmark/workspaces/vulnerable/programs/fx_corridor/src/instructions/convert.rs": "c4c8b4274f7086db3e650dbce0652ae0272c4190e09cd0f11c7e82538ae042dd",
        "benchmark/workspaces/vulnerable/programs/fx_corridor/src/instructions/liquidity.rs": "8c1709de6e3a49b21a27a05d2ef3964a3a2d3bd9de5bfd31a8a09dffa5cde464",
        "benchmark/workspaces/vulnerable/programs/fx_corridor/src/instructions/mod.rs": "f5d321f1e3f06ee1817a6fc62e0248314d3881e61aa7fabfc441995da5436072",
        "benchmark/workspaces/vulnerable/programs/fx_corridor/src/instructions/rate.rs": "f00834863022f21716f85ab547f8474fcfac658c19a820a667303254937242f9",
        "benchmark/workspaces/vulnerable/programs/fx_corridor/src/lib.rs": "1616193d0d800c17e79a71fb166d628fce58d7e7af445c7861b9b9f007f82855",
        "benchmark/workspaces/vulnerable/programs/fx_corridor/src/quote.rs": "30d4f158d972b20953a2ce03451a27e4ee6476d23b116aabaf2c4ac60a5457b0",
        "benchmark/workspaces/vulnerable/programs/fx_corridor/src/state.rs": "a75f4bd5005a8a2631b8101ab1b50013639f5ff885613ede40d43c03515f2ce6"
      }
    },
    {
      "id": "c11",
      "history": [
        {
          "at": "2026-10-28T19:21:37.146Z",
          "event": "add",
          "by": "case-author"
        },
        {
          "at": "2026-10-28T19:36:07.379Z",
          "event": "review-pass",
          "by": "case-reviewer",
          "note": "review-batchD"
        }
      ],
      "role": "heldout",
      "author": "case-author",
      "review": "pass",
      "readBy": [],
      "files": {
        "benchmark/cases/c11/CASE.md": "f353954cad9e6b6b8ff298a9f0dd995dea545043a06e27c82ca94ad9ef5e2c9f",
        "benchmark/cases/c11/rubric.md": "ca4a4ebad438ec4334c617f6fa7756af072a318a3e0bd334988e17fe94e4772b",
        "benchmark/fixtures/c11a/program/ARCHITECTURE.md": "e135b13cbf4082463d2b735090b79726f7d2627156163879a7e9ba5c8e39320e",
        "benchmark/fixtures/c11a/program/Cargo.toml": "04c15c24acee1975cf585788514e24a04b99766049dbee56fa5af79d2e4bad9e",
        "benchmark/fixtures/c11a/program/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/fixtures/c11a/program/src/errors.rs": "840a035114984c38e09351a8b15bf711ea746bb94a931bf313c997231863697b",
        "benchmark/fixtures/c11a/program/src/events.rs": "72a66b2ec9de15999aca1ccdf1a50c58a5b606fe973d0a07c1beef57a7c577d6",
        "benchmark/fixtures/c11a/program/src/instructions/admin.rs": "8f4865e77a679b0a6db3dd192edf17924853c23d1cba6e2786098930ca85c077",
        "benchmark/fixtures/c11a/program/src/instructions/card.rs": "412ab2440fc92d2058947dd5993b47a2bb041440429a83d9963acd6e58b696bf",
        "benchmark/fixtures/c11a/program/src/instructions/mod.rs": "12097f7beec9e3c77c13dd66ddea5ccd6811b9fda0b8c67342f724082d217fdb",
        "benchmark/fixtures/c11a/program/src/instructions/processor.rs": "3089ebaf8636489e47882b456d60db33770eed6e3f05e1f60ce9c0db600dcd72",
        "benchmark/fixtures/c11a/program/src/lib.rs": "e8cf56b90c677ae984f129fc09b54be15c6b47b573323a3dcc64839fca534b2e",
        "benchmark/fixtures/c11a/program/src/state.rs": "9cb8c508aafa1a45722f62f7a955c665e74cfce0ca915bd2c5da1813eb620b8f",
        "benchmark/fixtures/c11b/program/ARCHITECTURE.md": "c3db3be023c741608abe1a8059926edd8f6153d028dff819b6e5d11495afb3ba",
        "benchmark/fixtures/c11b/program/Cargo.toml": "04c15c24acee1975cf585788514e24a04b99766049dbee56fa5af79d2e4bad9e",
        "benchmark/fixtures/c11b/program/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/fixtures/c11b/program/src/errors.rs": "520711d07d536311ba585264f483a63b1304725324bdeb7838292a71d12f5604",
        "benchmark/fixtures/c11b/program/src/events.rs": "72a66b2ec9de15999aca1ccdf1a50c58a5b606fe973d0a07c1beef57a7c577d6",
        "benchmark/fixtures/c11b/program/src/instructions/admin.rs": "87cf91ed6471b338700197e9226afafefbc1326bfb7730f35c50a2253d99a0ca",
        "benchmark/fixtures/c11b/program/src/instructions/card.rs": "203f706bbae396cf8769405d6bc0293b9b207e6846c7d7d5f2a016fe151a80d6",
        "benchmark/fixtures/c11b/program/src/instructions/mod.rs": "12097f7beec9e3c77c13dd66ddea5ccd6811b9fda0b8c67342f724082d217fdb",
        "benchmark/fixtures/c11b/program/src/instructions/processor.rs": "f5d81c06b991abf34c03721ef44462db6a58205390f55b3cd5760d2c9109fe06",
        "benchmark/fixtures/c11b/program/src/lib.rs": "2ca0b12039b242871aab48c3c08df8d76c88e41808436474f52e5b621595f3df",
        "benchmark/fixtures/c11b/program/src/state.rs": "a49082cece939f3d9f7dd4c61d420417ebc2548481e1d765269620ab3abfb343",
        "benchmark/workspaces/fixed/programs/spend_card/ARCHITECTURE.md": "c3db3be023c741608abe1a8059926edd8f6153d028dff819b6e5d11495afb3ba",
        "benchmark/workspaces/fixed/programs/spend_card/Cargo.toml": "04c15c24acee1975cf585788514e24a04b99766049dbee56fa5af79d2e4bad9e",
        "benchmark/workspaces/fixed/programs/spend_card/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/workspaces/fixed/programs/spend_card/src/errors.rs": "520711d07d536311ba585264f483a63b1304725324bdeb7838292a71d12f5604",
        "benchmark/workspaces/fixed/programs/spend_card/src/events.rs": "72a66b2ec9de15999aca1ccdf1a50c58a5b606fe973d0a07c1beef57a7c577d6",
        "benchmark/workspaces/fixed/programs/spend_card/src/instructions/admin.rs": "87cf91ed6471b338700197e9226afafefbc1326bfb7730f35c50a2253d99a0ca",
        "benchmark/workspaces/fixed/programs/spend_card/src/instructions/card.rs": "203f706bbae396cf8769405d6bc0293b9b207e6846c7d7d5f2a016fe151a80d6",
        "benchmark/workspaces/fixed/programs/spend_card/src/instructions/mod.rs": "12097f7beec9e3c77c13dd66ddea5ccd6811b9fda0b8c67342f724082d217fdb",
        "benchmark/workspaces/fixed/programs/spend_card/src/instructions/processor.rs": "f5d81c06b991abf34c03721ef44462db6a58205390f55b3cd5760d2c9109fe06",
        "benchmark/workspaces/fixed/programs/spend_card/src/lib.rs": "2ca0b12039b242871aab48c3c08df8d76c88e41808436474f52e5b621595f3df",
        "benchmark/workspaces/fixed/programs/spend_card/src/state.rs": "a49082cece939f3d9f7dd4c61d420417ebc2548481e1d765269620ab3abfb343",
        "benchmark/workspaces/vulnerable/programs/spend_card/ARCHITECTURE.md": "e135b13cbf4082463d2b735090b79726f7d2627156163879a7e9ba5c8e39320e",
        "benchmark/workspaces/vulnerable/programs/spend_card/Cargo.toml": "04c15c24acee1975cf585788514e24a04b99766049dbee56fa5af79d2e4bad9e",
        "benchmark/workspaces/vulnerable/programs/spend_card/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/workspaces/vulnerable/programs/spend_card/src/errors.rs": "840a035114984c38e09351a8b15bf711ea746bb94a931bf313c997231863697b",
        "benchmark/workspaces/vulnerable/programs/spend_card/src/events.rs": "72a66b2ec9de15999aca1ccdf1a50c58a5b606fe973d0a07c1beef57a7c577d6",
        "benchmark/workspaces/vulnerable/programs/spend_card/src/instructions/admin.rs": "8f4865e77a679b0a6db3dd192edf17924853c23d1cba6e2786098930ca85c077",
        "benchmark/workspaces/vulnerable/programs/spend_card/src/instructions/card.rs": "412ab2440fc92d2058947dd5993b47a2bb041440429a83d9963acd6e58b696bf",
        "benchmark/workspaces/vulnerable/programs/spend_card/src/instructions/mod.rs": "12097f7beec9e3c77c13dd66ddea5ccd6811b9fda0b8c67342f724082d217fdb",
        "benchmark/workspaces/vulnerable/programs/spend_card/src/instructions/processor.rs": "3089ebaf8636489e47882b456d60db33770eed6e3f05e1f60ce9c0db600dcd72",
        "benchmark/workspaces/vulnerable/programs/spend_card/src/lib.rs": "e8cf56b90c677ae984f129fc09b54be15c6b47b573323a3dcc64839fca534b2e",
        "benchmark/workspaces/vulnerable/programs/spend_card/src/state.rs": "9cb8c508aafa1a45722f62f7a955c665e74cfce0ca915bd2c5da1813eb620b8f"
      }
    },
    {
      "id": "c12",
      "history": [
        {
          "at": "2026-10-28T19:21:37.232Z",
          "event": "add",
          "by": "case-author"
        },
        {
          "at": "2026-10-28T19:36:07.766Z",
          "event": "review-fail",
          "by": "case-reviewer",
          "note": "review-batchD: see logs/review-batchD.md"
        },
        {
          "at": "2026-10-28T19:37:59.176Z",
          "event": "rehash",
          "by": "executor",
          "note": "author fixes after review-batchD (errors.rs hint removed)"
        },
        {
          "at": "2026-10-28T19:40:04.256Z",
          "event": "review-pass",
          "by": "case-reviewer",
          "note": "re-review after author fixes (review-batchD)"
        }
      ],
      "role": "dev",
      "author": "case-author",
      "review": "pass",
      "readBy": [],
      "files": {
        "benchmark/cases/c12/CASE.md": "7e1ee148abd78a67ba2b728edd07245a81c12f0bedff16f29e2fed97950e200a",
        "benchmark/cases/c12/rubric.md": "fffd358e21b533a6985d5cb4a112548d21edd9219cb63c06e8f86c59832e49ef",
        "benchmark/fixtures/c12a/program/ARCHITECTURE.md": "4c16a03f626c9105faed80e5bff9e49c092f32cac1fe8478967f0d040c8fdc12",
        "benchmark/fixtures/c12a/program/Cargo.toml": "de5ea3768b5342fde27441c921dfaab6b89e89ccb7e3ad6d91ee95d19a85aa73",
        "benchmark/fixtures/c12a/program/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/fixtures/c12a/program/src/errors.rs": "78445b824485489c13daa67482b5bb5e31d05c69569d75acd148ef86d56929b9",
        "benchmark/fixtures/c12a/program/src/events.rs": "afaa064389305a85012a8c567404643de9cb44e1cf1b13d5ec31911de67dc73d",
        "benchmark/fixtures/c12a/program/src/instructions/buyer.rs": "93b66423bc52ddb4f0c77e4635df78fc8db6703b25d803466763077e58a4153a",
        "benchmark/fixtures/c12a/program/src/instructions/merchant.rs": "4e484ba84ad1615fb4d7476e346dd55a4f9e2565450ed53bd1acc6d4de0e2829",
        "benchmark/fixtures/c12a/program/src/instructions/mod.rs": "80e1bb35c0c0feb8a3f55b5f18f208bfeedcd8b9ca1bfcc268cf918e0aa69bf2",
        "benchmark/fixtures/c12a/program/src/lib.rs": "3fdea9c36c255eecc2025e686048c6693223312e576cf903308a7b2a55288d38",
        "benchmark/fixtures/c12a/program/src/state.rs": "3510361a5638bdebb10ec4617415691e9871108da5cdf4ff6988c8cca242643e",
        "benchmark/fixtures/c12b/program/ARCHITECTURE.md": "14d009f04a3b62c367bda0cbe219983aee57af9168f5908aefdb1602b91b475a",
        "benchmark/fixtures/c12b/program/Cargo.toml": "de5ea3768b5342fde27441c921dfaab6b89e89ccb7e3ad6d91ee95d19a85aa73",
        "benchmark/fixtures/c12b/program/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/fixtures/c12b/program/src/errors.rs": "78445b824485489c13daa67482b5bb5e31d05c69569d75acd148ef86d56929b9",
        "benchmark/fixtures/c12b/program/src/events.rs": "afaa064389305a85012a8c567404643de9cb44e1cf1b13d5ec31911de67dc73d",
        "benchmark/fixtures/c12b/program/src/instructions/buyer.rs": "63696e1bc0d5bf9942a00eb620370344ef8f7d8941d28e35e5b692d24d3751f7",
        "benchmark/fixtures/c12b/program/src/instructions/merchant.rs": "4e484ba84ad1615fb4d7476e346dd55a4f9e2565450ed53bd1acc6d4de0e2829",
        "benchmark/fixtures/c12b/program/src/instructions/mod.rs": "80e1bb35c0c0feb8a3f55b5f18f208bfeedcd8b9ca1bfcc268cf918e0aa69bf2",
        "benchmark/fixtures/c12b/program/src/lib.rs": "3fdea9c36c255eecc2025e686048c6693223312e576cf903308a7b2a55288d38",
        "benchmark/fixtures/c12b/program/src/state.rs": "2a3cc3d54e9c1068786c290c184a460e38f4d6839140dd12940f5976a8aea54a",
        "benchmark/workspaces/fixed/programs/group_buy/ARCHITECTURE.md": "14d009f04a3b62c367bda0cbe219983aee57af9168f5908aefdb1602b91b475a",
        "benchmark/workspaces/fixed/programs/group_buy/Cargo.toml": "de5ea3768b5342fde27441c921dfaab6b89e89ccb7e3ad6d91ee95d19a85aa73",
        "benchmark/workspaces/fixed/programs/group_buy/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/workspaces/fixed/programs/group_buy/src/errors.rs": "78445b824485489c13daa67482b5bb5e31d05c69569d75acd148ef86d56929b9",
        "benchmark/workspaces/fixed/programs/group_buy/src/events.rs": "afaa064389305a85012a8c567404643de9cb44e1cf1b13d5ec31911de67dc73d",
        "benchmark/workspaces/fixed/programs/group_buy/src/instructions/buyer.rs": "63696e1bc0d5bf9942a00eb620370344ef8f7d8941d28e35e5b692d24d3751f7",
        "benchmark/workspaces/fixed/programs/group_buy/src/instructions/merchant.rs": "4e484ba84ad1615fb4d7476e346dd55a4f9e2565450ed53bd1acc6d4de0e2829",
        "benchmark/workspaces/fixed/programs/group_buy/src/instructions/mod.rs": "80e1bb35c0c0feb8a3f55b5f18f208bfeedcd8b9ca1bfcc268cf918e0aa69bf2",
        "benchmark/workspaces/fixed/programs/group_buy/src/lib.rs": "3fdea9c36c255eecc2025e686048c6693223312e576cf903308a7b2a55288d38",
        "benchmark/workspaces/fixed/programs/group_buy/src/state.rs": "2a3cc3d54e9c1068786c290c184a460e38f4d6839140dd12940f5976a8aea54a",
        "benchmark/workspaces/vulnerable/programs/group_buy/ARCHITECTURE.md": "4c16a03f626c9105faed80e5bff9e49c092f32cac1fe8478967f0d040c8fdc12",
        "benchmark/workspaces/vulnerable/programs/group_buy/Cargo.toml": "de5ea3768b5342fde27441c921dfaab6b89e89ccb7e3ad6d91ee95d19a85aa73",
        "benchmark/workspaces/vulnerable/programs/group_buy/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/workspaces/vulnerable/programs/group_buy/src/errors.rs": "78445b824485489c13daa67482b5bb5e31d05c69569d75acd148ef86d56929b9",
        "benchmark/workspaces/vulnerable/programs/group_buy/src/events.rs": "afaa064389305a85012a8c567404643de9cb44e1cf1b13d5ec31911de67dc73d",
        "benchmark/workspaces/vulnerable/programs/group_buy/src/instructions/buyer.rs": "93b66423bc52ddb4f0c77e4635df78fc8db6703b25d803466763077e58a4153a",
        "benchmark/workspaces/vulnerable/programs/group_buy/src/instructions/merchant.rs": "4e484ba84ad1615fb4d7476e346dd55a4f9e2565450ed53bd1acc6d4de0e2829",
        "benchmark/workspaces/vulnerable/programs/group_buy/src/instructions/mod.rs": "80e1bb35c0c0feb8a3f55b5f18f208bfeedcd8b9ca1bfcc268cf918e0aa69bf2",
        "benchmark/workspaces/vulnerable/programs/group_buy/src/lib.rs": "3fdea9c36c255eecc2025e686048c6693223312e576cf903308a7b2a55288d38",
        "benchmark/workspaces/vulnerable/programs/group_buy/src/state.rs": "3510361a5638bdebb10ec4617415691e9871108da5cdf4ff6988c8cca242643e"
      }
    },
    {
      "id": "c13",
      "history": [
        {
          "at": "2026-10-28T19:21:37.315Z",
          "event": "add",
          "by": "case-author"
        },
        {
          "at": "2026-10-28T19:36:07.455Z",
          "event": "review-pass",
          "by": "case-reviewer",
          "note": "review-batchD"
        }
      ],
      "role": "heldout",
      "author": "case-author",
      "review": "pass",
      "readBy": [],
      "files": {
        "benchmark/cases/c13/CASE.md": "a1e8749b0cb82bfabbfccdff9f85888d5d74ae4182bc3e5568252913912931bb",
        "benchmark/cases/c13/rubric.md": "7ba9ded62ad899cc8478aafeccaa6272b5a0822618e1ae9f4451a07a7761f415",
        "benchmark/fixtures/c13a/program/Cargo.toml": "4b3da192363cc26069fac1152ba3e614b2e278fcb5bd31c8ba130f9ffce7fafe",
        "benchmark/fixtures/c13a/program/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/fixtures/c13a/program/src/errors.rs": "58c3a22b72fc585e1d3c406dc054c2e007427b0bdb41af1db197cea8634b8fe6",
        "benchmark/fixtures/c13a/program/src/events.rs": "2118952a9e5c7a4d699914192c5a303ad24e27db6a3ef7e27a731bb4859ff8e7",
        "benchmark/fixtures/c13a/program/src/instructions/borrower.rs": "1cdbe910ed15a8fcc8fa0bfe3ff10956f6610da951d1566e0a0b2b697b52c877",
        "benchmark/fixtures/c13a/program/src/instructions/lender.rs": "6da6aebf7de1a371f89d59d60b1518817fba654ddb0498bcdae9a41bc3d28ba0",
        "benchmark/fixtures/c13a/program/src/instructions/liquidate.rs": "357d50b567f0791e6c53221308da3d751b729a4cff7004c206719ac337938e75",
        "benchmark/fixtures/c13a/program/src/instructions/market.rs": "f6d1fa9e521dec2fb243d80e7091ea7f097da9dc80a95bdfb2bdc7377443805c",
        "benchmark/fixtures/c13a/program/src/instructions/mod.rs": "7d3d47204374412f72fb97f0e4788a628a2e3ff4feeff842ab0d24372143de5e",
        "benchmark/fixtures/c13a/program/src/lib.rs": "41371517088ae543d5c5ba5269f30f09a569401a7a209cdcd7dbcab08b65d8e5",
        "benchmark/fixtures/c13a/program/src/pricing.rs": "18ae94db37e716a08e4bf50557efe5601fa93d4fbdf4ac5fd69293ea64eee30e",
        "benchmark/fixtures/c13a/program/src/state.rs": "2a24c6739420ff01c2842f4ced8268a6008b0066c6a27d49569fb77525ae0239",
        "benchmark/fixtures/c13b/program/Cargo.toml": "4b3da192363cc26069fac1152ba3e614b2e278fcb5bd31c8ba130f9ffce7fafe",
        "benchmark/fixtures/c13b/program/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/fixtures/c13b/program/src/errors.rs": "58c3a22b72fc585e1d3c406dc054c2e007427b0bdb41af1db197cea8634b8fe6",
        "benchmark/fixtures/c13b/program/src/events.rs": "2118952a9e5c7a4d699914192c5a303ad24e27db6a3ef7e27a731bb4859ff8e7",
        "benchmark/fixtures/c13b/program/src/instructions/borrower.rs": "1cdbe910ed15a8fcc8fa0bfe3ff10956f6610da951d1566e0a0b2b697b52c877",
        "benchmark/fixtures/c13b/program/src/instructions/lender.rs": "6da6aebf7de1a371f89d59d60b1518817fba654ddb0498bcdae9a41bc3d28ba0",
        "benchmark/fixtures/c13b/program/src/instructions/liquidate.rs": "357d50b567f0791e6c53221308da3d751b729a4cff7004c206719ac337938e75",
        "benchmark/fixtures/c13b/program/src/instructions/market.rs": "f6d1fa9e521dec2fb243d80e7091ea7f097da9dc80a95bdfb2bdc7377443805c",
        "benchmark/fixtures/c13b/program/src/instructions/mod.rs": "7d3d47204374412f72fb97f0e4788a628a2e3ff4feeff842ab0d24372143de5e",
        "benchmark/fixtures/c13b/program/src/lib.rs": "41371517088ae543d5c5ba5269f30f09a569401a7a209cdcd7dbcab08b65d8e5",
        "benchmark/fixtures/c13b/program/src/pricing.rs": "c115454f6860c8b9661ebe0371ef3ccd6286c57ede3fc0443a65461f79489c2e",
        "benchmark/fixtures/c13b/program/src/state.rs": "2a24c6739420ff01c2842f4ced8268a6008b0066c6a27d49569fb77525ae0239",
        "benchmark/workspaces/fixed/programs/stake_credit/Cargo.toml": "4b3da192363cc26069fac1152ba3e614b2e278fcb5bd31c8ba130f9ffce7fafe",
        "benchmark/workspaces/fixed/programs/stake_credit/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/workspaces/fixed/programs/stake_credit/src/errors.rs": "58c3a22b72fc585e1d3c406dc054c2e007427b0bdb41af1db197cea8634b8fe6",
        "benchmark/workspaces/fixed/programs/stake_credit/src/events.rs": "2118952a9e5c7a4d699914192c5a303ad24e27db6a3ef7e27a731bb4859ff8e7",
        "benchmark/workspaces/fixed/programs/stake_credit/src/instructions/borrower.rs": "1cdbe910ed15a8fcc8fa0bfe3ff10956f6610da951d1566e0a0b2b697b52c877",
        "benchmark/workspaces/fixed/programs/stake_credit/src/instructions/lender.rs": "6da6aebf7de1a371f89d59d60b1518817fba654ddb0498bcdae9a41bc3d28ba0",
        "benchmark/workspaces/fixed/programs/stake_credit/src/instructions/liquidate.rs": "357d50b567f0791e6c53221308da3d751b729a4cff7004c206719ac337938e75",
        "benchmark/workspaces/fixed/programs/stake_credit/src/instructions/market.rs": "f6d1fa9e521dec2fb243d80e7091ea7f097da9dc80a95bdfb2bdc7377443805c",
        "benchmark/workspaces/fixed/programs/stake_credit/src/instructions/mod.rs": "7d3d47204374412f72fb97f0e4788a628a2e3ff4feeff842ab0d24372143de5e",
        "benchmark/workspaces/fixed/programs/stake_credit/src/lib.rs": "41371517088ae543d5c5ba5269f30f09a569401a7a209cdcd7dbcab08b65d8e5",
        "benchmark/workspaces/fixed/programs/stake_credit/src/pricing.rs": "18ae94db37e716a08e4bf50557efe5601fa93d4fbdf4ac5fd69293ea64eee30e",
        "benchmark/workspaces/fixed/programs/stake_credit/src/state.rs": "2a24c6739420ff01c2842f4ced8268a6008b0066c6a27d49569fb77525ae0239",
        "benchmark/workspaces/vulnerable/programs/stake_credit/Cargo.toml": "4b3da192363cc26069fac1152ba3e614b2e278fcb5bd31c8ba130f9ffce7fafe",
        "benchmark/workspaces/vulnerable/programs/stake_credit/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/workspaces/vulnerable/programs/stake_credit/src/errors.rs": "58c3a22b72fc585e1d3c406dc054c2e007427b0bdb41af1db197cea8634b8fe6",
        "benchmark/workspaces/vulnerable/programs/stake_credit/src/events.rs": "2118952a9e5c7a4d699914192c5a303ad24e27db6a3ef7e27a731bb4859ff8e7",
        "benchmark/workspaces/vulnerable/programs/stake_credit/src/instructions/borrower.rs": "1cdbe910ed15a8fcc8fa0bfe3ff10956f6610da951d1566e0a0b2b697b52c877",
        "benchmark/workspaces/vulnerable/programs/stake_credit/src/instructions/lender.rs": "6da6aebf7de1a371f89d59d60b1518817fba654ddb0498bcdae9a41bc3d28ba0",
        "benchmark/workspaces/vulnerable/programs/stake_credit/src/instructions/liquidate.rs": "357d50b567f0791e6c53221308da3d751b729a4cff7004c206719ac337938e75",
        "benchmark/workspaces/vulnerable/programs/stake_credit/src/instructions/market.rs": "f6d1fa9e521dec2fb243d80e7091ea7f097da9dc80a95bdfb2bdc7377443805c",
        "benchmark/workspaces/vulnerable/programs/stake_credit/src/instructions/mod.rs": "7d3d47204374412f72fb97f0e4788a628a2e3ff4feeff842ab0d24372143de5e",
        "benchmark/workspaces/vulnerable/programs/stake_credit/src/lib.rs": "41371517088ae543d5c5ba5269f30f09a569401a7a209cdcd7dbcab08b65d8e5",
        "benchmark/workspaces/vulnerable/programs/stake_credit/src/pricing.rs": "c115454f6860c8b9661ebe0371ef3ccd6286c57ede3fc0443a65461f79489c2e",
        "benchmark/workspaces/vulnerable/programs/stake_credit/src/state.rs": "2a24c6739420ff01c2842f4ced8268a6008b0066c6a27d49569fb77525ae0239"
      }
    },
    {
      "id": "c19",
      "history": [
        {
          "at": "2026-10-28T16:58:47.974Z",
          "event": "add",
          "by": "case-author"
        },
        {
          "at": "2026-10-28T17:03:30.753Z",
          "event": "review-pass",
          "by": "case-reviewer",
          "note": "epoch-rollback double-claim reproduces; fixed require(earned_total > claimed) closes it; no second flaw; no hint leaks"
        },
        {
          "at": "2026-10-28T17:51:31.505Z",
          "event": "rehash",
          "by": "case-author",
          "note": "CASE.md trusted roles"
        },
        {
          "at": "2026-10-28T17:52:18.378Z",
          "event": "rehash",
          "by": "case-author",
          "note": "remove narrating comment"
        },
        {
          "at": "2026-10-28T17:54:28.387Z",
          "event": "review-pass",
          "by": "case-reviewer",
          "note": "re-review after fixes"
        }
      ],
      "role": "dev",
      "author": "case-author",
      "review": "pass",
      "readBy": [
        {
          "at": "2026-10-28T17:03:30.543Z",
          "by": "case-reviewer",
          "note": "review"
        },
        {
          "at": "2026-10-28T17:05:05.628Z",
          "by": "executor",
          "note": "dev: author/reviewer summaries"
        },
        {
          "at": "2026-10-28T17:54:28.258Z",
          "by": "case-reviewer",
          "note": "re-review"
        }
      ],
      "files": {
        "benchmark/cases/c19/CASE.md": "c872931e47b96786b27f56ab02e1cc6a221e3870d15160bfd2b9cd232bf03238",
        "benchmark/cases/c19/rubric.md": "33c193eee25a7039068e08a0cb69c5b44e904893a49e2996582be16932662030",
        "benchmark/fixtures/c19a/program/Cargo.toml": "30a18340f3089294aa60010ded2e807a46a6a381f27c08ac859688c785d38a07",
        "benchmark/fixtures/c19a/program/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/fixtures/c19a/program/src/lib.rs": "67aa1cc537832271eaa20ad9702fa774085b4316f4bc5b7add0ce0f1ab71bb36",
        "benchmark/fixtures/c19b/program/Cargo.toml": "30a18340f3089294aa60010ded2e807a46a6a381f27c08ac859688c785d38a07",
        "benchmark/fixtures/c19b/program/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/fixtures/c19b/program/src/lib.rs": "f02237330e0a91a067484ff6fec2c53308953c5e88a523acc7c727b0267db36b",
        "benchmark/workspaces/fixed/programs/affiliate_rewards/Cargo.toml": "30a18340f3089294aa60010ded2e807a46a6a381f27c08ac859688c785d38a07",
        "benchmark/workspaces/fixed/programs/affiliate_rewards/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/workspaces/fixed/programs/affiliate_rewards/src/lib.rs": "67aa1cc537832271eaa20ad9702fa774085b4316f4bc5b7add0ce0f1ab71bb36",
        "benchmark/workspaces/vulnerable/programs/affiliate_rewards/Cargo.toml": "30a18340f3089294aa60010ded2e807a46a6a381f27c08ac859688c785d38a07",
        "benchmark/workspaces/vulnerable/programs/affiliate_rewards/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/workspaces/vulnerable/programs/affiliate_rewards/src/lib.rs": "f02237330e0a91a067484ff6fec2c53308953c5e88a523acc7c727b0267db36b"
      }
    },
    {
      "id": "c20",
      "history": [
        {
          "at": "2026-10-28T16:58:47.905Z",
          "event": "add",
          "by": "case-author"
        },
        {
          "at": "2026-10-28T17:03:30.823Z",
          "event": "review-pass",
          "by": "case-reviewer",
          "note": "self-registered reporter resolves any market; fixed address=market.reporter closes it; no second flaw; no hint leaks"
        },
        {
          "at": "2026-10-28T17:04:18.405Z",
          "event": "rehash",
          "by": "case-author",
          "note": "rubric: reviewer-suggested locations"
        },
        {
          "at": "2026-10-28T17:51:31.428Z",
          "event": "rehash",
          "by": "case-author",
          "note": "fix second flaw: report time vs closes_at"
        },
        {
          "at": "2026-10-28T17:54:28.449Z",
          "event": "review-pass",
          "by": "case-reviewer",
          "note": "re-review after fixes"
        }
      ],
      "role": "dev",
      "author": "case-author",
      "review": "pass",
      "readBy": [
        {
          "at": "2026-10-28T17:03:30.615Z",
          "by": "case-reviewer",
          "note": "review"
        },
        {
          "at": "2026-10-28T17:05:05.696Z",
          "by": "executor",
          "note": "dev: probe runs 3-5 traces and reports"
        },
        {
          "at": "2026-10-28T17:54:28.323Z",
          "by": "case-reviewer",
          "note": "re-review"
        }
      ],
      "files": {
        "benchmark/cases/c20/CASE.md": "737c4ebafd7747de02a1de94c5503a98f6e924d39a68c1ba860c1697c2ca744b",
        "benchmark/cases/c20/rubric.md": "7e80e4a681785ee754e97a46040d1f2e4b570807e111d3687a2a2728de7cef42",
        "benchmark/fixtures/c20a/program/Cargo.toml": "f18fd0afbf25f08d177cb716c2cc15ec74ca061f6d1d3e8879d99b36b81e7b78",
        "benchmark/fixtures/c20a/program/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/fixtures/c20a/program/src/lib.rs": "db53f36cddc6002e1b69604508cdde3272397e330c291c781258a16140f9a1c3",
        "benchmark/fixtures/c20b/program/Cargo.toml": "f18fd0afbf25f08d177cb716c2cc15ec74ca061f6d1d3e8879d99b36b81e7b78",
        "benchmark/fixtures/c20b/program/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/fixtures/c20b/program/src/lib.rs": "eaae90a59417f3a51fcfe9ce818717df3aa0b3957d0673ff7d17b8da3ce01f58",
        "benchmark/workspaces/fixed/programs/outcome_pools/Cargo.toml": "f18fd0afbf25f08d177cb716c2cc15ec74ca061f6d1d3e8879d99b36b81e7b78",
        "benchmark/workspaces/fixed/programs/outcome_pools/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/workspaces/fixed/programs/outcome_pools/src/lib.rs": "db53f36cddc6002e1b69604508cdde3272397e330c291c781258a16140f9a1c3",
        "benchmark/workspaces/vulnerable/programs/outcome_pools/Cargo.toml": "f18fd0afbf25f08d177cb716c2cc15ec74ca061f6d1d3e8879d99b36b81e7b78",
        "benchmark/workspaces/vulnerable/programs/outcome_pools/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/workspaces/vulnerable/programs/outcome_pools/src/lib.rs": "eaae90a59417f3a51fcfe9ce818717df3aa0b3957d0673ff7d17b8da3ce01f58"
      }
    },
    {
      "id": "c24",
      "history": [
        {
          "at": "2026-10-28T16:58:48.046Z",
          "event": "add",
          "by": "case-author"
        },
        {
          "at": "2026-10-28T17:03:30.892Z",
          "event": "review-pass",
          "by": "case-reviewer",
          "note": "authority-only close_epoch locks withdraw when keeper stops; fixed permissionless close with tip works; no second flaw; ARCHITECTURE neutral"
        },
        {
          "at": "2026-10-28T17:04:18.477Z",
          "event": "rehash",
          "by": "case-author",
          "note": "rubric: reviewer-suggested locations"
        },
        {
          "at": "2026-10-28T17:51:31.598Z",
          "event": "rehash",
          "by": "case-author",
          "note": "CASE.md trusted roles"
        }
      ],
      "role": "dev",
      "author": "case-author",
      "review": "pass",
      "readBy": [
        {
          "at": "2026-10-28T17:03:30.683Z",
          "by": "case-reviewer",
          "note": "review"
        },
        {
          "at": "2026-10-28T17:05:05.762Z",
          "by": "executor",
          "note": "dev: author/reviewer summaries"
        }
      ],
      "files": {
        "benchmark/cases/c24/CASE.md": "47e2b01c2fdc3ce095eb32e64a3bc966217077fde46cce862e2784814b607aca",
        "benchmark/cases/c24/rubric.md": "02479befc7e41b7886df045b62a8f26ceafadf7eea789de4c3627c5b05711644",
        "benchmark/fixtures/c24a/program/ARCHITECTURE.md": "f01b9b0b4e5e29404b6b5d8ee60afac197f56c996453d8aea88ff45c563f2af0",
        "benchmark/fixtures/c24a/program/Cargo.toml": "cfa64e577fa3d2cfabb8f341161acd90916cbc565c1ad81be2ee8ef3daec6984",
        "benchmark/fixtures/c24a/program/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/fixtures/c24a/program/src/lib.rs": "9c9b78e5baf2a93141af21fef423ba72afeac28fa95ad83a580b685406d728b1",
        "benchmark/fixtures/c24b/program/ARCHITECTURE.md": "64a64e17c0e166cc5d4c773506e0853b5e21db82b69c92d8d7fbc0d5252fe361",
        "benchmark/fixtures/c24b/program/Cargo.toml": "cfa64e577fa3d2cfabb8f341161acd90916cbc565c1ad81be2ee8ef3daec6984",
        "benchmark/fixtures/c24b/program/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/fixtures/c24b/program/src/lib.rs": "a046c40e344dd1acb0c54362073ddf2e0bdf28e1b844423eb9a3a709233ac486",
        "benchmark/workspaces/fixed/programs/points_staking/ARCHITECTURE.md": "64a64e17c0e166cc5d4c773506e0853b5e21db82b69c92d8d7fbc0d5252fe361",
        "benchmark/workspaces/fixed/programs/points_staking/Cargo.toml": "cfa64e577fa3d2cfabb8f341161acd90916cbc565c1ad81be2ee8ef3daec6984",
        "benchmark/workspaces/fixed/programs/points_staking/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/workspaces/fixed/programs/points_staking/src/lib.rs": "a046c40e344dd1acb0c54362073ddf2e0bdf28e1b844423eb9a3a709233ac486",
        "benchmark/workspaces/vulnerable/programs/points_staking/ARCHITECTURE.md": "f01b9b0b4e5e29404b6b5d8ee60afac197f56c996453d8aea88ff45c563f2af0",
        "benchmark/workspaces/vulnerable/programs/points_staking/Cargo.toml": "cfa64e577fa3d2cfabb8f341161acd90916cbc565c1ad81be2ee8ef3daec6984",
        "benchmark/workspaces/vulnerable/programs/points_staking/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/workspaces/vulnerable/programs/points_staking/src/lib.rs": "9c9b78e5baf2a93141af21fef423ba72afeac28fa95ad83a580b685406d728b1"
      }
    },
    {
      "id": "p03",
      "history": [
        {
          "at": "2026-10-28T18:08:33.340Z",
          "event": "add",
          "by": "case-author"
        },
        {
          "at": "2026-10-28T18:54:55.294Z",
          "event": "rehash",
          "by": "executor",
          "note": "public reviewer suggestions applied by case-author"
        },
        {
          "at": "2026-10-28T18:54:55.368Z",
          "event": "review-pass",
          "by": "case-reviewer",
          "note": "public review (logs/review-public.json)"
        }
      ],
      "role": "public",
      "author": "case-author",
      "review": "pass",
      "readBy": [],
      "files": {
        "benchmark/cases/p03/CASE.md": "6892303f77c8b4207f9488eab6d48b6bb9bc59ce486ed803bc0eca438ab1dd95",
        "benchmark/cases/p03/rubric.md": "1f635bee19c56d11e6b1e24ba12f8ca9a883eb88379ec938fac387f12df2e9f8",
        "benchmark/fixtures/p03a/program/Cargo.toml": "d2fe2aad5ba0f96b643eab7c592f27830b885c86b2e798e9a4386120c68417bf",
        "benchmark/fixtures/p03a/program/Xargo.toml": "f36d7db5970cc4260ebb5be4ecf2dcfc7cec0a7d644412e84c3cb10dd7d07a56",
        "benchmark/fixtures/p03a/program/src/ed25519.rs": "46c04cd77f7291124993f025df337f0297da651f7a6378d86e9390916574c9c3",
        "benchmark/fixtures/p03a/program/src/error.rs": "c53e8280511a289344291bbfdc1996102a4d025b3dd9bdbba0d7eaad74c909ae",
        "benchmark/fixtures/p03a/program/src/instructions/claim.rs": "5db15f8eeea5d38a2e1cb625f4554e4b574026e5273c55851f1bdcfddd3de6af",
        "benchmark/fixtures/p03a/program/src/instructions/initialize.rs": "dd029c311f4af8d8d2ce9119f3783c681d8ef523c8521a80b57477df940cc30d",
        "benchmark/fixtures/p03a/program/src/instructions/mod.rs": "cad00572025948698bf0e689a868f837f8e6775f837bb5eab1ffc4b34187e096",
        "benchmark/fixtures/p03a/program/src/instructions/register_device.rs": "b1784385fe21b0908a525ca9d5936dd4108a3e941a2ac94ec50997066c5d939c",
        "benchmark/fixtures/p03a/program/src/lib.rs": "c01bb82e755cc3f29728f4ed4fcabfcd2fea4970cda6c364a389d9f09a0e7f88",
        "benchmark/fixtures/p03a/program/src/state/config.rs": "f3edd4ca3440b3cd93a70c7018c59edd1a058f2cdeaef8e03397e91c62efb991",
        "benchmark/fixtures/p03a/program/src/state/device.rs": "8dd4f691c1aa8cad75db554f1dfb133e38e6d7753236949d5cab6d90b3a4948d",
        "benchmark/fixtures/p03a/program/src/state/mod.rs": "6b29b0a30b6cf71a8e0df46229e393dda8ac1d5c08cd69d7fc1f3b06441606dd"
      }
    },
    {
      "id": "p06",
      "history": [
        {
          "at": "2026-10-28T18:08:33.418Z",
          "event": "add",
          "by": "case-author"
        },
        {
          "at": "2026-10-28T18:54:55.448Z",
          "event": "rehash",
          "by": "executor",
          "note": "public reviewer suggestions applied by case-author"
        },
        {
          "at": "2026-10-28T18:54:55.523Z",
          "event": "review-pass",
          "by": "case-reviewer",
          "note": "public review (logs/review-public.json)"
        }
      ],
      "role": "public",
      "author": "case-author",
      "review": "pass",
      "readBy": [],
      "files": {
        "benchmark/cases/p06/CASE.md": "51bc38a1a218f6b7ff56b98501c906480a822aac9a0f40d5ec62cee2352cb13a",
        "benchmark/cases/p06/rubric.md": "e0b4817b2c4970c5a32bc45c7d3b9cb1ed053e35ccbe47d75e9ed44963e333c8",
        "benchmark/fixtures/p06a/program/Cargo.toml": "1d94c79756ba18a393eba463b428f2714c23b23e1377809d70f02d71e587361a",
        "benchmark/fixtures/p06a/program/Xargo.toml": "f36d7db5970cc4260ebb5be4ecf2dcfc7cec0a7d644412e84c3cb10dd7d07a56",
        "benchmark/fixtures/p06a/program/src/ed25519.rs": "632467af56e6a6fd6337e01c92eb090f0a79f53890d49447904afe0b3e40b59a",
        "benchmark/fixtures/p06a/program/src/error.rs": "d828f743a67bf2ca729299b13c7dd4e42d5cd6992606c774d921e0db5a5b21f9",
        "benchmark/fixtures/p06a/program/src/instructions/accrue.rs": "7bc091e7586b37d16465b5e45f3a4f0fe3a873fb8359f79d91336673e2dddc6b",
        "benchmark/fixtures/p06a/program/src/instructions/buy.rs": "21e0ed08fa422d557d179e45b9253bc5cff0af414bc4f5d39484767547dbf29a",
        "benchmark/fixtures/p06a/program/src/instructions/initialize.rs": "dff5261ef0a1221dc3da30fac71e55155a9cfdef08211511508eba26e910ea6b",
        "benchmark/fixtures/p06a/program/src/instructions/mod.rs": "51e2d98513ca2af2c6df60b354b167c743101ed8ccd91ffe6cbc74dc64c94030",
        "benchmark/fixtures/p06a/program/src/instructions/pay_reward.rs": "acaf8e88c85bfda9a0739001c5296a6655a95cc13d73950fc6eb083edce9a240",
        "benchmark/fixtures/p06a/program/src/instructions/set_price.rs": "60758c694dba76f7f0744b2503bfb9fe0f183bc43511e3949097d9e5e3e8aff1",
        "benchmark/fixtures/p06a/program/src/lib.rs": "0f99a7bb0f32eda18bb8607b0d4ab74e1b73fd0594aaef0e31b3f4fe3ccfb61a",
        "benchmark/fixtures/p06a/program/src/state/config.rs": "f2c5dc4f517e0109afbda9c49c4e8a92482782d7886d454dc1730b78c183548d",
        "benchmark/fixtures/p06a/program/src/state/mod.rs": "8a72f8f3bcdceb814520e72db8c9ce3e2472aaf693dc47f26265f419725f6602",
        "benchmark/fixtures/p06a/program/src/state/user.rs": "99301a3fcb09d432e697fb9a92af920abbe1658ab8c4d2de6d2fc4767e9d94b8"
      }
    },
    {
      "id": "p08",
      "history": [
        {
          "at": "2026-10-28T18:08:33.500Z",
          "event": "add",
          "by": "case-author"
        },
        {
          "at": "2026-10-28T18:54:55.601Z",
          "event": "rehash",
          "by": "executor",
          "note": "public reviewer suggestions applied by case-author"
        },
        {
          "at": "2026-10-28T18:54:55.679Z",
          "event": "review-pass",
          "by": "case-reviewer",
          "note": "public review (logs/review-public.json)"
        }
      ],
      "role": "public",
      "author": "case-author",
      "review": "pass",
      "readBy": [],
      "files": {
        "benchmark/cases/p08/CASE.md": "f91d4dd7ea1e9cf41de469f1ce1491f6fbbce04eef64313560e672cf00266347",
        "benchmark/cases/p08/rubric.md": "c613d965887e57dd9f7e43eb36a0fdbdb0b612f5f430dbd8ab08ce1e5e46538e",
        "benchmark/fixtures/p08a/program/Cargo.toml": "7ce1c9f5aa2b0d3767a2265715fb0dd9e6b47fc4be8f768568f988e361837da5",
        "benchmark/fixtures/p08a/program/Xargo.toml": "f36d7db5970cc4260ebb5be4ecf2dcfc7cec0a7d644412e84c3cb10dd7d07a56",
        "benchmark/fixtures/p08a/program/src/ed25519.rs": "ab00cfe54482051e77ea31a21126a01d1a0ee29633c9c69fdd198e810b3a31f2",
        "benchmark/fixtures/p08a/program/src/error.rs": "29e2b5bbd5a8c6e4cd1f18c4c17bce423fc9da731ab049764b97b456f2cbcbba",
        "benchmark/fixtures/p08a/program/src/instructions/create_allowlist.rs": "22e30cdd6ae48e10f6e5c1e89d4aa5e9a383c79aa7bea54d2a75b2021c06f0e3",
        "benchmark/fixtures/p08a/program/src/instructions/deposit.rs": "96dcb9766c12131a247cb151d3dcafd76a9941ace6d00a2193a2ac456cfc45da",
        "benchmark/fixtures/p08a/program/src/instructions/initialize.rs": "a0284f33f0c94691a0f467d7b8e7caf1e3efd6fddbe9da1038956535f8775389",
        "benchmark/fixtures/p08a/program/src/instructions/mod.rs": "79b49915aabd8ffe5c1a24e7624d7d340d24d42ef4ac96a359ad724fc27869b2",
        "benchmark/fixtures/p08a/program/src/instructions/withdraw.rs": "7a4c2f34a0a4cddce49d73932217c3b9ff3a2483d2ab40c4531e8f804fe2e3c5",
        "benchmark/fixtures/p08a/program/src/lib.rs": "b71c07cf9385af477b91196e612915ef0e59207415a9d8d84c3c3d3e56b735a6",
        "benchmark/fixtures/p08a/program/src/state/allowlist.rs": "6931db62a737343e37ff20a888ebc597411bdce6e2aa6c01ce37d79bb077a6bd",
        "benchmark/fixtures/p08a/program/src/state/config.rs": "6f1dd612ed5eac9316bab15162ef98d06c92a563bba5c00fcaafed41bddcdf7c",
        "benchmark/fixtures/p08a/program/src/state/mod.rs": "007c1c9498bcc0138e1f906e3b0021d492dc8e92e7721e45288fc8c7a6f272db",
        "benchmark/fixtures/p08a/program/src/sysvar.rs": "c2d6dc8fcf06b82d113a67d73ea20853fcf4abfd7d3f47547342fa0b3aee92d3"
      }
    },
    {
      "id": "p16",
      "history": [
        {
          "at": "2026-10-28T18:08:33.583Z",
          "event": "add",
          "by": "case-author"
        },
        {
          "at": "2026-10-28T18:54:55.776Z",
          "event": "rehash",
          "by": "executor",
          "note": "public reviewer suggestions applied by case-author"
        },
        {
          "at": "2026-10-28T18:54:55.847Z",
          "event": "review-pass",
          "by": "case-reviewer",
          "note": "public review (logs/review-public.json)"
        }
      ],
      "role": "public",
      "author": "case-author",
      "review": "pass",
      "readBy": [],
      "files": {
        "benchmark/cases/p16/CASE.md": "4606a894a2aeeac20ad7f028466ca1711a95122409ad9eb544ee7d9d6ff2eb2e",
        "benchmark/cases/p16/rubric.md": "2b27ee3c41d9ecb9b4a25c64bfcbec6d3b9c3f0b9b6cb0dfc244df4861187b56",
        "benchmark/fixtures/p16a/program/Cargo.toml": "d47fdc8e4e33e9a28f28a8674e456c904db0196a682986f7639614b1b6f2a938",
        "benchmark/fixtures/p16a/program/Xargo.toml": "815f2dfb6197712a703a8e1f75b03c6991721e9eb7c40dfaec8b0b49da4aa629",
        "benchmark/fixtures/p16a/program/src/constants.rs": "b3e86fdd3b9d1eab360c7dc30c4d0f219890de1b704fb3595e743740784b808e",
        "benchmark/fixtures/p16a/program/src/error.rs": "2201eeb397b76f3b44029f75c77ba36f0c39b31db4d405e6bef4b645b655affc",
        "benchmark/fixtures/p16a/program/src/instructions/initialize.rs": "5127c712594c0f95d3a087c90c25e8e0c2815dcf4048c3b993f1981d0f59c2db",
        "benchmark/fixtures/p16a/program/src/instructions/join.rs": "f1a53096bf16add5f75c1b354a8a63acd2b243d97eedf2f721097e7d8b392b0f",
        "benchmark/fixtures/p16a/program/src/instructions/join_with_swap.rs": "7638fe2523e9f530022fbf1e1b9ce376cff6994252af06169b99c43d8538fd62",
        "benchmark/fixtures/p16a/program/src/instructions/mod.rs": "8fa8f1dd6b0e8d5e80bdd68073c8ea8242f24acb6806a0f574f488c311526773",
        "benchmark/fixtures/p16a/program/src/instructions/open_round.rs": "96517bbfe9e24763396eeb4ea2c3b674b234503f6719222e95c39f927b40df3e",
        "benchmark/fixtures/p16a/program/src/instructions/settle.rs": "59fd96c69a2aef9ea16416d0e15895c10f676e2093d9b943c4a22cb9f2fe59e0",
        "benchmark/fixtures/p16a/program/src/lib.rs": "ff58c0912d556c8198ddb46ce149bed69af44283bc0980df98c150a26bb58f73",
        "benchmark/fixtures/p16a/program/src/state/config.rs": "7f7a0b505e1bb0b545285efac4a6658f9ab1b430c08fb027b24b4c213359fc9b",
        "benchmark/fixtures/p16a/program/src/state/entry.rs": "914131b0014e78846a9cef1510df1c056ce57a3111bda3bac3a57847ad4243b6",
        "benchmark/fixtures/p16a/program/src/state/mod.rs": "3d204161458eb79611b79730feedc55b88df6b42ebb4e0e4ae5fe56142dc2ea6",
        "benchmark/fixtures/p16a/program/src/state/round.rs": "b2299e2ebcbb9736f7b4b4a74d76f9072370bd9876b446288247e2ae3469f60f"
      }
    },
    {
      "id": "p18",
      "history": [
        {
          "at": "2026-10-28T18:08:33.664Z",
          "event": "add",
          "by": "case-author"
        },
        {
          "at": "2026-10-28T18:54:55.927Z",
          "event": "rehash",
          "by": "executor",
          "note": "public reviewer suggestions applied by case-author"
        },
        {
          "at": "2026-10-28T18:54:55.996Z",
          "event": "review-pass",
          "by": "case-reviewer",
          "note": "public review (logs/review-public.json)"
        }
      ],
      "role": "public",
      "author": "case-author",
      "review": "pass",
      "readBy": [],
      "files": {
        "benchmark/cases/p18/CASE.md": "a4c727a3a0c20856b7ad864c94fa35ce66d9a2797fcd4ea0b4a70c6d592094df",
        "benchmark/cases/p18/rubric.md": "5cf19cc793df976c41fe2e4926f9a7261dfcd92248143e07396e7a3538d4735e",
        "benchmark/fixtures/p18a/program/Cargo.toml": "cc053535cd15aa50ced66aa1ad98834fa36ee3d515ce30eb308178c3ce4c3c2c",
        "benchmark/fixtures/p18a/program/Xargo.toml": "f36d7db5970cc4260ebb5be4ecf2dcfc7cec0a7d644412e84c3cb10dd7d07a56",
        "benchmark/fixtures/p18a/program/src/error.rs": "360ed22ababcf15fe84ebba09523ef99267d24a10cc685498cd2948c4fc4a299",
        "benchmark/fixtures/p18a/program/src/instructions/approve.rs": "2a1ca4c13cc31564c6ee39d06ecb02d5e73b2b51b226d0251698bf2bf930d0b7",
        "benchmark/fixtures/p18a/program/src/instructions/create_proposal.rs": "f2eea59f247abdcf246cb722f0e7c964188c0fc0debcb6a02d3e1dccda3597b5",
        "benchmark/fixtures/p18a/program/src/instructions/execute.rs": "fec3fc242491444cf818165c501fc3364b237f4ebacb4d8b5164c9c6f6661239",
        "benchmark/fixtures/p18a/program/src/instructions/initialize.rs": "e4a987162681455012ab73840a1b04f989559657da13c4a06962ee4eed8f37ef",
        "benchmark/fixtures/p18a/program/src/instructions/mod.rs": "70a993ff12fa8c0a2be322aca71f6bc8a28865d4c5c8d9fc35fb0a28ece88635",
        "benchmark/fixtures/p18a/program/src/lib.rs": "2226079873b7a9aa3ce7b0fec99ab23e4b97c281e89ffdf4877407501c54391f",
        "benchmark/fixtures/p18a/program/src/state/config.rs": "c825f370575a2b8f8028cd2f4256ab6ca6c77402726b3751f88f4515c7ba4584",
        "benchmark/fixtures/p18a/program/src/state/mod.rs": "04cfb0dc7eda1705326db1e868e5d926e92416982fe7181443389563bb022c68",
        "benchmark/fixtures/p18a/program/src/state/proposal.rs": "9e3981757e7254d4dd4d9ca1b5abba2d0871cb2664c1788ecd4f879902f4f4fd"
      }
    },
    {
      "id": "p20",
      "history": [
        {
          "at": "2026-10-28T18:08:33.747Z",
          "event": "add",
          "by": "case-author"
        },
        {
          "at": "2026-10-28T18:54:56.075Z",
          "event": "rehash",
          "by": "executor",
          "note": "public reviewer suggestions applied by case-author"
        },
        {
          "at": "2026-10-28T18:54:56.147Z",
          "event": "review-pass",
          "by": "case-reviewer",
          "note": "public review (logs/review-public.json)"
        }
      ],
      "role": "public",
      "author": "case-author",
      "review": "pass",
      "readBy": [],
      "files": {
        "benchmark/cases/p20/CASE.md": "aac264ad660fe0b8f86685f7212a3dc4a1241f92dcc1005902575b97bbc604ae",
        "benchmark/cases/p20/rubric.md": "9fffec9fcd228997bf3402d3c9896686a1cf134e9ee8bad9d4f886a234cc80cb",
        "benchmark/fixtures/p20a/program/Cargo.toml": "66e78302751bb5c76de424f3915fa14285e8203107d1022c56959c1602e89f42",
        "benchmark/fixtures/p20a/program/Xargo.toml": "f36d7db5970cc4260ebb5be4ecf2dcfc7cec0a7d644412e84c3cb10dd7d07a56",
        "benchmark/fixtures/p20a/program/src/error.rs": "67a1cfb17c4201569151a67c17e97264b5763882ff470f70490708ff90c9c206",
        "benchmark/fixtures/p20a/program/src/instructions/add_rewards.rs": "82e30ca4e3e05612c3f3fc63cfe92ee8c1d7144da86a37e5b44e997a80f9e0b2",
        "benchmark/fixtures/p20a/program/src/instructions/claim.rs": "69b1137de9f4f760d0038d57e959683097107ce5e83fe5975e91983219c1d455",
        "benchmark/fixtures/p20a/program/src/instructions/initialize.rs": "3d7566d37ca261ef4e3f15f157e7755a5466bec124d0b05a0c11f352ae102f51",
        "benchmark/fixtures/p20a/program/src/instructions/mod.rs": "173c4845185dac22d3273a8229c134f9ebdb092bcaa1575f62fee5bec405f1df",
        "benchmark/fixtures/p20a/program/src/instructions/stake.rs": "b65e99ae604a5ed0bb61484f417efce67598bbee7eaf06d5dd5d5699b0e13dae",
        "benchmark/fixtures/p20a/program/src/instructions/unstake.rs": "62f20435afc64ea6423fc7bfcd1809881cd67a3498a799b57ff17cad71386f48",
        "benchmark/fixtures/p20a/program/src/lib.rs": "10de802355069c6cfb7707e8d794d4f1ceddd3237066d34be737d2353220473d",
        "benchmark/fixtures/p20a/program/src/state/mod.rs": "10340db3102fb657db9960a1cba8f5587a42fded58628846ae075a7603bfb0e3",
        "benchmark/fixtures/p20a/program/src/state/pool.rs": "aa9e8b80ffc1e03db050e4493d156b489667b4ea73f9d36a3786aee1923dd746",
        "benchmark/fixtures/p20a/program/src/state/staker.rs": "0eef25cd8713d3631a74ae89095df82d91b7f5c99badb4dcd0c01b36e2d973df"
      }
    }
  ]
}
```
