import * as anchor from "@coral-xyz/anchor";
import { program } from "./wallet";

export async function createToken(wallet: anchor.Wallet) {
  const [mint_pda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("mint_v1")],
    program.programId
  );
  return [
    mint_pda,
    await program.methods
      .createMint()
      .accounts({
        authority: wallet.publicKey,
      })
      .signers([wallet.payer])
      .rpc(),
  ];
}
