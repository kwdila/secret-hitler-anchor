import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SecretHitler } from "../target/types/secret_hitler";
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { assert } from "chai";

describe("secret-hitler", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.SecretHitler as Program<SecretHitler>;

  let maxPlayers = 6;
  let turnDuration = new anchor.BN(120);

  let depositAmount = new anchor.BN(LAMPORTS_PER_SOL);
  let betAmount = new anchor.BN(0.5 * LAMPORTS_PER_SOL);

  let host = anchor.web3.Keypair.generate();

  let player_1 = anchor.web3.Keypair.generate();
  let player_2 = anchor.web3.Keypair.generate();
  let player_3 = anchor.web3.Keypair.generate();
  let player_4 = anchor.web3.Keypair.generate();
  let players = [player_1, player_2, player_3, player_4];

  let [gameData, gameDataBump] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("secret_hitler"), host.publicKey.toBuffer()],
    program.programId,
  );
  let [hostData, hostDataBump] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("player_data"),
      gameData.toBuffer(),
      host.publicKey.toBuffer(),
    ],
    program.programId,
  );
  let [depositVault, depositBump] =
    anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("deposit_vault"), gameData.toBuffer()],
      program.programId,
    );
  let [betVault, betbump] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("bet_vault"), gameData.toBuffer()],
    program.programId,
  );

  let nomination = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("chancellor_nomination"), gameData.toBuffer()],
    program.programId,
  )[0];

  let president: Keypair;
  let chancellor: Keypair;

  players.forEach(async (player) => {
    await airdrop(provider.connection, player.publicKey);
  });

  let [gameData_1, gameDataBump_1] =
    anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("secret_hitler"), player_1.publicKey.toBuffer()],
      program.programId,
    );
  let playerData_1 = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("player_data"),
      gameData_1.toBuffer(),
      player_1.publicKey.toBuffer(),
    ],
    program.programId,
  )[0];

  it("Init host game with deposit and bet included", async () => {
    await airdrop(provider.connection, host.publicKey);

    await program.methods
      .initializeGame(maxPlayers, turnDuration, depositAmount, betAmount)
      .accountsPartial({
        host: host.publicKey,
        gameData,
        playerData: hostData,
        betVault,
        depositVault,
      })
      .signers([host])
      .rpc({ skipPreflight: true })
      .then(confirmTx);

    let gameAfter = await program.account.gameData.fetch(gameData);
    assert.strictEqual(
      gameAfter.activePlayers.length.toString(),
      "1",
      "expected player count is 1 but got " + gameAfter.activePlayers.length,
    );
    assert.strictEqual(
      gameAfter.bump.toString(),
      gameDataBump.toString(),
      "wrong bump detected",
    );
    let depositVaultAfter =
      await provider.connection.getAccountInfo(depositVault);
    assert.strictEqual(
      depositVaultAfter.lamports.toString(),
      depositAmount.toString(),
      "Deposit in vault mismatch",
    );
    let betVaultAfter = await provider.connection.getAccountInfo(betVault);
    assert.strictEqual(
      betVaultAfter.lamports.toString(),
      betAmount.toString(),
      "Bet in vault mismatch",
    );
  });
  it("Init player_1 game without deposit and bet", async () => {
    await program.methods
      .initializeGame(maxPlayers, turnDuration, null, null)
      .accountsPartial({
        host: player_1.publicKey,
        gameData: gameData_1,
        playerData: playerData_1,
        betVault: null,
        depositVault: null,
      })
      .signers([player_1])
      .rpc({ skipPreflight: true })
      .then(confirmTx);
    let gameAfter = await program.account.gameData.fetch(gameData_1);
    assert.strictEqual(
      gameAfter.activePlayers.length.toString(),
      "1",
      "expected player count is 1 but got " + gameAfter.activePlayers.length,
    );
    assert.strictEqual(
      gameAfter.bump.toString(),
      gameDataBump_1.toString(),
      "wrong bump detected",
    );
  });
  it("Join host game", async () => {
    await Promise.all(
      players.map(async (player) => {
        let playerData = anchor.web3.PublicKey.findProgramAddressSync(
          [
            Buffer.from("player_data"),
            gameData.toBuffer(),
            player.publicKey.toBuffer(),
          ],
          program.programId,
        )[0];
        await program.methods
          .joinGame()
          .accountsPartial({
            player: player.publicKey,
            playerData,
            gameData,
            depositVault,
            betVault,
          })
          .signers([player])
          .rpc()
          .then(confirmTx);
      }),
    );
    let gameAfter = await program.account.gameData.fetch(gameData);
    assert.strictEqual(
      gameAfter.activePlayers.length.toString(),
      "5",
      "expected player count is 5 but got " + gameAfter.activePlayers.length,
    );
  });
  it("Join initialized player_1 game", async () => {
    let player = players[2].publicKey;
    let playerData = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("player_data"), gameData_1.toBuffer(), player.toBuffer()],
      program.programId,
    )[0];
    await program.methods
      .joinGame()
      .accountsPartial({
        player,
        playerData,
        gameData: gameData_1,
        depositVault: null,
        betVault: null,
      })
      .signers([players[2]])
      .rpc()
      .then(confirmTx);
    let gameAfter = await program.account.gameData.fetch(gameData_1);
    assert.strictEqual(
      gameAfter.activePlayers.length.toString(),
      "2",
      "expected player count is 2 but got " + gameAfter.activePlayers.length,
    );
  });
  it("leave game player_1 game", async () => {
    let player = players[2].publicKey;
    let playerData = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("player_data"), gameData_1.toBuffer(), player.toBuffer()],
      program.programId,
    )[0];
    await program.methods
      .leaveGame()
      .accountsPartial({
        player: player_3.publicKey,
        playerData,
        gameData: gameData_1,
        depositVault: null,
        betVault: null,
      })
      .signers([players[2]])
      .rpc()
      .then(confirmTx);
    let gameAfter = await program.account.gameData.fetch(gameData_1);
    assert.strictEqual(
      gameAfter.activePlayers.length.toString(),
      "1",
      "expected player count is 1 but got " + gameAfter.activePlayers.length,
    );
  });
  it("Start host game", async () => {
    await airdrop(provider.connection, host.publicKey);

    await program.methods
      .startGame()
      .accountsPartial({
        host: host.publicKey,
        gameData,
      })
      .signers([host])
      .rpc()
      .then(confirmTx);

    let gameAfter = await program.account.gameData.fetch(gameData);
    assert.strictEqual(
      gameAfter.activePlayers.length.toString(),
      "5",
      "expected player count is 5 but got " + gameAfter.activePlayers.length,
    );
    assert.strictEqual(
      Object.keys(gameAfter.gameState)[0].toString(),
      "chancellorNomination",
      "wrong game state detected",
    );
    assert.strictEqual(
      gameAfter.currentPresidentIndex.toString(),
      "3",
      "wrong president index detected",
    );
  });
  it("try to eliminate player", async () => {
    await program.methods
      .eliminateInactivePlayer()
      .accountsPartial({
        player: host.publicKey,
        gameData,
        nomination: null,
      })
      .signers([host])
      .rpc({ skipPreflight: true })
      .then(confirmTx);
  });
  it("Nominate chancellor", async () => {
    let game = await program.account.gameData.fetch(gameData);
    let key = game.allStartingPlayers[game.currentPresidentIndex];
    // find president player keypair
    president = players.find(
      (player) => player.publicKey.toString() === key.toString(),
    );

    if (host.publicKey.toString() === key.toString()) president = host;

    await airdrop(provider.connection, president.publicKey);

    await program.methods
      .nominateChancelor(player_1.publicKey)
      .accountsPartial({
        president: president.publicKey,
        gameData,
        nomination,
      })
      .signers([president])
      .rpc({ skipPreflight: true })
      .then(confirmTx);

    let gameAfter = await program.account.gameData.fetch(gameData);
    assert.strictEqual(
      gameAfter.activePlayers.length.toString(),
      "5",
      "expected player count is 5 but got " + gameAfter.activePlayers.length,
    );
    assert.strictEqual(
      Object.keys(gameAfter.gameState)[0].toString(),
      "chancellorVoting",
      "wrong game state detected",
    );
    let nominationAfter = await program.account.nomination.fetch(nomination);
    assert.strictEqual(
      nominationAfter.ja.toString(),
      "1",
      "Unexpected ja votes",
    );
  });
  it("Vote chancellor", async () => {
    let game = await program.account.gameData.fetch(gameData);
    const majorityVote = Math.floor(game.activePlayers.length / 2) + 1;
    let voteCount = 0;

    for (const player of players) {
      if (
        player.publicKey.toString() ===
        game.allStartingPlayers[game.currentPresidentIndex].toString()
      ) {
        continue;
      }

      await program.methods
        .voteChancellor({ ja: {} })
        .accountsPartial({
          player: player.publicKey,
          gameData,
          nomination,
        })
        .signers([player])
        .rpc({ skipPreflight: true })
        .then(confirmTx);

      voteCount++;

      let nominationAfter = await program.account.nomination.fetch(nomination);
      if (
        nominationAfter.ja >= majorityVote ||
        nominationAfter.nein >= majorityVote
      ) {
        break; // Stop voting when we reach a majority
      }

      if (voteCount >= game.activePlayers.length - 1) {
        break; // Stop if all players except the president have voted
      }
    }

    let gameAfter = await program.account.gameData.fetch(gameData);
    assert.strictEqual(
      gameAfter.activePlayers.length.toString(),
      "5",
      "expected player count is 5 but got " + gameAfter.activePlayers.length,
    );
    assert.strictEqual(
      Object.keys(gameAfter.gameState)[0].toString(),
      "legislativePresident",
      "wrong game state detected",
    );
  });
  it("president Enact policy", async () => {
    let game = await program.account.gameData.fetch(gameData);
    let key = game.activePlayers[game.currentPresidentIndex];
    // find president player keypair
    president = players.find(
      (player) => player.publicKey.toString() === key.toString(),
    );

    if (host.publicKey.toString() === key.toString()) president = host;

    await airdrop(provider.connection, president.publicKey);

    await program.methods
      .enactPolicy(null)
      .accountsPartial({
        player: president.publicKey,
        gameData,
      })
      .signers([president])
      .rpc({ skipPreflight: true })
      .then(confirmTx);

    let gameAfter = await program.account.gameData.fetch(gameData);
    assert.strictEqual(
      gameAfter.activePlayers.length.toString(),
      "5",
      "expected player count is 5 but got " + gameAfter.activePlayers.length,
    );
    assert.strictEqual(
      Object.keys(gameAfter.gameState)[0].toString(),
      "legislativeChancellor",
      "wrong game state detected",
    );
  });
  it("Chancellor Enact policy", async () => {
    let game = await program.account.gameData.fetch(gameData);
    let key = game.activePlayers[game.currentChancellorIndex];
    // find chancellor player keypair
    chancellor = players.find(
      (player) => player.publicKey.toString() === key.toString(),
    );

    if (host.publicKey.toString() === key.toString()) chancellor = host;

    await airdrop(provider.connection, president.publicKey);

    await program.methods
      .enactPolicy({ liberal: {} })
      .accountsPartial({
        player: chancellor.publicKey,
        gameData,
      })
      .signers([chancellor])
      .rpc({ skipPreflight: true })
      .then(confirmTx);

    let gameAfter = await program.account.gameData.fetch(gameData);
    assert.strictEqual(
      gameAfter.liberalPoliciesEnacted.toString(),
      "1",
      "expected player count is 5 but got " + gameAfter.activePlayers.length,
    );
    assert.strictEqual(
      Object.keys(gameAfter.gameState)[0].toString(),
      "chancellorNomination",
      "wrong game state detected",
    );
  });
  it("Nominate chancellor", async () => {
    let game = await program.account.gameData.fetch(gameData);
    let key = game.activePlayers[game.currentPresidentIndex];
    // find president player keypair
    president = players.find(
      (player) => player.publicKey.toString() === key.toString(),
    );

    if (host.publicKey.toString() === key.toString()) president = host;

    await airdrop(provider.connection, president.publicKey);

    await program.methods
      .nominateChancelor(host.publicKey)
      .accountsPartial({
        president: president.publicKey,
        gameData,
        nomination,
      })
      .signers([president])
      .rpc({ skipPreflight: true })
      .then(confirmTx);

    let gameAfter = await program.account.gameData.fetch(gameData);
    assert.strictEqual(
      gameAfter.activePlayers.length.toString(),
      "5",
      "expected player count is 5 but got " + gameAfter.activePlayers.length,
    );
    assert.strictEqual(
      Object.keys(gameAfter.gameState)[0].toString(),
      "chancellorVoting",
      "wrong game state detected",
    );
    let nominationAfter = await program.account.nomination.fetch(nomination);
    assert.strictEqual(
      nominationAfter.ja.toString(),
      "1",
      "Unexpected ja votes",
    );
  });
  it("Vote chancellor", async () => {
    let game = await program.account.gameData.fetch(gameData);
    await Promise.all(
      players.slice(2, 4).map(async (player) => {
        if (player.publicKey.toString() === president.publicKey.toString()) {
          return;
        }
        await program.methods
          .voteChancellor({ ja: {} })
          .accountsPartial({
            player: player.publicKey,
            gameData,
            nomination,
          })
          .signers([player])
          .rpc({ skipPreflight: true })
          .then(confirmTx);
      }),
    );
    let gameAfter = await program.account.gameData.fetch(gameData);
    assert.strictEqual(
      gameAfter.activePlayers.length.toString(),
      "5",
      "expected player count is 5 but got " + gameAfter.activePlayers.length,
    );
    assert.strictEqual(
      Object.keys(gameAfter.gameState)[0].toString(),
      "legislativePresident",
      "wrong game state detected",
    );
  });
  it("president Enact policy", async () => {
    let game = await program.account.gameData.fetch(gameData);
    let key = game.activePlayers[game.currentPresidentIndex];
    // find president player keypair
    president = players.find(
      (player) => player.publicKey.toString() === key.toString(),
    );

    if (host.publicKey.toString() === key.toString()) president = host;

    await airdrop(provider.connection, president.publicKey);

    await program.methods
      .enactPolicy(null)
      .accountsPartial({
        player: president.publicKey,
        gameData,
      })
      .signers([president])
      .rpc({ skipPreflight: true })
      .then(confirmTx);

    let gameAfter = await program.account.gameData.fetch(gameData);
    assert.strictEqual(
      gameAfter.activePlayers.length.toString(),
      "5",
      "expected player count is 5 but got " + gameAfter.activePlayers.length,
    );
    assert.strictEqual(
      Object.keys(gameAfter.gameState)[0].toString(),
      "legislativeChancellor",
      "wrong game state detected",
    );
  });
  it("Chancellor initiate veto", async () => {
    let game = await program.account.gameData.fetch(gameData);
    let key = game.activePlayers[game.currentChancellorIndex];
    // find chancellor player keypair
    chancellor = players.find(
      (player) => player.publicKey.toString() === key.toString(),
    );

    if (host.publicKey.toString() === key.toString()) chancellor = host;

    await airdrop(provider.connection, president.publicKey);

    await program.methods
      .chancellorInitiateVeto()
      .accountsPartial({
        player: chancellor.publicKey,
        gameData,
      })
      .signers([chancellor])
      .rpc({ skipPreflight: true })
      .then(confirmTx);

    let gameAfter = await program.account.gameData.fetch(gameData);
    assert.strictEqual(
      gameAfter.liberalPoliciesEnacted.toString(),
      "1",
      "expected player count is 5 but got " + gameAfter.activePlayers.length,
    );
    assert.strictEqual(
      Object.keys(gameAfter.gameState)[0].toString(),
      "legislativePresidentVeto",
      "wrong game state detected",
    );
  });
  it("accept chancellor veto", async () => {
    let game = await program.account.gameData.fetch(gameData);
    let key = game.activePlayers[game.currentPresidentIndex];
    // find president player keypair
    president = players.find(
      (player) => player.publicKey.toString() === key.toString(),
    );

    if (host.publicKey.toString() === key.toString()) president = host;

    await airdrop(provider.connection, president.publicKey);

    await program.methods
      .presidentAnswerVeto(true)
      .accountsPartial({
        president: president.publicKey,
        gameData,
      })
      .signers([president])
      .rpc({ skipPreflight: true })
      .then(confirmTx);

    let gameAfter = await program.account.gameData.fetch(gameData);
    assert.strictEqual(
      gameAfter.failedElections.toString(),
      "1",
      "expected player count is 1 but got " +
        gameAfter.failedElections.toString(),
    );
    assert.strictEqual(
      Object.keys(gameAfter.gameState)[0].toString(),
      "chancellorNomination",
      "wrong game state detected",
    );
  });
});

const confirmTx = async (signature: string) => {
  const latestBlockhash = await anchor
    .getProvider()
    .connection.getLatestBlockhash();
  await anchor.getProvider().connection.confirmTransaction(
    {
      signature,
      ...latestBlockhash,
    },
    "confirmed",
  );
};
async function airdrop(
  connection: anchor.web3.Connection,
  address: PublicKey,
  amount = 10 * LAMPORTS_PER_SOL,
) {
  await connection.requestAirdrop(address, amount).then(confirmTx);
}
