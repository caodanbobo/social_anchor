import * as anchor from "@coral-xyz/anchor";
import { program } from "./wallet";
export async function createProfile(wallet: anchor.Wallet, name: string) {
  return await program.methods
    .initialize(name)
    .accounts({
      authority: wallet.publicKey,
    })
    .signers([wallet.payer])
    .rpc();
}

export async function getProfile(wallet: anchor.Wallet) {
  const [seed] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("profile"), wallet.publicKey.toBuffer()],
    program.programId
  );
  return await program.account.socialProfile.fetch(seed);
}
