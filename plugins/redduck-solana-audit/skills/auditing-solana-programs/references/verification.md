# Verifying a finding

This is the procedure the `finding-verifier` agent applies to every High and Critical finding. Apply it yourself, in the same session, when that agent is not available. Its job is to refute the finding if it can. A finding survives only if every step holds.

1. **Reachability.**
   - Which instruction does the attack call, and what must the attacker sign?
   - Read the account constraints and the handler. Name the exact line that would stop the attack if one exists: Anchor attributes, seeds, `has_one`, `address`, owner checks or manual `require!`s.
   - If such a line exists, the finding is refuted.
2. **Preconditions.**
   - Can an ordinary user meet every precondition, by creating accounts, choosing arguments and ordering transactions?
   - If the attack needs a privileged key, it is a design finding about that key, not an exploit.
3. **Impact.**
   - What exactly does the attacker gain, or what does a user lose? Is it bounded?
   - Recompute the severity against the table in `report-format.md`.
4. **Environment.** Check the claim against how Solana actually behaves:
   - atomic transactions;
   - a failing CPI aborts everything;
   - no reentrancy through other programs;
   - only an account's owner program can change its data or debit its lamports;
   - rent;
   - the Anchor version's checks;
   - the token program's semantics.
5. **Known misreports.** Compare the finding with `false-positives.md`.
6. **Scenario replay.** Walk the scenario step by step against the code with real values. Any step that does not hold breaks the finding.

Verdict, with evidence citing `file:line`:
- `CONFIRMED`: every step holds.
- `DOWNGRADE <severity>`: the flaw is real but the impact or likelihood is lower than claimed. Give the reason.
- `REFUTED`: a step fails. Name the blocking line or the false assumption.

For design findings, also confirm that the dependency really exists, for example that the instruction is really restricted to the operator. Then confirm the consequence: funds locked, or only a feature paused.
