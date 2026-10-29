# spend_card deployment

- The program is deployed on mainnet with its upgrade authority removed.
- The card issuer's operations multisig is `admin` and created the single `Config` in use. The vault holds USDC.
- The card processor is an external company. Its signing service holds the key compiled into the program as `PROCESSOR`.
- For every cleared card purchase the processor's service calls `settle_spend` with the card's next authorization id. The funds go to the issuer's settlement account (`config.settlement`).
- Card holders call `request_withdrawal`. The processor's service watches for these requests, checks that the card has no pending authorizations, and then co-signs `complete_withdrawal` together with the holder, usually within a few minutes.
- The admin replaces the settlement account with `queue_config` followed by `apply_config`; a change takes effect 72 hours after it is queued.
