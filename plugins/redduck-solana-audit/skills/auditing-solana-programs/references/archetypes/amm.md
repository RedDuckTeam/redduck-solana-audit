# AMM and CLMM pools

This file is not a pattern and has no row in the coverage table. It lists the invariants a constant-product or concentrated-liquidity pool must hold, so that the pattern questions have concrete targets. Report a broken invariant under the pattern or checklist item named next to it, and cite it by its id, such as CP-3.

## When to read
The program holds two token reserves and prices swaps from them: `reserve_a` / `reserve_b`, an LP or position mint, `swap` / `add_liquidity` / `remove_liquidity`, or `sqrt_price`, `tick_current`, `tick_spacing`, tick arrays.

## Constant-product invariants
- **CP-1. k never falls.** After every swap, in either direction and for exact-in and exact-out, `new_x * new_y >= old_x * old_y`, computed in `u128`. The fee stays in the reserves, so k grows. *(defensive-computation)*
- **CP-2. Rounding favours the pool.** For exact-in, the fee is taken from the input and the output is floored: `floor(y * dx * (1 - fee) / (x + dx * (1 - fee)))`. For exact-out, the required input rounds up. *(defensive-computation)*
- **CP-3. LP mint uses the smaller ratio.** Later deposits mint `floor(min(dx / x, dy / y) * supply)`, and the excess of the larger side is not pulled or is refunded. Using one side, `max` or an average mints LP for free. *(defensive-computation)*
- **CP-4. The first deposit cannot inflate the share price.** It mints `sqrt(dx * dy) - MINIMUM_LIQUIDITY`, and the minimum is burned or held by a PDA with no spend path, or stored reserves ignore donations. *(internal-oracle)*
- **CP-5. Withdrawals pay pro rata, floored.** `floor(lp * reserve / supply)` per side. LP supply changes only through add, remove and a documented protocol-fee mint computed from the growth of k. *(defensive-computation)*
- **CP-6. Stored reserves match what the vaults received.** Every path updates them by the measured amount, which matters for Token-2022 transfer fees on either leg. *(checklist-token-2022, internal-oracle)*
- **CP-7. Each pool has its own vaults.** Pool, vaults and LP mint are PDAs seeded by both mints in canonical order (`require!(mint_a < mint_b)`) plus any config or fee tier, the two mints differ, and each vault is bound by `address = pool.vault_a` or a per-pool authority. *(checklist-duplicate-accounts, checklist-seed-collisions)*
- **CP-8. Every user action has a bound.** Swap takes a minimum out, add takes maximum amounts in, remove takes minimum amounts out. *(boundary-of-consent)*

## Concentrated-liquidity invariants
- **CL-1. Ticks are valid.** For every position, `tick_lower < tick_upper`, both are multiples of `tick_spacing`, and both lie within `[MIN_TICK, MAX_TICK]`. A position on ticks the swap loop never crosses keeps its liquidity active after the price leaves its range. *(defensive-computation)*
- **CL-2. Fee tier and spacing are part of the pool's identity.** Pool seeds include the fee tier or tick spacing along with the ordered mints. *(checklist-seed-collisions)*
- **CL-3. Deposits match the range.** A range above the current price takes only token 0, a range below takes only token 1, and amounts round up on deposit and down on withdrawal. *(defensive-computation)*
- **CL-4. The position NFT is the authority.** When positions have an NFT, whoever holds it controls the position. A stored owner that does not follow the NFT lets a former owner collect fees or withdraw. *(root-of-trust Q6)*
- **CL-5. Fee growth is snapshotted at entry.** Opening or increasing a position stores the current fee growth inside the range, so fees from before entry cannot be claimed. *(lazy-loading Q2, Q3)*
- **CL-6. Fee growth wraps on purpose.** Global and per-tick fee growth are modular accumulators, and fee growth inside a range and fees owed are computed with `wrapping_sub`. That is not an arithmetic finding. `checked_sub` there is the bug, because it can fail a valid position. *(defensive-computation)*
- **CL-7. Tick arrays belong to the pool.** Arrays passed for a swap are bound to this pool and checked for order. *(complexity-asymmetry Q4)*
- **CL-8. Prices for other protocols come from a TWAP.** The TWAP is the difference of two cumulative tick observations over elapsed time, with a window the observation buffer can hold. Spot `sqrt_price` is movable in one transaction, and the orientation flips when the mint order flips. *(internal-oracle, defensive-computation)*

## Severity
- Critical when a broken invariant lets any user take reserves: k falls on some path, LP is minted from one side, a position is controlled by a former owner, or vaults can be swapped between pools.
- High when liquidity on invalid ticks distorts swaps against honest LPs, or when integrators can value positions at a movable spot price.
- Medium when the first-deposit inflation needs a large donation to hurt the next depositor.
- Several pools for one pair are a design choice: Info, or Low when an integrator in scope trusts a pool by its mint pair alone.
- Low for rounding a user cannot repeat for profit.
