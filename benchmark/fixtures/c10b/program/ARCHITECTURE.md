# fx_corridor deployment

- One desk is deployed per currency pair. The company treasury funds both vaults through the admin key and rebalances them with `provide_liquidity` and `withdraw_liquidity`.
- FX rates come from an external rate publisher. Every second it signs `FXQUOTE1 || base mint || quote mint || rate || published_at` with its Ed25519 key and serves the latest signed quote from a public HTTPS endpoint. The publisher's key is stored in the desk as `publisher`.
- `max_rate_age` is 15 seconds and `fee_bps` is 25.
- Partner wallets fetch the latest signed quote from the publisher's endpoint and build each user's transaction as an Ed25519 signature-verify instruction, `post_rate` and then `convert`, with `min_out` set from that quote. The team runs no rate relay.
- The team's monitoring uses the operator key to call `set_pause` when the publisher reports an incident.
