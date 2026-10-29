# Virtual execution

## Rule
When an action's full effect cannot be carried out in its own transaction, record the result as an obligation now and settle it later in a permissionless step:
- The record is final when the action happens. The amount and the owner are fixed then.
- Settlement pays each record once, only to its recorded owner, and anyone can run it.
- Until settlement, the recorded value cannot be spent twice or lost, and the protocol holds enough to pay every open record.

## Why Solana forces it
An instruction can touch only the accounts its transaction lists. A taker that matches ten resting orders would need ten makers' token accounts, which the taker may not know and which may not fit in one transaction. Some follow-up work, such as rewriting a merkle tree, is too expensive to repeat on every action. So the action writes what is owed into state it already has. The transfer or update happens later, when someone brings the missing accounts. Bugs live in the gap between the two steps.

## Code signals
- **Obligation fields:** `owed`, `unsettled`, `claimable`, `pending_*`, `base_free` / `quote_free`, `filled`, `settled`, `credit` or `fees_accrued` on an order, position or open-orders account.
- **Queues:** `EventQueue`, `FillEvent`, `OutEvent`, a ring buffer with `head` and `count`, a nullifier or spend queue processed in batches.
- **Settlement instructions:** `settle`, `settle_funds`, `consume_events`, `crank`, `claim`, `process_queue`, `withdraw_unsettled`.
- **Danger signs:**
  - a settle that does not clear or mark the record (`owed = 0`, `settled = true`) in the same instruction that pays;
  - a destination token account checked only for its mint, not with `token::authority = record.owner`, an ATA derivation, or `owner == record.owner` after `unpack` in native code;
  - a record with no owner field, so the payee is whatever account is passed;
  - settlement gated to an operator (`has_one = crank_authority`);
  - an order or position account that can be closed while something is owed, or reopened with `init_if_needed`;
  - owed amounts counted both as a free balance and as collateral, or withdrawable before settlement;
  - vault accounting that ignores open records;
  - a fixed-size queue that only an operator can drain.

## Auditor questions
1. For each record, who is owed and where is that stored? Does settlement check the destination against it?
2. Can a record be settled twice? Is it cleared in the same instruction that pays, and can it be recreated with the old value?
3. Who can settle? If only an operator, what stays stuck when it stops? If anyone, can the caller pick the destination or take more than a stated fee?
4. Between the action and settlement, can the recorded value be spent, withdrawn or counted twice? Is a spent input blocked at once, not only after the queue is processed?
5. Is the protocol solvent while records are open? Does the vault hold at least the sum of what is owed, with rounding against the user?
6. Can the queue or the record storage fill up? What happens to new actions then, and who can drain it?
7. Can an owner's account be closed, or the record overwritten, while something is still owed to it?

## How to confirm
- **Double or redirected settlement:** run the action, then settle twice, or settle with the attacker's token account as the destination. In a test (see `exploit-tests.md`), compare balances after each step.
- **Spend before settle:** run the action, then spend the same input again before the queue is processed, and show the second spend succeeds.
- **Stuck queue:** fill the queue with the smallest valid actions and show that new actions fail while only an operator can drain it.
- **Insolvency:** after a sequence of actions with fees and rounding, sum the open records and compare the total with the vault balance.

## Not a bug if
- The record stores the owner, settlement constrains the destination to that owner, and the record is cleared or closed in the same instruction.
- Anyone can settle, and the caller gets at most a fixed reward.
- An unsettled balance counts as the owner's in other checks by design, because the vault already holds those tokens and they cannot leave twice.
- A full queue only makes new actions fail until somebody drains it, and draining is permissionless and paid.

## Real-world cases
- **Done right.** A betting exchange matches a taker against resting orders without the makers' token accounts in the transaction. The match writes what each maker is owed. A later settle, which anyone can call, pays the maker once their token account is supplied and clears the record.
- **Done right.** A central limit order book writes fills to an event queue. Any cranker can process the queue to credit makers' open-orders accounts, and each owner later moves the credited tokens to a wallet.
- **Done right.** A state-compression layer keeps account data as leaves under an on-chain merkle root. A spend puts the old leaf into a queue at once, so it cannot be spent again. Independent workers later write the queued changes into the tree in batches and are paid for it.

## Severity
Critical when anyone can settle a record twice or send it to an account they choose. High when records can be lost before settlement, when the protocol cannot pay all open records under realistic conditions, or when only an operator can settle and users have no other way to get paid. Medium when a full queue blocks new actions until someone drains it, or when settlement is operator-only but a permissionless fallback opens after a delay. Low when a settler can take a small amount beyond the stated reward.
