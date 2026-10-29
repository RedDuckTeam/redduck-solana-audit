# fx_corridor deployment

- One desk is deployed per currency pair. The company treasury funds both vaults through the admin key and rebalances them with `provide_liquidity` and `withdraw_liquidity`.
- FX rates come from an external rate publisher. Every second it signs `FXQUOTE1 || base mint || quote mint || rate || published_at` with its Ed25519 key and serves the latest signed quote from a public HTTPS endpoint. The publisher's key is stored in the desk as `publisher`.
- A relay service run by the team fetches the latest signed quote every 30 seconds and submits it in a transaction made of an Ed25519 signature-verify instruction followed by `post_rate`, signed with the operator key.
- `max_rate_age` is 120 seconds and `fee_bps` is 25.
- Partner wallets build `convert` transactions for their users and set `min_out` from the desk's current rate.
- The team's monitoring uses the operator key to call `set_pause` when the publisher reports an incident.
