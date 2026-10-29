# Dependency independency

## Rule
Keep external dependencies few, and give each one an exit:
- the exit must not need a redeploy or the dependency's cooperation;
- users must be able to get their funds back while the dependency is down;
- the switch is itself a power, so it needs controls that match what it can redirect, such as a multisig and a timelock.

The test for each dependency is whether the product would still make sense without it. If yes, it is a replaceable part and needs an exit. If the product exists only because of it, there is nothing to plan.

## Why Solana forces it
Programs name other programs, markets and feeds by address, and those addresses are often constants compiled into the binary. Changing a constant takes a program upgrade, so the exit depends on the upgrade authority being alive and willing. A failing CPI aborts the whole transaction. When a dependency is paused, drained or upgraded to a new interface, every instruction that calls it fails, withdrawals included. A dependency's own upgrade authority can also change its behaviour under you.

## Code signals
- **Hardcoded ids:** `pubkey!("...")` or `const ...: Pubkey` for an external program, market or feed; `address = other_crate::ID`; `Program<'info, External>`; feed ids as `[u8; 32]` constants.
- **Configurable ids:** oracle, market or program fields in a config account, with setters such as `set_oracle`, `update_market` or `set_program`. Check who signs and whether a delay applies.
- **Exits through a dependency:** `withdraw`, `redeem`, `close` or `unwind` that CPI into an external protocol, with no path that skips it.
- **Modes:** `paused`, `reduce_only`, `withdraw_only`, `emergency_withdraw`. Check that they bypass the dependency, not only block new activity.
- **Keys held by others:** an oracle signer, a bridge guardian set, a sequencer or matcher key in config, with no instruction to rotate it.
- **Upgrade authority:** in deployment notes or `solana program show` output. It may be a single key, a multisig (for example Squads), a timelock, or none, which makes the program immutable.

## Auditor questions
1. List every program the code calls, every feed, every off-chain signer or service, every layer or bridge, and the upgrade authority of this program and of its key dependencies.
2. For each one, would the product make sense without it? Which ones are replaceable parts?
3. For each replaceable part, is it a constant or a config field? If it is a constant, the only exit is an upgrade.
4. If it stops, is paused or is drained, which instructions fail? Can users still withdraw, repay or close?
5. Who can switch it, and how fast? Could the holder of that switch point the protocol at a feed or program they control, and what could they take?
6. Who holds the upgrade authority, and what would its compromise allow?

## How to confirm
Simulate the failure in a test (see `exploit-tests.md`). Leave the external program out of LiteSVM, load one that always fails, or freeze the feed's `publish_time`. Then call each user exit and list which ones fail. For a constant id, cite the line and show that no instruction can change it. For a switch without delay, show the admin repointing the feed and the very next transaction using the new value.

## Not a bug if
- The product exists only because of the dependency and says so, such as a booster built on one exchange.
- The id belongs to the platform: System, SPL Token, Token-2022, Associated Token, the Ed25519 program, sysvars.
- The dependency is a config field behind a multisig or timelock, and user exits do not go through it.
- A failure of the dependency only pauses new activity, while withdrawals stay open.
- The program is immutable on purpose, and all its external ids are platform programs.

## Real-world cases
- **Done right.** A perpetuals exchange keeps each market's oracle in its settings. Pointing a market at a different feed is an admin instruction, not a redeploy. The same setting is an attack path for whoever holds the admin key, so it belongs behind a multisig and a delay.
- **Done right.** A casino runs rounds on an ephemeral rollup but keeps every deposit on Solana. The rollup holds one round at a time and never holds funds, so leaving it is part of every round.
- **Done right.** A yield booster built on top of one exchange has nothing to plan. Without that exchange it would have no reason to exist.
- **Failure.** A delta-neutral stablecoin kept its hedge on one perpetuals venue, with no way to move it. When that venue was exploited, closing the product was the only option left.

## Severity
High when user funds can be recovered only through a dependency that has no exit, so its failure freezes them for good. Medium when the exit needs a redeploy by a live upgrade authority, or when a failure halts the protocol while funds stay withdrawable. A trusted admin's power to switch a dependency without delay is a design finding at Medium at most. It is High or Critical only when the switch reaches beyond its stated purpose, or when untrusted users can use it. Low when a hardcoded id sits on no exit path.
