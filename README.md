# Secret Hitler Board Game on Solana

onchain implementation of the popular board game Secret Hitler using Solana's blockchain. The game integrates off-chain components for role assignment and deck shuffling.

## Player Inactivity

The official game rules have been cafully followed and some gaps have been filled such as player inactivity. Games can be set up with a required entry deposit which will be returned if the player completes the game regardless of game outcome. If a player is inactive past there turn period, then over players (or any signer) is insentivesed to call the "[Eliminate Player](./anchor-program/programs/secret-hitler/src/instructions/eliminate_player.rs) instruction which automatically kicks out inactive players from the game and their deposit and/or bet will be later distributed amongst the remaining active players at the end of the game.

This solutions uses game theory to eliminate the need for an offchain game server or cron job to continuesly listen to the game events.

## Solana-SDK

The [solana-sdk](./backend/src/solana.rs) has been utilized to interact with the anchor program and partially sign a transaction. This is to make sure that the game can only begin when the needed information has been recorded in the SQLite database in memory.

## Game Overview

Program on Devnet: FxKWd5DzXvHPVfrGo3DvXgTeG25TGUGUAFvjQ1zSMp1B

### Key Features

- **Inactivity Management**: On-chain functionality to handle inactive players and manage their removal, reducing reliance on external servers or cron jobs.
- **Off-Chain Role Assignment and Deck Shuffling**: Managed via a Rust server to keep critical game data secure. Future enhancements may include ZKPs for on-chain processing. (In progress)

### Testing

- **Frameworks Used**: Solana test validator, [TypeScript test files](./anchor-program/tests/secret-hitler.ts), and [Rust test files](./anchor-program/programs/secret-hitler/tests/).

## Setup Instructions

```bash
cd anchor-program
anchor build
anchor test
```

### Future Enhancements

- **Zero-Knowledge Proofs (ZKPs)**: Plan to implement ZKPs for on-chain role assignment and deck shuffling, eliminating the need for off-chain servers.
- **SPL Tokens**: Consider integrating SPL tokens for player rewards, enhancing the gameplay experience.

## Important Notes

- **Progress Status**: onchain anchor program is finished and tested. Rust Axum backend in development and has made significant progress. front-end is not yet implemented
