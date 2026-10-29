# group_buy deployment

- The operator onboards retailers as merchants. Each merchant calls `register_merchant` once and creates deals from the operator's dashboard.
- Deals typically offer 300 to 2,000 units of one product, with a minimum of about half the units, a per-buyer limit of 1 to 5 units, a USDC unit price of 5 to 50, and a deadline 3 to 14 days ahead.
- Buyers use the operator's web app. The first time a buyer opens a deal page the app sends `join_deal`, which creates the buyer's `Commitment` account (the buyer pays its rent); paying sends `commit`. The app sends `refund` when a buyer asks for their money back on a failed or cancelled deal.
- After `claim_proceeds`, the operator's fulfilment service lists the deal's `Commitment` accounts (`getProgramAccounts` filtered by the deal key) and ships each buyer's units.
