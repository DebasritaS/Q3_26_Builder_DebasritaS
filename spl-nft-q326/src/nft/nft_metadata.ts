import {
  createSignerFromKeypair,
  signerIdentity,
} from "@metaplex-foundation/umi";
import wallet from "/home/debasritasaha/.config/solana/id.json";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys";

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
    //change the image uri to your image uri obtained from nft_image.ts
    const image =
      "https://gateway.irys.xyz/4JGp35Yh1hDSgb55HZsqFzBT3bUuw4A1nbvCavDxbr25";

    //json scheme : https://www.metaplex.com/docs/smart-contracts/core/json-schema
    //change the metadata
    // const metadata =
    // const myUri =
    // console.log(`metadata uri: ${myUri} `);


    //Not updated metadata

    //  const metadata = {
    //   "name": "Debrug ",
    //   "description": "This is my first NFT on Solana using Metaplex Umi.",
    //   "image": image,
    //   category: "image"
    //  }
     
    //Updated metadata

     const metadata = {
      "name": "De rug ",
      "description": "Updated metadata for my first NFT on Solana using Metaplex Umi.",
      "image": image,
      category: "image"
     }

      const myUri = await umi.uploader.uploadJson(metadata);

      console.log(`metadata uri: ${myUri} `)


  } catch (error) {
    console.log("error", error);
  }
})();
