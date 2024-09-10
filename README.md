# Secret Hitler Board Game on Solana

This document outlines the on-chain implementation of the popular board game Secret Hitler using Solana's blockchain. The game integrates off-chain components for role assignment and deck shuffling, with future plans to incorporate Zero-Knowledge Proofs (ZKPs) for complete on-chain management.

## Game Overview

### Key Features

- **Inactivity Management**: On-chain functionality to handle inactive players and manage their removal, reducing reliance on external servers or cron jobs.
- **Off-Chain Role Assignment and Deck Shuffling**: Managed via a Rust server to keep critical game data secure. Future enhancements may include ZKPs for on-chain processing.

### Testing

- **Frameworks Used**: Solana test validator, [TypeScript test files](./anchor-program/tests/secret-hitler.ts), and [Rust test files](./anchor-program/programs/secret-hitler/tests/).

### Client and Server Integration

- **JS Client**: Communicates with the Solana Anchor program and uses session keys for auto-approving transactions.
- **Session Keys**: Optional feature allowing automatic transaction approval for a session of up to 23 hours.

## Setup Instructions

### Solana Dependencies

1. **Install Solana CLI**

   ```bash
   sh -c "$(curl -sSfL https://release.solana.com/v1.16.18/install)"
   ```

2. **Install Anchor CLI**: Follow the guide [here](https://project-serum.github.io/anchor/getting-started/installation.html).

### Anchor Program

1. **Build the Program**

   ```bash
   cd program
   anchor build
   ```

2. **Deploy the Program**

   ```bash
   anchor deploy
   ```

3. **Update Program ID**: Copy the program ID from the terminal and update it in:

   - `lib.rs`
   - `anchor.toml`
   - Unity project (AnchorService)
   - JavaScript client (anchor.ts)

4. **Rebuild and Redeploy**
   ```bash
   anchor build
   anchor deploy
   ```

### Next.js Client

1. **Install Node.js**: Download and install [Node.js](https://nodejs.org/en/download/).

2. **Update Program ID**: Add the program ID to `app/utils/anchor.ts`.

3. **Start the Client**

   ```bash
   cd app
   yarn install
   yarn dev
   ```

4. **Update Types**: After modifying the Anchor program, copy the updated types from `target/idl` into the client for compatibility.

### Connecting to Localhost (Optional)

- **Unity Setup**: For testing on localhost, configure the wallet holder game object with:
  - HTTP: `http://localhost:8899`
  - WebSocket: `ws://localhost:8900`

### Session Keys

- **Functionality**: Allows transactions to be auto-approved for up to 23 hours, reducing manual transaction handling.
- **Expiry**: Session keys expire after 23 hours; players must renew them to continue using the feature.

### Future Enhancements

- **Zero-Knowledge Proofs (ZKPs)**: Plan to implement ZKPs for on-chain role assignment and deck shuffling, eliminating the need for off-chain servers.
- **SPL Tokens**: Consider integrating SPL tokens for player rewards, enhancing the gameplay experience.

## Important Notes

- **Audit Status**: Neither the program nor session keys are audited. Use at your own risk.

This setup offers a comprehensive starting point for on-chain games with off-chain elements, leveraging blockchain technology for secure and efficient game management.
