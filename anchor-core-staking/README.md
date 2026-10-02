# Solana Metaplex Core NFT Staking Program

A non-custodial NFT staking program built on Solana using the Anchor framework, Metaplex Core standard (`mpl-core`), and SPL Token interfaces. The program leverages Metaplex Core's plugin architecture to lock NFTs directly within user wallets while streaming proportional rewards, tracking live collection-wide staking counters, and supporting a permanent burn mechanism for a 3x reward multiplier.

---

## Overview

The Metaplex Core Staking program enables collection-based NFT staking without taking custody of user assets:

1. **Non-Custodial Architecture**: Assets remain in the owner's wallet at all times. Staking locks transferability on-chain via Metaplex Core's `FreezeDelegate` plugin and records staking timestamps via the `Attributes` plugin.
2. **Collection-Level Metrics**: Dynamically tracks a `total_staked` counter directly in the parent collection's metadata attributes plugin.
3. **Configurable Reward Distribution**: Yields fungible SPL reward tokens calculated per full 24-hour period based on basis points (`rewards_bps`).
4. **Lockup Enforcement**: Enforces a configurable freeze period (`freeze_period` in days) before assets can be unstaked or burned.
5. **3x Burn Reward Multiplier**: Allows stakers to permanently burn their staked NFT to claim three times (3x) their accrued staking yield.
6. **Restake Resilience**: Enables seamless re-staking of previously unstaked assets without plugin collision.

---

## Program Details

- **Program ID**: `FkP21JjeMc9gcq9iBQBHfLocaLsxCYGLg4jgwin6pBwG`
- **Framework**: Anchor 0.30.1 / Rust
- **NFT Standard**: Metaplex Core (`mpl-core` v1.5.0)
- **Token Standard**: SPL Token / Token Interface (`anchor-spl`)

---

## Metaplex Core Plugin Architecture

Instead of moving assets into escrow vaults, staking is managed through authority-controlled plugins on the Metaplex Core asset:

| Plugin | Authority | Purpose |
| :--- | :--- | :--- |
| `FreezeDelegate` | `UpdateAuthority` (PDA) | Set to `frozen: true` on stake to prevent asset transfers; set to `frozen: false` on unstake/burn. |
| `Attributes` (Asset) | `UpdateAuthority` (PDA) | Stores staking state (`staked: "true" \| "false"`), initial stake timestamp (`staked_at`), and reward cursor (`last_claimed_at`). |
| `Attributes` (Collection) | `UpdateAuthority` (PDA) | Maintains the global collection counter (`total_staked`), incremented (+1) on stake and decremented (-1) on unstake or burn. |

---

## Account Architecture & State

### 1. Config Account (`Config`)

Stores pool parameters and bump seeds for a specific NFT collection.

| Field | Type | Description |
| :--- | :--- | :--- |
| `rewards_bps` | `u16` | Reward rate basis points per day (e.g., 10000 bps = 1.00 token/day) |
| `freeze_period` | `u16` | Minimum lockup duration required before unstaking or burning (in days) |
| `rewards_bump` | `u8` | Canonical bump seed for the `rewards_mint` PDA |
| `bump` | `u8` | Canonical bump seed for the `Config` account PDA |

### 2. PDA Derivations

- **Update Authority PDA**:
  ```text
  seeds = [b"update_authority", collection_pubkey.as_ref()]
  ```
- **Config PDA**:
  ```text
  seeds = [b"config", collection_pubkey.as_ref()]
  ```
- **Rewards Mint PDA**:
  ```text
  seeds = [b"rewards_mint", config_pda.as_ref()]
  ```

---

## Reward Calculations

Staking rewards accrue on a daily basis ($86,400\text{ seconds}$) without intermediate rounding drift. 

1. **Elapsed Days**:
   $$\Delta t_{\text{days}} = \left\lfloor \frac{t_{\text{current}} - t_{\text{cursor}}}{86,400} \right\rfloor$$
2. **Standard Reward Amount**:
   $$\text{Rewards} = \frac{\Delta t_{\text{days}} \cdot \text{rewards\_bps} \cdot 10^{\text{decimals}}}{10,000}$$
3. **Burn Reward Amount (3x Multiplier)**:
   $$\text{Burn Rewards} = \text{Rewards} \cdot 3$$

---

## Instructions

### 1. `create_collection`

Creates a new Metaplex Core collection configured with the program's `update_authority` PDA as its update authority.

- **Parameters**: `name: String`, `uri: String`
- **Signers**: `payer`, `collection` (Keypair)

### 2. `mint_asset`

Mints a new Metaplex Core NFT directly into the user's wallet as a verified member of the collection.

- **Parameters**: `name: String`, `uri: String`
- **Signers**: `user`, `asset` (Keypair)

### 3. `initialize`

Initializes the staking pool configuration for a collection, derives the `rewards_mint` SPL token mint, and initializes the collection's `total_staked` attribute counter to `0`.

- **Parameters**: `rewards_bps: u16`, `freeze_period: u16`
- **Signer**: `admin`

### 4. `stake`

Executes non-custodial staking on a Metaplex Core asset:
1. Validates ownership and collection association.
2. Attaches or updates the `Attributes` plugin with `staked = "true"`, `staked_at = current_timestamp`, and `last_claimed_at = "0"`.
3. Activates the `FreezeDelegate` plugin (`frozen: true`) signed by the `update_authority` PDA.
4. Increments `total_staked` on the collection attributes plugin (+1).

- **Signer**: Asset Owner

### 5. `claim_rewards`

Calculates accrued rewards since `last_claimed_at` (or `staked_at` if first claim), advances the `last_claimed_at` timestamp cursor to prevent double claims, and mints reward tokens to the user's ATA.

- **Signer**: Asset Owner

### 6. `unstake`

Unlocks and unstakes the asset:
1. Enforces minimum lockup period (`staked_time >= config.freeze_period`).
2. Thaws the asset (`FreezeDelegate` set to `frozen: false`).
3. Resets staking attributes (`staked = "false"`).
4. Decrements `total_staked` on the collection attributes plugin (-1).
5. Mints any remaining accrued reward tokens to the user's ATA.

- **Signer**: Asset Owner

### 7. `burn_staked_nft`

Permanently destroys the staked NFT in exchange for maximum yield:
1. Enforces minimum lockup period (`staked_time >= config.freeze_period`).
2. Thaws the asset to permit deletion.
3. Invokes Metaplex Core `BurnV1` to permanently destroy the asset.
4. Decrements `total_staked` on the collection attributes plugin (-1).
5. Mints accrued rewards with a **3x multiplier** directly to the user's ATA.

- **Signer**: Asset Owner

---

## Project Structure

```text
anchor-core-staking/
├── Anchor.toml
├── Cargo.toml
├── package.json
├── programs/
│   └── anchor-core-staking/
│       ├── Cargo.toml
│       ├── src/
│       │   ├── lib.rs                      # Program entrypoint & instruction routes
│       │   ├── constants.rs                # Seed constants
│       │   ├── error.rs                    # Custom program error codes
│       │   ├── state/
│       │   │   ├── mod.rs
│       │   │   └── config.rs               # Staking pool Config struct
│       │   └── instructions/
│       │       ├── mod.rs
│       │       ├── initialize.rs           # Pool config & rewards mint setup
│       │       ├── create_collection.rs    # Metaplex Core collection creation
│       │       ├── mint_asset.rs           # Core asset minting
│       │       ├── stake.rs                # Non-custodial freeze & attribute staking
│       │       ├── unstake.rs              # Lockup verification, thaw & claim
│       │       ├── claim_rewards.rs        # Mid-stake reward streaming
│       │       ├── burn_staked_nft.rs      # Asset burn & 3x reward multiplier
│       │       └── update_total_staked.rs  # Collection counter update CPI helper
└── tests/
    └── anchor-core-staking.ts              # End-to-end integration test suite
└── proof/
    └── image.png                           # Test suite execution proof
```

---

## Building and Testing

### Prerequisites

- Rust `1.75.0+`
- Solana CLI `1.18+`
- Anchor CLI `0.30.1`
- Node.js `v18+` / Yarn

### Build

```bash
anchor build
```

### Test

Execute the complete end-to-end integration suite against a local validator or Surfpool environment:

```bash
anchor test
```

### Test Suite Flow

1. `Create a collection`: Generates collection with program update authority PDA.
2. `Mint an NFT`: Mints Metaplex Core asset to user wallet.
3. `Initialize Config`: Deploys pool configuration and rewards mint (`total_staked = 0`).
4. `Stake an NFT`: Freezes asset, writes staking attributes (`total_staked = 1`).
5. `Immediate Claim Rejection`: Verifies 0 days elapsed rejects zero-reward claim.
6. `Early Unstake Rejection`: Verifies unstake attempt prior to freeze period fails.
7. `Early Burn Rejection`: Verifies burn attempt prior to freeze period fails.
8. `Time Travel & Mid-Stake Claim`: Advances time past freeze period, validates exact token payout without changing staked count.
9. `Double-Claim Rejection`: Confirms reward cursor prevented repeat claims.
10. `Incremental Claim & Unstake`: Validates remainder reward distribution and thaw (`total_staked = 0`).
11. `Restake Verification`: Confirms asset can be restaked without plugin conflicts (`total_staked = 1`).
12. `Burn for 3x Rewards`: Burns restaked asset, decrements collection counter, and verifies 3x multiplier payout.

---

## Execution & Test Proof

Integration test execution validating all lifecycle instructions, error constraints, and reward multipliers:

![Anchor Core Staking Test Proof](./proof/image.png)
