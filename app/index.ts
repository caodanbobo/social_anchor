import { createProfile, getProfile } from "./api/profile";
import { getTweet, postTweet, smashLike } from "./api/tweet";
import { createToken } from "./api/token";
import { useDefaultWallet, useVisitorWallet } from "./api/wallet";

(async () => {
  const defaultWallet = useDefaultWallet();
  const visitorWallet = useVisitorWallet();

  const [mint_pda, res] = await createToken(defaultWallet);
  console.log(mint_pda, res);
  const r1 = await createProfile(defaultWallet, "Bob");
  console.log(r1);

  const r2 = await getProfile(defaultWallet);
  console.log(r2);

  const r3 = await createProfile(visitorWallet, "Alice");
  console.log(r3);

  const r4 = await getProfile(visitorWallet);
  console.log(r4);

  const [tweetPda, r5] = await postTweet(
    defaultWallet,
    "bob's 1st post, hahaha"
  );
  console.log(r5);

  const r6 = await getTweet(defaultWallet, tweetPda);
  let author = r6.authorKey;
  console.log(r6);

  const r7 = await smashLike(visitorWallet, tweetPda, author);
  console.log(r7);

  const r8 = await getTweet(defaultWallet, tweetPda);
  console.log(r8);
})();
