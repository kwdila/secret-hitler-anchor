# Solana Game Preset

This game is ment as a starter game for on chain games.
There is a js and a unity client for this game and both are talking to a solana anchor program.

This game uses gum session keys for auto approval of transactions.
Note that neither the program nor session keys are audited. Use at your own risk.

# How to run this example

## Quickstart

The unity client and the js client are both connected to the same program and should work out of the box connecting to the already deployed program.

### Js Client

To start the js client open the project in visual studio code and run:

```bash
cd app
yarn install
yarn dev
```

To start changing the program and connecting to your own program follow the steps below.

## Installing Solana dependencies

Follow the installation here: https://www.anchor-lang.com/docs/installation
Install the latest 1.16 solana version (1.17 is not supported yet)
sh -c "$(curl -sSfL https://release.solana.com/v1.16.18/install)"

Anchor program

1. Install the [Anchor CLI](https://project-serum.github.io/anchor/getting-started/installation.html)
2. `cd program` to end the program directory
3. Run `anchor build` to build the program
4. Run `anchor deploy` to deploy the program
5. Copy the program id from the terminal into the lib.rs, anchor.toml and within the unity project in the AnchorService and if you use js in the anchor.ts file
6. Build and deploy again

Next js client

1. Install [Node.js](https://nodejs.org/en/download/)
2. Copy the program id into app/utils/anchor.ts
3. `cd app` to end the app directory
4. Run `yarn install` to install node modules
5. Run `yarn dev` to start the client
6. After doing changes to the anchor program make sure to copy over the types from the program into the client so you can use them. You can find the js types in the target/idl folder.

## Connect to local host (optional)

To connect to local host from Unity add these links on the wallet holder game object:
http://localhost:8899
ws://localhost:8900

## Video walkthroughs

Here are two videos explaining the energy logic and session keys:
Session keys:
https://www.youtube.com/watch?v=oKvWZoybv7Y&t=17s&ab_channel=Solana
Energy system:
https://www.youtube.com/watch?v=YYQtRCXJBgs&t=4s&ab_channel=Solana

### Session keys

Session keys is an optional component. What it does is creating a local key pair which is toped up with some sol which can be used to autoapprove transactions. The session token is only allowed on certain functions of the program and has an expiry of 23 hours. Then the player will get the sol back and can create a new session.

With this you can now build any energy based game and even if someone builds a bot for the game the most he can do is play optimally, which maybe even easier to achieve when playing normally depending on the logic of your game.

This game becomes even better when combined with the Token example from Solana Cookbook and you actually drop some spl token to the players.
