import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";

import {
  createSignerFromKeypair,
  signerIdentity,
  publicKey,
} from "@metaplex-foundation/umi";

import {
  update,
  mplCore,
  fetchAsset,
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
  "8vbYaa2Xr9N3cH449iWWWPGshcG9HzJ4ATKgHsDtB23R"
);


(async () => {
  try {

    // 1. Fetch the existing NFT
    const asset = await fetchAsset(
      umi,
      assetAddress
    );
    
    


    console.log("Current NFT:");
    console.log("Name:", asset.name);
    console.log("URI:", asset.uri);


    // 2. Update the NFT
    const result = await update(umi, {
      asset,
      name: "De rug",
      uri: "https://gateway.irys.xyz/759aG2G9uJEosR7hDFeGPRvTLEHS3iec4tBPxxiAb68w",
    }).sendAndConfirm(umi);


   
    console.log("Asset updated successfully");
    

    

  } catch (e) {
    console.log("Error:", e);
  }
})();