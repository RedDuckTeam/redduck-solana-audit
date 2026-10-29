# Action as data

## Rule
Store a pending action on-chain as data, and let anyone execute it once its conditions hold. A program PDA signs for the owner when it runs. Three properties keep this safe:
- What runs is exactly what was approved: the same program, the same accounts with the same flags, the same data. The executor chooses only what the action leaves open, such as timing or a fill price inside the limit.
- It runs at most once, never after cancel or expiry, and never on approvals that no longer count.
- Every pending action can be cancelled by its owner or expires, and the escrow and rent go back to the owner.

## Why Solana forces it
A signed transaction stays valid only while its recent blockhash does, about 150 slots. A user cannot sign an order today for someone to submit next week, and approvals cannot be gathered as signatures over several days. A durable nonce keeps one transaction alive, but it fixes every account and works once. The alternative, a server that acts with its own key, turns that server into a custodian that can stop, censor or be hacked. Storing the action in an account removes both problems. The price is that execute becomes an instruction where a stranger asks a PDA to sign.

## Code signals
- **Stored instructions:** accounts holding `program_id: Pubkey`, an account list with `is_signer` and `is_writable`, and `data: Vec<u8>`. Typical names are `VaultTransaction`, `ProposalTransaction`, `TransactionMessage`, `PendingAction` and `instructions: Vec<CompiledInstruction>`.
- **Typed pending actions:** `LimitOrder { price, amount, expiry }`, `Dca { interval, next_run }`, `StopLoss`, `Trigger`, `Schedule`, with escrowed tokens in a PDA vault.
- **Execution:** `invoke_signed(&ix, ctx.remaining_accounts, seeds)` where `ix` is built from stored bytes. In native or Pinocchio code, an `Instruction { program_id, accounts, data }` assembled from account data. Instructions named `execute`, `execute_transaction`, `fill`, `trigger`, `crank`.
- **Danger signs:**
  - account metas built from `remaining_accounts` and never compared with the stored list, or signer and writable flags copied from the caller's metas;
  - the target program taken from a passed account instead of the stored id;
  - an `amount`, recipient or `data` argument on execute;
  - no status field, or an execute that does not check `Approved` and then set `Executed`;
  - approvals kept as a bare count, with no link to the current members, threshold or a version;
  - no `cancel`, and no `expiry` or `deadline`;
  - `close = executor` on the pending account, so the caller takes the rent;
  - one PDA, such as `seeds = [b"vault"]`, that signs for every user's actions.

## Auditor questions
1. Compare what was approved with what execute sends: program id, every account key in order, signer and writable flags, data. Can the executor add, drop, reorder or swap anything?
2. Which funds can the signing PDA move? Can an action created by one user reach another user's vault?
3. Can the action run twice, after cancel, after expiry, or before its condition holds? Where is each condition read from, and can the executor supply that account?
4. If members, the threshold or the config change after approval, do the old approvals still count?
5. Can the owner cancel at any time before execution? Does every action expire? Where do the escrow and rent go on cancel and close?
6. What does the executor control: timing, partial fill size, the price inside the limit? Can any of it profit them at the owner's expense beyond the stated reward?

## How to confirm
- **Substitution:** create and approve an action, then call execute with one account replaced by the attacker's, for example their token account as the recipient, or with an extra writable account. Show it succeeds and moves value. In a test (see `exploit-tests.md`), assert the attacker's balance.
- **Replay and staleness:** use two-transaction tests. Execute twice. Cancel, then execute. Let it expire, then execute. Change the member set, then execute on the old approvals.
- **Stuck funds:** show that no instruction returns the escrow of an action whose condition will never hold.

## Not a bug if
- Execute rebuilds the instruction only from stored fields and requires each passed account to equal the stored key in order, with flags from storage and extra accounts rejected.
- The executor's freedom is part of the order: a limit order filled at any price at least as good as the limit, partial fills the order allows, a keeper tip paid from a prepaid reward.
- Status only moves forward, from pending to approved to executed or cancelled. It is checked on execute, and the account is closed afterwards.
- The signing PDA's seeds include the owner or the multisig, so an action can move only what its approvers control.
- Only a trusted admin can create actions. Their power is a design fact (see "Trusted roles" in `report-format.md`).

## Real-world cases
- **Done right.** A multisig wallet stores each proposed transaction, with its full instruction list, in its own account. Members approve one at a time. Once the threshold is reached, anyone can trigger execution, and the vault PDA signs only the stored instructions with the stored accounts. A change of members makes pending proposals stale, so they can no longer be approved.
- **Done right.** A DAO governance program keeps the instructions inside the proposal. After the vote passes and a hold-up period ends, any wallet can run them.
- **Done right.** A swap aggregator's limit orders and recurring buys are accounts holding the escrowed tokens together with a price or a schedule. Keepers fill them once the price is reached or the next interval begins, and the owner can cancel and take the escrow back.

## Severity
Critical when the executor can change the payload, substitute an account, or make the PDA sign for funds outside the action's owner. When an action that moves funds can run twice, after cancel, after expiry or on approvals that no longer count: Critical if any caller can do it, High if it needs specific timing or a pending action of the right shape. High when escrowed funds have no cancel or expiry path and can stay locked forever. Medium when an action can run before its condition with bounded loss, or the executor can take a bounded amount beyond the stated reward. Low when only the rent goes to the wrong account.
