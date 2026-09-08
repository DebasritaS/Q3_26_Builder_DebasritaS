# Week1 Assignment on SPL Token and NFT

## Cloned From https://github.com/ShrinathNR/spl-nft-q326/tree/main

> Before running the scripts, go through these docs:
> - [Solana token docs](https://solana.com/docs/tokens) — mint accounts, token accounts, and ATAs
> - [Solana Kit](https://www.solanakit.com/) — the JS SDK used for building and sending transactions
> - [Metaplex Token Metadata](https://www.metaplex.com/docs/smart-contracts/token-metadata) — attaching metadata to SPL tokens
> - [Metaplex Core](https://www.metaplex.com/docs/smart-contracts/core) — the NFT standard used in the NFT scripts

---

## 📁 Project Structure

```text
├── assets/                       # Proof screenshots & media assets
│   ├── deb-nft.json              # SPL metadata JSON
│   ├── deb-nft.png               # SPL image PNG
│   ├── generug.png               # NFT source image for NFT 1   "Debrug" after updation "De rug"
│   ├── generug2.png              # NFT source image for NFT 2   "Purple Dream" which is later transferred 
│   └── generug3.png              # NFT source image for NFT 3   "Picnic" which is later burned
│
├── src/
│   ├── spl/                      # SPL Token scripts 
│   │   ├── spl_init.ts           # Initialize token mint account
│   │   ├── spl_metadata.ts       # Attach token metadata 
│   │   ├── spl_mint.ts           # Create ATA and mint supply (7 tokens minted)
│   │   └── spl_transfer.ts       # Transfer tokens (2 tokens transferred)
│   │
│   ├── nft/                      # NFT scripts 
│   │   ├── nft_image.ts          # Upload image asset to Irys (All NFT 1,2 & 3)
│   │   ├── nft_metadata.ts       # Upload metadata JSON to Irys (All NFT 1,2 & 3)
│   │   ├── nft_mint.ts           # Mint Metaplex Core NFT (All NFT 1,2 & 3)
│   │   │
│   │   ├── nft_update.ts         # Update metadata & authority (NFT 1 "Debrug" to "De rug" and metadata)
│   │   ├── nft_transfer.ts       # Transfer NFT ownership (NFT 2 "Purple Dream")
│   │   └── nft_burn.ts           # Burn NFT & reclaim rent (NFT 3 "Picnic")
│   │
│   │
│   └── Proof/
│       └── Images                # Images to prove all the transactions
│
├── package.json
└── tsconfig.json
```

---

## Steps I had done

I used My wallet from my laptop

### SPL (Assignment1 Task-1)
1. ran spl_int.ts
2. ran spl_metadata.ts deb-nft.png & deb-nft.json
3. ran spl_mint.ts  created 7 DBS tokens
4. ran spl_transfer.ts transferred 2 tokens to another wallet

### NFT

(Assignment1 Task-2)
1. ran nft_image.ts  For NFT #1 "Debrug" generug.png
2. ran nft_metadata.ts
3. ran nft_mint.ts Minted NFT #1 "Debrug" generug.png

(Assignment1 Task-3)
1. ran modified nft_metadata NFT #1 to "De rug" and metadata generug.png
2. ran modified nft_update.ts Updated NFT #1 to "De rug" generug.png

(Assignment1 Task-4)
1. ran modified nft_image.ts  For NFT #2 "Purple Dream" generug2.png
2. ran modified nft_metadata.ts
3. ran modified nft_mint.ts Minted NFT #2 "Purple Dream" generug2.png

(Assignment1 Task-5)
1. ran nft_transfer.ts  For NFT #2 "Purple Dream" to another account generug2.png

(Assignment1 Task-6)
1. ran modified nft_image.ts  For NFT #3 "Picnic" generug.png
2. ran modified nft_metadata.ts
3. ran modified nft_mint.ts Minted NFT #3 "Picnic" generug.png
4. ran nft_burn.ts  Burned NFT #3 "Picnic" generug.png

ALL Transaction Signatures are at the end of this file

---
### 1. Prerequisites & Installation


```bash
npm install
```

Put Your OWN Wallet address in "import wallet from "OWN_WALLET_ADDRESS";"

---
## SPL Token

Uses **@solana/kit** and **@solana-program/token** for transactions, and **mpl-token-metadata** via UMI for on-chain metadata.

| Script | Command | What it does |
|---|---|---|
| `spl_init.ts` | `npm run spl:init` | Creates a new mint account |
| `spl_metadata.ts` | `npm run spl:metadata` | Attaches a name, symbol, and URI to the mint |
| `spl_mint.ts` | `npm run spl:mint` | Creates your associated token account and mints tokens into it |
| `spl_transfer.ts` | `npm run spl:transfer` | Sends tokens to another wallet i.e ata to ata |

Run them in order. Each script logs the addresses/signatures you'll need to paste into the next one.

---

## NFT

Uses **@solana/kit** and **mpl-core** via UMI. Images and metadata are stored on Irys (decentralized storage).

| Script | Command | What it does |
|---|---|---|
| `nft_image.ts` | `npm run nft:image` | Uploads your image to Irys, logs the image URI |
| `nft_metadata.ts` | `npm run nft:metadata` | Builds the metadata JSON and uploads it, logs the metadata URI |
| `nft_mint.ts` | `npm run nft:mint` | Mints the NFT on-chain using the metadata URI |

---

## 📸 Proof of Execution

Screenshots are inside spl-nft-q326/Proof

---
### Proof 

spl_init.ts
mint address: HoeRyA8MGTpVLhgB1fbbWfdbkFvUe3obfrHLzY6wJG6c , 
Transaction signature: 2QhmKer1ZL3Qe8bZwA9mwPWf85AqdMM5PBgHXDoFJ1CPuUBDZy445vMncT2ceUmpzMJaEz3UCu2NXsr4viuDSFd6
spl_metadata.ts
signature:  5UhcGChR16Bbkmrmb8vbcZrpokZmH5MWZuTrDTvZfNF2jws49oYnfd4WNBSiALQzqRi2U9FwFbG9umvjc2gvWpLe

spl_mint.ts
Your ata is : HLPsiqbfj5uQE1rvq6NYJokkbw4Lm34TrtFZrKgvmtwY
mint txid: 4cWgDMnngHYeQkJifHBe3uZFhX8rtaWtuvPjjSSGjw7cG3p3CUDUHCKWzFKwiQgqoBiZKcj21zREM5jHUqDKaVbR

spl_transfer.ts
Your fromAta is : HLPsiqbfj5uQE1rvq6NYJokkbw4Lm34TrtFZrKgvmtwY
Your toAta is : HvxgNLMiYSCRvEae27ggUhDbPrXQ9VxhHScsnUwbDJcG
mint txid: jw21AEfMxgLPaRoEQdZtE8Nb4nkKTXxaLCjS9bVzErHyNKDmEhCjPWeQ9YeHuYGtW8o9ZJKZpAGVfL79DXYiV5R


nft_image.ts
Your image URI:  https://gateway.irys.xyz/4JGp35Yh1hDSgb55HZsqFzBT3bUuw4A1nbvCavDxbr25

nft_metadata.ts
metadata uri: https://gateway.irys.xyz/3cdZ1uJ3RofiQd8A4wZKUna2ccMDsDyNW5HcWzRQJ8nq

nft_mint.ts
signature 4xe6KKoLCbDaQubYDgJ6a3jK9FEYsyM27Qtx3mevYKPoSrSPLwGWYkiMZv7rkfWx2jFzKbjekHy7eGoTnmvtGE5i , 
asset : 8vbYaa2Xr9N3cH449iWWWPGshcG9HzJ4ATKgHsDtB23R

updated nft_metadata.ts
metadata uri: https://gateway.irys.xyz/759aG2G9uJEosR7hDFeGPRvTLEHS3iec4tBPxxiAb68w

nft_update.ts
signature:   3pViq2ELWgUkkuTGeYpfFbun46tpNPFMoV2JsVV3Cu4eRB9tVvKvevACV49uCwBohDBDLCDQ9nVzhxSSFLKkB14a


nft2 image
Your image URI:  https://gateway.irys.xyz/43v6DzZbNEQcwfnF7g9JbGV9t6M14gScLyRUHKTXtiFf

nft 2 metadata
metadata uri: https://gateway.irys.xyz/5Uq29GzRa9KBnu6zovtgUCJz781Ew21E2xJTiKAbaNtL

nft 2 mint
signature 5C8LkEVTkfxhuEKuMX4uctFeQeM4grDZUKqACjvWWcC6wfmaiSAw9FY958YH1GRcQKHQHzPxfbrMAp2ddtb3mBMY , 
asset : Eu2YxsTk3HV681X9ADXM7LGDskxnUX4stgW8SAk8Gc5h


nft 3 image
Your image URI:   
https://gateway.irys.xyz/4deTo1qh7yrqF2v7zLNUhhJTwarHCFLx8CWFyBDs5hMD

nft 3 metadata
metadata uri: https://gateway.irys.xyz/HvjJLs2jZwxEyfio5rh2J8zSijJCwMPHAFKCKk88mokn

nft 3 mint
signature 4DhSmS1FDamaBk78oLDoot9KTxLPgQNstcfVjH8tDitiKjmV5B7wz5QGQr4A9KtZBboHBtS25J4JB4kE3sc83LQ4 , 
asset : C9ThPjEN1G7qJMhapcLs1jMYxUfqNN855p3EtWXasf59

nft 3 burn 
signature  3z3M9Ga8NmnS19KSERaSTcFWt78m6LfaNUqSKQHZXqtKP5UAJjB6hqJjBQ3PnfcvXnKRp1QeyWNQjeTH2NFL4oQu




# anchor_vault_starter_q3_26

Overview
- Starter vault program scaffold intended for the Q3 2026 Builder cohort.
- Primary purpose: a minimal Anchor-based Solana program that demonstrates a vault/escrow pattern (locks funds, manages ownership/authority, withdraw flow).
- Expected tech: Rust + Anchor. (Repository language breakdown includes Rust — adapt commands to your local toolchain.)

Prerequisites
- Rust toolchain (rustup + stable toolchain)
- Solana CLI (solana)
- Anchor CLI (anchor)
- Optionally Node.js / npm (if there are client scripts or tests that use JS/TS)
- A local Solana validator (solana-test-validator) for local testing

Quick start
1. Install prerequisites (Rust, Solana CLI, Anchor).
2. Start local validator:
   ```bash
   solana-test-validator --reset
   ```
3. Build the Anchor program:
   ```bash
   anchor build
   ```
4. Run tests (Anchor/Jest or Cargo tests depending on the setup):
   ```bash
   anchor test
   ```
   or
   ```bash
   cargo test
   ```

Folder structure (typical)
- Cargo.toml — Rust package manifest for the program
- Anchor.toml — Anchor workspace/config
- programs/ — Rust program source (lib.rs, processors, state definitions)
- tests/ — integration tests (Anchor JS or Rust tests)
- migrations/ — Anchor migrations (if used)
- examples/ or client/ — optional client scripts showing how to interact

Configuration
- Anchor.toml controls build/deploy settings (cluster, provider, wallet, program IDs).
- Use a `.env` or local config for private keys, RPC URLs, or program IDs if you prefer.
- Ensure your Solana CLI is pointed to the expected cluster:
  ```bash
  solana config set --url http://127.0.0.1:8899
  solana config get
  ```

Build & deploy (local)
- Build:
  ```bash
  anchor build
  ```
- Deploy to local validator:
  ```bash
  anchor deploy --provider.cluster Localnet
  ```
- To deploy to devnet/mainnet, configure Anchor.toml and provider (be careful with real funds).

Running tests
- Run the local validator, then:
  ```bash
  anchor test
  ```
- If tests are JS/TS-based, ensure `npm install` in repo root (if a JS client exists) and run:
  ```bash
  npm test
  ```

Tips & troubleshooting
- If Anchor build fails, ensure Rust toolchain and Solana version compatibility.
- Common fix: update Anchor CLI to match the Anchor framework version in Cargo.toml.
- If program IDs mismatch, reset Anchor.toml or set IDs via migrations.

Contributing
- Keep changes small and documented.
- Add/extend tests for new behavior.
- Document protocol or account layout changes in comments.

License & attribution
- Add your preferred license file at repo root (e.g., LICENSE).
- Note: This README is a scaffold — adapt commands to exact project files if they differ.




# escrow-q3-26

Overview
- Escrow service/client for the Q3 2026 Builder cohort assignments.
- Purpose: TypeScript/Node-based service or client that interacts with the Vault/Anchor program or implements escrow logic off-chain.
- Expected tech: TypeScript (Node.js). (Repository language breakdown lists TypeScript — adjust if you use plain JS.)

Prerequisites
- Node.js (recommended LTS)
- npm or yarn
- If interacting with a blockchain program: a local node/network or RPC endpoint and any required CLI tools (e.g., Solana CLI)

Quick start
1. Install dependencies:
   ```bash
   npm install
   ```
   or
   ```bash
   yarn install
   ```
2. Configure environment:
   - Copy `.env.example` to `.env` and fill required variables (RPC URL, keypaths, API keys).
3. Start in development mode:
   ```bash
   npm run dev
   ```
   or
   ```bash
   yarn dev
   ```
4. Build for production:
   ```bash
   npm run build
   npm start
   ```

Common scripts (edit package.json if names differ)
- dev — start with ts-node or nodemon for live reload
- build — compile TypeScript to JS (tsc)
- start — run compiled build
- test — run unit/integration tests (jest, vitest, etc.)
- lint / format — static checks and formatting

Folder structure (typical)
- package.json — npm scripts and dependencies
- tsconfig.json — TypeScript configuration
- src/ — TypeScript source files
  - index.ts / server.ts — app entrypoint
  - controllers/, services/, utils/ — modular code
- tests/ — unit and integration tests
- .env.example — example environment variables
- scripts/ — helper scripts (optional)

Configuration
- Provide required env vars in `.env`:
  - Example keys: RPC_URL, PRIVATE_KEY_PATH, API_PORT, LOG_LEVEL
- Avoid committing real secrets to source control.

Testing
- Run unit tests:
  ```bash
  npm test
  ```
- For integration tests that need a local blockchain or test validator, run the validator and point RPC_URL to the local endpoint before testing.

Usage examples
- If this folder exposes an HTTP API: use curl or Postman to hit endpoints, e.g.:
  ```bash
  curl -X POST http://localhost:3000/escrow/create -d '{...}' -H "Content-Type: application/json"
  ```
- If it provides CLI scripts, use:
  ```bash
  node dist/scripts/createEscrow.js --params ...
  ```

Deployment
- Build (`npm run build`), set environment variables on the host, and run (`npm start`).
- For containerized deployment, create a Dockerfile that installs Node, copies build output, and runs the start script.

Troubleshooting
- If TypeScript build fails, check tsconfig paths and installed types.
- If RPC connections fail, verify RPC_URL and network accessibility.
- Increase logging (LOG_LEVEL=debug) to see detailed errors.

Contributing
- Add tests for any new logic.
- Keep API/endpoint changes backward compatible or document breaking changes.
- Run lint/format before opening a PR.

Contact / Maintainers
- Add maintainer contact info or link to project owner for questions.

License
- Add a LICENSE file in the repo root to specify licensing.