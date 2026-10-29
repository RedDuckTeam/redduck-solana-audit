# Identity rejection

## Rule
Guard the protocol by checking what a transaction does to state, not who sent it. Only two kinds of identity checks belong in a program:
- ownership: the owner signs to move their own assets;
- a stated authority: the admin signs to change a parameter.

Everything else that looks at the caller (allow lists, deny lists, one-per-wallet limits, "only our bot", "no programs") is a guard an attacker walks around with a new key. A guard on an action must also cover every path that performs that action, not one instruction name.

## Why Solana forces it
Keypairs cost nothing, and one wallet can fund a hundred new ones in a single transaction. A transaction can hold many instructions, and an attacker's own program can call yours through CPI. The instructions sysvar lists only top-level instructions, so a check that scans it never sees calls made through CPI. Limits keyed to a wallet, or to one instruction's discriminator, measure the wrong thing.

## Code signals
- **Lists:** `whitelist`, `blacklist`, `allowlist`, `blocked`, `is_bot` fields; `config.list.contains(&signer.key())`; a `Vec<Pubkey>` in a config account that an admin edits.
- **Per-wallet limits as protection:** `max_per_wallet`, `user.bought + amount <= cap`, a PDA seeded by `[b"user", signer.key()]` whose `init` is the only thing stopping a second purchase or mint.
- **Caller-type rules:** `get_stack_height() == TRANSACTION_LEVEL_STACK_HEIGHT` to forbid CPI, checks that the signer is a system-owned wallet, `fee_payer == config.bot`.
- **Introspection rate limits:** loops over `load_instruction_at_checked` comparing `ix.program_id == crate::ID` and `ix.data[..8]` with one discriminator, such as "only one `swap` per transaction".
- **Sibling instructions:** `swap` and `swap_v2`, `buy` and `buy_exact_in`, `deposit` and `deposit_with_referral`, `withdraw` and `withdraw_all`. Check that each one calls the same guard.
- **Launch protection names:** `anti_bot`, `sniper`, `launch_ts`, `cooldown`, `fee_scheduler`, `rate_limiter`.

## Auditor questions
1. Which checks look at the caller's key rather than at amounts, time or state? For each, what does an attacker gain with ten fresh keypairs?
2. What invariant is the check really protecting (amount bought per slot, swaps per pool, price impact, total minted)? Could the program check that invariant on state directly?
3. List every instruction, old and new, that performs the guarded action. Does each one pass through the same guard? Can the action also be reached through CPI?
4. If the guard reads the instructions sysvar, does it count every instruction that changes the guarded state, or only one name? What does it do with calls that arrive through CPI?
5. Are the identity checks that remain limited to ownership and stated authority?

## How to confirm
- **List or per-wallet guard:** split the action across several fresh keypairs funded from one wallet and show the combined result exceeds what the guard allows. In a test (see `exploit-tests.md`), create and fund the keypairs in LiteSVM or bankrun, run the action from each, and compare the total with the cap.
- **Path gap:** send the transaction the guard should reject, using the unguarded sibling instruction or a CPI from a small test program. Show it succeeds and count the effect, for example swaps on one pool in one transaction.

## Not a bug if
- The identity check is ownership (`has_one = owner` on a withdrawal) or a stated role (`has_one = admin` on `set_fee`).
- The limit is tied to something that cannot be multiplied: a merkle leaf fixed before the launch, a stake, a held NFT, an attested identity.
- The guard measures state and lives in the shared logic every path calls, for example a per-slot counter in the pool account updated inside the common swap function.
- The list is a documented compliance requirement, such as a permissioned pool, and the design never claims it stops bots or sybils.

## Real-world cases
- **Done right.** Launchpad DEXes fight snipers with fees, not lists. One charges a launch fee that starts close to the whole trade and decays over the first seconds or minutes. Another adds a fee that rises with the size of a single buy. Elapsed time and trade size are the same for every wallet. The trade-off is that honest early buyers pay too.
- **Done right.** A pull oracle accepts a price update from any wallet. It checks the publisher's signature and the update's age, and ignores who submitted it.
- **Failure.** A bonding-curve launch program allowed one swap per transaction by scanning for a single swap discriminator. A second swap instruction, added later, was not in the scan. A transaction could hold one of the first and many of the second, so the limiter did nothing. An audit caught it before anyone lost funds.

## Severity
Rate the impact of bypassing what the guard protects.
- Critical when the bypass lets any user drain or mint, for example a per-wallet cap that was the only bound on a subsidy paid from a vault.
- High when it defeats a protection of user funds under realistic conditions, such as a launch limiter that a sniper bypasses to buy supply ahead of everyone.
- Medium when the result is unfairness or bounded loss. A design that depends on an admin keeping a list current is a design finding, Medium at most.
- Low when the identity check is only defence in depth behind a state check that already holds.
