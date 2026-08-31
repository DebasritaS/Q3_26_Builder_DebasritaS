import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import {
  createGenericFile,
  createSignerFromKeypair,
  signerIdentity,
} from "@metaplex-foundation/umi";
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys";
import { readFile } from "fs/promises";

import wallet from "/home/debasritasaha/.config/solana/id.json";

const umi = createUmi(
  process.env.SOLANA_RPC_URL ?? "https://api.devnet.solana.com",
);

const keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(
  irysUploader({
    address: "https://devnet.irys.xyz/",
  }),
);

umi.use(signerIdentity(signer));

(async () => {
  try {
    //chanege image path to your image path
    const image = await readFile(
  "/mnt/c/Users/DEBASRITA SAHA/Documents/GitHub/Q3_26_Builder_DebasritaS/spl-nft-q326/assets/generug.png"
);

    //change the image name and mime type
    const file = createGenericFile(
  image,
  "generug.png",
  { contentType: "image/png" },
);

     const [myUri] = await umi.uploader.upload([file]);
     console.log("Your image URI: ", myUri); // https://gateway.irys.xyz/4JGp35Yh1hDSgb55HZsqFzBT3bUuw4A1nbvCavDxbr25


  } catch (error) {
    console.log(error);
  }
})();

// nft-image.ts
// Your image URI:  https://gateway.irys.xyz/4JGp35Yh1hDSgb55HZsqFzBT3bUuw4A1nbvCavDxbr25
