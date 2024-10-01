import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { Keypair } from '@solana/web3.js';
import { App } from '../target/types/app';

describe('app', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const payer = provider.wallet as anchor.Wallet;

  const program = anchor.workspace.App as Program<App>;

  const appKeypair = Keypair.generate();

  it('Initialize App', async () => {
    await program.methods
      .initialize()
      .accounts({
        app: appKeypair.publicKey,
        payer: payer.publicKey,
      })
      .signers([appKeypair])
      .rpc();

    const currentCount = await program.account.app.fetch(appKeypair.publicKey);

    expect(currentCount.count).toEqual(0);
  });

  it('Increment App', async () => {
    await program.methods
      .increment()
      .accounts({ app: appKeypair.publicKey })
      .rpc();

    const currentCount = await program.account.app.fetch(appKeypair.publicKey);

    expect(currentCount.count).toEqual(1);
  });

  it('Increment App Again', async () => {
    await program.methods
      .increment()
      .accounts({ app: appKeypair.publicKey })
      .rpc();

    const currentCount = await program.account.app.fetch(appKeypair.publicKey);

    expect(currentCount.count).toEqual(2);
  });

  it('Decrement App', async () => {
    await program.methods
      .decrement()
      .accounts({ app: appKeypair.publicKey })
      .rpc();

    const currentCount = await program.account.app.fetch(appKeypair.publicKey);

    expect(currentCount.count).toEqual(1);
  });

  it('Set app value', async () => {
    await program.methods.set(42).accounts({ app: appKeypair.publicKey }).rpc();

    const currentCount = await program.account.app.fetch(appKeypair.publicKey);

    expect(currentCount.count).toEqual(42);
  });

  it('Set close the app account', async () => {
    await program.methods
      .close()
      .accounts({
        payer: payer.publicKey,
        app: appKeypair.publicKey,
      })
      .rpc();

    // The account should no longer exist, returning null.
    const userAccount = await program.account.app.fetchNullable(
      appKeypair.publicKey
    );
    expect(userAccount).toBeNull();
  });
});
