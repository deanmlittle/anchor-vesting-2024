# Anchor Vesting 2024
A simple vesting contact that allows you to set up scheduled unlocks of tokens.

# Usage
1) Initialize a `Config` with the `initialize` function. This means defining a mint, a recovery address and an admin account.
2) Create all the vests required with the admin account using `createVesting`. You may also need to initialize the target token account. We do not initialize token accounts in the contract as this allows us to also support PDA-owned token accounts.
3) Cancel any vests you have erroneously allocated with `cancelVesting`
4) When the vest is properly configured, call `finalize`. This will lock the vest, disallowing any further creation or cancellation of vesting accounts.
5) Transfer the tokens from your multisig or similar into the tken vault of the contract. This should likely be the same recovery address defined in step 1.
6) Users may now begin to claim their vests as they mature.
7) If you accidentally overallocate tokens to the contract, it will allow any key to withdraw tokens in excess of outstanding vesting amounts to the recovery address from step 1 at any time using `withdrawSurplus`

See tests for more information.