# Chainlink Treasury Bridge (Solana Program)

## Overview
This Anchor program implements a Cross-Chain Treasury Management system capable of handling multiple asset types. It serves as the **Destination Chain** component in a Lock-and-Mint architecture, interacting with an off-chain Relayer.

## Architecture
The program is modularized to support scalability and clean testing:
- **State (`/state`):** Stores global configuration (Admin keys, Token addresses).
- **Instructions (`/instructions`):** Contains isolated business logic for each action.

## Supported Assets & Logic

### 1. wLINK (Wrapped LINK)
- **Mechanism:** Standard Lock & Mint.
- **Flow:** User locks LINK on Ethereum -> Relayer calls `mint_wlink` -> Program mints SPL wLINK on Solana.
- **Security:** Requires signature from the authorized Relayer (Admin).

### 2. USDC (CCTP Simulation)
- **Mechanism:** Burn & Mint (Circle CCTP Pattern).
- **Flow:** User burns USDC on Ethereum -> Circle emits event -> Relayer calls `receive_cctp_message`.
- **Note:** For this POC, the Circle Message Transmitter verification is mocked via Relayer signature.

## Project Structure

```bash
src/
├── lib.rs                  # Entry point and instruction routing
├── state/
│   ├── mod.rs
│   └── config.rs           # Defines the 'Config' account structure
└── instructions/
    ├── mod.rs
    ├── initialize.rs       # Sets up the Admin and Token Mints
    ├── mint_wlink.rs       # Handles wLINK minting logic
    └── receive_cctp.rs     # Handles USDC minting logic
```

## Security Considerations
1. Access Control: All state-changing instructions require the signer to match the admin key stored in the Config account. 
2. Token Validation: The program explicitly checks (constraint) if the provided Mint accounts match the official addresses stored in the configuration to prevent spoofing.
3. Data Separation: Program logic is stateless; all state is stored in the persistent Config account.