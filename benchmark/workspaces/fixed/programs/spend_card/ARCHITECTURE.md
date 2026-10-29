# spend_card deployment

- The program is deployed on mainnet with its upgrade authority removed.
- The card issuer's operations multisig is `admin` and created the single `Config` in use. The vault holds USDC.
- The card processor is an external company. Its signing service holds the key stored in `config.processor`, set by `initialize`.
- For every cleared card purchase the processor's service calls `settle_spend` with the card's next authorization id. The funds go to the issuer's settlement account (`config.settlement`).
- Card holders call `request_withdrawal`. The processor's service watches for these requests, settles any pending authorizations for that card, stops approving new ones, and then co-signs `complete_withdrawal` together with the holder, usually within a few minutes. If it has not co-signed 48 hours after the request, the holder can call `complete_withdrawal` alone.
- The admin replaces the settlement account and the processor key with `queue_config` followed by `apply_config`; a change takes effect 72 hours after it is queued.
