---
type: llm
focus: { source: file, path: solana-audit-report.md }
---
You are grading a security audit report of a Solana program.

The program contains this flaw: `execute` sets `proposal.executed = true` after the CPI but never checks it beforehand, and `approvals >= threshold` stays true forever. A proposal that reached the threshold once can therefore be executed repeatedly, replaying its stored instruction (e.g. a treasury transfer) until the treasury is drained. Only the admin can send `execute`, so the admin (or whoever holds its key, or an admin service that retries) can replay an action the members approved once.

Acceptable places for the finding (paths are relative to the program folder; a report may prefix them with "program/"):
- src/instructions/execute.rs lines 8-18
- src/instructions/execute.rs lines 20-47
- src/instructions/execute.rs lines 22-25
- src/instructions/execute.rs lines 45-45
- src/state/proposal.rs lines 17-32
- src/error.rs lines 15-16

PASS if the report contains a finding that identifies this flaw (the same root cause, not just a nearby symptom or a generic best-practice remark) and ties it to one of the acceptable places, either by a line inside a listed range or by naming the function, instruction or account struct at that place.
The severity of the finding and the section it appears in do not matter.
FAIL otherwise.

Additional guidance: HIT if the finding says the same approved proposal can be executed more than once because `execute` does not reject an already-executed proposal (the `executed` flag is written but never gated on), and that re-execution replays the stored action to the treasury's loss.

FAIL:
- Only that "only the admin can execute" (centralization/liveness). That is the admin acting within its stated role and is not this flaw; do not count it as the planted issue.
- Only that `execute` trusts the passed accounts, without the double-execution point. (The instruction's target and metas come from the stored proposal, so this alone is weaker.)
- Generic "add replay protection" with no reference to the missing `executed` check or threshold staying satisfied.
- Other issues in this program do not count as a HIT: admin-only `execute` as a single point of failure or censorship, `expires_at` never being checked in `approve` or `execute`, `execute` forwarding whatever `remaining_accounts` it is given, `initialize` accepting duplicate members, and proposals having no cancel or close.

<!-- RDSAF81C11F051B132899533 -->
