# points_staking deployment

- The team creates one pool per stake token with its operator wallet. That wallet is stored as the pool's `authority` and pays for the pool and vault accounts.
- Users stake through the team's web app. Each user has one `Staker` account per pool and at most one pending unstake request at a time.
- An off-chain points engine reads the `EpochClosed` events and the staker balances, and credits points for every closed epoch. Points are not stored on chain.
- A keeper service run by the team on its own server watches every pool and calls `close_epoch`, signed with the operator wallet, as soon as `epoch_length` has passed. The team tops up the keeper's fee wallet.
- Tokens moved out with `request_unstake` can be taken out with `withdraw` once the epoch in which the request was made has been closed.
- Mainnet pools use an epoch length of 7 days.
