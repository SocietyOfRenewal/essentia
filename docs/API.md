# HTTP API

## Read endpoints

### `GET /health`
Returns `ok`.

### `GET /v1/status`
Returns node identity, chain height, latest hash, peer list, mempool size, and scheduled next proposer.

### `GET /v1/state`
Returns a full state dump.

### `GET /v1/accounts/{did}`
Returns:

- confirmed nonce,
- next usable nonce,
- liquid `ℰ`,
- locked `ℰ`,
- total `𝒰`,
- pending vendor units,
- roles,
- personhood eligibility.

### `GET /v1/blocks?from=<height>`
Returns blocks from the requested height onward.

## Write endpoints

### `POST /v1/transactions`
Body:

```json
{
  "tx": { "...": "signed transaction envelope" },
  "propagate": true
}
```

The node validates the signature, nonce, and state transition before accepting the transaction into mempool.

### `POST /v1/blocks/propose`
Triggers local proposal if the node holds the scheduled validator key for the next height.

### `POST /v1/blocks/import`
Imports a block after verifying:

- proposer schedule,
- block signature,
- previous hash,
- state root,
- every transaction in order.

### `POST /v1/peers/register`
Adds a peer URL to the local peer set.

## Transaction envelope

A transaction is signed over `(signer, tx)` where `tx` includes:

- `chain_id`,
- `nonce`,
- `created_at_unix_ms`,
- `kind`.

The node rejects any nonce that is not exactly the next nonce for that signer.

## Transaction kinds

- `register_did`
- `issue_personhood`
- `issue_role`
- `create_epoch_budget`
- `create_purpose`
- `create_quest`
- `submit_claim`
- `submit_review`
- `challenge_claim`
- `resolve_challenge`
- `finalize_claim`
- `transfer_essent`
- `issue_freedom_floor`
- `spend_essential_units`
- `redeem_vendor_settlement`
- `create_proposal`
- `cast_vote`
- `tally_proposal`
