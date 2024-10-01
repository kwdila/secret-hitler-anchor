// Here we export some useful types and functions for interacting with the Anchor program.
import { AnchorProvider, Program } from '@coral-xyz/anchor';
import { Cluster, PublicKey } from '@solana/web3.js';
import AppIDL from '../target/idl/app.json';
import type { App } from '../target/types/app';

// Re-export the generated IDL and type
export { App, AppIDL };

// The programId is imported from the program IDL.
export const APP_PROGRAM_ID = new PublicKey(AppIDL.address);

// This is a helper function to get the App Anchor program.
export function getAppProgram(provider: AnchorProvider) {
  return new Program(AppIDL as App, provider);
}

// This is a helper function to get the program ID for the App program depending on the cluster.
export function getAppProgramId(cluster: Cluster) {
  switch (cluster) {
    case 'devnet':
    case 'testnet':
    case 'mainnet-beta':
    default:
      return APP_PROGRAM_ID;
  }
}
