# Escrow On Chain : Program

Native Solana escrow program written in Rust.
## Deployed Program

- **Network:** Devnet
- **Program ID:** `61pt94pUbXPzTxjJj8NBW3tZcdaB9FcmTwJRrLxECcWr`
- **Explorer:** https://explorer.solana.com/address/61pt94pUbXPzTxjJj8NBW3tZcdaB9FcmTwJRrLxECcWr?cluster=devnet

# Escrow Program — Design Doc

A two-party SOL escrow between a depositor and a recipient, designed using the 7-step Solana program design template.

## 1. State

One escrow account per depositor↔recipient relationship.

| Field | Type | Notes |
|---|---|---|
| `depositor` | `Pubkey` | Party that deposits and can cancel |
| `recipient` | `Pubkey` | Party that can claim |
| `amount` | `u64` | Lamports held in escrow |
| `status` | `enum { Pending, Claimed, Cancelled }` | Current lifecycle state |

Stored on-chain rather than derived, since a PDA address can't be reverse-derived back into its seeds.

## 2. Instructions

- `Initialize(amount)` — creates the escrow, deposits SOL
- `Claim` — recipient withdraws
- `Cancel` — depositor reclaims

**Out of scope:** partial claims, arbitration, time-locks, multiple recipients.

## 3. Accounts

| Instruction | Accounts |
|---|---|
| `Initialize` | depositor (signer, writable), escrow PDA (writable, to be created), system program |
| `Claim` | recipient (signer, writable), escrow PDA (writable) |
| `Cancel` | depositor (signer, writable), escrow PDA (writable) |

## 4. Validation

Each instruction is checked against 6 fixed categories, in order: Account meta flags → Identity/authorization → Ownership → Derivation → State/business logic → Arithmetic/invariants. A category is marked N/A when it doesn't apply.

**Initialize**
1. **Account meta flags:**
   - Depositor must be a signer (authorizing the deposit) and writable (their lamport balance decreases)
   - Escrow PDA account must be writable (data is being created); no signer check applies — a PDA has no private key and can't set `is_signer`, it only "signs" via `invoke_signed` using seeds

2. **Ownership:** N/A — account doesn't exist yet, nothing to own
3. **Identity/authorization:** N/A — nothing exists yet, no stored party to check against
4. **Derivation:**
   - Recompute the escrow PDA from seeds and verify it matches the account passed in
   - Verify the escrow account does not already exist (not yet owned by the program) — prevents re-initializing and overwriting existing state
   - Verify `system_program.key == SYSTEM_PROGRAM_ID` — don't trust the account's position alone
5. **State/business logic:** N/A — no prior state to compare against; nothing stored yet
6. **Arithmetic:**
   - Verify `amount > 0`
   - Verify depositor has enough lamports to fund `amount` + rent

**Claim**
1. **Account meta flags:**
   - Recipient must be a signer (authorizing the claim) and writable (their lamport balance increases)
   - Escrow account must be writable (data/lamports change on close-out)
2. **Ownership:** verify `escrow_account.owner == program_id`
3. **Identity/authorization:** verify signer == `stored.recipient`
4. **Derivation:**
   - Recompute PDA from stored `depositor`/`recipient`
   - Verify it matches `escrow_account.key`
5. **State/business logic:** verify `status == Pending`
6. **Arithmetic:** N/A — moves the stored `amount` in full, no calculation performed

**Cancel**
1. **Account meta flags:**
   - Depositor must be a signer (authorizing the cancel) and writable (their lamport balance increases, refund)
   - Escrow account must be writable (data/lamports change on close-out)
2. **Ownership:** verify `escrow_account.owner == program_id`
3. **Identity/authorization:** verify signer == `stored.depositor`
4. **Derivation:**
   - Recompute PDA from stored `depositor`/`recipient`
   - Verify it matches `escrow_account.key`
5. **State/business logic:** verify `status == Pending`
6. **Arithmetic:** N/A — moves the stored `amount` in full, no calculation performed

## 5. PDA Design

Per-relationship seed shape:

```
["escrow", depositor.key(), recipient.key()]
```

## 6. State Transitions

- `Pending → Claimed` — gated by `Claim`, recipient-only — **terminal**
- `Pending → Cancelled` — gated by `Cancel`, depositor-only — **terminal**
- No transitions out of `Claimed` or `Cancelled` — both are end states, no reversibility

## 7. CPIs

- **Initialize:** `invoke_signed` into System Program's `create_account` (PDA has no keypair, so the program signs on its behalf via seeds + bump)
- **Claim / Cancel:** no CPI — direct lamport debit/credit on accounts already owned by the program
