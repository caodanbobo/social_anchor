import { createProfile, getProfile } from "./api/profile";
import { useDefaultWallet, useVisitorWallet } from "./api/wallet";

(async () => {
  const defaultWallet = useDefaultWallet();
  const visitorWallet = useVisitorWallet();

  //   const r1 = await createProfile(defaultWallet, "Bob");
  //   console.log(r1);

  const r2 = await getProfile(defaultWallet);
  console.log(r2);

  const r3 = await createProfile(visitorWallet, "Alice");
  console.log(r3);

  const r4 = await getProfile(visitorWallet);
  console.log(r4);
})();
