// import * as anchor from "@coral-xyz/anchor";
// import { Program } from "@coral-xyz/anchor";
// import { SecretHitler } from "../target/types/secret_hitler";
// import { Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
// import { assert } from "chai";
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
// describe("secret-hitler", () => {
//   const provider = anchor.AnchorProvider.env();
//   anchor.setProvider(provider);
//   const program = anchor.workspace.SecretHitler as Program<SecretHitler>;

//   let maxPlayers = 6;
//   let turnDuration = new anchor.BN(120);

//   let depositAmount = new anchor.BN(0.01 * LAMPORTS_PER_SOL);
//   let betAmount = new anchor.BN(0.05 * LAMPORTS_PER_SOL);

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
//   let [depositVault, depositBump] =
//     anchor.web3.PublicKey.findProgramAddressSync(
//       [Buffer.from("deposit_vault"), gameData.toBuffer()],
//       program.programId,
//     );
//   let [betVault, betbump] = anchor.web3.PublicKey.findProgramAddressSync(
//     [Buffer.from("bet_vault"), gameData.toBuffer()],
//     program.programId,
//   );

//   let nomination = anchor.web3.PublicKey.findProgramAddressSync(
//     [Buffer.from("chancellor_nomination"), gameData.toBuffer()],
//     program.programId,
//   )[0];

//   let president: Keypair;
//   let chancellor: Keypair;

//   players.forEach(async (player) => {
//     await airdrop(provider.connection, player.publicKey);
//   });

//   let [gameData_1, gameDataBump_1] =
//     anchor.web3.PublicKey.findProgramAddressSync(
//       [Buffer.from("secret_hitler"), player_1.publicKey.toBuffer()],
//       program.programId,
//     );
//   let playerData_1 = anchor.web3.PublicKey.findProgramAddressSync(
//     [
//       Buffer.from("player_data"),
//       gameData_1.toBuffer(),
//       player_1.publicKey.toBuffer(),
//     ],
//     program.programId,
//   )[0];

//   it("Init host game with deposit and bet included", async () => {
//     await airdrop(provider.connection, host.publicKey);

//     await program.methods
//       .initializeGame(maxPlayers, turnDuration, depositAmount, betAmount)
//       .accountsPartial({
//         host: host.publicKey,
//         gameData,
//         playerData: hostData,
//         betVault,
//         depositVault,
//       })
//       .signers([host])
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
//     let depositVaultAfter =
//       await provider.connection.getAccountInfo(depositVault);
//     assert.strictEqual(
//       depositVaultAfter.lamports.toString(),
//       depositAmount.toString(),
//       "Deposit in vault mismatch",
//     );
//     let betVaultAfter = await provider.connection.getAccountInfo(betVault);
//     assert.strictEqual(
//       betVaultAfter.lamports.toString(),
//       betAmount.toString(),
//       "Bet in vault mismatch",
//     );
//     console.log("host key: ", host.publicKey);
//     console.log("\n");

//     console.log("game key: ", gameData);
//   });
// });
