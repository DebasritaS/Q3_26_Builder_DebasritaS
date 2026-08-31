import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";

import {
  createSignerFromKeypair,
  signerIdentity,
  publicKey,
} from "@metaplex-foundation/umi";

import {
    burn,
  mplCore,
  fetchAsset
  } from "@metaplex-foundation/mpl-core";

import wallet from "/home/debasritasaha/.config/solana/id.json";


const umi = createUmi(
  "https://api.devnet.solana.com"
).use(mplCore());


// Load your wallet
const keypair = umi.eddsa.createKeypairFromSecretKey(
  new Uint8Array(wallet)
);


// Create signer
const signer = createSignerFromKeypair(umi, keypair);


// Tell Umi this wallet is the identity/update authority
umi.use(signerIdentity(signer));


const assetAddress = publicKey(
  "C9ThPjEN1G7qJMhapcLs1jMYxUfqNN855p3EtWXasf59"
);


(async () => {
  try {

   
    // Fetch the complete NFT asset from Solana
    const asset = await fetchAsset(umi, assetAddress);

    console.log("Current owner:", asset.owner);

    // Burn the NFT "Picnic" of generug3.png
    const result = await burn(umi, {
      asset 
      }).sendAndConfirm(umi);


   
    console.log("Asset burned successfully");
    

    

  } catch (e) {
    console.log("Error:", e);
  }
})();

// nft-burn.ts
// signature:   3z3M9Ga8NmnS19KSERaSTcFWt78m6LfaNUqSKQHZXqtKP5UAJjB6hqJjBQ3PnfcvXnKRp1QeyWNQjeTH2NFL4oQu
