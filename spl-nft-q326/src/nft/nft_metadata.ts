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
      "https://gateway.irys.xyz/4deTo1qh7yrqF2v7zLNUhhJTwarHCFLx8CWFyBDs5hMD";

    //json scheme : https://www.metaplex.com/docs/smart-contracts/core/json-schema
    //change the metadata
    // const metadata =
    // const myUri =
    // console.log(`metadata uri: ${myUri} `);


    //Not updated metadata for generug.png

    //  const metadata = {
    //   "name": "Debrug ",
    //   "description": "This is my first NFT on Solana using Metaplex Umi.",
    //   "image": image,
    //   category: "image"
    //  }
     
    //Updated metadata for generug.png

    //  const metadata = {
    //   "name": "De rug ",
    //   "description": "Updated metadata for my first NFT on Solana using Metaplex Umi.",
    //   "image": image,
    //   category: "image"
    //  }

     //Metadata for generug2.png

    //  const metadata = {
    //   "name": "Purple Dream ",
    //   "description": "Metadata for my second NFT on Solana using Metaplex Umi.",
    //   "image": image,
    //   category: "image"
    //  }

        //Metadata for generug3.png  which will be burned after minting the NFT

     const metadata = {
      "name": "Picnic ",
      "description": "Metadata for my third NFT which will be burned later... on Solana using Metaplex Umi.",
      "image": image,
      category: "image"
     }



      const myUri = await umi.uploader.uploadJson(metadata);

      console.log(`metadata uri: ${myUri} `)


  } catch (error) {
    console.log("error", error);
  }
})();

// nft-metadata.ts generug.png "Debrug"
// metadata uri: https://gateway.irys.xyz/3cdZ1uJ3RofiQd8A4wZKUna2ccMDsDyNW5HcWzRQJ8nq

// updated nft-metadata.ts generug.png  "De rug"
// metadata uri: https://gateway.irys.xyz/759aG2G9uJEosR7hDFeGPRvTLEHS3iec4tBPxxiAb68w

//generug2.png  "Purple Dream"
// metadata uri: https://gateway.irys.xyz/5Uq29GzRa9KBnu6zovtgUCJz781Ew21E2xJTiKAbaNtL 

//generug3.png  "Picnic"
//metadata uri: https://gateway.irys.xyz/HvjJLs2jZwxEyfio5rh2J8zSijJCwMPHAFKCKk88mokn