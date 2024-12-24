import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SocialAnchor } from "../../target/types/social_anchor";

let provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

const program = anchor.workspace.SocialAnchor as Program<SocialAnchor>;
export { program, provider };
export function useDefaultWallet() {
  return anchor.Wallet.local();
}

export function useVisitorWallet() {
  //3bsZA5S7K9vmqDuyVXuemJdLtZBAZDpid42VWX11rqkn
  const keypair = anchor.web3.Keypair.fromSecretKey(
    new Uint8Array([
      53, 193, 178, 189, 68, 97, 40, 239, 127, 215, 9, 22, 175, 157, 62, 181,
      132, 100, 191, 164, 164, 15, 138, 253, 234, 232, 5, 253, 104, 141, 176,
      148, 38, 166, 72, 142, 200, 233, 103, 133, 224, 58, 238, 58, 213, 168, 99,
      61, 110, 6, 238, 103, 133, 109, 83, 174, 10, 181, 5, 93, 50, 120, 192, 83,
    ])
  );
  return new anchor.Wallet(keypair);
}
