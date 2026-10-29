---
id: p18
canary: RDSAF81C11F051B132899533
---
## Mechanism
`execute` sets `proposal.executed = true` after the CPI but never checks it beforehand, and `approvals >= threshold` stays true forever. A proposal that reached the threshold once can therefore be executed repeatedly, replaying its stored instruction (e.g. a treasury transfer) until the treasury is drained. Only the admin can send `execute`, so the admin (or whoever holds its key, or an admin service that retries) can replay an action the members approved once.

## Locations
- src/instructions/execute.rs:8-18
- src/instructions/execute.rs:20-47
- src/instructions/execute.rs:22-25
- src/instructions/execute.rs:45
- src/state/proposal.rs:17-32
- src/error.rs:15-16

## Judge criteria
HIT if the finding says the same approved proposal can be executed more than once because `execute` does not reject an already-executed proposal (the `executed` flag is written but never gated on), and that re-execution replays the stored action to the treasury's loss.

FAIL:
- Only that "only the admin can execute" (centralization/liveness). That is the admin acting within its stated role and is not this flaw; do not count it as the planted issue.
- Only that `execute` trusts the passed accounts, without the double-execution point. (The instruction's target and metas come from the stored proposal, so this alone is weaker.)
- Generic "add replay protection" with no reference to the missing `executed` check or threshold staying satisfied.
- Other issues in this program do not count as a HIT: admin-only `execute` as a single point of failure or censorship, `expires_at` never being checked in `approve` or `execute`, `execute` forwarding whatever `remaining_accounts` it is given, `initialize` accepting duplicate members, and proposals having no cancel or close.
