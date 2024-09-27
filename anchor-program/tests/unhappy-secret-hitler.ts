// import * as anchor from "@coral-xyz/anchor";
// import { Program } from "@coral-xyz/anchor";
// import { SecretHitler } from "../target/types/secret_hitler";
// import { Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
// import { assert } from "chai";

// describe("Unhappy hitler", () => {
//   const provider = anchor.AnchorProvider.env();
//   anchor.setProvider(provider);
//   const program = anchor.workspace.SecretHitler as Program<SecretHitler>;

//   let maxPlayers = 6;
//   let turnDuration = new anchor.BN(120);

//   let depositAmount = new anchor.BN(LAMPORTS_PER_SOL);
//   let betAmount = new anchor.BN(0.5 * LAMPORTS_PER_SOL);

//   let host = anchor.web3.Keypair.generate();

//   let player_1 = anchor.web3.Keypair.generate();
//   let player_2 = anchor.web3.Keypair.generate();
//   let player_3 = anchor.web3.Keypair.generate();
//   let player_4 = anchor.web3.Keypair.generate();
//   let players = [player_1, player_2, player_3, player_4];

//   let [gameData, gameDataBump] = anchor.web3.PublicKey.findProgramAddressSync(
//     [Buffer.from("secret_hitler"), host.publicKey.toBuffer()],
//     program.programId,
//   );
//   let [hostData, hostDataBump] = anchor.web3.PublicKey.findProgramAddressSync(
//     [
//       Buffer.from("player_data"),
//       gameData.toBuffer(),
//       host.publicKey.toBuffer(),
//     ],
//     program.programId,
//   );
//   it("Can not init game without the bet or deposit", async () => {
//     airdrop(provider.connection, host.publicKey);
//     await program.methods
//       .initializeGame(maxPlayers, turnDuration, betAmount, depositAmount)
//       .accountsPartial({
//         host: host.publicKey,
//         gameData: gameData,
//         playerData: hostData,
//         betVault: null,
//         depositVault: null,
//       })
//       .signers([player_1])
//       .rpc({ skipPreflight: true })
//       .then(confirmTx);
//     let gameAfter = await program.account.gameData.fetch(gameData);
//     assert.strictEqual(
//       gameAfter.activePlayers.length.toString(),
//       "1",
//       "expected player count is 1 but got " + gameAfter.activePlayers.length,
//     );
//     assert.strictEqual(
//       gameAfter.bump.toString(),
//       gameDataBump.toString(),
//       "wrong bump detected",
//     );
//   });
// });

// const confirmTx = async (signature: string) => {
//   const latestBlockhash = await anchor
//     .getProvider()
//     .connection.getLatestBlockhash();
//   await anchor.getProvider().connection.confirmTransaction(
//     {
//       signature,
//       ...latestBlockhash,
//     },
//     "confirmed",
//   );
// };
// async function airdrop(
//   connection: anchor.web3.Connection,
//   address: PublicKey,
//   amount = 10 * LAMPORTS_PER_SOL,
// ) {
//   await connection.requestAirdrop(address, amount).then(confirmTx);
// }
