# One-way state

## Rule
Design so that order and repetition do not matter:
- Messages and claims carry the destination, such as a cumulative total or the latest version, not a step such as "add 5".
- The program accepts only moves that go forward or narrow rights, and judges each message against current state, never against another message.
- Any two valid transactions, swapped or repeated, must end in the same state.

## Why Solana forces it
Solana has no per-sender nonce. Transactions sent together can land in either order, and a signed message can be submitted again by anyone who saw it. Replicating EVM-style nonces or message ids costs an account per message and still does not make deltas safe: applying "+5" twice is not applying it once.

## Code signals
- Instructions that take an amount to **add** or **subtract** from an off-chain signer, oracle or admin (`credit`, `reward`, `add_points`, `settle_delta`) instead of a cumulative total.
- Signed payloads without an expiry, a monotonic version or a link to current state. Ed25519 messages parsed and then applied as deltas.
- Counters, statuses or versions that can be **set** rather than only increased. Instructions that reopen, re-initialize or roll back a record. `init_if_needed` used to reset per-user state.
- `paid` or `claimed` fields that can decrease, or are compared with `==` instead of `>=`.
- Approvals or permissions that can widen after a change of signers, members or config, while older approvals remain valid.
- Status machines where a later step can run before an earlier one and leave a different result.

## Auditor questions
1. Take every message or instruction that changes balances or rights. If it lands twice, what happens? If two of them land in the other order?
2. Does the payload say where state should end up (a total, a version), or by how much it should move?
3. Can any field that represents progress (paid amount, version, epoch, status) go backwards? Through which instruction?
4. When the set of signers or the config changes, do previously collected approvals stop counting?
5. Is "already processed" derived from state, such as the total already paid or the existence of a PDA, rather than from a list the program must keep growing?

## How to confirm
Write the two-transaction test: submit the same valid message twice, or two valid messages in both orders, and compare the final balances. A delta-based claim pays twice, and an older message applied after a newer one rolls state back. In a test (see `exploit-tests.md`), replay the exact signed instruction in a second transaction.

## Not a bug if
- The payload is a cumulative total and the program pays `total - paid` only when `total > paid`, then sets `paid = total`.
- A version or sequence is stored and the program requires `new > stored`, or treats `<=` as a no-op.
- Replays are blocked because the message consumes a PDA (`init` on a receipt account keyed by message id) and that account cannot be closed and recreated.
- Rights only narrow: reduce-only modes, revocations, a stale-proposal counter.

## Real-world cases
- **Done right.** A network of devices gets rewards from an oracle that signs the total each device has earned since the beginning. The device account stores how much was paid. A claim pays the difference and raises the stored amount, so replaying a claim pays zero, and an older, smaller total pays nothing.
- **Done right.** A multisig marks every older proposal as stale with a single counter when the member set changes. Stale proposals can be cancelled but not approved.
- **Done right.** A perpetuals venue switches expiring markets to reduce-only, so positions can shrink but not grow.

## Severity
Critical when replay or reordering creates or drains funds, such as a double claim or credits applied twice. High when it rolls back state that other users rely on. Medium when it only lets a user redo an action they were entitled to with bounded extra effect.
