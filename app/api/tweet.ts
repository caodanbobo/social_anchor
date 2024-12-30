import * as anchor from "@coral-xyz/anchor";
import { program } from "./wallet";
export async function postTweet(
  wallet: anchor.Wallet,
  content: string
): Promise<[anchor.web3.PublicKey, string]> {
  const [profilePda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("profile"), wallet.publicKey.toBuffer()],
    program.programId
  );

  const profile = await program.account.socialProfile.fetch(profilePda);
  const [tweetPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("tweet_v1"),
      profilePda.toBuffer(),
      Buffer.from(`${profile.tweetCount + 1}`),
    ],
    program.programId
  );
  return [
    tweetPda,
    await program.methods
      .postTweet(content)
      .accounts({
        authority: wallet.publicKey,
        socialTweet: tweetPda,
      })
      .signers([wallet.payer])
      .rpc(),
  ];
}

export async function getTweet(
  wallet: anchor.Wallet,
  tweetPaa: anchor.web3.PublicKey
) {
  return await program.account.socialTweet.fetch(tweetPaa);
}

export async function smashLike(
  wallet: anchor.Wallet,
  pda: anchor.web3.PublicKey,
  author_key: anchor.web3.PublicKey
) {
  return await program.methods
    .smashLike()
    .accounts({
      authority: wallet.publicKey,
      socialTweet: pda,
      authorWallet: author_key,
    })
    .signers([wallet.payer])
    .rpc();
}
