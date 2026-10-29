---
name: auditing-solana-programs
description: Audits Solana programs written in Rust (Anchor, native solana-program, Pinocchio) for architecture-level security flaws using 18 named design patterns distilled from the top Solana protocols, such as root of trust, untrusted executors, one-way state, defensive oracle computation, fail-fast and boundary of consent, plus a compact checklist of classic Solana bug classes. Use when asked to audit, security-review or threat-model a Solana or Anchor program, smart contract or on-chain protocol; to review account validation, oracle usage, crank or keeper design, upgrade and dependency risks; or to produce a findings report with severities.
license: MIT
---

# Auditing Solana programs

This skill reviews a Solana program the way an architect would. A short list of bug signatures does not catch everything. The question for every instruction is whether the design holds when the caller, the operator, the oracle or a dependency behaves in the worst way it can.

Solana makes some mistakes easy, and the method follows from that:

- The caller chooses every account an instruction sees.
- Nothing runs unless somebody sends and pays for a transaction.
- A transaction can touch only a bounded set of accounts and compute.
- Two transactions can land in either order.

## Output

Write the report to `solana-audit-report.md` in the working directory unless the user names another path, using [references/report-format.md](references/report-format.md). Every finding has a `Severity:`, a `Location: <file>:<line>` and a `Mechanism:` line. Repeat the list of findings in your final message.

## Workflow

Every step is required. Steps 3, 4, 7 and 8 are where this skill finds what a quick read misses, so do not skip them to save time. Copy this checklist into your notes and tick it off:

```
- [ ] 1. Scope
- [ ] 2. Account map
- [ ] 3. Root-of-trust pass
- [ ] 4. Attack-surface routing
- [ ] 5. Baseline checklist
- [ ] 6. Draft findings
- [ ] 7. Verify findings
- [ ] 8. Write the report
```

**1. Scope.**
- Find the framework: Anchor (`#[program]`, `anchor_lang`), native (`entrypoint!`, `process_instruction`) or Pinocchio (`pinocchio::`).
- List every program, every instruction and every account type.
- Read any `README`, `ARCHITECTURE` or deployment notes. Facts about who runs what off-chain are part of the design.

**2. Account map.** For each instruction, note for every account:
- who can create it;
- which constraints apply (signer, writable, owner, seeds and bump, `has_one`, `address`, discriminator);
- what the instruction does with it.

Anchor's `Account<T>` checks the owner and discriminator, `Signer` checks the signature and `Program<T>` checks the program id. `#[account(mut)]` checks nothing about who owns the account.

**3. Root-of-trust pass.** This is always done, for every instruction. Take each account and value the instruction trusts and ask "if this one is fake, what happens?". Follow the answer to the next check, until the chain ends. It must end at something the caller cannot create:
- a PDA of this program with verified seeds;
- a hardcoded address;
- a stored authority that signed.

Then ask whether an attacker could create a fresh set of accounts that agree with each other and pass every check. Details: [references/patterns/root-of-trust.md](references/patterns/root-of-trust.md).

**4. Attack-surface routing.** For each surface present in the code, open the pattern files the table names with the Read tool and work through their auditor questions against this program. Answering from memory is not enough: the files list code signals and "not a bug if" conditions you will need. Note which question in the file decided each verdict, because the coverage table cites it. The report's Scope section lists the surfaces you found and the pattern files you read. A surface listed with no file read means step 4 is not done. Skip files whose surface is absent, but still give every pattern a verdict in the coverage table, using the one-line rules below.

| Surface in the code | Read |
|---|---|
| Instruction introspection (`sysvar::instructions`, `load_instruction_at*`, `get_instruction_relative`), Ed25519 or Secp256k1 signature checks | root-of-trust, identity-rejection |
| Allow lists, deny lists, per-wallet limits, anti-bot rules, checks on who calls rather than on what happens | identity-rejection |
| An action the protocol needs (settle, distribute, close a round, liquidate, crank, update a price) gated to an admin, operator or keeper key | untrusted-executor, role-inversion, dependency-independency |
| Data pushed by an operator (`set_price`, `update_rate`, `report_result`) | role-inversion, defensive-computation, fail-fast |
| Loops over accounts, vectors or `remaining_accounts`; "for every user" updates; per-period jobs | lazy-loading, pay-as-you-act, complexity-asymmetry |
| Rewards, interest, funding, fees accrued over time; index or accumulator fields | lazy-loading, pay-as-you-act, one-way-state |
| One global account written by every user instruction; accounts that grow with users | divide-and-conquer, complexity-asymmetry |
| Sorted structures, order books, on-chain search, merkle proofs, client-supplied hints | complexity-asymmetry, divide-and-conquer |
| Randomness, outcomes a user can preview or time, first-come mechanics, auctions | commit-execution-split |
| Stored instructions, proposals, limit orders, scheduled actions executed later | action-as-data, untrusted-executor |
| Fills or matches that owe tokens to parties who are not in the transaction; queues of obligations | virtual-execution |
| Signed messages, claims, nonces, sequence numbers, deltas, counters, status transitions, close and reopen | one-way-state |
| Swaps, deposits or withdrawals at a computed rate, liquidations, any amount the user cannot know in advance | boundary-of-consent |
| Prices or rates derived from the program's own state (pool reserves, balances, TWAP accumulators) | internal-oracle, defensive-computation |
| External oracle reads (Pyth, Switchboard, others): price, confidence, publish time, exponent | defensive-computation, fail-fast |
| Valuation of collateral or debt, share math, rounding | defensive-computation |
| Rollups, ephemeral rollups, delegation programs, bridges, off-chain matching | execution-layer-shift, dependency-independency |
| CPI into external protocols, hardcoded external program ids, upgrade authority, keys held by third parties | dependency-independency |

Pattern files live in `references/patterns/<slug>.md`.

**5. Baseline checklist.** Work through [references/baseline-checklist.md](references/baseline-checklist.md). It covers the classic bug classes the patterns do not, such as arithmetic, duplicate accounts, account closing and Token-2022.

**6. Draft findings.** One finding per root cause, each with:
- the location, at the line where the missing or wrong check belongs;
- the mechanism, saying what is wrong and how it is abused;
- a step-by-step scenario.

Report design problems (liveness, centralization, scalability) as findings in their own section. Do not report generic advice that is not tied to a line of this program.

**7. Verify findings.** Do not write the report until every High or Critical finding has been through this step. Do the following for each one:
- **Test it if you can.** If a Solana toolchain is available, write and run an exploit test as described in [references/exploit-tests.md](references/exploit-tests.md). Mark the result `confirmed` or `refuted`. Without a toolchain, mark it `unconfirmed (no toolchain)`.
- **Ask the verifier.** Call the Agent tool with `subagent_type: "redduck-solana-audit:finding-verifier"`, once per finding. Several calls can go in one message. Pass the finding (severity, location, mechanism, scenario) and the absolute path to the program. A second reader catches mistakes the author of a finding cannot see, so this call is not optional. Fall back to applying [references/verification.md](references/verification.md) yourself only in two cases:
  - you have no Agent tool at all, as in agents other than Claude Code;
  - the Agent tool returns an error or says the agent is not registered, which happens in installs that copy only the skill folder.

  Quote the reason in the finding's `Verification:` line.
- **Rule out known misreports.** Compare each finding against [references/false-positives.md](references/false-positives.md).
- **Act on the verdict.** Drop refuted findings and lower downgraded ones. Record the verdict in the finding.

**8. Write the report.** Read [references/report-format.md](references/report-format.md) first and follow it exactly. The report must include:
- the finding blocks;
- the `## Design findings` section;
- the coverage table, which gives every pattern and checklist item `found`, `clear` (with the line of the protection) or `n/a` (with a reason).

The table has exactly 29 rows, one per item, never merged:
- the 18 pattern slugs below;
- `checklist-arithmetic`, `checklist-duplicate-accounts`, `checklist-unsafe-and-panics`, `checklist-reload-after-cpi`, `checklist-token-2022`, `checklist-closing`, `checklist-seed-collisions`, `checklist-init-frontrun`, `checklist-realloc`, `checklist-authority-transfer`, `checklist-signers-bumps-cpi`.

A report without the full table is incomplete.

## The 18 patterns at a glance

**Trust and identity**
- **root-of-trust.** Every chain of checks ends at something the caller cannot create. *Where does each trusted account's chain end? Could a fresh, self-consistent set of fake accounts pass?*
- **identity-rejection.** Guard what the action does, not who sends it. *Does safety rest on who calls, through lists or one-per-wallet rules? Does every instruction that performs the guarded action pass through the same guard?*

**Permissionless execution**
- **untrusted-executor.** Work the protocol needs must be callable by anyone, paid for by an incentive, and safe whoever calls it. *If the team's bot stops forever, what is stuck? If a stranger calls it, can value go anywhere but to its owner?*
- **role-inversion.** The party that needs an action brings it, and the program only verifies. *Is an operator pushing data the consumer could bring signed? After the flip, does the party that needs the action still control when it happens?*

**Cost distribution**
- **lazy-loading.** Settle each account when it is touched, from a global running total and a per-account marker. *Does anything iterate over all holders? Is the accrued difference settled before every balance change?*
- **pay-as-you-act.** Store the answer, not the inputs: every action updates the aggregate it changes. *Is there a final step that must process many items at once? Are aggregates updated on every path, including withdrawals and cancels?*

**State partitioning**
- **divide-and-conquer.** Split state so each user touches only their part, and keep a small central summary. *Does every user write to one hot account, or to an account that keeps growing? Do partitions open and close without breaking the totals?*
- **complexity-asymmetry.** Search off-chain and verify on-chain in bounded work. *Does any on-chain work grow with user-controlled data? When the client supplies a position or proof, is it fully verified and the size bounded?*

**Deferred execution**
- **commit-execution-split.** If knowing the outcome first is profitable, commit and resolve in different transactions, with the user not controlling resolution. *Can a user preview or time the outcome? Can they abandon a losing commitment or start another before resolution?*
- **action-as-data.** Store pending actions on-chain so anyone can execute them when conditions hold, with cancel and expiry. *Is the executed payload exactly what was approved, with no account substitution? Can it run twice, after cancel, or on stale approvals?*
- **virtual-execution.** Record obligations now and settle them later in a permissionless step. *Can an obligation be settled twice or to the wrong owner? Can anyone settle it, and is the protocol solvent meanwhile?*

**Order and consent**
- **one-way-state.** Messages carry destinations (totals), not deltas, and state only moves forward or narrows. *Do two valid transactions in either order, or twice, leave the same state? Can any counter, status or permission move back?*
- **boundary-of-consent.** Users sign bounds (minimum out, maximum in, limit, maximum age, deadline), not exact expectations. *Can the user bound every value computed from mutable state? Are the bounds checked on the amounts actually moved, after fees?*

**Outside data**
- **internal-oracle.** Derive data from your own state, but only from state that cannot be moved in the same transaction. *Can the attacker move the source within the transaction, for example spot reserves or donations? Is the derived state updated on every path?*
- **defensive-computation.** Value everything at its worst plausible case: collateral low, debt high, rounding against the user. *Is oracle confidence used, and does rounding favour the protocol? Can a user profit from the gap between two prices or roundings?*
- **fail-fast.** When inputs are not good enough, refuse rather than act. *Are staleness, confidence, deviation and availability checked before every use, with thresholds that fit each action? Is "not ready yet" a state that blocks dependent actions?*

**Dependencies and exits**
- **execution-layer-shift.** Move to another layer only for a real constraint, and keep funds and settlement on L1. *What state or funds leave L1, and what happens to them if that layer stops? Can users exit on their own?*
- **dependency-independency.** Every external dependency needs an exit that does not need a redeploy or the dependency's cooperation. *Which programs, oracles, keys or services must keep working for users to get funds back? Who can switch away, and how?*

## Ground rules

- Cite `file:line` for every claim. A protection counts only when you can point at it.
- Do not trust names. Read the condition. A function called `validate_owner` may not validate the owner.
- A role the protocol deliberately trusts (admin, authority, oracle signer) is a design fact, not a vulnerability. Report its power as a design finding at Medium at most, unless the role can do more than its stated purpose or untrusted users can take it. The cap is only for harm that needs the role to misbehave: if an ordinary user profits while the role behaves honestly, rate by impact. See "Trusted roles" in [references/report-format.md](references/report-format.md).
- Solana transactions are atomic, a failing CPI aborts the whole transaction, and a program cannot be re-entered through another program. Check [references/false-positives.md](references/false-positives.md) before reporting anything that relies on the opposite.
- If a reference file cannot be read, continue with the one-line rules and questions above. Do not stop the audit.
